#!/usr/bin/env python3
"""
dep_check.py

Dependency checker + optional auto-installer (Chocolatey) for REBIOS toolchain pieces.

Usage:
    python dep_check.py

Notes:
    - Designed for Windows. Some OVMF acquisition uses WSL if available.
    - If you want AutoInstall to function, run as Administrator and have Chocolatey installed.
    - Edit LIMINE_PATH and OVMF_PATH below if you want different locations.
"""

import os
import sys
import subprocess
import shutil
import glob
import zipfile
import urllib.request
import tempfile
import time
from pathlib import Path

# ===== Colorama =====

from colorama import init, Fore, Style
init()

# ===== Configuration =====
BASE_DEPS = Path(__file__).parent.parent / "build_deps"
LIMINE_PATH = BASE_DEPS / "limine"
OVMF_PATH = BASE_DEPS / "OVMF"
LIMINE_GIT_BRANCH = "v10.x-binary"
LIMINE_GIT_REPO = "https://github.com/limine-bootloader/limine.git"
LIMINE_ZIP_URL = f"https://github.com/limine-bootloader/limine/archive/refs/heads/{LIMINE_GIT_BRANCH}.zip"

# ===== Utilities =====
def echo(msg, color=None):
    color_map = {
        "red": Fore.RED,
        "green": Fore.GREEN,
        "yellow": Fore.YELLOW,
        "magenta": Fore.MAGENTA,
        "cyan": Fore.CYAN,
        "gray": Fore.WHITE,
        None: ""
    }
    print(f"{color_map.get(color, '')}{msg}{Style.RESET_ALL}")

def is_windows():
    return os.name == "nt"

def is_admin():
    if not is_windows():
        return False
    try:
        import ctypes
        return ctypes.windll.shell32.IsUserAnAdmin() != 0
    except Exception:
        return False

def run_cmd(cmd, capture=False, shell=False):
    """Run command silently, return (exitcode, stdout_text). Does not raise."""
    try:
        if capture:
            proc = subprocess.run(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                shell=shell,
                check=False
            )
            return proc.returncode, proc.stdout.decode(errors="ignore")
        else:
            proc = subprocess.run(
                cmd,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                shell=shell,
                check=False
            )
            return proc.returncode, None
    except FileNotFoundError:
        return 127, None
    except Exception as e:
        return 1, str(e)

def which(cmd):
    return shutil.which(cmd) is not None

# ===== Checks & installers =====
def check_command_version(cmd_list):
    code, _ = run_cmd(cmd_list, capture=False, shell=isinstance(cmd_list, str))
    return code == 0

def try_choco_install(package):
    if not which("choco"):
        echo("Chocolatey is not in PATH.", "yellow")
        return False
    cmd = ["choco", "install", package, "-y"]
    echo(f"Running: {' '.join(cmd)}", "cyan")
    code, _ = run_cmd(cmd)
    return code == 0

# ===== Limine handling =====
def limine_has_required(path: Path):
    required = ["limine-bios.sys","limine-bios-cd.bin","limine-uefi-cd.bin","BOOTX64.EFI","BOOTIA32.EFI"]
    if not path.exists() or not path.is_dir():
        return False
    for f in required:
        if not (path / f).exists():
            return False
    return True

def install_limine(path: Path):
    path = Path(path)
    try:
        path.mkdir(parents=True, exist_ok=True)
    except Exception as e:
        echo(f"Could not create {path}: {e}", "red")
        return False

    if which("git"):
        echo("Git found — attempting to clone Limine (best effort).", "cyan")
        target = path / "limine-src"
        if target.exists():
            echo(f"Removing previous clone at {target}", "gray")
            shutil.rmtree(target, ignore_errors=True)
        cmd = ["git", "clone", "--branch", LIMINE_GIT_BRANCH, LIMINE_GIT_REPO, str(target)]
        code, out = run_cmd(cmd)
        if code != 0:
            echo("Git clone failed (continuing to try alternate methods).", "yellow")
        else:
            echo("Clone succeeded — searching for prebuilt binaries to copy...", "green")
            copied = 0
            for pattern in ["**/limine-*.sys", "**/limine-*-cd.bin", "**/BOOT*.EFI", "**/*.EFI"]:
                for p in target.glob(pattern):
                    try:
                        shutil.copy(p, path)
                        copied += 1
                    except Exception:
                        pass
            if copied:
                echo(f"Copied {copied} files from repo to {path}", "green")
            else:
                echo("No prebuilt limine binaries found in repo. You may need to build Limine or copy the files manually.", "yellow")
            return True

    echo("Attempting to download Limine archive zip (best-effort).", "cyan")
    try:
        tmp = tempfile.mktemp(suffix=".zip")
        echo(f"Downloading {LIMINE_ZIP_URL} ...", "gray")
        urllib.request.urlretrieve(LIMINE_ZIP_URL, tmp)
        echo("Download complete, extracting...", "gray")
        with zipfile.ZipFile(tmp, "r") as zf:
            ext_dir = path / "limine-src"
            if ext_dir.exists():
                shutil.rmtree(ext_dir, ignore_errors=True)
            zf.extractall(ext_dir)
        copied = 0
        for p in ext_dir.rglob("limine-*"):
            if p.is_file():
                try:
                    shutil.copy(p, path)
                    copied += 1
                except Exception:
                    pass
        if copied:
            echo(f"Copied {copied} files into {path}", "green")
            return True
        else:
            echo("Downloaded source archive but no prebuilt binaries were found. You may need to build limine or copy compiled files into the directory manually.", "yellow")
            return True
    except Exception as e:
        echo(f"Limine download/extract failed: {e}", "red")
        return False

# ===== OVMF handling =====
def ovmf_has_required(path: Path):
    if not path.exists() or not path.is_dir():
        return False
    code_candidates = list(path.glob("OVMF*.fd"))
    var_candidates = list(path.glob("OVMF_VARS*.fd"))
    return bool(code_candidates and var_candidates)

def try_copy_from_wsl(dest: Path):
    if not which("wsl"):
        return False
    echo("WSL exists — attempting to copy OVMF from WSL /usr/share/OVMF ...", "cyan")
    try:
        code_check = run_cmd('wsl bash -lc "test -f /usr/share/OVMF/OVMF_CODE.fd && echo OK"', capture=True, shell=True)
        if code_check[0] != 0 or "OK" not in (code_check[1] or ""):
            echo("OVMF files not found in WSL /usr/share/OVMF.", "yellow")
            return False
        dest_win = str(dest.resolve())
        dest_win_wsl = "/mnt/" + dest_win[0].lower() + dest_win[2:].replace("\\", "/")
        dest.mkdir(parents=True, exist_ok=True)
        run_cmd(f'wsl bash -lc "cp /usr/share/OVMF/OVMF_CODE.fd {dest_win_wsl}/"', shell=True)
        run_cmd(f'wsl bash -lc "cp /usr/share/OVMF/OVMF_VARS.fd {dest_win_wsl}/"', shell=True)
        code_fd = dest / "OVMF_CODE.fd"
        if code_fd.exists():
            try:
                (dest / "OVMF.fd").unlink(missing_ok=True)
                code_fd.rename(dest / "OVMF.fd")
            except Exception:
                pass
        echo(f"Copied OVMF files to {dest}", "green")
        return True
    except Exception as e:
        echo(f"WSL copy failed: {e}", "red")
        return False

def try_find_in_common_paths(dest: Path):
    possible = [
        Path(os.environ.get("ProgramFiles", "")) / "qemu",
        Path(os.environ.get("ProgramFiles(x86)", "")) / "qemu",
        Path("C:/msys64/usr/share/OVMF"),
        Path("C:/msys64/usr/share/ovmf"),
        Path("C:/Program Files/qemu/share/ovmf"),
        Path("C:/Program Files (x86)/qemu/share/ovmf"),
    ]
    for p in possible:
        try:
            if p.exists():
                code_files = list(p.rglob("OVMF_CODE*.fd"))
                vars_files = list(p.rglob("OVMF_VARS*.fd"))
                if code_files and vars_files:
                    dest.mkdir(parents=True, exist_ok=True)
                    try:
                        shutil.copy(code_files[0], dest)
                        shutil.copy(vars_files[0], dest)
                    except Exception:
                        pass
                    for f in dest.glob("OVMF_CODE*.fd"):
                        try:
                            (dest / "OVMF.fd").unlink(missing_ok=True)
                            f.rename(dest / "OVMF.fd")
                        except Exception:
                            pass
                    echo(f"Copied OVMF files from {p} -> {dest}", "green")
                    return True
        except Exception:
            pass
    return False

def install_ovmf(dest: Path):
    dest = Path(dest)
    dest.mkdir(parents=True, exist_ok=True)
    if try_copy_from_wsl(dest):
        return True
    if try_find_in_common_paths(dest):
        return True
    echo("Automatic OVMF acquisition failed. Please place OVMF_CODE*.fd and OVMF_VARS*.fd into: " + str(dest), "yellow")
    echo("Common sources: installed QEMU packages, WSL distros (/usr/share/OVMF), or build OVMF from EDK2 upstream.", "gray")
    return False

# ===== Main driver =====
def main():
    echo("\n\n" + "*"*80, "magenta")
    echo("Dependency checker — attempting to be useful while you burn the midnight oil.", "magenta")
    echo("*"*80 + "\n", "magenta")

    admin = is_admin()
    echo(f"Running as Administrator: {'YES' if admin else 'NO (AutoInstall disabled)'}", "green" if admin else "yellow")

    choco_available = which("choco")
    if choco_available:
        code, out = run_cmd(["choco", "-v"], capture=True)
        if code == 0:
            echo(f"Chocolatey detected: {out.strip().splitlines()[0] if out else 'version unknown'}", "green")
        else:
            echo("Chocolatey present but did not respond correctly.", "yellow")
            choco_available = False
    else:
        echo("Chocolatey not found in PATH.", "yellow")

    auto_install = admin and choco_available
    echo(f"AutoInstall is {'ENABLED' if auto_install else 'DISABLED'}.", "green" if auto_install else "yellow")

    deps = [
        {"name": "Rust (rustc)", "check": lambda: check_command_version(["rustc", "--version"]), "install": lambda: try_choco_install("rust"), "hint": "choco install rust -y"},
        {"name": "Cargo", "check": lambda: check_command_version(["cargo", "--version"]), "install": lambda: try_choco_install("rust"), "hint": "cargo comes with rust; choco install rust -y"},
        {"name": "mkisofs (cdrtools)", "check": lambda: check_command_version(["mkisofs", "-version"]), "install": lambda: try_choco_install("cdrtools"), "hint": "choco install cdrtools -y"},
        {"name": "QEMU (qemu-system-x86_64)", "check": lambda: check_command_version(["qemu-system-x86_64", "--version"]), "install": lambda: try_choco_install("qemu"), "hint": "choco install qemu -y"},
        {"name": "MSYS2", "check": lambda: check_command_version(["msys2", "-v"]), "install": lambda: try_choco_install("msys2"), "hint": "choco install msys2 -y"},
        {"name": "Limine (limine-bios.sys etc.)", "check": lambda: limine_has_required(LIMINE_PATH), "install": lambda: install_limine(LIMINE_PATH), "hint": f"Clone {LIMINE_GIT_REPO} ({LIMINE_GIT_BRANCH}) and copy limine-bios.sys, limine-*-cd.bin, BOOT*.EFI into {LIMINE_PATH}"},
        {"name": "OVMF (OVMF*.fd + OVMF_VARS*.fd)", "check": lambda: ovmf_has_required(OVMF_PATH), "install": lambda: install_ovmf(OVMF_PATH), "hint": f"Copy OVMF_CODE*.fd and OVMF_VARS*.fd into {OVMF_PATH} (WSL: sudo cp /usr/share/OVMF/* /mnt/c/OVMF/)"}
    ]

    echo("\n=== Dependency Check ===\n", "gray")
    for dep in deps:
        name = dep["name"]
        ok = False
        try:
            ok = dep["check"]()
        except Exception as e:
            echo(f"[ERROR during check] {e}", "red")
            ok = False

        if ok:
            echo(f"[OK] {name}", "green")
            continue

        echo(f"[MISSING] {name}", "red")
        if auto_install and callable(dep.get("install")):
            echo(f"[INSTALLING] {name} ...", "yellow")
            try:
                installed = dep["install"]()
            except Exception as e:
                echo(f"[INSTALL ERROR] {e}", "red")
                installed = False
            time.sleep(1)
            try:
                ok_after = dep["check"]()
            except Exception:
                ok_after = False

            if ok_after:
                echo(f"[INSTALLED] {name}", "magenta")
            else:
                echo(f"[INSTALL FAILED] {name} — automatic install couldn't verify presence.", "red")
                echo("Hint: " + dep.get("hint", "No hint available."), "gray")
        else:
            echo("Auto-install unavailable or disabled.", "yellow")
            echo("Hint: " + dep.get("hint", "No hint available."), "gray")

    echo("\n=== Done ===\n", "gray")
    if sys.stdin.isatty():
        try:
            input("Press Enter to exit...")
        except Exception:
            pass

if __name__ == "__main__":
    main()

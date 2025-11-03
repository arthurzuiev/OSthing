#!/usr/bin/env python3
"""
env_check.py

Environment checker + auto-fix for Rust toolchain components, Limine, and OVMF paths.

Usage:
    python env_check.py
"""

import os
import subprocess
import ctypes
from pathlib import Path
from colorama import init, Fore, Style

init()

# ===== Colors =====
def echo(msg, color=None):
    color_map = {
        "red": Fore.RED,
        "green": Fore.GREEN,
        "yellow": Fore.YELLOW,
        "magenta": Fore.MAGENTA,
        "cyan": Fore.CYAN,
        "gray": Fore.LIGHTBLACK_EX,
        None: ""
    }
    print(f"{color_map.get(color, '')}{msg}{Style.RESET_ALL}")

# ===== Admin check =====
def is_admin():
    try:
        return ctypes.windll.shell32.IsUserAnAdmin() != 0
    except Exception:
        return False

ADMIN = is_admin()
if not ADMIN:
    echo("[WARNING] Not running as Administrator — permanent env changes will not be applied.", "yellow")

# ===== Paths =====
BASE_DEPS = Path(__file__).parent.parent / "build_deps"
LIMINE_PATH = BASE_DEPS / "limine"
OVMF_PATH = BASE_DEPS / "OVMF"

# ===== Helper to run shell commands =====
def run(cmd, capture=True):
    try:
        result = subprocess.run(
            cmd, 
            shell=True, 
            stdout=subprocess.PIPE if capture else None, 
            stderr=subprocess.STDOUT if capture else None
        )
        out = result.stdout.decode().strip() if capture and result.stdout else ""
        return result.returncode, out
    except Exception as e:
        return 1, str(e)
    
def set_user_env_var(name, value):
    """
    Update current process env and attempt to set the user-level env var via PowerShell.
    Returns (success: bool, message: str).
    """
    # Always update the current process so this script sees the variable immediately
    os.environ[name] = value

    # If not admin, do not attempt permanent change
    if not ADMIN:
        return False, f"Set in current session only: {name}={value}. (Permanent change skipped: Admin required.)"

    # Build PowerShell command to set user-level environment variable
    # Use [System.Environment]::SetEnvironmentVariable('NAME','VALUE','User')
    ps_cmd = (
        f"[System.Environment]::SetEnvironmentVariable('{name}', '{value}', 'Machine')"
    )


    # Run PowerShell with minimal profile and no interactive prompts
    try:
        proc = subprocess.run(
            ["powershell", "-NoProfile", "-NonInteractive", "-Command", ps_cmd],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
        )
    except FileNotFoundError:
        return False, "PowerShell not found on PATH; cannot set env var permanently."
    except Exception as e:
        return False, f"Failed to invoke PowerShell: {e}"

    if proc.returncode != 0:
        msg = proc.stderr.strip() or proc.stdout.strip() or f"Exit code {proc.returncode}"
        return False, f"PowerShell failed to set {name}: {msg}"

    # Success — note that other processes (Explorer) won't see it until they reload env
    return True, f"{name} set to {value} (permanent for user). You may need to restart Explorer or log out/in for other processes to see it."


# ===== Environment checks & fixes =====
env_checks = {
    "Rust Src Component": {
        "check": lambda: "installed" in run("rustup component list | findstr rust-src")[1],
        "fix": lambda: run("rustup component add rust-src")
    },
    "LLVM Tools": {
        "check": lambda: any(x in run("rustup component list | findstr llvm-tools")[1] for x in ["default", "installed"]),
        "fix": lambda: run("rustup component add llvm-tools")
    },
    "Bootimage": {
        "check": lambda: run("cargo bootimage --version")[0] == 0,
        "fix": lambda: run("cargo install bootimage")
    },
    "Nightly Installed": {
        "check": lambda: "nightly" in run("rustup toolchain list")[1],
        "fix": lambda: run("rustup toolchain install nightly")
    },
    "Nightly Default": {
        "check": lambda: "nightly" in run("rustup show active-toolchain")[1],
        "fix": lambda: run("rustup default nightly")
    },
    "Limine EnvVar": {
        "check": lambda: os.environ.get("LIMINE_PATH") == str(LIMINE_PATH),
        "fix": lambda: set_user_env_var("LIMINE_PATH", str(LIMINE_PATH))
    },
    "OVMF EnvVar": {
        "check": lambda: os.environ.get("OVMF_PATH") == str(OVMF_PATH),
        "fix": lambda: set_user_env_var("OVMF_PATH", str(OVMF_PATH))
    }
}

# ===== Driver =====
def main():
    echo("\n=== Environment Check ===\n", "gray")
    for name, cfg in env_checks.items():
        try:
            ok = cfg["check"]()
        except Exception as e:
            echo(f"[ERROR during check] {name}: {e}", "red")
            ok = False

        if ok:
            echo(f"[OK] {name}", "green")
        else:
            echo(f"[MISSING] {name}", "yellow")
            if "fix" in cfg and callable(cfg["fix"]):
                echo(f"[FIXING] {name} ...", "magenta")
                ret, out = cfg["fix"]()  # capture both status and message
                echo(out, "cyan")
                # recheck
                try:
                    recheck = cfg["check"]()
                except Exception:
                    recheck = False
                if recheck:
                    echo(f"[FIXED] {name}", "green")
                else:
                    echo(f"[FAILED TO FIX] {name}", "red")
            else:
                echo(f"[NO FIX AVAILABLE] {name}", "red")

    echo("\n=== Environment Check Complete ===\n", "gray")


if __name__ == "__main__":
    main()

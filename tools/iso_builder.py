#!/usr/bin/env python3
import os
import sys
from pathlib import Path
import subprocess
from shutil import copy2

def log(msg):
    print(f"[LOG] {msg}")

def error(msg):
    print(f"[ERROR] {msg}", file=sys.stderr)

def ensure_copy(src: Path, dst: Path):
    log(f"Copying {src} → {dst}")
    dst.parent.mkdir(parents=True, exist_ok=True)
    copy2(src, dst)
    log("Copying done.")

def main():
    # env / args
    OUT_DIR = Path(os.environ.get("OUT_DIR", "build"))
    RUNNER_DIR = Path(os.environ.get("CARGO_MANIFEST_DIR", "."))
    LIMINE_PATH = Path(os.environ.get("LIMINE_PATH", RUNNER_DIR / "build_deps/limine"))

    # take kernel path from first arg if provided
    if len(sys.argv) > 1:
        KERNEL_PATH = Path(sys.argv[1])
    else:
        KERNEL_PATH = Path(os.environ.get("KERNEL_PATH", RUNNER_DIR / "target/kernel.elf"))

    iso_root = OUT_DIR / "iso_root"
    iso_boot = iso_root / "boot"
    iso_efi = iso_root / "EFI/BOOT"

    # create dirs
    iso_root.mkdir(parents=True, exist_ok=True)
    iso_boot.mkdir(parents=True, exist_ok=True)
    iso_efi.mkdir(parents=True, exist_ok=True)

    # limine.conf
    ensure_copy(RUNNER_DIR / "limine.conf", iso_root / "limine.conf")

    # kernel
    ensure_copy(KERNEL_PATH, iso_boot / "kernel.elf")

    # Limine files
    for file in ["limine-bios.sys", "limine-bios-cd.bin", "limine-uefi-cd.bin",
                 "BOOTX64.EFI", "BOOTIA32.EFI"]:
        src = LIMINE_PATH / file
        if file.lower().endswith(".efi"):
            dst = iso_efi / file
        else:
            dst = iso_boot / "limine" / file
            dst.parent.mkdir(parents=True, exist_ok=True)
        ensure_copy(src, dst)

    output_iso = OUT_DIR / "os.iso"
    log(f"Building ISO at {output_iso}")

    # xorriso through WSL if Windows
    if os.name == "nt":
        cmd = [
            "wsl", "xorriso", "-as", "mkisofs",
            "--follow-links",
            "-b", "boot/limine/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size", "4",
            "-boot-info-table",
            "--efi-boot", "boot/limine/limine-uefi-cd.bin",
            "--efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            iso_root.as_posix(),
            "-o", output_iso.as_posix()
        ]
    else:
        cmd = [
            "xorriso", "-as", "mkisofs",
            "--follow-links",
            "-b", "boot/limine/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size", "4",
            "-boot-info-table",
            "--efi-boot", "boot/limine/limine-uefi-cd.bin",
            "--efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            str(iso_root),
            "-o", str(output_iso)
        ]

    subprocess.run(cmd, check=True)

    # Limine BIOS install
    LIMINE_EXE = LIMINE_PATH / "limine-src" / "limine.exe"
    subprocess.run([str(LIMINE_EXE), "bios-install", str(output_iso)], check=True)

    log(f"ISO created at {output_iso}")

if __name__ == "__main__":
    main()

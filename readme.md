REUEFI
Rust Eats UEFI (REUEFI) is a project on coding an opearting system without too much C. Will be heavily based off rust and Assembly.

Requirements
512MB of RAM
At least a 327MHz CPU
700MB of Space
UEFI support in BIOS

DEV - Requirements
- [Rust](<https://rust-lang.org/tools/install/>)
- wsl (if on windows)

Tools
- iso_creator_new.py reates .iso image from .efi file provided. (install requirements.txt first)

Notes:
- UAGoose: Ive managed to boot the resolting .efi file in QEMY (on wsl) by using iso-like folder structure (you can see it in my-uefi-app/esp folder) and I also booted it on my own laptop (physical) by putting .iso i created with iso_creator_new.py on my USB using tool I found "Rufus".
- UAGoose: BUT no VM was able to boot the file from .iso image (nor VBox nor QEMU), QEMU was able to boot it only through raw file.
- UAGoose: also... qemu_boot_requirements are stuff u need to boot qemu in UEFI mode.
import os
import shutil
import tempfile
from pathlib import Path
import pycdlib

def add_folder_to_iso(iso, folder_path, iso_base_path="/"):
    """Recursively add a folder to the ISO."""
    for item in folder_path.iterdir():
        if item.is_dir():
            # Add directory
            dir_iso_path = iso_base_path + item.name.upper()
            iso.add_directory(dir_iso_path, rr_name=item.name)
            # Recurse
            add_folder_to_iso(iso, item, dir_iso_path + '/')
        else:
            # Add file
            iso_file_path = iso_base_path + item.name.upper() + ";1"
            iso.add_file(str(item), iso_path=iso_file_path, rr_name=item.name)

def create_efi_system_partition(efi_file_path):
    tmp_dir = Path(tempfile.mkdtemp())
    esp_dir = tmp_dir / "EFI" / "BOOT"
    esp_dir.mkdir(parents=True, exist_ok=True)

    # Copy EFI file
    bootx64_path = esp_dir / "BOOTX64.EFI"
    shutil.copy(efi_file_path, bootx64_path)

    # Create ISO with Rock Ridge enabled
    iso_path = Path.cwd() / "MyUEFI.iso"
    iso = pycdlib.PyCdlib()
    iso.new(interchange_level=3, vol_ident="MY_UEFI_APP", rock_ridge='1.09')  # <-- enable RR

    # Recursively add the temp folder
    add_folder_to_iso(iso, tmp_dir)

    # Add EFI boot entry
    iso.add_eltorito(bootfile_path='/EFI/BOOT/BOOTX64.EFI;1', platform_id=0xEF)


    iso.write(str(iso_path))
    iso.close()
    shutil.rmtree(tmp_dir)

# Usage
efi_file_path = r"C:\desctop_folders\Projects\Perosnal\Rust_Eats_UEFI\my-uefi-app\target\x86_64-unknown-uefi\debug\my-uefi-app.efi"
create_efi_system_partition(efi_file_path)

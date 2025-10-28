@echo off
setlocal enabledelayedexpansion

echo === Ultimate Rust UEFI ISO Structure Organizer ===

REM Ask for EFI file path
set /p EFIPATH="Enter full path to your .efi file: "
if not exist "%EFIPATH%" (
    echo File not found: %EFIPATH%
    pause
    exit /b
)

REM Set ISO root folder in project
set ISODIR=%~dp0iso
set EFIBOOT=%ISODIR%\EFI\BOOT

REM Make folders if they don't exist
if not exist "%EFIBOOT%" (
    mkdir "%EFIBOOT%"
    echo Created folder: %EFIBOOT%
)

REM Copy and rename .efi to BOOTX64.EFI
copy /Y "%EFIPATH%" "%EFIBOOT%\BOOTX64.EFI"
if errorlevel 1 (
    echo Failed to copy EFI file!
    pause
    exit /b
)
echo Copied EFI file to %EFIBOOT%\BOOTX64.EFI

pause

@echo off

REM for some stuff later
set CALLER_DIR=%CD%

echo [TOOL] Build and Run BIOS Bootable Image

call _formatting\separator.bat

echo [TOOL] Building...

REM first we go into RUST template directory so Cargo.toml is visible to build tools
cd OS

REM then we build
@echo on
call buildtools\build.bat
call buildtools\build_bootable.bat
@echo off

REM go back to project root
cd ..

REM === Construct the absolute path to the bootable binary ===
set RELATIVE_BIN_PATH=.\OS\target\x86_64-os_target\debug\bootimage-OS.bin
set ABSOLUTE_PATH_TO_BIN=%CALLER_DIR%\%RELATIVE_BIN_PATH%
REM Optional: normalize slashes (Windows usually fine)
REM set ABSOLUTE_PATH_TO_BIN=%ABSOLUTE_PATH_TO_BIN:/=\%

REM Check if file exists
if not exist "%ABSOLUTE_PATH_TO_BIN%" (
    call _formatting\error.bat "Binary not found at %ABSOLUTE_PATH_TO_BIN%"
    echo Consider checking the relative path set in this .bat file, folder names may have changed | OR build was unsuccessful. | OR you runned this file not from project root, moron.
    pause
    exit /b 1
)

echo [TOOL] Bootable binary located at "%ABSOLUTE_PATH_TO_BIN%"

REM now launch QEMU with the binary
tools\qemu_boot_bare.bat "%ABSOLUTE_PATH_TO_BIN%"

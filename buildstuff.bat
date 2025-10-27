@echo off
REM === Configure EDK2 environment ===
SET EDK2_ROOT=C:\EDK2
SET HELLO_DIR=C:\desctop_folders\Projects\Perosnal\Baremetal_Eats_MicroPython\hello_uefi

REM Run edksetup to set PATH for this session
CALL "%EDK2_ROOT%\edksetup.bat" Rebuild

REM Explicitly add BaseTools BinWrappers to PATH
SET PATH=%EDK2_ROOT%\BaseTools\BinWrappers\WindowsLike;%PATH%

REM Make sure Python is available
SET PYTHON_COMMAND=py -3

REM Build the HelloWorld EFI
build -a X64 -t VS2022 -p "%HELLO_DIR%\HelloWorldApp.inf" -b DEBUG

PAUSE

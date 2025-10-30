@echo off

REM for python
echo [TOOL] Ensuring Tool Dependencies

REM ensure python is installed
where python >nul 2>&1
if %errorlevel% neq 0 (
    call tools\_formatting\error.bat "Python is not installed. Please install Python to use this tool."
    exit /b 1
)

REM install required python packages

call tools/_formatting/separator.bat

echo [TOOL] Ensuring required python packages are installed
@echo on
pip install -r tools/requirements.txt
@echo off

REM for wsl
call tools\_formatting\separator.bat

echo [TOOL] Ensuring WSL is installed

where wsl >nul 2>&1
if %errorlevel% neq 0 (
    call tools\_formatting\error.bat "WSL is not installed. Please install WSL to use this tool."
    exit /b 1
)

echo [TOOL] Checking WSL Dependencies

REM Check if qemu-system-x86 is installed
wsl bash -c "dpkg -s qemu-system-x86 >/dev/null 2>&1"
if %errorlevel% neq 0 (
    set INSTALL_QEMU=1
) else (
    set INSTALL_QEMU=0
)

REM Check if ovmf is installed
wsl bash -c "dpkg -s ovmf >/dev/null 2>&1"
if %errorlevel% neq 0 (
    set INSTALL_OVMF=1
) else (
    set INSTALL_OVMF=0
)

REM Only ask for sudo if either is missing
if %INSTALL_QEMU%==1 (
    set INSTALL_ANY=1
)
if %INSTALL_OVMF%==1 (
    set INSTALL_ANY=1
)

if defined INSTALL_ANY (
    call tools\_formatting\separator.bat
    call tools\_formatting\warning.bat "Installing required dependencies in WSL enviornemnt (wsl may ask for password as it uses sudo commands)"
    @echo on
    wsl bash -c "sudo apt update && sudo apt install qemu-system-x86 ovmf -y"
    @echo off
)

echo [TOOL] WSL Dependencies are Satisfied

REM installing build dependencies for Rust stuff

call tools\_formatting\separator.bat

echo [TOOL] Installing build dependencies for Rust tools
cd os_thing
@echo on
call .\buildtools\install_deps.bat
@echo off
cd ..
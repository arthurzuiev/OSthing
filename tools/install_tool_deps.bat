@echo off

REM for python
echo [TOOL] Installing Tool Dependencies

REM ensure python is installed
where python >nul 2>&1
if %errorlevel% neq 0 (
    call tools\_formatting\error.bat "Python is not installed. Please install Python to use this tool."
    exit /b 1
)

REM install required python packages

call tools/_formatting/separator.bat

echo [TOOL] Installing required python packages
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

REM wsl stuff
call tools\_formatting\warning.bat "Ensuring required dependencies in WSL enviornemnt (wsk may ask for password as it uses sudo commands)"

call tools\_formatting\separator.bat

echo [TOOL][SUDO COMMAND]
@echo on
wsl bash -c "sudo apt update && sudo apt install qemu-system-x86 ovmf -y"
@echo off

REM installing build dependencies for Rust stuff

call tools\_formatting\separator.bat

echo [TOOL] Installing build dependencies for Rust tools
@echo on
cd OS
call .\buildtools\install_deps.bat
cd ..
@echo off
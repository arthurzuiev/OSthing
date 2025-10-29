@echo off

echo [TOOL] Qemu Bare-Boot

REM get path to bin from user
set /p BIN_PATH=Enter the absolute path to the binary image: 

REM check if path is correct if not exit
if not exist "%BIN_PATH%" (
    call tools\_formatting\error.bat "The specified path does not exist."
    exit /b 1
)

REM convert windows path to format wsl will understand
for /f "usebackq tokens=*" %%i in (`wsl wslpath "%BIN_PATH%"`) do set BIN_PATH=%%i


echo [TOOL] Ensuring dependencies...
call tools/install_tool_deps.bat


call tools\_formatting\separator.bat

REM run qemu with the provided binary
echo [TOOL] Launching Qemu with the provided binary
wsl qemu-system-x86_64 -drive format=raw,file=%BIN_PATH%
pause
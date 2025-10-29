@echo off

echo [TOOL] Qemu Bare-Boot

REM check if there is an argument provided
if "%~1" neq "" (
    REM use the provided argument as path to bin
    set BIN_PATH=%~1
) else (
    REM no argument provided, we ask user for path
    REM get path to bin from user
    set /p BIN_PATH=Enter the absolute path to the binary image: 
)

REM check if path is correct if not exit
if not exist "%BIN_PATH%" (
    REM print path for debug and the error too
    call tools\_formatting\error.bat "The specified path does not exist."
    echo Provided path: %BIN_PATH%
    pause
    exit /b 1
)

REM convert windows path to format wsl will understand
for /f "usebackq tokens=*" %%i in (`wsl wslpath "%BIN_PATH%"`) do set BIN_PATH=%%i

call tools/install_tool_deps.bat


call tools\_formatting\separator.bat

REM run qemu with the provided binary
echo [TOOL] Launching Qemu with provided binary
wsl qemu-system-x86_64 -drive format=raw,file=%BIN_PATH%
pause
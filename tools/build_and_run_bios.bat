@echo off

REM for some stuff later
set CALLER_DIR=%CD%

echo [TOOL] Build and Run BIOS Bootable Image
call tools\_formatting\separator.bat

REM Check if -test is passed first
if "%1"=="-test" (
    echo [TOOL] Ensuring Build Requirements
    cd os_thing
    call buildtools\install_deps.bat
    cd ..
    call tools\_formatting\separator.bat
    echo [TOOL] Running tests via WSL...
    cd os_thing

    if "%2"=="-c" (
        cargo clean
    ) else if not "%2"=="" (
        REM Run a specific test target
        wsl bash -l -c "cargo test --test %2"
        cd ..
        exit /b
    )

    REM Default: run all tests
    wsl bash -l -c "cargo test"
    cd ..
    exit /b
)

REM Default behavior: build only
set BUILD_ONLY=1
set CLEAN_BUILD=0

REM Check arguments for -c and -bo
if "%1"=="-c" (
    set CLEAN_BUILD=1
    set BUILD_ONLY=0
) else if "%1"=="-bo" (
    set BUILD_ONLY=1
    set CLEAN_BUILD=0
)

echo [TOOL] Building...

REM go into Rust template directory
cd os_thing

REM then we build
if %CLEAN_BUILD%==1 (
    @echo on
    call .\buildtools\build.bat -c
    @echo off
) else (
    @echo on
    call .\buildtools\build.bat -bo
    @echo off
)

call .\buildtools\build_bootable.bat

REM go back to project root
cd ..

call tools\_formatting\separator.bat

REM === Construct the absolute path to the bootable binary ===
set RELATIVE_BIN_PATH=.\os_thing\target\x86_64-os_target\debug\bootimage-os_thing.bin
set ABSOLUTE_PATH_TO_BIN=%CALLER_DIR%\%RELATIVE_BIN_PATH%

REM Check if file exists
if not exist "%ABSOLUTE_PATH_TO_BIN%" (
    call tools\_formatting\error.bat "Binary not found at %ABSOLUTE_PATH_TO_BIN%"
    echo Consider checking the relative path set in this .bat file, folder names may have changed ^| OR build was unsuccessful. ^| OR you ran this file not from project root, moron.
    pause
    exit /b 1
)

echo [TOOL] Bootable binary located at "%ABSOLUTE_PATH_TO_BIN%"

REM launch QEMU with the binary
tools\qemu_boot_bare.bat "%ABSOLUTE_PATH_TO_BIN%"

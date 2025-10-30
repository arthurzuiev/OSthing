@echo off
REM Build script for OS project

REM Default behavior: build only
set BUILD_ONLY=1
set CLEAN_BUILD=0

set CARGO_MANIFEST_DIR=%CD%\

REM Check arguments
if "%1"=="-c" (
    set CLEAN_BUILD=1
    set BUILD_ONLY=0
) else if "%1"=="-bo" (
    set BUILD_ONLY=1
    set CLEAN_BUILD=0
)

REM Perform actions
if %CLEAN_BUILD%==1 (
    echo [TOOL] Performing clean build...
    cargo clean
    cargo build
) else if %BUILD_ONLY%==1 (
    echo [TOOL] Performing build only...
    cargo build
) else (
    echo [TOOL] Unknown argument %1. Use -c for clean build, -bo for build only.
)

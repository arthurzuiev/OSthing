@echo off 
REM simple... but first for safety deps first
echo [BUILD TOOL] Ensuring requiremenets for compiling bootloader...
call buildtools\install_deps.bat
echo [BUILD TOOL] Compiling bootloader:
cargo bootimage
REM lol
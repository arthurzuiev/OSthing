@echo off
REM installs rust dependency stuff we need for building the os
@echo on
cargo install bootimage
rustup component add llvm-tools-preview
@echo off
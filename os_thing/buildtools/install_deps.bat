@echo off
REM installs rust dependency stuff we need for building the os
echo [BUILD TOOL] Ensuring Rust Components
cargo install bootimage
rustup component add llvm-tools-preview

REM install stuff for wsl

echo [BUILD TOOL] Ensuring Rust Nightly Toolchain and Components in WSL

REM === Install rustup if missing (safe to rerun) ===
wsl bash -l -c "curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y"

REM === Update toolchains ===
wsl bash -l -c "~/.cargo/bin/rustup update"

REM === Make sure nightly exists ===
wsl bash -l -c "~/.cargo/bin/rustup toolchain install nightly --profile default"

REM === Force-install essential components ===
wsl bash -l -c "~/.cargo/bin/rustup component add rust-src --toolchain nightly || true"
wsl bash -l -c "~/.cargo/bin/rustup component add llvm-tools-preview --toolchain nightly || true"
wsl bash -l -c "~/.cargo/bin/rustup component add rustfmt --toolchain nightly || true"
wsl bash -l -c "~/.cargo/bin/rustup component add clippy --toolchain nightly || true"

REM === Manually nudge LLVM just in case ===
wsl bash -l -c "if [ ! -d ~/.rustup/toolchains/nightly-*/lib/rustlib/*/bin ]; then ~/.cargo/bin/rustup component add llvm-tools-preview --toolchain nightly; fi"

REM === Install cargo tools ===
wsl bash -l -c "~/.cargo/bin/cargo install bootimage --locked"

REM === Set nightly as default ===
wsl bash -l -c "~/.cargo/bin/rustup default nightly"

echo [BUILD TOOL] Requirements should be satisfied
@echo off

REM UPDATE 1
REM compile it for the windows system... but we want freestanfing bin
REM its just here to show some linker arguymnets :D dont worry abt it and absolutly ignore it
REM cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

REM UPDATE 2
REM now most of build info is inside:
REM ./.cargo/config.toml
REM ./x86_64-os_target.json
REM ./Cargo.toml
REM so this is all we need:
cargo clean
cargo build
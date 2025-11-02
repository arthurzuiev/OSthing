# REBIOS
REBIOS or REBI OS (Rust Eats BIOS OS)

## DEV - Requirements
- [Rust](https://rust-lang.org/tools/install/) (required for the project to work)
- [MSYS2](https://www.msys2.org/)
- QEMU - install using the MSYS2 terminal: `pacman -S mingw-w64-x86_64-qemu` or via Chocolatey

## DEV - Useful Info
- [UEFI Rust basics](https://rust-osdev.github.io/uefi-rs/tutorial/app.html)
- [BIOS Rust basics](https://os.phil-opp.com/)

## Tools
- **NOTE:** Tools are located in the `.\tools\` folder.
- **NOTE:** Tools prefixed with `_` are internal tools (meant to be used by other more abstract tools), e.g., `.\tools\_check_deps.ps1`.
- `setup.bat` - Checks your dependencies and environment. It is recommended to run it **as administrator** to allow automatic installation of some dependencies (optional, though).

## Build, Test, and Run
- **NOTE:** Perform these actions from the `.\os_thing` directory, so tools can locate the `Cargo.toml` file.
- **NOTE:** Test output is directed to the terminal through serial. To create a test case, create a file `<test_name>.rs` in the `.\os_thing\tests` folder. Ensure it defines a `_start` function and annotate test functions with `#[test_case]`. It is recommended to define a local `#[panic_handler]` (using `os_thing::test_panic_handler`). For "should panic" tests, define your own `test_runner` function. See examples in `.\os_thing\tests\basic_boot.rs` and `.\os_thing\tests\should_panic.rs`.
- **TO BUILD:** Run `cargo build` (just builds the project).
- **TO GET BIN:** Run `cargo bootimage` (generates a `.bin` file that can be used to create an image or run in QEMU; it will provide the path to the file).
- **TO RUN:** Run `cargo run`.
- **TO TEST:** Run `cargo test`. To test a specific test case (from `.\os_thing\tests`), add the file name as the `--test` argument. Example:  
  ```powershell
  cargo test --test basic_boot.rs

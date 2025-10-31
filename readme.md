## DEV - Requirements
- [Rust](<https://rust-lang.org/tools/install/>) (for it to work lol)
- [MSYS2](<https://www.msys2.org/>)
- QEMU - install by using MSYS2 terminal. `pacman -S mingw-w64-x86_64-qemu` | or use choco

## DEV - useful info
- [UEFI Rust basics](<https://rust-osdev.github.io/uefi-rs/tutorial/app.html>)
- [BIOS Rust basics](<https://os.phil-opp.com/>)

## Tools
- NOTE: Tools are located in `.\tools\` folder.
- NOTE: Tools prefixed with `_` are internal tools (they are meant to be used by other more abstract tools) (e.g. `.\tools\_check_deps.ps1`)
- `setup.bat` - Checks you dependencies and environment. It is recomended to run it "as administrator" to give it ability to install some dependencies automatically (it is totally optional tho)

## Build, Test and Run
- NOTE: Make sure when you ready to perform these actions you do them from `.\os_thing` directory. As tools that build, test and run need to see the `Cargo.toml` file that is located in there.
- NOTE: Test output is directed into terminal through serial. | to make test case create file `<test_name>.rs` in `.\os_thing\tests` folder. Make sure it has correctly defined `_start` function and add `#[test_case]` above test function. It is also recomended to define local `#[panic_handler]` (that just uses `os_thing::test_panic_handler`). If you wish to make "should panic" tests you will need to define own `test_runner` function. | See examples in: `.\os_thing\tests\basic_boot.rs` and `.\os_thing\tests\should_panic.rs` 
- TO BUILD: run `cargo build` | Will just build.
- TO GET BIN: run `cargo bootimage` | Will generate `.bin` file that can be used to create image or ran in QEMU. It will take you the path of it.
- TO RUN: run `cargo run`
- TO TEST: run `cargo test` | If you want to test any specifci test case (located in `.\os_thing\tests`)  add name of the `.rs` file as `--test` argument | Example: `cargo test --test basic_boot.rs` (will test the `.\os_thing\tests\basic_boot.rs` individually)
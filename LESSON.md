# Suggested Extensions for VSCode

- Rust Analyzer

# Lesson 1: Cargo Build + Run program

- add new println!() statement after hello world

```shell
cargo build && ./target/rustasteria.exe
```

# Crates

```shell
cargo add chrono --features serde
cargo add serde --features derive
cargo add uuid --features v7 --features serde
cargo add serde_json
cargo add clap
```

Things that this example showcases:

- Building
  - `cargo build`
- Adding dependencies with crates.io and cargo
- Modules with `use` and `mod`
- Structs
- Functions
- Lists
- Generics with `<T>`
- Iterators
- Sorting and filtering
- File I/O
- DateTimes with `chrono`
- CLI interaction with `clap`
- JSON serialization with `serde`
- Macros
  - `println`, `assert!`, `format_args!`
- Stdout and stderr
- Borrows and mutability
  - `&`, `mut`, `&mut`
- Visibily modifiers with `pub`
- Enums including `Result<T,E>` and `Option<T>`
- Error propagation with `?`
- Variable declarations including `const`
- Derived traits, custom traits, and autotraits
  - `Eq`, `Deref`
  - `Into`, `From`
- Getting `to_string()` with `Display`
- Custom `Error`
- Doctests
- code comments
- Module tests
  - `cargo test`
- comparing enums with `mem::discriminant`
- Pattern matching with `match` and `if let`
- Control flow with `if` and `else`
- Loop with `for .. in`
- exiting and error codes
- Building both a binary executable and an importable library from the same
  codebase

# Ways to improve the program

- add more doctests
- add more module tests
- Make the ToDo read and write mthods take an arbitrary filename
- Add more filters to the `TodoFilter` and the `list` command

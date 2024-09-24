<strong>This repo is intended to serve as a tour of the Rust language for people
who know how to program but don't know Rust.</strong>

It was built as a way to show Rust to my coworkers.

## Concepts demonstrated

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
- Transforming `Option` into `Result` with `ok_or()`
- Mapping a Result's error into a new error type
- exiting and error codes
- Building both a binary executable and an importable library from the same
  codebase
- Simple lifetimes with ``<`a>``

## Crates used

The following crates are used:

- uuid
- chrono
- serde, serde_json
- clap

## Lessons left as exercises to the reader

- add more doctests
- add more module tests
- Make the ToDo read and write mthods take an arbitrary filename
- Add more filters to the `TodoFilter` and the `list` command
    - use the v7 uuid to sort by creation time

## Further reading

* https://doc.rust-lang.org/rust-by-example/ja/index.html\
* https://blog.jetbrains.com/rust/2024/09/20/how-to-learn-rust/\
* https://doc.rust-lang.org/nomicon/

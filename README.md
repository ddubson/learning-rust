# Rust Scratch

- `rustup` - Rust version manager
- `rustfmt` - external Rust formatter

## Common commands

### Project management

`cargo new [project]` - Create a new Rust project

### Compiling

Using Rust compiler

- `rustc src/main.rs --out-dir bin` - Compile **Hello, World** program
- `./bin/main`

Using Cargo

- `cargo build`
- `./target/debug/rust-scratch`

### Releasing

- `cargo build --release` - stores binary in `target/release`

### Linting 

`cargo check` - Check if code is compilable

`rustfmt src/*` - lint and autofmt Rust source. [rustfmt](https://github.com/rust-lang/rustfmt)

### CI

#### GitHub Actions

The CI definition file is located in `.github/workflows/rust.yml`
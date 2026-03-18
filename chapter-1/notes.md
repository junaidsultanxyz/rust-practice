# Chapter 1

leart about creating, compiling and running project. learnt basic commands for **cargo** (build tool for rust)

### rustc (rust compiler)
- `rustc --version`
- `rustc <.rs file>`

### cargo (build tool)
- `cargo new <project-name>`
- `cargo build` (debug profile: faster to build)
- `cargo build --release` (prod profile: optimized but slower in build)
- `cargo run` (builds and runs the project)
- `cargo check` (checks for errors and correct compilation without actually building the file)
- `cargo add <name>@<version>` (add new dependancy)
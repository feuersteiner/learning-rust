# Lesson 01 - Hello World & Cargo

## What I Learned

- Installed Rust via `rustup` (stable toolchain, default profile)
- Rust editions (2015, 2018, 2021, 2024) are language milestones, not calendar versions
- `Cargo.toml` is like `package.json` - project metadata and dependencies
- `Cargo.lock` is like `package-lock.json` - exact dependency versions
- `target/` is like `node_modules/` + `dist/` - should be gitignored

## Key Commands

| Command | Purpose |
|---------|---------|
| `cargo new <name>` | Create a new project |
| `cargo run` | Compile and run |
| `cargo build` | Compile only |
| `cargo check` | Fast compile check, no binary (like `tsc --noEmit`) |

## Tools Installed

| Tool | Purpose |
|------|---------|
| `rustc` | The Rust compiler |
| `cargo` | Package manager + build tool |
| `rustup` | Manages Rust installation |
| `rustfmt` | Code formatter (cosmetic - whitespace, indentation) |
| `clippy` | Linter (code analysis, best practices, beyond compiler warnings) |

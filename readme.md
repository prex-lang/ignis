# Ignis

Ignis is a lightweight and user-friendly command-line interface (CLI) tool for building and running projects written in the compiled **Prex** programming language. Similar to `cargo` in Rust, Ignis provides a consistent workflow to create, build, and execute Prex projects.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)

  - [`ignis new`](#ignis-new)
  - [`ignis build`](#ignis-build)
  - [`ignis run`](#ignis-run)

- [Configuration](#configuration)
- [Project Structure](#project-structure)
- [Example](#example)
- [License](#license)
- [Author](#author)

---

## Installation

1. Clone the repository and build Ignis using Cargo:

```bash
cargo build --release
```

2. Move the built binary to a directory in your `PATH`, for example:

```bash
cp target/release/ignis ~/.cargo/bin
```

3. Ensure the `prex` binary is available. Ignis will try to auto-detect it, or you can manually specify the path when prompted.

---

## Usage

### `ignis new <name>`

Creates a new Prex project with the given name:

```bash
ignis new hello
```

This will generate the following directory structure:

```
hello/
├── prex.toml
├── src/
│   └── main.prx
```

### `ignis build`

Builds the project in the current directory:

```bash
ignis build
```

Ignis will:

- Load `prex.toml`
- Compile `.prx` source files in the `src/` directory using `prex`
- Move the generated binary to the path specified in `build.output`

### `ignis run`

Builds and runs the project:

```bash
ignis run
```

This command combines `build` and immediately executes the compiled binary.

---

## Configuration

Ignis searches for the `prex` binary in the following order:

1. From `.ignis.conf` file in the current directory
2. In a relative `../prex` path
3. In the system `PATH`

If not found, it will prompt the user for a path and store it in `.ignis.conf`.

---

## Project Structure

The `prex.toml` file defines project metadata:

```toml
[package]
name = "project-name"
version = "0.1.0"

[dependencies]
# (optional dependencies)

[build]
output = "output.elf"
```

Source code should be placed in the `src/` directory with `.prx` files.

---

## Example

```bash
ignis new calc
cd calc
ignis run
```

---

## License

MIT

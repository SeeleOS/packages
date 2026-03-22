# SeeleOS Packages

This directory is a Rust-based package collection for SeeleOS.

It is closer to a small ports collection than a full package manager.

## Usage

Build and install a package:

```sh
cargo run install bash
cargo run install busybox
cargo run install tinycc
```

Clean a package work directory:

```sh
cargo run clean bash
```

The CLI currently supports:

- `install <package>`
- `clean <package>`

## Adding a Package

1. Add a package asset directory under `packages/<name>/` if patches or extra files are needed.
2. Add a new recipe under `src/package/`.
3. Implement `Package` for it.
4. Register it in `src/main.rs`.

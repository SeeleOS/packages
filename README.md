# SeeleOS Packages

This directory now uses a Rust CLI instead of per-package `Makefile` recipes.
Package metadata and build logic are modeled as Rust types implementing `trait Package`.

## Usage

Build and install a package:

```sh
cargo run -- install bash
cargo run -- install busybox
cargo run -- install tinycc
```

If you prefer the old shortcuts, these still work:

```sh
make -C packages bash
make -C packages busybox
make -C packages tcc
```

List supported packages:

```sh
cargo run -- list
```

## Commands

The `pkgs` binary supports:

- `pkgs fetch <name>`
- `pkgs patch <name>`
- `pkgs configure <name>`
- `pkgs build <name>`
- `pkgs install <name>`
- `pkgs clean <name>`
- `pkgs list`

## Layout

- `src/` contains the Rust package manager implementation.
- `bash/`, `busybox/`, `tinycc/` keep package-specific assets such as patches and config files.
- `work/<pkg>/` stores downloaded sources, stamps, build output, and other temporary artifacts.

## Adding a package

1. Add a new package asset directory under `packages/<name>/` if patches or extra files are needed.
2. Add a new Rust type under `src/package/`.
3. Implement `Package` for that type.
4. Register the package in `src/main.rs`.

# SeeleOS Packages

This directory contains the package collection for SeeleOS.  
Each subdirectory describes how to fetch, patch, build and install one piece of software into the Seele sysroot.

## Installing packages

For example, to install the `tcc` package

```sh
# From the project root
make -C packages tcc

# Or if your already in the packages directory
make tcc
```

## Common conventions

Each package directory (`packages/<name>/`) has:

- `Makefile` – package recipe, usually implementing:

  - `fetch`      – get source (tarball or git clone), normally cached under `work/<pkg>/`
  - `patch`      – apply `patches/*.patch` once per source tree
  - `configure`  – run upstream configure step if needed and cache it with stamps
  - `build`      – build with the Seele cross‑toolchain and `relibc-seele`
  - `install`    – install into `$(INSTALL_DIR)/<name>` and do a basic size check
  - `clean`      – remove that package’s `work/<name>` directory

All temporary output (unpacked sources, build artifacts, etc.) lives under:

- `work/<pkg>/...`

Most package recipes should keep fetch/patch/configure incremental by storing stamp files under
`work/<pkg>/.stamp/`, so rebuilding after a `relibc` change does not redownload or unpack sources.

You can delete `work/` at any time; the next build will recreate it.

## Adding a new package (short version)

1. Create `packages/<name>/` and (optionally) `packages/<name>/patches/`.
2. Write `packages/<name>/Makefile`:

   - `include ../config.mk`
   - set `PKG_NAME`, and source location (tarball or git)
   - implement `fetch/patch/configure/build/install`

3. Optionally add a convenience target to `packages/Makefile`, e.g.:

   ```make
   <name>:
   	$(MAKE) -C <name>
   ```

Then you can build it with:

```sh
make -C packages <name>
```

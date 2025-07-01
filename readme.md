# cargo-hilly 🏔️

cargo-hilly is a utility for Rust projects that helps migrate flat module files (e.g., "foo.rs") into the more idiomatic Rust module layout using subdirectories with mod.rs (e.g., "foo/mod.rs"). It scans your workspace for Rust crates and identifies modules that can be moved to this structure, then performs the migration for you.

## Why?

I don't like the new way of organizing the modules. It's hard to go through a complex project when you need to go in the parent directory to find a file related to the current module.

## Features

- Recursively finds all Rust crates in the current directory.
- Detects .rs files that have a sibling directory (e.g., "foo.rs" and "foo/").
- Moves eligible files to the corresponding mod.rs location (e.g., src/foo.rs ’ src/foo/mod.rs).
- Supports dry-run and check modes for safe migrations.

## Usage

Run the tool from the root of your workspace:

```sh
cargo install cargo-hilly
```

By default, it will migrate all eligible modules and print the actions taken.

### Options

- `--dry-run`:
  Shows what would be migrated without making any changes. Useful for previewing the migration.

  ```sh
  cargo hilly --dry-run
  ```
- `--check`:
  Checks if there are modules that need to be migrated. Exits with a non-zero status if migration is needed, but does not make any changes. Implies dry-run.

  ```sh
  cargo hilly --check
  ```

## Example

Suppose you have:

```
src/
  foo.rs
  foo/
    bar.rs
```

After running `cargo-hilly`, if `src/foo/mod.rs` does not exist, `foo.rs` will be moved to `foo/mod.rs`.

# License

MIT

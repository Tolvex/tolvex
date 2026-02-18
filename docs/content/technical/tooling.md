# Tooling

Tolvex's current compiler/CLI tool is `tlvxc`.

## Building

From the repository root:

```bash
cargo build -p tlvxc
```

## Quick check

Typecheck a file and print diagnostics:

```bash
cargo run -p tlvxc -- check path/to/file.tlvx
```

## JSON output

Emit machine-readable diagnostics and type information:

```bash
cargo run -p tlvxc -- json path/to/file.tlvx
```

You can also provide function type metadata via `--types-json`:

```bash
cargo run -p tlvxc -- check --types-json path/to/types.json path/to/file.tlvx
```

## REPL

Interactive parsing/typechecking feedback:

```bash
cargo run -p tlvxc -- repl
```

## Docs generator

Generate Markdown documentation from `///` / `//!` doc comments:

```bash
cargo run -p tlvxc -- docs --out-dir out_docs path/to/file.tlvx
```

## Package manifest (experimental)

Initialize a minimal `tolvex.toml` manifest:

```bash
cargo run -p tlvxc -- pack init
```

List manifest contents:

```bash
cargo run -p tlvxc -- pack list
```

## LLVM backend (optional)

Some targets and `--emit/--out` flags are available only when building with the `llvm-backend` feature (requires system LLVM 15.x).

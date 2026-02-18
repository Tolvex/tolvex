# Development Environment

## Editor support

Tolvex source files use the `.tlvx` extension. Any editor can be used today, but you’ll get the best experience with:

- A Rust-enabled setup for working on the compiler and stdlib (`rust-analyzer`).
- A text editor with syntax highlighting for `.tlvx` (basic highlighting can be configured as a custom file type).

## Building the compiler and running tests

From the repository root:

```bash
cargo test --workspace
```

For faster feedback while iterating:

```bash
cargo test -p tlvxc_lexer
cargo test -p tlvxc_parser
```

## Running examples

Examples live under `examples/`.

- `examples/use_cases/` contains end-to-end healthcare-oriented Tolvex programs and companion datasets.
- `examples/wasm32-wasi/` and `examples/browser/` contain WASM-focused demos.

## Benchmarks

Compiler and stdlib benchmarks are organized under:

- `compiler/benches/` (compiler pipeline and lexer/parser benchmarks)
- `stdlib/*/benches/` (crate-specific Criterion benchmarks)

Run benchmarks with:

```bash
cargo bench
```

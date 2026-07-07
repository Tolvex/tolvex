# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Tolvex is a compiled programming language purpose-built for healthcare analytics (native FHIR/HL7/DICOM/genomics
support, compile-time PHI/privacy tracking, HIPAA-style `regulate` blocks). It's implemented as a Rust workspace:
a lexer → parser → typeck → borrowck pipeline feeding an optional LLVM codegen backend, plus a `.tlvx` standard
library and Python bindings.

The project was previously named "Medi" (file extension `.medi`, crates `medic_*`) and was renamed to Tolvex
(`.tlvx`, crates `tlvxc_*`/`tolvex_*`) as of v0.1.6 — see `CHANGELOG.md` and `MIGRATION.md`. Some older docs
(e.g. `tests/README.md`) still reference the old `medic_*` crate names; the actual crate names in `Cargo.toml`
are the source of truth.

## Common commands

Build from the **repo root** (the canonical workspace root — see "Two workspace manifests" below):

```sh
cargo build --workspace                       # build everything (LLVM backend excluded by default)
cargo build --workspace --features llvm-backend  # include the LLVM codegen backend (needs system LLVM 15.x)
cargo test --workspace                        # run all tests
cargo test -p tlvxc_parser -p tlvxc_lexer     # test a single crate
cargo test -p tlvxc_typeck -- --nocapture     # run a subset with output
cargo test -p tests some_test_name             # run a single integration test by name

cargo fmt --all -- --check                    # formatting check (required by pre-commit hook)
cargo clippy --workspace --all-targets -- -D warnings

# Compile a .tlvx program (requires --features llvm-backend on tlvxc)
cargo run -p tlvxc --features llvm-backend -- --emit=x86_64 --out=program.o example.tlvx
```

The `.pre-commit-config.yaml` hook (`scripts/pre-commit-rust.sh`) runs `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings`, `cargo check --workspace --all-targets`, and `cargo test --workspace`
on every commit touching Rust files — run these yourself before committing to catch failures early.

Building with `--features llvm-backend` (or anything under `compiler/tlvxc_codegen_llvm`) requires **LLVM 15.x**
on the system, with `LLVM_SYS_150_PREFIX` and `LIBCLANG_PATH` pointed at it (see `.github/workflows/ci.yml` for
the exact install steps per OS). Without it, the workspace still builds and tests fine — LLVM is an optional
dependency behind the `llvm` feature specifically so the rest of the compiler doesn't require LLVM to hack on.

Python bindings (`bindings/pytolvex`, PyO3 + maturin):

```sh
maturin build -m bindings/pytolvex/Cargo.toml --release --out dist
pytest -q bindings/pytolvex/tests
```

## Two workspace manifests — use the root one

There are **two** `[workspace]` definitions: the root `Cargo.toml` (the one CI and `README.md` use — build/test
from here) and a second, inconsistent one at `compiler/Cargo.toml` (its member list references crates by
package name rather than directory path in places, e.g. `"tolvexpack"` for the `compiler/tvx` directory, and
omits crates like `tests` and `stdlib/*`). Treat `compiler/Cargo.toml` as legacy/unused — don't build from
inside `compiler/`. `tlvxc_borrowck` and `tlvxc_runtime` aren't listed as members in the root manifest either,
but they're pulled into the root workspace implicitly because `tlvxc` depends on them by path and they don't
declare their own `[workspace]`.

## Architecture: the compiler pipeline

Each stage is its own crate under `compiler/`, in dependency order:

1. **`tlvxc_lexer`** — tokenizes `.tlvx` source; recognizes medical codes (ICD-10, LOINC, SNOMED, CPT) as first-class
   tokens. Has streaming/chunked variants for large files.
2. **`tlvxc_ast`** — AST node definitions, a visitor (`visit` module), optional serde JSON (de)serialization.
   Depends only on `tlvxc_lexer` (for spans/tokens).
3. **`tlvxc_parser`** — recursive-descent parser (built on `nom`) producing `tlvxc_ast` nodes; has
   error-recovery/diagnostics (`parse_program_recovering`, `render_snippet`) shared by the CLI and IDE server.
   Note: `|>` is *not* a single pipeline token by default — it's lexed as `|` + `>`.
4. **`tlvxc_type`** — foundational type-system crate, zero internal deps. Defines `MediType` and
   `PrivacyAnnotation` (`PHI` / `Pseudonymized` / `Anonymized` / `Authorized` / `AuthorizedFor`);
   `domain.rs` assigns `PrivacyAnnotation::PHI` by default to healthcare-domain types (patient, observation, ...).
5. **`tlvxc_env`** — scope/environment (`TypeEnv`) for variables and functions; tracks `SinkKind` (Log/Print/etc.)
   used for PHI-sink detection.
6. **`tlvxc_typeck`** — the type checker, plus **compile-time** compliance checking (`compliance.rs`): turns
   privacy-flow violations into `ComplianceViolation`s (`UnprotectedPhiToSink`, `PhiReturned`, `PolicyViolation`).
   Also owns dimensional/quantity units (`units.rs`, behind the experimental `quantity_ir` feature).
7. **`tlvxc_borrowck`** — borrow/lifetime checker over the AST (visitor-based). Also hosts `RtConstraintChecker`,
   which forbids GC/alloc/spawn/channel calls inside `rt_begin`/`rt_end` real-time regions. `conservative_fn_ref`
   feature makes call-site alias analysis stricter.
8. **`tlvxc_codegen_llvm`** — LLVM IR/codegen, entirely behind the `llvm` feature (Inkwell is optional) so the
   rest of the workspace never needs LLVM installed. Targets x86_64, WASM, and RISC-V.
9. **`tlvxc_runtime`** — the runtime compiled programs execute against: task scheduler (crossbeam), incremental
   GC (`gc` feature), real-time zone stubs (`rt_zones` feature), optional `distributed` module.

Driving binaries:
- **`tlvxc`** (`compiler/tlvxc`) — the main CLI: `check`, `json` (IDE-friendly diagnostics), `repl`, `docs`,
  `test`, `pack` subcommands. Wires lexer → parser → borrowck → typeck(+compliance) → optional codegen.
  LLVM-dependent flags (`--emit`, `--out`, `--opt`, `--cpu`, `--opt-pipeline`) require `--features llvm-backend`.
- **`tvx`** (package name `tolvexpack`, `compiler/tvx`) — the package manager, analogous to `cargo`:
  `init`/`build`/`check`/`clean`/`publish`, reads `tolvex.toml` manifests and a `src/main.tlvx` entrypoint,
  outputs to `target/`. Has its own `registry.rs`/`security.rs` for publishing.
- **`tlvxc_ide_server`** — Axum HTTP server exposing lex/parse/typecheck as a service (`/analyze`, `/complete`)
  for editor tooling, plus a stub package registry (`registry_api.rs`, `auth.rs` is explicitly a placeholder —
  don't treat it as real auth).

## Architecture: privacy/compliance (the flagship feature) has two independent layers

Don't conflate these — they operate at different times and don't share code:

- **Compile-time (type-system level)**: `tlvxc_type::traits` (`privacy_annotation()`), `tlvxc_type::domain`
  (default PHI classification for healthcare types), and `tlvxc_typeck::compliance` (turns type-checker privacy
  flow violations into `ComplianceViolation`s during `regulate` block checking).
- **Runtime/library level**: `stdlib/tolvex_privacy` (differential privacy — `budget.rs` epsilon accounting,
  `mechanisms.rs` noise mechanisms, `anonymize.rs`, `federated.rs` federated aggregation) and
  `stdlib/tolvex_compliance` (a separate, JSON-rule-driven HIPAA pattern scanner over data, using regex/sha2 —
  unrelated to the type checker's compliance logic despite the similar name).

## Architecture: standard library (`stdlib/`)

- **`tolvex_core`** — minimal collections/runtime primitives (`MediMap`, `MediVec`, async shim). No deps.
- **`tolvex_data`** — the largest stdlib crate: FHIR/HL7/DICOM/ICD/LOINC/SNOMED/OMOP interop, optional AES-GCM
  at-rest encryption (`storage_encrypted.rs`), `sanitize.rs`. Most other stdlib crates depend on this one.
- **`tolvex_standards`** — thin re-export facade over `tolvex_data`'s standards modules; no own logic.
- **`tolvex_stats`** — clinical/statistical library (t-tests, survival analysis, epi, power analysis, bootstrap).
  Optional `fhir` feature pulls in `tolvex_data`.
- **`tolvex_rt`** — stdlib-facing real-time I/O/device/stream primitives, complementing `tlvxc_runtime`'s
  compiler-side RT zone scaffolding.
- **`tolvex_ai`** / **`tolvex_model`** — clinical ML: `tolvex_ai` does risk scoring, bias/fairness, calibration,
  explainability; `tolvex_model` is the pluggable model-backend registry (ONNX/TFLite are stubs) with its own
  calibration/fairness/quantization. They overlap conceptually — `tolvex_ai` is the clinical-scoring layer,
  `tolvex_model` is the backend/runtime plumbing layer.

## Testing conventions

- Unit tests live inside each crate; cross-crate integration/system tests live in `tests/` (crate `tests`,
  files directly in `tests/` and under `tests/src/`) — see `tests/README.md` for the full suite breakdown
  (lexer/parser, type/HIPAA/privacy, codegen per-target, RT/perf sanity, end-to-end exec).
- CI (`.github/workflows/ci.yml`) runs the Windows matrix leg with only non-LLVM-dependent crates:
  `cargo test -p tlvxc_ast -p tlvxc_parser -p tlvxc_type -p tlvxc_typeck --all-features`.
- Coverage (`cargo tarpaulin`, threshold 70% via `codecov.yml`) is scoped to `tlvxc_parser`, `tlvxc_lexer`, and
  `tests` only — codegen/runtime/borrowck/stdlib are excluded from the coverage gate, don't expect their
  coverage numbers to move CI.
- A borrow-checker perf gate (`borrowck-perf-gate` job) fails CI if the borrow checker's median benchmark time
  exceeds 80ms — be mindful of this when touching `tlvxc_borrowck`.

## Commit and documentation conventions

- Do **not** include `Co-Authored-By:` trailers in commit messages, regardless of which AI tool produced the
  commit. Commit attribution stays with the human author. These trailers have been retroactively stripped from
  past commits.
- All `Plans.md` content, harness output, and documentation must be in English — headers, table columns, task
  descriptions, and status markers. No Japanese characters in status markers (use `cc:done` / `cc:wip`, not
  `cc:完了` / `cc:WIP`). This applies strictly to tracked files.

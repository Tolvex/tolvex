# Migration: v0.0.4 → v0.0.5

- tolvex_data: new FHIR types (Medication, Procedure, Condition, Encounter, DiagnosticReport)
- tolvex_compliance: profile-aware rule engine scaffolding
- tolvex_ai: risk helpers consolidated
- tolvex_stats: added bootstrap, survival, streaming modules

## Steps
- Update Cargo.toml to latest versions
- Adjust imports for new modules and re-exports
- Validate examples still compile: `cargo test --workspace`

# Benchmarks

Latest published benchmark results:

--8<-- "technical/healthcare_workloads_latest.md"

## LLVM pipeline comparison (medical workloads)

--8<-- "technical/medical_workloads_latest.md"

## Regenerating benchmarks

```bash
cargo run -p tlvxc_benchmarks --bin benchmark_healthcare_workloads --release
cargo bench -p tolvex_data --bench medical_workloads
```

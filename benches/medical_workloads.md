# Medical workloads benchmark (criterion)

Command:
```
cargo bench -p tolvex_data --bench medical_workloads
```
Environment: default (no gnuplot; plotters backend). Bench harness sets `TOLVEX_LLVM_PIPE` per run: `minimal` vs `medical`.

Results (microseconds, Criterion point estimates):

| workload            | pipeline  | time (µs)          |
| ------------------- | --------- | ------------------ |
| lab_stats_1000      | minimal   | 47.676 – 48.574    |
| lab_stats_1000      | medical   | 50.174 – 51.117    |
| lab_stats_10000     | minimal   | 510.69 – 526.15    |
| lab_stats_10000     | medical   | 512.00 – 527.56    |

Notes:
- Criterion emitted warnings for 10k samples about target time; defaults were kept.
- Gnuplot not installed; HTML reports live under `target/criterion/` (gitignored). This file is the persisted summary for the repo.

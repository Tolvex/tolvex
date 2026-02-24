## Medical workloads (LLVM pipeline comparison)

Command:

```bash
cargo bench -p tolvex_data --bench medical_workloads
```

Pipelines toggled per run via `TOLVEX_LLVM_PIPE` (`minimal` baseline vs `medical`). Plot backend: plotters (gnuplot not installed).

Results (Criterion point estimates, µs):

| workload        | pipeline  | time (µs)        |
| --------------- | --------- | ---------------- |
| lab_stats_1000  | minimal   | 47.680 – 48.294  |
| lab_stats_1000  | medical   | 48.970 – 50.002  |
| lab_stats_10000 | minimal   | 495.87 – 509.44  |
| lab_stats_10000 | medical   | 499.26 – 512.86  |

Raw output (captured from the same run):

```
lab_stats_1000          time:   [47.680 µs 48.008 µs 48.294 µs]
                        change: [-2.3779% +1.1314% +4.4166%] (p = 0.50 > 0.05)
                        No change in performance detected.

lab_stats_1000_medical  time:   [48.970 µs 49.542 µs 50.002 µs]
                        change: [-3.6790% -1.2930% +1.1052%] (p = 0.29 > 0.05)
                        No change in performance detected.

lab_stats_10000         time:   [495.87 µs 503.26 µs 509.44 µs]
                        change: [-4.7061% -1.4474% +1.9520%] (p = 0.39 > 0.05)
                        No change in performance detected.

lab_stats_10000_medical time:   [499.26 µs 506.78 µs 512.86 µs]
                        change: [-4.4851% -1.3055% +2.1907%] (p = 0.45 > 0.05)
                        No change in performance detected.
```

Regeneration:
- Requires LLVM pipeline toggle env handled by the bench harness.
- Run the command above; HTML/CSV reports remain in `target/criterion/` (gitignored).
- Bench sources: `stdlib/tolvex_data/benches/medical_workloads.rs`.
- Captured output also stored in `benches/medical_workloads_output.txt` (repo-tracked).

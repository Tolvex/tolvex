---
_harness_template: "Plans.md.template"
_harness_version: "4.10.0"
---

# Plans.md - Task Tracking

> **Project**: tolvex
> **Last updated**: 2026-07-07
> **Updated by**: Claude Code

---

## In Progress

<!-- Add tasks with cc:wip here. -->

(none)

---

## Not Started

<!-- Add tasks with cc:todo or pm:requested here. -->

### Standard Library Expansion (v0.1.7) — TODO.md §2.2

- [ ] SL02: Meta-analysis, SIR/SEIR epidemiological models, and sample size calculations in `tolvex_stats` `cc:todo` [P]
  - DoD: fixed/random-effects `meta_analysis`, `sir_model`/`seir_model` simulators, and `sample_size_*` functions added with unit tests; `cargo test -p tolvex_stats` passes.
- [ ] SL03: Local differential privacy and secure aggregation in `tolvex_privacy` `cc:todo` [P]
  - DoD: randomized-response-based local DP mechanism added to `mechanisms.rs`; secure aggregation protocol added to `aggregate.rs`/`federated.rs`; unit tests cover both; `cargo test -p tolvex_privacy` passes.
  - Note: Laplace/Gaussian mechanisms (`mechanisms.rs`) and federated averaging (`federated.rs::fedavg`) are already implemented — do not duplicate, only add what's missing.
- [ ] SL04: `tolvex_rt` streaming: tumbling/sliding/session windowing, alert/notification system, MQTT/AMQP device adapters `cc:todo` [P]
  - DoD: windowing functions added to `stream.rs` (a sliding-sum helper already exists — extend, don't duplicate), an alert/notification dispatch primitive added, MQTT/AMQP adapter stubs added to `device.rs` behind feature flags; unit tests cover windowing math; `cargo test -p tolvex_rt` passes.
- [ ] SL05: Web-based interactive dashboard runtime (chart components, layout, real-time data binding) `cc:todo` [P]
  - DoD: decide crate placement first (extend `tolvex_stats::viz` vs. new `tolvex_viz` crate) and record the decision in this task before implementing; interactive chart component, dashboard layout, and real-time data binding primitives added with unit tests; `cargo test` passes for the chosen crate.

---

## Completed

<!-- Add tasks with cc:done or pm:confirmed here. -->

- [x] SL00: Enhance `tolvex_data` with HL7 v2.x, DICOM, FASTQ/VCF/BAM, and OMOP CDM support `cc:done`
  - DoD: parsers/extractors present in `tolvex_data`; matches TODO.md §2.2 first bullet (checked) and recent commits (`db7634c8`, `52cb0e3c`, `d7db84c4`).
- [x] SL01: Cox proportional hazards + competing risks analysis in `tolvex_stats::survival` `cc:done [e61aa39]`
  - DoD: `cox_ph` (Newton-Raphson on the Breslow partial likelihood) fits regression coefficients and `competing_risks` (Aalen-Johansen estimator) computes cumulative incidence functions; both covered by unit tests including a hand-computed CIF fixture and a finite-difference stationarity check on `cox_partial_log_likelihood`; `cargo test -p tolvex_stats` passes; `cargo clippy -p tolvex_stats --all-targets -- -D warnings` clean for the new code (one pre-existing, unrelated `viz.rs` clippy error left untouched).

---

## Archive

<!-- Move older completed tasks here. -->

---

## Status Marker Legend

These markers are protocol values used by Harness tooling. Keep them unchanged
unless the project has tested parser aliases. No Japanese characters in status
markers per this project's CLAUDE.md.

| Marker | Meaning |
|--------|---------|
| `pm:requested` | PM requested work |
| `cc:todo` | Not started by Claude Code |
| `cc:wip` | Claude Code is working |
| `cc:done` | Claude Code completed the task and is awaiting confirmation |
| `pm:confirmed` | PM confirmed completion |
| `blocked` | Blocked; include the reason next to the task |

---

## Optional Extended Syntax

For larger plans, you may add task IDs, dependencies, and parallel markers.

### Task ID / Dependency / Parallel Marker

```markdown
- [ ] T001: Authentication `cc:todo`
- [ ] T002: User API `cc:todo` depends:T001
- [ ] T003: Product API `cc:todo` [P]
- [ ] T004: Order API `cc:todo` depends:T001,T003
```

| Syntax | Meaning | Example |
|--------|---------|---------|
| `T001:` | Optional task ID | Used for references and dependencies |
| `depends:ID` | Dependency task | `depends:T001,T002` |
| `[P]` | Parallelizable | Can run at the same time as other ready tasks |

**Note**: Extended syntax is optional. The plain checklist format still works.

---

## Last Update

- **Updated at**: 2026-07-07
- **Last session owner**: Claude Code
- **Branch**: main

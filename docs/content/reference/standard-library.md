# Standard Library

The Tolvex standard library is implemented as a set of Rust crates under `stdlib/`.

## Modules

- `tolvex_data`: healthcare data structures and ingestion helpers
- `tolvex_stats`: basic statistics (mean, stddev, Welch t-test)
- `tolvex_compliance`: HIPAA/compliance primitives
- `tolvex_ai`: minimal risk scorer and utilities
- `tolvex_model`: model interfaces and stubs

## API overview (Rust host)

The following examples show how the current stdlib crates are consumed from Rust. This mirrors how Tolvex's runtime and compiler integrate today.

### `tolvex_data`

- **HL7 ingestion**: `tolvex_data::hl7_fhir::{hl7_to_fhir_patient_minimal, hl7_to_medical_event_minimal}`
- **Observation ingestion**: `tolvex_data::hl7_fhir_obx::hl7_to_observations_minimal`
- **Sanitization**: `tolvex_data::sanitize::{trim_patient_names, normalize_patient_birth_date, canonize_observation_code, normalize_observation_unit}`
- **Validation**: `tolvex_data::validate::{validate_patient, validate_observation, validate_medical_event}`
- **Query builder**: `tolvex_data::fhir_query("...")`

```rust
use tolvex_data::hl7_fhir::hl7_to_fhir_patient_minimal;
use tolvex_data::hl7_fhir_obx::hl7_to_observations_minimal;
use tolvex_data::sanitize::{canonize_observation_code, normalize_observation_unit, trim_patient_names};
use tolvex_data::validate::{validate_observation, validate_patient};

let mut patient = hl7_to_fhir_patient_minimal(hl7_msg)?;
trim_patient_names(&mut patient);
validate_patient(&patient)?;

let mut observations = hl7_to_observations_minimal(hl7_msg)?;
for o in &mut observations {
    canonize_observation_code(o);
    normalize_observation_unit(o);
    validate_observation(o)?;
}
```

### `tolvex_stats`

- **Descriptive stats**: `mean`, `median`, `stddev_sample`
- **Inferential**: `t_test_welch`

```rust
use tolvex_stats::{mean, stddev_sample, t_test_welch};

let a = [1.0, 2.0, 3.0, 4.0];
let b = [2.0, 3.0, 4.0, 5.0];
let mu_a = mean(&a);
let sd_a = stddev_sample(&a);
let res = t_test_welch(&a, &b);
println!("mean={mu_a} sd={sd_a} t={:.3} df={:.1}", res.t_stat, res.df);
```

### `tolvex_compliance`

- **Keyword checks**: `HipaaKeywordRule`, `run_hipaa_bundle`
- **Summaries**: `build_compliance_report_summary`

```rust
use tolvex_compliance::{
    build_compliance_report_summary, run_hipaa_bundle, ComplianceProfile, HipaaKeywordRule,
    ReportKind, RuleSeverity,
};

let rules = vec![HipaaKeywordRule {
    id: "k1".into(),
    description: "Detect SSN".into(),
    keywords: vec!["ssn".into(), "social security".into()],
    severity: RuleSeverity::Error,
}];

let report = run_hipaa_bundle(
    "Patient SSN: 123-45-6789",
    &rules,
    None,
    None,
    None,
    Some(400),
    &[],
);

let summary = build_compliance_report_summary(
    ReportKind::Fda21Cfr11,
    Some(ComplianceProfile::Hipaa),
    &report.keyword_results,
);
println!("passed={}", summary.summary.passed);
```

### `tolvex_ai`

- **Scoring**: `RiskScorer::predict`
- **End-to-end helpers**: `predict_risk`, `stratify_risk`

```rust
use tolvex_ai::{predict_risk, stratify_risk, RiskScorer};
use serde_json::json;

let model = RiskScorer {
    weights: vec![0.2, 0.5, 0.3],
    bias: 0.1,
    model_name: "demo".into(),
};
let score = model.predict(&[0.4, 0.2, 0.9]);
println!("score={score}");

let patient = json!({"age": 65, "bmi": 33.1, "a1c": 7.8});
let pred = predict_risk(&patient, "diabetes", "5y", &["age".into(), "bmi".into(), "a1c".into()]);
let stratum = stratify_risk(&pred);
println!("risk={:.3} stratum={:?}", pred.score, stratum);
```

### `tolvex_model`

- **Registry**: `ModelRegistry`
- **Metadata**: `ModelMetadata`

```rust
use tolvex_model::{ModelMetadata, ModelRegistry};

let mut reg = ModelRegistry::default();
reg.register(ModelMetadata {
    id: "risk_demo".into(),
    name: "Risk Demo".into(),
    version: "0.1.0".into(),
    task: "risk".into(),
    input_schema: None,
    output_schema: None,
});
assert!(reg.get("risk_demo").is_some());
```

## Running tests

```bash
cargo test -p tolvex_data
cargo test -p tolvex_stats
cargo test -p tolvex_compliance
cargo test -p tolvex_ai
cargo test -p tolvex_model
```

## Benchmarks

Some stdlib crates include Criterion benchmarks.

```bash
cargo bench -p tolvex_data
cargo bench -p tolvex_compliance
```

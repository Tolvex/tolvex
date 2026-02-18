# Tolvex Cookbook

This cookbook shows common Tolvex patterns using the standard library crates. Copy snippets into your own code/tests and adapt as needed.

## FHIR validation and basic queries (tolvex_data)

```rust
use tolvex_data::{validate_patient, fhir_query};

// Validate a minimal patient (example structure only)
let patient = tolvex_data::testing::patient_minimal("patient_001");
validate_patient(&patient).expect("valid patient");

// Build a simple query
let query = fhir_query("Patient")
    .filter_family_name_ci_contains("doe")
    .build();
let results = query.execute_patients(&[patient]);
assert!(results.len() >= 0);
```

## De-identification and compliance (tolvex_compliance)

```rust
use tolvex_compliance::{default_hipaa_rules, ComplianceEngine};

let rules = default_hipaa_rules();
let engine = ComplianceEngine::new(rules);
let json = serde_json::json!({
  "name": "Jane Doe",
  "ssn": "123-45-6789"
});
let result = engine.evaluate_json(&json);
assert!(result.violations().len() >= 0);
```

## Risk prediction (tolvex_ai)

```rust
use tolvex_ai::{load_model, RiskPrediction, stratify_risk};

let features = tolvex_ai::testing::example_features();
let model = load_model("models/diabetes_risk.json").expect("model");
let score = model.predict(&features);
let stratum = stratify_risk(&RiskPrediction {
    score,
    condition: "diabetes".into(),
    timeframe: "5y".into(),
});
assert!(matches!(stratum, tolvex_ai::RiskStratum::Low | tolvex_ai::RiskStratum::Medium | tolvex_ai::RiskStratum::High));
```

## Simple stats (tolvex_stats)

```rust
use tolvex_stats::mean;

let xs = [1.0, 2.0, 3.0];
assert_eq!(mean(&xs), 2.0);
```

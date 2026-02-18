# Getting Started: tolvex_data

Ingest HL7 → FHIR-like types → sanitize → validate → query.

```rust
use tolvex_data::hl7_fhir::hl7_to_fhir_patient_minimal;
use tolvex_data::validate::validate_patient;

let hl7 = "MSH|^~\\&|...";
let patient = hl7_to_fhir_patient_minimal(hl7).unwrap();
validate_patient(&patient).unwrap();
```

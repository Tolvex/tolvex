//! OMOP Common Data Model (CDM) support for standardized observational healthcare data.
//!
//! This module provides data structures and utilities for working with the OMOP CDM,
//! a standardized data model developed by the Observational Medical Outcomes Partnership
//! for observational healthcare research.
//!
//! ## Key Features
//! - Core OMOP entity structures (Person, Observation, Condition, Procedure, etc.)
//! - Date/time handling with optional precision
//! - Concept code mapping utilities
//! - Data conversion helpers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OMOP Person entity representing a patient in the CDM
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Person {
    pub person_id: i64,
    pub gender_concept_id: i32,
    pub year_of_birth: i32,
    pub month_of_birth: Option<i32>,
    pub day_of_birth: Option<i32>,
    pub birth_datetime: Option<String>,
    pub race_concept_id: Option<i32>,
    pub ethnicity_concept_id: Option<i32>,
    pub location_id: Option<i64>,
    pub provider_id: Option<i64>,
    pub care_site_id: Option<i64>,
    pub person_source_value: Option<String>,
    pub gender_source_value: Option<String>,
    pub gender_source_concept_id: Option<i32>,
    pub race_source_value: Option<String>,
    pub race_source_concept_id: Option<i32>,
    pub ethnicity_source_value: Option<String>,
    pub ethnicity_source_concept_id: Option<i32>,
}

/// OMOP Observation entity for clinical observations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Observation {
    pub observation_id: i64,
    pub person_id: i64,
    pub observation_concept_id: i32,
    pub observation_date: String,
    pub observation_datetime: Option<String>,
    pub observation_type_concept_id: i32,
    pub value_as_number: Option<f64>,
    pub value_as_string: Option<String>,
    pub value_as_concept_id: Option<i32>,
    pub qualifier_concept_id: Option<i32>,
    pub unit_concept_id: Option<i32>,
    pub provider_id: Option<i64>,
    pub visit_occurrence_id: Option<i64>,
    pub visit_detail_id: Option<i64>,
    pub observation_source_value: Option<String>,
    pub observation_source_concept_id: Option<i32>,
    pub unit_source_value: Option<String>,
    pub qualifier_source_value: Option<String>,
    pub value_source_value: Option<String>,
    pub observation_event_id: Option<i64>,
    pub obs_event_field_concept_id: Option<i32>,
}

/// OMOP Condition Occurrence entity for diagnoses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionOccurrence {
    pub condition_occurrence_id: i64,
    pub person_id: i64,
    pub condition_concept_id: i32,
    pub condition_start_date: String,
    pub condition_start_datetime: Option<String>,
    pub condition_end_date: Option<String>,
    pub condition_end_datetime: Option<String>,
    pub condition_type_concept_id: i32,
    pub condition_status_concept_id: Option<i32>,
    pub stop_reason: Option<String>,
    pub provider_id: Option<i64>,
    pub visit_occurrence_id: Option<i64>,
    pub visit_detail_id: Option<i64>,
    pub condition_source_value: Option<String>,
    pub condition_source_concept_id: Option<i32>,
    pub condition_status_source_value: Option<String>,
}

/// OMOP Procedure Occurrence entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcedureOccurrence {
    pub procedure_occurrence_id: i64,
    pub person_id: i64,
    pub procedure_concept_id: i32,
    pub procedure_date: String,
    pub procedure_datetime: Option<String>,
    pub procedure_type_concept_id: i32,
    pub modifier_concept_id: Option<i32>,
    pub quantity: Option<i32>,
    pub provider_id: Option<i64>,
    pub visit_occurrence_id: Option<i64>,
    pub visit_detail_id: Option<i64>,
    pub procedure_source_value: Option<String>,
    pub procedure_source_concept_id: Option<i32>,
    pub modifier_source_value: Option<String>,
}

/// OMOP Drug Exposure entity for medications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrugExposure {
    pub drug_exposure_id: i64,
    pub person_id: i64,
    pub drug_concept_id: i32,
    pub drug_exposure_start_date: String,
    pub drug_exposure_start_datetime: Option<String>,
    pub drug_exposure_end_date: Option<String>,
    pub drug_exposure_end_datetime: Option<String>,
    pub verbatim_end_date: Option<String>,
    pub drug_type_concept_id: i32,
    pub stop_reason: Option<String>,
    pub refills: Option<i32>,
    pub quantity: Option<f64>,
    pub days_supply: Option<i32>,
    pub sig: Option<String>,
    pub route_concept_id: Option<i32>,
    pub lot_number: Option<String>,
    pub provider_id: Option<i64>,
    pub visit_occurrence_id: Option<i64>,
    pub visit_detail_id: Option<i64>,
    pub drug_source_value: Option<String>,
    pub drug_source_concept_id: Option<i32>,
    pub route_source_value: Option<String>,
    pub dose_unit_source_value: Option<String>,
}

/// OMOP Measurement entity for lab results and measurements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Measurement {
    pub measurement_id: i64,
    pub person_id: i64,
    pub measurement_concept_id: i32,
    pub measurement_date: String,
    pub measurement_datetime: Option<String>,
    pub measurement_time: Option<String>,
    pub measurement_type_concept_id: i32,
    pub operator_concept_id: Option<i32>,
    pub value_as_number: Option<f64>,
    pub value_as_concept_id: Option<i32>,
    pub unit_concept_id: Option<i32>,
    pub range_low: Option<f64>,
    pub range_high: Option<f64>,
    pub provider_id: Option<i64>,
    pub visit_occurrence_id: Option<i64>,
    pub visit_detail_id: Option<i64>,
    pub measurement_source_value: Option<String>,
    pub measurement_source_concept_id: Option<i32>,
    pub unit_source_value: Option<String>,
    pub value_source_value: Option<String>,
    pub measurement_event_id: Option<i64>,
    pub meas_event_field_concept_id: Option<i32>,
}

/// OMOP Visit Occurrence entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisitOccurrence {
    pub visit_occurrence_id: i64,
    pub person_id: i64,
    pub visit_concept_id: i32,
    pub visit_start_date: String,
    pub visit_start_datetime: Option<String>,
    pub visit_end_date: String,
    pub visit_end_datetime: Option<String>,
    pub visit_type_concept_id: i32,
    pub provider_id: Option<i64>,
    pub care_site_id: Option<i64>,
    pub visit_source_value: Option<String>,
    pub visit_source_concept_id: Option<i32>,
    pub admitting_source_concept_id: Option<i32>,
    pub admitting_source_value: Option<String>,
    pub discharge_to_concept_id: Option<i32>,
    pub discharge_to_source_value: Option<String>,
    pub preceding_visit_occurrence_id: Option<i64>,
}

/// OMOP Death entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Death {
    pub person_id: i64,
    pub death_date: String,
    pub death_datetime: Option<String>,
    pub death_type_concept_id: Option<i32>,
    pub cause_concept_id: Option<i32>,
    pub cause_source_value: Option<String>,
    pub cause_source_concept_id: Option<i32>,
}

/// OMOP Concept entity for vocabulary concepts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Concept {
    pub concept_id: i32,
    pub concept_name: String,
    pub domain_id: String,
    pub vocabulary_id: String,
    pub concept_class_id: String,
    pub standard_concept: Option<String>,
    pub concept_code: String,
    pub valid_start_date: String,
    pub valid_end_date: String,
    pub invalid_reason: Option<String>,
}

/// OMOP Concept Relationship entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConceptRelationship {
    pub concept_id_1: i32,
    pub concept_id_2: i32,
    pub relationship_id: String,
    pub valid_start_date: String,
    pub valid_end_date: String,
    pub invalid_reason: Option<String>,
}

/// OMOP Provider entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Provider {
    pub provider_id: i64,
    pub provider_name: Option<String>,
    pub npi: Option<String>,
    pub dea: Option<String>,
    pub specialty_concept_id: Option<i32>,
    pub care_site_id: Option<i64>,
    pub year_of_birth: Option<i32>,
    pub gender_concept_id: Option<i32>,
    pub provider_source_value: Option<String>,
    pub specialty_source_value: Option<String>,
    pub specialty_source_concept_id: Option<i32>,
    pub gender_source_value: Option<String>,
    pub gender_source_concept_id: Option<i32>,
}

/// Common OMOP concept IDs for standard concepts
pub mod concept_ids {
    // Gender
    pub const MALE: i32 = 8507;
    pub const FEMALE: i32 = 8532;
    pub const UNKNOWN_GENDER: i32 = 0;

    // Race
    pub const WHITE: i32 = 8527;
    pub const BLACK_OR_AFRICAN_AMERICAN: i32 = 8516;
    pub const ASIAN: i32 = 8515;

    // Visit types
    pub const INPATIENT_VISIT: i32 = 9201;
    pub const OUTPATIENT_VISIT: i32 = 9202;
    pub const EMERGENCY_ROOM_VISIT: i32 = 9203;

    // Condition types
    pub const EHR_PROBLEM_LIST_ENTRY: i32 = 32817;
    pub const EHR_ENCOUNTER_DIAGNOSIS: i32 = 32840;

    // Procedure types
    pub const EHR_PROCEDURE_RECORD: i32 = 32817;

    // Observation types
    pub const EHR_OBSERVATION: i32 = 32817;
    pub const LABORATORY_RESULT: i32 = 32856;

    // Drug types
    pub const EHR_PRESCRIPTION: i32 = 32838;
    pub const EHR_MEDICATION_ADMINISTRATION: i32 = 32818;
}

/// Create a new Person entity with minimal required fields
pub fn create_person(person_id: i64, gender_concept_id: i32, year_of_birth: i32) -> Person {
    Person {
        person_id,
        gender_concept_id,
        year_of_birth,
        month_of_birth: None,
        day_of_birth: None,
        birth_datetime: None,
        race_concept_id: None,
        ethnicity_concept_id: None,
        location_id: None,
        provider_id: None,
        care_site_id: None,
        person_source_value: None,
        gender_source_value: None,
        gender_source_concept_id: None,
        race_source_value: None,
        race_source_concept_id: None,
        ethnicity_source_value: None,
        ethnicity_source_concept_id: None,
    }
}

/// Create an Observation entity with minimal required fields
pub fn create_observation(
    observation_id: i64,
    person_id: i64,
    observation_concept_id: i32,
    observation_date: &str,
    observation_type_concept_id: i32,
) -> Observation {
    Observation {
        observation_id,
        person_id,
        observation_concept_id,
        observation_date: observation_date.to_string(),
        observation_datetime: None,
        observation_type_concept_id,
        value_as_number: None,
        value_as_string: None,
        value_as_concept_id: None,
        qualifier_concept_id: None,
        unit_concept_id: None,
        provider_id: None,
        visit_occurrence_id: None,
        visit_detail_id: None,
        observation_source_value: None,
        observation_source_concept_id: None,
        unit_source_value: None,
        qualifier_source_value: None,
        value_source_value: None,
        observation_event_id: None,
        obs_event_field_concept_id: None,
    }
}

/// Create a ConditionOccurrence entity with minimal required fields
pub fn create_condition(
    condition_occurrence_id: i64,
    person_id: i64,
    condition_concept_id: i32,
    condition_start_date: &str,
    condition_type_concept_id: i32,
) -> ConditionOccurrence {
    ConditionOccurrence {
        condition_occurrence_id,
        person_id,
        condition_concept_id,
        condition_start_date: condition_start_date.to_string(),
        condition_start_datetime: None,
        condition_end_date: None,
        condition_end_datetime: None,
        condition_type_concept_id,
        condition_status_concept_id: None,
        stop_reason: None,
        provider_id: None,
        visit_occurrence_id: None,
        visit_detail_id: None,
        condition_source_value: None,
        condition_source_concept_id: None,
        condition_status_source_value: None,
    }
}

/// Create a ProcedureOccurrence entity with minimal required fields
pub fn create_procedure(
    procedure_occurrence_id: i64,
    person_id: i64,
    procedure_concept_id: i32,
    procedure_date: &str,
    procedure_type_concept_id: i32,
) -> ProcedureOccurrence {
    ProcedureOccurrence {
        procedure_occurrence_id,
        person_id,
        procedure_concept_id,
        procedure_date: procedure_date.to_string(),
        procedure_datetime: None,
        procedure_type_concept_id,
        modifier_concept_id: None,
        quantity: None,
        provider_id: None,
        visit_occurrence_id: None,
        visit_detail_id: None,
        procedure_source_value: None,
        procedure_source_concept_id: None,
        modifier_source_value: None,
    }
}

/// Create a DrugExposure entity with minimal required fields
pub fn create_drug_exposure(
    drug_exposure_id: i64,
    person_id: i64,
    drug_concept_id: i32,
    drug_exposure_start_date: &str,
    drug_type_concept_id: i32,
) -> DrugExposure {
    DrugExposure {
        drug_exposure_id,
        person_id,
        drug_concept_id,
        drug_exposure_start_date: drug_exposure_start_date.to_string(),
        drug_exposure_start_datetime: None,
        drug_exposure_end_date: None,
        drug_exposure_end_datetime: None,
        verbatim_end_date: None,
        drug_type_concept_id,
        stop_reason: None,
        refills: None,
        quantity: None,
        days_supply: None,
        sig: None,
        route_concept_id: None,
        lot_number: None,
        provider_id: None,
        visit_occurrence_id: None,
        visit_detail_id: None,
        drug_source_value: None,
        drug_source_concept_id: None,
        route_source_value: None,
        dose_unit_source_value: None,
    }
}

/// Create a Measurement entity with minimal required fields
pub fn create_measurement(
    measurement_id: i64,
    person_id: i64,
    measurement_concept_id: i32,
    measurement_date: &str,
    measurement_type_concept_id: i32,
) -> Measurement {
    Measurement {
        measurement_id,
        person_id,
        measurement_concept_id,
        measurement_date: measurement_date.to_string(),
        measurement_datetime: None,
        measurement_time: None,
        measurement_type_concept_id,
        operator_concept_id: None,
        value_as_number: None,
        value_as_concept_id: None,
        unit_concept_id: None,
        range_low: None,
        range_high: None,
        provider_id: None,
        visit_occurrence_id: None,
        visit_detail_id: None,
        measurement_source_value: None,
        measurement_source_concept_id: None,
        unit_source_value: None,
        value_source_value: None,
        measurement_event_id: None,
        meas_event_field_concept_id: None,
    }
}

/// Create a VisitOccurrence entity with minimal required fields
pub fn create_visit(
    visit_occurrence_id: i64,
    person_id: i64,
    visit_concept_id: i32,
    visit_start_date: &str,
    visit_end_date: &str,
    visit_type_concept_id: i32,
) -> VisitOccurrence {
    VisitOccurrence {
        visit_occurrence_id,
        person_id,
        visit_concept_id,
        visit_start_date: visit_start_date.to_string(),
        visit_start_datetime: None,
        visit_end_date: visit_end_date.to_string(),
        visit_end_datetime: None,
        visit_type_concept_id,
        provider_id: None,
        care_site_id: None,
        visit_source_value: None,
        visit_source_concept_id: None,
        admitting_source_concept_id: None,
        admitting_source_value: None,
        discharge_to_concept_id: None,
        discharge_to_source_value: None,
        preceding_visit_occurrence_id: None,
    }
}

/// Create a Concept entity
#[allow(clippy::too_many_arguments)]
pub fn create_concept(
    concept_id: i32,
    concept_name: &str,
    domain_id: &str,
    vocabulary_id: &str,
    concept_class_id: &str,
    concept_code: &str,
    valid_start_date: &str,
    valid_end_date: &str,
) -> Concept {
    Concept {
        concept_id,
        concept_name: concept_name.to_string(),
        domain_id: domain_id.to_string(),
        vocabulary_id: vocabulary_id.to_string(),
        concept_class_id: concept_class_id.to_string(),
        standard_concept: None,
        concept_code: concept_code.to_string(),
        valid_start_date: valid_start_date.to_string(),
        valid_end_date: valid_end_date.to_string(),
        invalid_reason: None,
    }
}

/// Calculate age from year of birth
pub fn calculate_age(year_of_birth: i32, current_year: i32) -> i32 {
    current_year - year_of_birth
}

/// Check if a concept is valid on a given date
pub fn is_concept_valid(concept: &Concept, check_date: &str) -> bool {
    concept.valid_start_date.as_str() <= check_date && check_date <= concept.valid_end_date.as_str()
}

/// Validate a concept ID is valid per OMOP CDM spec (non-negative)
pub fn is_valid_concept_id(concept_id: i32) -> bool {
    concept_id >= 0
}

/// Convert a FHIR Patient to OMOP Person
#[cfg(feature = "fhir-conversion")]
pub fn fhir_to_omop_person(patient: &crate::fhir::FHIRPatient) -> Option<Person> {
    let person_id = patient.id.parse::<i64>().ok()?;

    let year_of_birth = patient
        .birth_date
        .as_ref()
        .and_then(|date| date.split('-').next())
        .and_then(|year| year.parse::<i32>().ok())?;

    // FHIRPatient doesn't have gender field, use unknown
    let gender_concept_id = concept_ids::UNKNOWN_GENDER;

    Some(Person {
        person_id,
        gender_concept_id,
        year_of_birth,
        month_of_birth: None,
        day_of_birth: None,
        birth_datetime: patient.birth_date.clone(),
        race_concept_id: None,
        ethnicity_concept_id: None,
        location_id: None,
        provider_id: None,
        care_site_id: None,
        person_source_value: Some(patient.id.clone()),
        gender_source_value: None,
        gender_source_concept_id: None,
        race_source_value: None,
        race_source_concept_id: None,
        ethnicity_source_value: None,
        ethnicity_source_concept_id: None,
    })
}

/// Create a batch of test OMOP records for testing
pub fn create_test_dataset() -> (
    Vec<Person>,
    Vec<Observation>,
    Vec<ConditionOccurrence>,
    Vec<VisitOccurrence>,
) {
    let person1 = create_person(1, concept_ids::MALE, 1980);
    let person2 = create_person(2, concept_ids::FEMALE, 1990);

    let mut observation1 = create_observation(
        1,
        1,
        3012888, // Body weight concept
        "2024-01-15",
        concept_ids::EHR_OBSERVATION,
    );
    observation1.value_as_number = Some(70.5);

    let condition1 = create_condition(
        1,
        1,
        31967, // Diabetes mellitus concept
        "2024-01-10",
        concept_ids::EHR_PROBLEM_LIST_ENTRY,
    );

    let visit1 = create_visit(
        1,
        1,
        concept_ids::OUTPATIENT_VISIT,
        "2024-01-15",
        "2024-01-15",
        concept_ids::EHR_PROCEDURE_RECORD,
    );

    (
        vec![person1, person2],
        vec![observation1],
        vec![condition1],
        vec![visit1],
    )
}

/// Serialize OMOP entities to JSON
pub fn to_json<T: Serialize>(entity: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(entity)
}

/// Deserialize OMOP entities from JSON
pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json)
}

/// Simple in-memory OMOP database for testing and small datasets
pub struct OmopDatabase {
    pub persons: HashMap<i64, Person>,
    pub observations: HashMap<i64, Observation>,
    pub conditions: HashMap<i64, ConditionOccurrence>,
    pub procedures: HashMap<i64, ProcedureOccurrence>,
    pub drug_exposures: HashMap<i64, DrugExposure>,
    pub measurements: HashMap<i64, Measurement>,
    pub visits: HashMap<i64, VisitOccurrence>,
    pub concepts: HashMap<i32, Concept>,
}

impl OmopDatabase {
    /// Create a new empty OMOP database
    pub fn new() -> Self {
        Self {
            persons: HashMap::new(),
            observations: HashMap::new(),
            conditions: HashMap::new(),
            procedures: HashMap::new(),
            drug_exposures: HashMap::new(),
            measurements: HashMap::new(),
            visits: HashMap::new(),
            concepts: HashMap::new(),
        }
    }

    /// Insert a person into the database
    pub fn insert_person(&mut self, person: Person) {
        self.persons.insert(person.person_id, person);
    }

    /// Insert an observation into the database
    pub fn insert_observation(&mut self, observation: Observation) {
        self.observations
            .insert(observation.observation_id, observation);
    }

    /// Insert a condition into the database
    pub fn insert_condition(&mut self, condition: ConditionOccurrence) {
        self.conditions
            .insert(condition.condition_occurrence_id, condition);
    }

    /// Insert a procedure into the database
    pub fn insert_procedure(&mut self, procedure: ProcedureOccurrence) {
        self.procedures
            .insert(procedure.procedure_occurrence_id, procedure);
    }

    /// Insert a drug exposure into the database
    pub fn insert_drug_exposure(&mut self, drug_exposure: DrugExposure) {
        self.drug_exposures
            .insert(drug_exposure.drug_exposure_id, drug_exposure);
    }

    /// Insert a measurement into the database
    pub fn insert_measurement(&mut self, measurement: Measurement) {
        self.measurements
            .insert(measurement.measurement_id, measurement);
    }

    /// Insert a visit into the database
    pub fn insert_visit(&mut self, visit: VisitOccurrence) {
        self.visits.insert(visit.visit_occurrence_id, visit);
    }

    /// Insert a concept into the database
    pub fn insert_concept(&mut self, concept: Concept) {
        self.concepts.insert(concept.concept_id, concept);
    }

    /// Get all observations for a person
    pub fn get_person_observations(&self, person_id: i64) -> Vec<&Observation> {
        self.observations
            .values()
            .filter(|obs| obs.person_id == person_id)
            .collect()
    }

    /// Get all conditions for a person
    pub fn get_person_conditions(&self, person_id: i64) -> Vec<&ConditionOccurrence> {
        self.conditions
            .values()
            .filter(|cond| cond.person_id == person_id)
            .collect()
    }

    /// Get all visits for a person
    pub fn get_person_visits(&self, person_id: i64) -> Vec<&VisitOccurrence> {
        self.visits
            .values()
            .filter(|visit| visit.person_id == person_id)
            .collect()
    }

    /// Get person by ID
    pub fn get_person(&self, person_id: i64) -> Option<&Person> {
        self.persons.get(&person_id)
    }

    /// Get concept by ID
    pub fn get_concept(&self, concept_id: i32) -> Option<&Concept> {
        self.concepts.get(&concept_id)
    }
}

impl Default for OmopDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_person() {
        let person = create_person(1, concept_ids::MALE, 1980);
        assert_eq!(person.person_id, 1);
        assert_eq!(person.gender_concept_id, 8507);
        assert_eq!(person.year_of_birth, 1980);
    }

    #[test]
    fn test_create_observation() {
        let obs = create_observation(1, 1, 3012888, "2024-01-15", concept_ids::EHR_OBSERVATION);
        assert_eq!(obs.observation_id, 1);
        assert_eq!(obs.person_id, 1);
        assert_eq!(obs.observation_concept_id, 3012888);
        assert_eq!(obs.observation_date, "2024-01-15");
    }

    #[test]
    fn test_create_condition() {
        let condition = create_condition(
            1,
            1,
            31967,
            "2024-01-10",
            concept_ids::EHR_PROBLEM_LIST_ENTRY,
        );
        assert_eq!(condition.condition_occurrence_id, 1);
        assert_eq!(condition.person_id, 1);
        assert_eq!(condition.condition_concept_id, 31967);
    }

    #[test]
    fn test_create_visit() {
        let visit = create_visit(
            1,
            1,
            concept_ids::OUTPATIENT_VISIT,
            "2024-01-15",
            "2024-01-15",
            concept_ids::EHR_PROCEDURE_RECORD,
        );
        assert_eq!(visit.visit_occurrence_id, 1);
        assert_eq!(visit.visit_concept_id, 9202);
    }

    #[test]
    fn test_calculate_age() {
        assert_eq!(calculate_age(1980, 2024), 44);
        assert_eq!(calculate_age(1990, 2024), 34);
    }

    #[test]
    fn test_concept_validity() {
        let concept = create_concept(
            8507,
            "MALE",
            "Gender",
            "Gender",
            "Gender",
            "M",
            "1970-01-01",
            "2099-12-31",
        );
        assert!(is_concept_valid(&concept, "2024-01-15"));
        assert!(!is_concept_valid(&concept, "1969-01-01"));
    }

    #[test]
    fn test_omop_database() {
        let mut db = OmopDatabase::new();

        let person = create_person(1, concept_ids::MALE, 1980);
        db.insert_person(person);

        let observation =
            create_observation(1, 1, 3012888, "2024-01-15", concept_ids::EHR_OBSERVATION);
        db.insert_observation(observation);

        let condition = create_condition(
            1,
            1,
            31967,
            "2024-01-10",
            concept_ids::EHR_PROBLEM_LIST_ENTRY,
        );
        db.insert_condition(condition);

        assert!(db.get_person(1).is_some());
        assert_eq!(db.get_person_observations(1).len(), 1);
        assert_eq!(db.get_person_conditions(1).len(), 1);
    }

    #[test]
    fn test_json_serialization() {
        let person = create_person(1, concept_ids::MALE, 1980);
        let json = to_json(&person).unwrap();
        let deserialized: Person = from_json(&json).unwrap();
        assert_eq!(person.person_id, deserialized.person_id);
        assert_eq!(person.gender_concept_id, deserialized.gender_concept_id);
    }

    #[test]
    fn test_create_test_dataset() {
        let (persons, observations, conditions, visits) = create_test_dataset();
        assert_eq!(persons.len(), 2);
        assert_eq!(observations.len(), 1);
        assert_eq!(conditions.len(), 1);
        assert_eq!(visits.len(), 1);
    }

    #[test]
    fn test_concept_ids() {
        assert_eq!(concept_ids::MALE, 8507);
        assert_eq!(concept_ids::FEMALE, 8532);
        assert_eq!(concept_ids::INPATIENT_VISIT, 9201);
        assert_eq!(concept_ids::OUTPATIENT_VISIT, 9202);
    }

    #[test]
    fn test_is_valid_concept_id() {
        assert!(is_valid_concept_id(0)); // Unknown is valid
        assert!(is_valid_concept_id(8507)); // Standard concept
        assert!(!is_valid_concept_id(-1)); // Negative is invalid
    }

    #[test]
    fn test_create_procedure() {
        let procedure = create_procedure(
            1,
            1,
            2000001,
            "2024-01-15",
            concept_ids::EHR_PROCEDURE_RECORD,
        );
        assert_eq!(procedure.procedure_occurrence_id, 1);
        assert_eq!(procedure.procedure_concept_id, 2000001);
    }

    #[test]
    fn test_create_drug_exposure() {
        let drug = create_drug_exposure(1, 1, 1112807, "2024-01-15", concept_ids::EHR_PRESCRIPTION);
        assert_eq!(drug.drug_exposure_id, 1);
        assert_eq!(drug.drug_concept_id, 1112807);
    }

    #[test]
    fn test_create_measurement() {
        let measurement =
            create_measurement(1, 1, 3000963, "2024-01-15", concept_ids::LABORATORY_RESULT);
        assert_eq!(measurement.measurement_id, 1);
        assert_eq!(measurement.measurement_concept_id, 3000963);
    }

    #[test]
    fn test_create_concept() {
        let concept = create_concept(
            8507,
            "MALE",
            "Gender",
            "Gender",
            "Gender",
            "M",
            "1970-01-01",
            "2099-12-31",
        );
        assert_eq!(concept.concept_id, 8507);
        assert_eq!(concept.concept_name, "MALE");
        assert_eq!(concept.vocabulary_id, "Gender");
    }
}

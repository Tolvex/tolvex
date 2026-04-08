#[derive(Debug, Clone, PartialEq)]
pub struct HL7Message {
    pub segments: Vec<HL7Segment>,
    pub field_sep: char,
    pub component_sep: char,
    pub repetition_sep: char,
    pub escape_char: char,
    pub subcomponent_sep: char,
    pub version: String,
    pub message_type: Option<HL7MessageType>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HL7Segment {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HL7MessageType {
    pub event: String,
    pub trigger: String,
    pub structure: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HL7SegmentType {
    MSH, PID, PV1, ORC, OBR, OBX, IN1, IN2, IN3, DG1, PR1, NK1, AL1, IAM,
    SCH, ARD, ARQ, TQ1, TQ2, RGS, RXA, RXR, RXC, RXE, RXO, RXD, RXG, NTE,
    ERR, QAK, QBP, QPD, QID, QRI, RCP, RQD, RQA, RQB, RDF, RDR, RDT,
    SFT, UAC, BTS, EQL, EQQ, EQR, ERQ, ERS, ERI, EVN, FT1, FT2,
    FT3, FT4, FT5, FT6, FT7, FT8, FT9, FT10, FT11, FT12, FT13, FT14, FT15,
    FT16, FT17, FT18, FT19, FT20, FT21, FT22, FT23, FT24, FT25, FT26, FT27,
    FT28, FT29, FT30, FT31, FT32, FT33, FT34, FT35, FT36, FT37, FT38, FT39,
    FT40, FT41, FT42, FT43, FT44, FT45, FT46, FT47, FT48, FT49, FT50, FT51,
    FT52, FT53, FT54, FT55, FT56, FT57, FT58, FT59, FT60, FT61, FT62, FT63,
    FT64, FT65, FT66, FT67, FT68, FT69, FT70, FT71, FT72, FT73, FT74, FT75,
    FT76, FT77, FT78, FT79, FT80, FT81, FT82, FT83, FT84, FT85, FT86, FT87,
    FT88, FT89, FT90, FT91, FT92, FT93, FT94, FT95, FT96, FT97, FT98, FT99,
    FT100, FT101, FT102, FT103, FT104, FT105, FT106, FT107, FT108, FT109,
    FT110, FT111, FT112, FT113, FT114, FT115, FT116, FT117, FT118, FT119,
    FT120, FT121, FT122, FT123, FT124, FT125, FT126, FT127, FT128, FT129,
    FT130, FT131, FT132, FT133, FT134, FT135, FT136, FT137, FT138, FT139,
    FT140, FT141, FT142, FT143, FT144, FT145, FT146, FT147, FT148, FT149,
    FT150, FT151, FT152, FT153, FT154, FT155, FT156, FT157, FT158, FT159,
    FT160, FT161, FT162, FT163, FT164, FT165, FT166, FT167, FT168, FT169,
    FT170, FT171, FT172, FT173, FT174, FT175, FT176, FT177, FT178, FT179,
    FT180, FT181, FT182, FT183, FT184, FT185, FT186, FT187, FT188, FT189,
    FT190, FT191, FT192, FT193, FT194, FT195, FT196, FT197, FT198, FT199,
    FT200, FT201, FT202, FT203, FT204, FT205, FT206, FT207, FT208, FT209,
    FT210, FT211, FT212, FT213, FT214, FT215, FT216, FT217, FT218, FT219,
    FT220, FT221, FT222, FT223, FT224, FT225, FT226, FT227, FT228, FT229,
    FT230, FT231, FT232, FT233, FT234, FT235, FT236, FT237, FT238, FT239,
    FT240, FT241, FT242, FT243, FT244, FT245, FT246, FT247, FT248, FT249,
    FT250, FT251, FT252, FT253, FT254, FT255,
    // Add more segment types as needed
}

impl HL7SegmentType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "MSH" => Some(HL7SegmentType::MSH),
            "PID" => Some(HL7SegmentType::PID),
            "PV1" => Some(HL7SegmentType::PV1),
            "ORC" => Some(HL7SegmentType::ORC),
            "OBR" => Some(HL7SegmentType::OBR),
            "OBX" => Some(HL7SegmentType::OBX),
            "IN1" => Some(HL7SegmentType::IN1),
            "IN2" => Some(HL7SegmentType::IN2),
            "IN3" => Some(HL7SegmentType::IN3),
            "DG1" => Some(HL7SegmentType::DG1),
            "PR1" => Some(HL7SegmentType::PR1),
            "NK1" => Some(HL7SegmentType::NK1),
            "AL1" => Some(HL7SegmentType::AL1),
            "IAM" => Some(HL7SegmentType::IAM),
            "SCH" => Some(HL7SegmentType::SCH),
            "ARD" => Some(HL7SegmentType::ARD),
            "ARQ" => Some(HL7SegmentType::ARQ),
            "TQ1" => Some(HL7SegmentType::TQ1),
            "TQ2" => Some(HL7SegmentType::TQ2),
            "RGS" => Some(HL7SegmentType::RGS),
            "RXA" => Some(HL7SegmentType::RXA),
            "RXR" => Some(HL7SegmentType::RXR),
            "RXC" => Some(HL7SegmentType::RXC),
            "RXE" => Some(HL7SegmentType::RXE),
            "RXO" => Some(HL7SegmentType::RXO),
            "RXD" => Some(HL7SegmentType::RXD),
            "RXG" => Some(HL7SegmentType::RXG),
            "NTE" => Some(HL7SegmentType::NTE),
            "ERR" => Some(HL7SegmentType::ERR),
            "QAK" => Some(HL7SegmentType::QAK),
            "QBP" => Some(HL7SegmentType::QBP),
            "QPD" => Some(HL7SegmentType::QPD),
            "QID" => Some(HL7SegmentType::QID),
            "QRI" => Some(HL7SegmentType::QRI),
            "RCP" => Some(HL7SegmentType::RCP),
            "RQD" => Some(HL7SegmentType::RQD),
            "RQA" => Some(HL7SegmentType::RQA),
            "RQB" => Some(HL7SegmentType::RQB),
            "RDF" => Some(HL7SegmentType::RDF),
            "RDR" => Some(HL7SegmentType::RDR),
            "RDT" => Some(HL7SegmentType::RDT),
            "SFT" => Some(HL7SegmentType::SFT),
            "UAC" => Some(HL7SegmentType::UAC),
            "BTS" => Some(HL7SegmentType::BTS),
            "EQL" => Some(HL7SegmentType::EQL),
            "EQQ" => Some(HL7SegmentType::EQQ),
            "EQR" => Some(HL7SegmentType::EQR),
            "ERQ" => Some(HL7SegmentType::ERQ),
            "ERS" => Some(HL7SegmentType::ERS),
            "ERI" => Some(HL7SegmentType::ERI),
            "EVN" => Some(HL7SegmentType::EVN),
            _ => None,
        }
    }
}

impl HL7Segment {
    pub fn get_segment_type(&self) -> Option<HL7SegmentType> {
        HL7SegmentType::from_str(&self.name)
    }
    
    pub fn get_field(&self, index: usize) -> Option<&String> {
        self.fields.get(index)
    }
    
    pub fn get_field_as_string(&self, index: usize) -> Option<String> {
        self.fields.get(index).cloned()
    }
    
    pub fn get_field_components(&self, index: usize) -> Vec<String> {
        if let Some(field) = self.get_field(index) {
            field.split('^').map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn get_field_subcomponents(&self, index: usize) -> Vec<Vec<Vec<String>>> {
        if let Some(field) = self.get_field(index) {
            parse_field_components(field)
        } else {
            Vec::new()
        }
    }
}

pub fn parse_hl7(input: &str) -> Result<HL7Message, HL7Error> {
    if input.trim().is_empty() {
        return Ok(HL7Message {
            segments: vec![],
            field_sep: '|',
            component_sep: '^',
            repetition_sep: '~',
            escape_char: '\\',
            subcomponent_sep: '&',
            version: "2.5".to_string(),
            message_type: None,
            timestamp: None,
        });
    }

    let lines: Vec<&str> = input.split('\r').collect();
    if lines.is_empty() {
        return Ok(HL7Message {
            segments: vec![],
            field_sep: '|',
            component_sep: '^',
            repetition_sep: '~',
            escape_char: '\\',
            subcomponent_sep: '&',
            version: "2.5".to_string(),
            message_type: None,
            timestamp: None,
        });
    }

    let first_line = lines[0];
    if !first_line.starts_with("MSH") {
        return Err(HL7Error::new("HL7 message must start with MSH segment"));
    }

    let (field_sep, component_sep, repetition_sep, escape_char, subcomponent_sep) =
        extract_separators(first_line)?;

    let mut segments = Vec::new();
    let mut version = "2.5".to_string();
    let mut message_type = None;
    let mut timestamp = None;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let seg_name = if line.len() >= 3 {
            &line[0..3]
        } else {
            return Err(HL7Error::new(format!("invalid segment name: {line}")));
        };

        if seg_name.len() != 3
            || !seg_name
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            return Err(HL7Error::new(format!("invalid segment name: {seg_name}")));
        }

        let fields = if seg_name == "MSH" {
            let fields = parse_msh_fields(line, field_sep)?;
            
            // Extract version from MSH-12
            if fields.len() > 12 {
                version = fields[12].clone();
            }
            
            // Extract message type from MSH-9
            if fields.len() > 9 {
                let msg_type_parts: Vec<&str> = fields[9].split('^').collect();
                if msg_type_parts.len() >= 2 {
                    message_type = Some(HL7MessageType {
                        event: msg_type_parts[0].to_string(),
                        trigger: msg_type_parts[1].to_string(),
                        structure: msg_type_parts.get(2).unwrap_or(&"").to_string(),
                    });
                }
            }
            
            // Extract timestamp from MSH-7
            if fields.len() > 7 {
                timestamp = Some(fields[7].clone());
            }
            
            fields
        } else {
            parse_segment_fields(line, field_sep)?
        };

        segments.push(HL7Segment {
            name: seg_name.to_string(),
            fields,
        });
    }

    Ok(HL7Message {
        segments,
        field_sep,
        component_sep,
        repetition_sep,
        escape_char,
        subcomponent_sep,
        version,
        message_type,
        timestamp,
    })
}

/// Generate an HL7 message from components
pub fn generate_hl7(message: &HL7Message) -> Result<String, HL7Error> {
    let mut result = String::new();
    
    for (i, segment) in message.segments.iter().enumerate() {
        if i > 0 {
            result.push('\r');
        }
        
        if segment.name == "MSH" {
            // Special handling for MSH segment
            result.push_str(&segment.name);
            result.push(message.field_sep);
            
            // Add encoding characters
            result.push(message.field_sep);
            result.push(message.component_sep);
            result.push(message.repetition_sep);
            result.push(message.escape_char);
            result.push(message.subcomponent_sep);
            
            // Add remaining fields
            for (j, field) in segment.fields.iter().enumerate() {
                if j > 0 {
                    result.push(message.field_sep);
                }
                result.push_str(field);
            }
        } else {
            // Regular segment
            result.push_str(&segment.name);
            result.push(message.field_sep);
            
            for (j, field) in segment.fields.iter().enumerate() {
                if j > 0 {
                    result.push(message.field_sep);
                }
                result.push_str(field);
            }
        }
    }
    
    Ok(result)
}

/// Create a new HL7 message with default separators
pub fn create_hl7_message() -> HL7Message {
    HL7Message {
        segments: Vec::new(),
        field_sep: '|',
        component_sep: '^',
        repetition_sep: '~',
        escape_char: '\\',
        subcomponent_sep: '&',
        version: "2.5".to_string(),
        message_type: None,
        timestamp: None,
    }
}

/// Add an MSH segment to a message
pub fn add_msh_segment(
    message: &mut HL7Message,
    sending_app: &str,
    sending_facility: &str,
    receiving_app: &str,
    receiving_facility: &str,
    timestamp: &str,
    message_type: &str,
    message_control_id: &str,
    processing_id: &str,
    version_id: &str,
) -> Result<(), HL7Error> {
    let mut fields = Vec::new();
    fields.push(message.field_sep.to_string()); // MSH-1 (field separator)
    fields.push(format!("{}{}{}{}", 
        message.component_sep, 
        message.repetition_sep, 
        message.escape_char, 
        message.subcomponent_sep
    )); // MSH-2 (encoding characters)
    fields.push(sending_app.to_string()); // MSH-3
    fields.push(sending_facility.to_string()); // MSH-4
    fields.push(receiving_app.to_string()); // MSH-5
    fields.push(receiving_facility.to_string()); // MSH-6
    fields.push(timestamp.to_string()); // MSH-7
    fields.push("".to_string()); // MSH-8 (security)
    fields.push(message_type.to_string()); // MSH-9
    fields.push(message_control_id.to_string()); // MSH-10
    fields.push(processing_id.to_string()); // MSH-11
    fields.push(version_id.to_string()); // MSH-12
    
    message.segments.push(HL7Segment {
        name: "MSH".to_string(),
        fields,
    });
    
    // Update message metadata
    message.version = version_id.to_string();
    message.timestamp = Some(timestamp.to_string());
    
    let msg_type_parts: Vec<&str> = message_type.split('^').collect();
    if msg_type_parts.len() >= 2 {
        message.message_type = Some(HL7MessageType {
            event: msg_type_parts[0].to_string(),
            trigger: msg_type_parts[1].to_string(),
            structure: msg_type_parts.get(2).unwrap_or(&"").to_string(),
        });
    }
    
    Ok(())
}

/// Add a PID segment to a message
pub fn add_pid_segment(
    message: &mut HL7Message,
    set_id: &str,
    patient_id: &str,
    patient_identifier_list: &str,
    patient_name: &str,
    mother_maiden_name: &str,
    birth_date: &str,
    gender: &str,
    address: &str,
    phone_number: &str,
) -> Result<(), HL7Error> {
    let mut fields = Vec::new();
    fields.push(set_id.to_string()); // PID-1
    fields.push(patient_id.to_string()); // PID-2
    fields.push(patient_identifier_list.to_string()); // PID-3
    fields.push("".to_string()); // PID-4
    fields.push(patient_name.to_string()); // PID-5
    fields.push(mother_maiden_name.to_string()); // PID-6
    fields.push(birth_date.to_string()); // PID-7
    fields.push(gender.to_string()); // PID-8
    fields.push("".to_string()); // PID-9
    fields.push("".to_string()); // PID-10
    fields.push(address.to_string()); // PID-11
    fields.push("".to_string()); // PID-12
    fields.push(phone_number.to_string()); // PID-13
    
    message.segments.push(HL7Segment {
        name: "PID".to_string(),
        fields,
    });
    
    Ok(())
}

/// Add an OBX segment to a message
pub fn add_obx_segment(
    message: &mut HL7Message,
    set_id: &str,
    value_type: &str,
    observation_identifier: &str,
    observation_sub_id: &str,
    observation_value: &str,
    units: &str,
    reference_range: &str,
    abnormal_flags: &str,
    probability: &str,
) -> Result<(), HL7Error> {
    let mut fields = Vec::new();
    fields.push(set_id.to_string()); // OBX-1
    fields.push(value_type.to_string()); // OBX-2
    fields.push(observation_identifier.to_string()); // OBX-3
    fields.push(observation_sub_id.to_string()); // OBX-4
    fields.push(observation_value.to_string()); // OBX-5
    fields.push(units.to_string()); // OBX-6
    fields.push(reference_range.to_string()); // OBX-7
    fields.push(abnormal_flags.to_string()); // OBX-8
    fields.push(probability.to_string()); // OBX-9
    
    message.segments.push(HL7Segment {
        name: "OBX".to_string(),
        fields,
    });
    
    Ok(())
}

/// Validate an HL7 message structure
pub fn validate_hl7_message(message: &HL7Message) -> Result<(), HL7Error> {
    // Check if message has at least an MSH segment
    if message.segments.is_empty() {
        return Err(HL7Error::new("Message has no segments"));
    }
    
    if message.segments[0].name != "MSH" {
        return Err(HL7Error::new("Message must start with MSH segment"));
    }
    
    // Validate MSH segment has required fields
    let msh = &message.segments[0];
    if msh.fields.len() < 12 {
        return Err(HL7Error::new("MSH segment missing required fields"));
    }
    
    // Validate segment names
    for segment in &message.segments {
        if segment.name.len() != 3 {
            return Err(HL7Error::new(format!("Invalid segment name length: {}", segment.name)));
        }
        
        if !segment.name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            return Err(HL7Error::new(format!("Invalid segment name characters: {}", segment.name)));
        }
        
        if HL7SegmentType::from_str(&segment.name).is_none() {
            return Err(HL7Error::new(format!("Unknown segment type: {}", segment.name)));
        }
    }
    
    Ok(())
}

/// Get all segments of a specific type
pub fn get_segments_by_type<'a>(message: &'a HL7Message, segment_type: &str) -> Vec<&'a HL7Segment> {
    message.segments
        .iter()
        .filter(|seg| seg.name == segment_type)
        .collect()
}

/// Get the first segment of a specific type
pub fn get_first_segment_by_type<'a>(message: &'a HL7Message, segment_type: &str) -> Option<&'a HL7Segment> {
    message.segments
        .iter()
        .find(|seg| seg.name == segment_type)
}

/// Extract patient information from PID segments
pub fn extract_patient_info(message: &HL7Message) -> Result<Vec<PatientInfo>, HL7Error> {
    let pid_segments = get_segments_by_type(message, "PID");
    let mut patients = Vec::new();
    
    for pid in pid_segments {
        let name_field = pid.get_field(4).cloned().unwrap_or_default();
        let address_field = pid.get_field(10).cloned().unwrap_or_default();
        let name_components: Vec<&str> = name_field.split('^').collect();
        let address_components: Vec<&str> = address_field.split('^').collect();
        
        patients.push(PatientInfo {
            set_id: pid.get_field_as_string(0),
            patient_id: pid.get_field_as_string(1),
            patient_identifier_list: pid.get_field_as_string(2),
            last_name: name_components.get(0).map(|s| s.to_string()),
            first_name: name_components.get(1).map(|s| s.to_string()),
            middle_name: name_components.get(2).map(|s| s.to_string()),
            suffix: name_components.get(3).map(|s| s.to_string()),
            prefix: name_components.get(4).map(|s| s.to_string()),
            birth_date: pid.get_field_as_string(6),
            gender: pid.get_field_as_string(7),
            address_street: address_components.get(0).map(|s| s.to_string()),
            address_city: address_components.get(2).map(|s| s.to_string()),
            address_state: address_components.get(3).map(|s| s.to_string()),
            address_zip: address_components.get(4).map(|s| s.to_string()),
            phone_number: pid.get_field_as_string(12),
        });
    }
    
    Ok(patients)
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatientInfo {
    pub set_id: Option<String>,
    pub patient_id: Option<String>,
    pub patient_identifier_list: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub phone_number: Option<String>,
}

fn extract_separators(msh_line: &str) -> Result<(char, char, char, char, char), HL7Error> {
    if msh_line.len() < 9 {
        return Err(HL7Error::new("MSH segment too short to extract separators"));
    }

    let field_sep = msh_line.chars().nth(3).unwrap_or('|');
    let encoding = &msh_line[4..8];
    if encoding.len() != 4 {
        return Err(HL7Error::new("MSH encoding characters must be 4 chars"));
    }

    let component_sep = encoding.chars().next().unwrap_or('^');
    let repetition_sep = encoding.chars().nth(1).unwrap_or('~');
    let escape_char = encoding.chars().nth(2).unwrap_or('\\');
    let subcomponent_sep = encoding.chars().nth(3).unwrap_or('&');

    Ok((
        field_sep,
        component_sep,
        repetition_sep,
        escape_char,
        subcomponent_sep,
    ))
}

fn parse_msh_fields(line: &str, field_sep: char) -> Result<Vec<String>, HL7Error> {
    if line.len() < 4 {
        return Err(HL7Error::new("MSH segment too short"));
    }

    let mut fields = Vec::new();
    fields.push(line[3..4].to_string());

    let rest = &line[4..];
    let field_parts: Vec<&str> = rest.split(field_sep).collect();
    for part in field_parts {
        fields.push(part.to_string());
    }

    Ok(fields)
}

fn parse_segment_fields(line: &str, field_sep: char) -> Result<Vec<String>, HL7Error> {
    if line.len() < 4 {
        return Err(HL7Error::new("segment too short"));
    }

    let rest = &line[4..];
    let field_parts: Vec<&str> = rest.split(field_sep).collect();
    let mut fields = Vec::new();

    for part in field_parts {
        fields.push(part.to_string());
    }

    Ok(fields)
}

#[derive(Debug, Clone, PartialEq)]
pub struct HL7Error {
    pub message: String,
}

impl HL7Error {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

/// Parse one HL7 field into repetitions/components/subcomponents using separators ~, ^, &.
/// Returns a 3-level nested vector: [repetition][component][subcomponent]
pub fn parse_field_components(field: &str) -> Vec<Vec<Vec<String>>> {
    field
        .split('~')
        .map(|rep| {
            rep.split('^')
                .map(|comp| {
                    comp.split('&')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .collect::<Vec<Vec<Vec<String>>>>()
}

/// Escape special characters in HL7 fields
pub fn escape_hl7_field(input: &str) -> String {
    input
        .replace('\\', "\\E")
        .replace('|', "\\F")
        .replace('^', "\\S")
        .replace('~', "\\R")
        .replace('&', "\\T")
}

/// Unescape special characters in HL7 fields
pub fn unescape_hl7_field(input: &str) -> String {
    input
        .replace("\\E", "\\")
        .replace("\\F", "|")
        .replace("\\S", "^")
        .replace("\\R", "~")
        .replace("\\T", "&")
}

/// Parse HL7 timestamp to datetime
pub fn parse_hl7_timestamp(timestamp: &str) -> Result<chrono::NaiveDateTime, HL7Error> {
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    
    let timestamp = timestamp.trim();
    
    // HL7 timestamp format: YYYYMMDDHHMMSS[.SSSS][+/-ZZZZ]
    if timestamp.len() < 8 {
        return Err(HL7Error::new("Timestamp too short"));
    }
    
    let year: i32 = timestamp[0..4].parse().map_err(|_| HL7Error::new("Invalid year"))?;
    let month: u32 = timestamp[4..6].parse().map_err(|_| HL7Error::new("Invalid month"))?;
    let day: u32 = timestamp[6..8].parse().map_err(|_| HL7Error::new("Invalid day"))?;
    
    let hour = if timestamp.len() >= 10 {
        timestamp[8..10].parse().map_err(|_| HL7Error::new("Invalid hour"))?
    } else {
        0
    };
    
    let minute = if timestamp.len() >= 12 {
        timestamp[10..12].parse().map_err(|_| HL7Error::new("Invalid minute"))?
    } else {
        0
    };
    
    let second = if timestamp.len() >= 14 {
        timestamp[12..14].parse().map_err(|_| HL7Error::new("Invalid second"))?
    } else {
        0
    };
    
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| HL7Error::new("Invalid date"))?;
    
    let time = NaiveTime::from_hms_opt(hour, minute, second)
        .ok_or_else(|| HL7Error::new("Invalid time"))?;
    
    Ok(NaiveDateTime::new(date, time))
}

/// Format datetime to HL7 timestamp
pub fn format_hl7_timestamp(dt: chrono::NaiveDateTime) -> String {
    dt.format("%Y%m%d%H%M%S").to_string()
}

/// Parse HL7 date to NaiveDate
pub fn parse_hl7_date(date: &str) -> Result<chrono::NaiveDate, HL7Error> {
    use chrono::NaiveDate;
    
    let date = date.trim();
    
    if date.len() < 8 {
        return Err(HL7Error::new("Date too short"));
    }
    
    let year: i32 = date[0..4].parse().map_err(|_| HL7Error::new("Invalid year"))?;
    let month: u32 = date[4..6].parse().map_err(|_| HL7Error::new("Invalid month"))?;
    let day: u32 = date[6..8].parse().map_err(|_| HL7Error::new("Invalid day"))?;
    
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| HL7Error::new("Invalid date"))
}

/// Format NaiveDate to HL7 date
pub fn format_hl7_date(date: chrono::NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};
    
    #[test]
    fn test_parse_hl7_message() {
        let hl7 = "MSH|^~\\&|SRC|FAC|DST|HOSP|202501010101||ADT^A01|123|P|2.5\rPID|1|ALTID|P12345||Doe^Jane||19851224|F\r";
        
        let message = parse_hl7(hl7).unwrap();
        
        assert_eq!(message.version, "2.5");
        assert_eq!(message.segments.len(), 2);
        assert_eq!(message.segments[0].name, "MSH");
        assert_eq!(message.segments[1].name, "PID");
        
        if let Some(msg_type) = &message.message_type {
            assert_eq!(msg_type.event, "ADT");
            assert_eq!(msg_type.trigger, "A01");
        }
    }
    
    #[test]
    fn test_generate_hl7_message() {
        let mut message = create_hl7_message();
        
        add_msh_segment(
            &mut message,
            "SRC",
            "FAC",
            "DST",
            "HOSP",
            "202501010101",
            "ADT^A01",
            "123",
            "P",
            "2.5",
        ).unwrap();
        
        add_pid_segment(
            &mut message,
            "1",
            "ALTID",
            "P12345",
            "Doe^Jane",
            "",
            "19851224",
            "F",
            "",
            "",
        ).unwrap();
        
        let generated = generate_hl7(&message).unwrap();
        let parsed = parse_hl7(&generated).unwrap();
        
        assert_eq!(parsed.segments.len(), 2);
        assert_eq!(parsed.segments[0].name, "MSH");
        assert_eq!(parsed.segments[1].name, "PID");
    }
    
    #[test]
    fn test_validate_hl7_message() {
        let mut message = create_hl7_message();
        
        // Invalid: no segments
        assert!(validate_hl7_message(&message).is_err());
        
        // Invalid: starts with non-MSH segment
        message.segments.push(HL7Segment {
            name: "PID".to_string(),
            fields: vec!["1".to_string()],
        });
        assert!(validate_hl7_message(&message).is_err());
        
        // Valid: starts with MSH
        message.segments.clear();
        add_msh_segment(
            &mut message,
            "SRC",
            "FAC",
            "DST",
            "HOSP",
            "202501010101",
            "ADT^A01",
            "123",
            "P",
            "2.5",
        ).unwrap();
        assert!(validate_hl7_message(&message).is_ok());
    }
    
    #[test]
    fn test_extract_patient_info() {
        let mut message = create_hl7_message();
        add_msh_segment(
            &mut message,
            "SRC",
            "FAC",
            "DST",
            "HOSP",
            "202501010101",
            "ADT^A01",
            "123",
            "P",
            "2.5",
        ).unwrap();
        
        add_pid_segment(
            &mut message,
            "1",
            "ALTID",
            "P12345",
            "Doe^Jane^Elizabeth",
            "",
            "19851224",
            "F",
            "123 Main St^^Anytown^CA^12345",
            "555-123-4567",
        ).unwrap();
        
        let patients = extract_patient_info(&message).unwrap();
        assert_eq!(patients.len(), 1);
        
        let patient = &patients[0];
        assert_eq!(patient.last_name, Some("Doe".to_string()));
        assert_eq!(patient.first_name, Some("Jane".to_string()));
        assert_eq!(patient.middle_name, Some("Elizabeth".to_string()));
        assert_eq!(patient.birth_date, Some("19851224".to_string()));
        assert_eq!(patient.gender, Some("F".to_string()));
        assert_eq!(patient.address_street, Some("123 Main St".to_string()));
        assert_eq!(patient.address_city, Some("Anytown".to_string()));
        assert_eq!(patient.address_state, Some("CA".to_string()));
        assert_eq!(patient.address_zip, Some("12345".to_string()));
    }
    
    #[test]
    fn test_escape_unescape() {
        let original = "Field with |special^characters~and&subcomponents";
        let escaped = escape_hl7_field(original);
        let unescaped = unescape_hl7_field(&escaped);
        
        assert_eq!(original, unescaped);
        assert!(escaped.contains("\\F"));
        assert!(escaped.contains("\\S"));
        assert!(escaped.contains("\\R"));
        assert!(escaped.contains("\\T"));
    }
    
    #[test]
    fn test_timestamp_parsing() {
        let timestamp = "20250101010130";
        let parsed = parse_hl7_timestamp(timestamp).unwrap();
        
        assert_eq!(parsed.year(), 2025);
        assert_eq!(parsed.month(), 1);
        assert_eq!(parsed.day(), 1);
        assert_eq!(parsed.hour(), 1);
        assert_eq!(parsed.minute(), 1);
        assert_eq!(parsed.second(), 30);
        
        let formatted = format_hl7_timestamp(parsed);
        assert_eq!(formatted, timestamp);
    }
    
    #[test]
    fn test_date_parsing() {
        let date = "20251224";
        let parsed = parse_hl7_date(date).unwrap();
        
        assert_eq!(parsed.year(), 2025);
        assert_eq!(parsed.month(), 12);
        assert_eq!(parsed.day(), 24);
        
        let formatted = format_hl7_date(parsed);
        assert_eq!(formatted, date);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DicomObject {
    pub has_preamble: bool,
    pub magic_ok: bool,
    pub tags: Vec<DicomTag>,
    pub metadata: DicomMetadata,
    pub image_data: Option<DicomImage>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DicomTag {
    pub group: u16,
    pub element: u16,
    pub vr: String,
    pub value_length: u32,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DicomMetadata {
    pub patient_name: Option<String>,
    pub patient_id: Option<String>,
    pub patient_birth_date: Option<String>,
    pub patient_sex: Option<String>,
    pub study_instance_uid: Option<String>,
    pub series_instance_uid: Option<String>,
    pub sop_instance_uid: Option<String>,
    pub study_date: Option<String>,
    pub series_date: Option<String>,
    pub modality: Option<String>,
    pub body_part_examined: Option<String>,
    pub manufacturer: Option<String>,
    pub institution_name: Option<String>,
    pub rows: Option<u32>,
    pub columns: Option<u32>,
    pub bits_allocated: Option<u16>,
    pub bits_stored: Option<u16>,
    pub high_bit: Option<u16>,
    pub pixel_representation: Option<u16>,
    pub samples_per_pixel: Option<u16>,
    pub photometric_interpretation: Option<String>,
    pub planar_configuration: Option<u16>,
    pub window_center: Option<f64>,
    pub window_width: Option<f64>,
    pub rescale_intercept: Option<f64>,
    pub rescale_slope: Option<f64>,
    pub pixel_spacing: Option<(f64, f64)>,
    pub slice_thickness: Option<f64>,
    pub slice_location: Option<f64>,
    pub image_position: Option<[f64; 3]>,
    pub image_orientation: Option<[f64; 6]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DicomImage {
    pub width: u32,
    pub height: u32,
    pub depth: u16,
    pub samples_per_pixel: u16,
    pub photometric_interpretation: String,
    pub pixel_data: Vec<u8>,
    pub window_center: Option<f64>,
    pub window_width: Option<f64>,
    pub rescale_intercept: Option<f64>,
    pub rescale_slope: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DicomVR {
    // Application Entity
    AE,
    // Age String
    AS,
    // Attribute Tag
    AT,
    // Code String
    CS,
    // Date
    DA,
    // Decimal String
    DS,
    // Date Time
    DT,
    // Floating Point Double
    FD,
    // Floating Point Single
    FL,
    // Integer String
    IS,
    // Long String
    LO,
    // Long Text
    LT,
    // Other Byte
    OB,
    // Other Double
    OD,
    // Other Float
    OF,
    // Other Long
    OL,
    // Other Word
    OW,
    // Person Name
    PN,
    // Short String
    SH,
    // Signed Long
    SL,
    // Sequence of Items
    SQ,
    // Signed Short
    SS,
    // Short Text
    ST,
    // Time
    TM,
    // Unique Identifier (UID)
    UI,
    // Unsigned Long
    UL,
    // Unknown
    UN,
    // Universal Resource Identifier
    UR,
    // Unsigned Short
    US,
    // Unlimited Text
    UT,
}

impl std::str::FromStr for DicomVR {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AE" => Ok(DicomVR::AE),
            "AS" => Ok(DicomVR::AS),
            "AT" => Ok(DicomVR::AT),
            "CS" => Ok(DicomVR::CS),
            "DA" => Ok(DicomVR::DA),
            "DS" => Ok(DicomVR::DS),
            "DT" => Ok(DicomVR::DT),
            "FD" => Ok(DicomVR::FD),
            "FL" => Ok(DicomVR::FL),
            "IS" => Ok(DicomVR::IS),
            "LO" => Ok(DicomVR::LO),
            "LT" => Ok(DicomVR::LT),
            "OB" => Ok(DicomVR::OB),
            "OD" => Ok(DicomVR::OD),
            "OF" => Ok(DicomVR::OF),
            "OL" => Ok(DicomVR::OL),
            "OW" => Ok(DicomVR::OW),
            "PN" => Ok(DicomVR::PN),
            "SH" => Ok(DicomVR::SH),
            "SL" => Ok(DicomVR::SL),
            "SQ" => Ok(DicomVR::SQ),
            "SS" => Ok(DicomVR::SS),
            "ST" => Ok(DicomVR::ST),
            "TM" => Ok(DicomVR::TM),
            "UI" => Ok(DicomVR::UI),
            "UL" => Ok(DicomVR::UL),
            "UN" => Ok(DicomVR::UN),
            "UR" => Ok(DicomVR::UR),
            "US" => Ok(DicomVR::US),
            "UT" => Ok(DicomVR::UT),
            _ => Err(()),
        }
    }
}

// Standard DICOM tags
pub mod tags {
    // Patient Information
    pub const PATIENT_NAME: (u16, u16) = (0x0010, 0x0010);
    pub const PATIENT_ID: (u16, u16) = (0x0010, 0x0020);
    pub const PATIENT_BIRTH_DATE: (u16, u16) = (0x0010, 0x0030);
    pub const PATIENT_SEX: (u16, u16) = (0x0010, 0x0040);

    // Study Information
    pub const STUDY_INSTANCE_UID: (u16, u16) = (0x0020, 0x000D);
    pub const STUDY_DATE: (u16, u16) = (0x0008, 0x0020);
    pub const STUDY_TIME: (u16, u16) = (0x0008, 0x0030);
    pub const STUDY_DESCRIPTION: (u16, u16) = (0x0008, 0x1030);

    // Series Information
    pub const SERIES_INSTANCE_UID: (u16, u16) = (0x0020, 0x000E);
    pub const SERIES_NUMBER: (u16, u16) = (0x0020, 0x0011);
    pub const SERIES_DATE: (u16, u16) = (0x0008, 0x0021);
    pub const SERIES_TIME: (u16, u16) = (0x0008, 0x0031);
    pub const SERIES_DESCRIPTION: (u16, u16) = (0x0008, 0x103E);
    pub const MODALITY: (u16, u16) = (0x0008, 0x0060);
    pub const BODY_PART_EXAMINED: (u16, u16) = (0x0018, 0x0015);

    // Instance Information
    pub const SOP_INSTANCE_UID: (u16, u16) = (0x0008, 0x0018);
    pub const SOP_CLASS_UID: (u16, u16) = (0x0008, 0x0016);
    pub const INSTANCE_NUMBER: (u16, u16) = (0x0020, 0x0013);

    // Equipment Information
    pub const MANUFACTURER: (u16, u16) = (0x0008, 0x0070);
    pub const INSTITUTION_NAME: (u16, u16) = (0x0008, 0x0080);

    // Image Information
    pub const ROWS: (u16, u16) = (0x0028, 0x0010);
    pub const COLUMNS: (u16, u16) = (0x0028, 0x0011);
    pub const BITS_ALLOCATED: (u16, u16) = (0x0028, 0x0100);
    pub const BITS_STORED: (u16, u16) = (0x0028, 0x0101);
    pub const HIGH_BIT: (u16, u16) = (0x0028, 0x0102);
    pub const PIXEL_REPRESENTATION: (u16, u16) = (0x0028, 0x0103);
    pub const SAMPLES_PER_PIXEL: (u16, u16) = (0x0028, 0x0002);
    pub const PHOTOMETRIC_INTERPRETATION: (u16, u16) = (0x0028, 0x0004);
    pub const PLANAR_CONFIGURATION: (u16, u16) = (0x0028, 0x0006);
    pub const PIXEL_SPACING: (u16, u16) = (0x0028, 0x0030);
    pub const SLICE_THICKNESS: (u16, u16) = (0x0018, 0x0050);
    pub const SLICE_LOCATION: (u16, u16) = (0x0020, 0x1041);
    pub const IMAGE_POSITION_PATIENT: (u16, u16) = (0x0020, 0x0032);
    pub const IMAGE_ORIENTATION_PATIENT: (u16, u16) = (0x0020, 0x0037);

    // Pixel Data
    pub const PIXEL_DATA: (u16, u16) = (0x7FE0, 0x0010);

    // Window/Level
    pub const WINDOW_CENTER: (u16, u16) = (0x0028, 0x1050);
    pub const WINDOW_WIDTH: (u16, u16) = (0x0028, 0x1051);

    // Rescale
    pub const RECALE_INTERCEPT: (u16, u16) = (0x0028, 0x1052);
    pub const RECALE_SLOPE: (u16, u16) = (0x0028, 0x1053);
}

pub fn parse_dicom(bytes: &[u8]) -> Result<DicomObject, DicomError> {
    if bytes.len() < 132 {
        return Err(DicomError::new("DICOM file too small"));
    }

    let magic_ok = &bytes[128..132] == b"DICM";
    if !magic_ok {
        return Err(DicomError::new("Missing DICM magic"));
    }

    if bytes.len() < 140 {
        return Err(DicomError::new("DICOM truncated after preamble"));
    }

    let mut tags = Vec::new();
    let mut pos = 132;

    while pos + 8 <= bytes.len() {
        let group = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]);
        let element = u16::from_le_bytes([bytes[pos + 2], bytes[pos + 3]]);

        if group == 0 && element == 0 && pos == 132 {
            return Err(DicomError::new("Invalid first tag (0x0000,0x0000)"));
        }

        let vr0 = bytes[pos + 4];
        let vr1 = bytes[pos + 5];

        if !(vr0.is_ascii_uppercase() && vr1.is_ascii_uppercase()) {
            break;
        }

        let vr = format!("{}{}", vr0 as char, vr1 as char);

        let (value_length, tag_header_len) = parse_vr_length(bytes, pos + 6, &vr)?;

        let value_start = pos + 6 + tag_header_len;
        let value_end = value_start + value_length as usize;

        if value_end > bytes.len() {
            break;
        }

        let value = bytes[value_start..value_end].to_vec();

        tags.push(DicomTag {
            group,
            element,
            vr,
            value_length,
            value,
        });

        pos = value_end;
    }

    // Extract metadata from tags
    let metadata = extract_metadata(&tags)?;

    // Extract image data if available (handle gracefully if metadata is missing)
    let image_data = extract_image_data(&tags, &metadata).ok().flatten();

    Ok(DicomObject {
        has_preamble: true,
        magic_ok,
        tags,
        metadata,
        image_data,
    })
}

/// Extract metadata from DICOM tags
fn extract_metadata(tags: &[DicomTag]) -> Result<DicomMetadata, DicomError> {
    let mut metadata = DicomMetadata {
        patient_name: None,
        patient_id: None,
        patient_birth_date: None,
        patient_sex: None,
        study_instance_uid: None,
        series_instance_uid: None,
        sop_instance_uid: None,
        study_date: None,
        series_date: None,
        modality: None,
        body_part_examined: None,
        manufacturer: None,
        institution_name: None,
        rows: None,
        columns: None,
        bits_allocated: None,
        bits_stored: None,
        high_bit: None,
        pixel_representation: None,
        samples_per_pixel: None,
        photometric_interpretation: None,
        planar_configuration: None,
        window_center: None,
        window_width: None,
        rescale_intercept: None,
        rescale_slope: None,
        pixel_spacing: None,
        slice_thickness: None,
        slice_location: None,
        image_position: None,
        image_orientation: None,
    };

    for tag in tags {
        match (tag.group, tag.element) {
            tags::PATIENT_NAME => {
                metadata.patient_name = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::PATIENT_ID => {
                metadata.patient_id = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::PATIENT_BIRTH_DATE => {
                metadata.patient_birth_date = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::PATIENT_SEX => {
                metadata.patient_sex = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::STUDY_INSTANCE_UID => {
                metadata.study_instance_uid = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::SERIES_INSTANCE_UID => {
                metadata.series_instance_uid = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::SOP_INSTANCE_UID => {
                metadata.sop_instance_uid = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::STUDY_DATE => {
                metadata.study_date = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::SERIES_DATE => {
                metadata.series_date = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::MODALITY => {
                metadata.modality = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::BODY_PART_EXAMINED => {
                metadata.body_part_examined = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::MANUFACTURER => {
                metadata.manufacturer = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::INSTITUTION_NAME => {
                metadata.institution_name = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::ROWS if tag.value.len() >= 2 => {
                metadata.rows = Some(u16::from_le_bytes([tag.value[0], tag.value[1]]) as u32);
            }
            tags::COLUMNS if tag.value.len() >= 2 => {
                metadata.columns = Some(u16::from_le_bytes([tag.value[0], tag.value[1]]) as u32);
            }
            tags::BITS_ALLOCATED if tag.value.len() >= 2 => {
                metadata.bits_allocated = Some(u16::from_le_bytes([tag.value[0], tag.value[1]]));
            }
            tags::BITS_STORED if tag.value.len() >= 2 => {
                metadata.bits_stored = Some(u16::from_le_bytes([tag.value[0], tag.value[1]]));
            }
            tags::HIGH_BIT if tag.value.len() >= 2 => {
                metadata.high_bit = Some(u16::from_le_bytes([tag.value[0], tag.value[1]]));
            }
            tags::PIXEL_REPRESENTATION if tag.value.len() >= 2 => {
                metadata.pixel_representation =
                    Some(u16::from_le_bytes([tag.value[0], tag.value[1]]));
            }
            tags::SAMPLES_PER_PIXEL if tag.value.len() >= 2 => {
                metadata.samples_per_pixel = Some(u16::from_le_bytes([tag.value[0], tag.value[1]]));
            }
            tags::PHOTOMETRIC_INTERPRETATION => {
                metadata.photometric_interpretation = Some(
                    String::from_utf8_lossy(&tag.value)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            tags::PLANAR_CONFIGURATION if tag.value.len() >= 2 => {
                metadata.planar_configuration =
                    Some(u16::from_le_bytes([tag.value[0], tag.value[1]]));
            }
            tags::WINDOW_CENTER => {
                metadata.window_center = parse_ds_string(&String::from_utf8_lossy(&tag.value));
            }
            tags::WINDOW_WIDTH => {
                metadata.window_width = parse_ds_string(&String::from_utf8_lossy(&tag.value));
            }
            tags::RECALE_INTERCEPT => {
                metadata.rescale_intercept = parse_ds_string(&String::from_utf8_lossy(&tag.value));
            }
            tags::RECALE_SLOPE => {
                metadata.rescale_slope = parse_ds_string(&String::from_utf8_lossy(&tag.value));
            }
            tags::PIXEL_SPACING => {
                metadata.pixel_spacing = parse_ds_pair(&String::from_utf8_lossy(&tag.value));
            }
            tags::SLICE_THICKNESS => {
                metadata.slice_thickness = parse_ds_string(&String::from_utf8_lossy(&tag.value));
            }
            tags::SLICE_LOCATION => {
                metadata.slice_location = parse_ds_string(&String::from_utf8_lossy(&tag.value));
            }
            tags::IMAGE_POSITION_PATIENT => {
                if let Some(arr) = parse_ds_array(&String::from_utf8_lossy(&tag.value)) {
                    metadata.image_position = Some([arr[0], arr[1], arr[2]]);
                }
            }
            tags::IMAGE_ORIENTATION_PATIENT => {
                metadata.image_orientation = parse_ds_array(&String::from_utf8_lossy(&tag.value));
            }
            _ => {}
        }
    }

    Ok(metadata)
}

/// Extract image data from DICOM tags
fn extract_image_data(
    tags: &[DicomTag],
    metadata: &DicomMetadata,
) -> Result<Option<DicomImage>, DicomError> {
    let pixel_data_tag = tags
        .iter()
        .find(|tag| tag.group == tags::PIXEL_DATA.0 && tag.element == tags::PIXEL_DATA.1);

    if let Some(pixel_tag) = pixel_data_tag {
        let width = metadata
            .columns
            .ok_or_else(|| DicomError::new("Missing columns for image"))?;
        let height = metadata
            .rows
            .ok_or_else(|| DicomError::new("Missing rows for image"))?;
        let bits_allocated = metadata.bits_allocated.unwrap_or(16);
        let samples_per_pixel = metadata.samples_per_pixel.unwrap_or(1);
        let photometric_interpretation = metadata
            .photometric_interpretation
            .clone()
            .unwrap_or_else(|| "MONOCHROME2".to_string());

        let image = DicomImage {
            width,
            height,
            depth: bits_allocated,
            samples_per_pixel,
            photometric_interpretation,
            pixel_data: pixel_tag.value.clone(),
            window_center: metadata.window_center,
            window_width: metadata.window_width,
            rescale_intercept: metadata.rescale_intercept,
            rescale_slope: metadata.rescale_slope,
        };

        Ok(Some(image))
    } else {
        Ok(None)
    }
}

/// Parse DS (Decimal String) value
fn parse_ds_string(s: &str) -> Option<f64> {
    s.trim().parse().ok()
}

/// Parse DS pair (e.g., "0.5\\0.5")
fn parse_ds_pair(s: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = s.trim().split('\\').collect();
    if parts.len() >= 2 {
        let x = parts[0].trim().parse().ok()?;
        let y = parts[1].trim().parse().ok()?;
        Some((x, y))
    } else {
        None
    }
}

/// Parse DS array (e.g., "1.0\\2.0\\3.0")
fn parse_ds_array(s: &str) -> Option<[f64; 6]> {
    let parts: Vec<&str> = s.trim().split('\\').collect();
    if parts.len() >= 6 {
        let mut array = [0.0; 6];
        for (i, part) in parts.iter().take(6).enumerate() {
            array[i] = part.trim().parse().ok()?;
        }
        Some(array)
    } else if parts.len() >= 3 {
        let mut array = [0.0; 6];
        for (i, part) in parts.iter().take(3).enumerate() {
            array[i] = part.trim().parse().ok()?;
        }
        Some(array)
    } else {
        None
    }
}

/// Get a tag value by group and element
pub fn get_tag_value(tags: &[DicomTag], group: u16, element: u16) -> Option<&[u8]> {
    tags.iter()
        .find(|tag| tag.group == group && tag.element == element)
        .map(|tag| tag.value.as_slice())
}

/// Get a tag value as string
pub fn get_tag_value_string(tags: &[DicomTag], group: u16, element: u16) -> Option<String> {
    get_tag_value(tags, group, element).map(|value| {
        String::from_utf8_lossy(value)
            .trim_end_matches('\0')
            .to_string()
    })
}

/// Get a tag value as u16
pub fn get_tag_value_u16(tags: &[DicomTag], group: u16, element: u16) -> Option<u16> {
    get_tag_value(tags, group, element).and_then(|value| {
        if value.len() >= 2 {
            Some(u16::from_le_bytes([value[0], value[1]]))
        } else {
            None
        }
    })
}

/// Get a tag value as u32
pub fn get_tag_value_u32(tags: &[DicomTag], group: u16, element: u16) -> Option<u32> {
    get_tag_value(tags, group, element).and_then(|value| {
        if value.len() >= 4 {
            Some(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
        } else {
            None
        }
    })
}

/// Convert DICOM image to grayscale pixel array
pub fn dicom_to_grayscale(image: &DicomImage) -> Result<Vec<u8>, DicomError> {
    let pixel_count = (image.width * image.height) as usize;
    let mut grayscale_pixels = Vec::with_capacity(pixel_count);

    match image.depth {
        8 => {
            // 8-bit grayscale
            for pixel in &image.pixel_data {
                grayscale_pixels.push(*pixel);
            }
        }
        16 => {
            // 16-bit grayscale
            for chunk in image.pixel_data.chunks(2) {
                if chunk.len() == 2 {
                    let value = u16::from_le_bytes([chunk[0], chunk[1]]);
                    let normalized = ((value as f64 / 65535.0) * 255.0) as u8;
                    grayscale_pixels.push(normalized);
                }
            }
        }
        _ => {
            return Err(DicomError::new(format!(
                "Unsupported bit depth: {}",
                image.depth
            )));
        }
    }

    Ok(grayscale_pixels)
}

/// Apply window/level to DICOM image
pub fn apply_window_level(
    image: &DicomImage,
    window_center: f64,
    window_width: f64,
) -> Result<Vec<u8>, DicomError> {
    let pixel_count = (image.width * image.height) as usize;
    let mut windowed_pixels = Vec::with_capacity(pixel_count);

    let min_val = window_center - window_width / 2.0;
    let max_val = window_center + window_width / 2.0;

    match image.depth {
        8 => {
            for pixel in &image.pixel_data {
                let value = *pixel as f64;
                let windowed =
                    ((value - min_val) / (max_val - min_val) * 255.0).clamp(0.0, 255.0) as u8;
                windowed_pixels.push(windowed);
            }
        }
        16 => {
            for chunk in image.pixel_data.chunks(2) {
                if chunk.len() == 2 {
                    let value = u16::from_le_bytes([chunk[0], chunk[1]]) as f64;
                    let windowed =
                        ((value - min_val) / (max_val - min_val) * 255.0).clamp(0.0, 255.0) as u8;
                    windowed_pixels.push(windowed);
                }
            }
        }
        _ => {
            return Err(DicomError::new(format!(
                "Unsupported bit depth: {}",
                image.depth
            )));
        }
    }

    Ok(windowed_pixels)
}

/// Apply rescale slope and intercept to pixel values
pub fn apply_rescale(image: &DicomImage) -> Result<Vec<f64>, DicomError> {
    let slope = image.rescale_slope.unwrap_or(1.0);
    let intercept = image.rescale_intercept.unwrap_or(0.0);
    let pixel_count = (image.width * image.height) as usize;
    let mut rescaled_pixels = Vec::with_capacity(pixel_count);

    match image.depth {
        8 => {
            for pixel in &image.pixel_data {
                let value = *pixel as f64;
                let rescaled = value * slope + intercept;
                rescaled_pixels.push(rescaled);
            }
        }
        16 => {
            for chunk in image.pixel_data.chunks(2) {
                if chunk.len() == 2 {
                    let value = u16::from_le_bytes([chunk[0], chunk[1]]) as f64;
                    let rescaled = value * slope + intercept;
                    rescaled_pixels.push(rescaled);
                }
            }
        }
        _ => {
            return Err(DicomError::new(format!(
                "Unsupported bit depth: {}",
                image.depth
            )));
        }
    }

    Ok(rescaled_pixels)
}

/// Create a simple DICOM file for testing
pub fn create_test_dicom() -> DicomObject {
    let tags = vec![
        DicomTag {
            group: tags::PATIENT_NAME.0,
            element: tags::PATIENT_NAME.1,
            vr: "LO".to_string(),
            value_length: 6,
            value: b"Test^P".to_vec(),
        },
        DicomTag {
            group: tags::PATIENT_ID.0,
            element: tags::PATIENT_ID.1,
            vr: "LO".to_string(),
            value_length: 8,
            value: b"12345678".to_vec(),
        },
        DicomTag {
            group: tags::ROWS.0,
            element: tags::ROWS.1,
            vr: "US".to_string(),
            value_length: 2,
            value: (64u16).to_le_bytes().to_vec(),
        },
        DicomTag {
            group: tags::COLUMNS.0,
            element: tags::COLUMNS.1,
            vr: "US".to_string(),
            value_length: 2,
            value: (64u16).to_le_bytes().to_vec(),
        },
    ];

    let metadata = extract_metadata(&tags).unwrap();
    let image_data = None;

    DicomObject {
        has_preamble: true,
        magic_ok: true,
        tags,
        metadata,
        image_data,
    }
}

fn parse_vr_length(bytes: &[u8], pos: usize, vr: &str) -> Result<(u32, usize), DicomError> {
    if pos + 2 > bytes.len() {
        return Err(DicomError::new("VR length field truncated"));
    }

    match vr {
        "OB" | "OD" | "OF" | "OL" | "OW" | "UN" | "UC" | "UR" | "UT" => {
            if pos + 6 > bytes.len() {
                return Err(DicomError::new("Explicit VR with reserved bytes truncated"));
            }
            let length = u32::from_le_bytes([
                bytes[pos + 2],
                bytes[pos + 3],
                bytes[pos + 4],
                bytes[pos + 5],
            ]);
            Ok((length, 6))
        }
        _ => {
            let length = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as u32;
            Ok((length, 2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dicom_vr_from_str() {
        assert_eq!("AE".parse(), Ok(DicomVR::AE));
        assert_eq!("US".parse(), Ok(DicomVR::US));
        assert!("XX".parse::<DicomVR>().is_err());
    }

    #[test]
    fn test_parse_ds_string() {
        assert_eq!(parse_ds_string("123.45"), Some(123.45));
        assert_eq!(parse_ds_string("  123.45  "), Some(123.45));
        assert_eq!(parse_ds_string("invalid"), None);
    }

    #[test]
    fn test_parse_ds_pair() {
        assert_eq!(parse_ds_pair("0.5\\0.75"), Some((0.5, 0.75)));
        assert_eq!(parse_ds_pair("  0.5  \\  0.75  "), Some((0.5, 0.75)));
        assert_eq!(parse_ds_pair("0.5"), None);
        assert_eq!(parse_ds_pair("invalid\\invalid"), None);
    }

    #[test]
    fn test_parse_ds_array() {
        assert_eq!(
            parse_ds_array("1.0\\2.0\\3.0\\4.0\\5.0\\6.0"),
            Some([1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
        );
        assert_eq!(
            parse_ds_array("1.0\\2.0\\3.0"),
            Some([1.0, 2.0, 3.0, 0.0, 0.0, 0.0])
        );
        assert_eq!(parse_ds_array("1.0\\2.0"), None);
    }

    #[test]
    fn test_get_tag_value() {
        let tags = vec![
            DicomTag {
                group: 0x0010,
                element: 0x0010,
                vr: "LO".to_string(),
                value_length: 8,
                value: b"TestName".to_vec(),
            },
            DicomTag {
                group: 0x0010,
                element: 0x0020,
                vr: "LO".to_string(),
                value_length: 8,
                value: b"TestID123".to_vec(),
            },
        ];

        assert_eq!(get_tag_value(&tags, 0x0010, 0x0010), Some(&b"TestName"[..]));
        assert_eq!(
            get_tag_value(&tags, 0x0010, 0x0020),
            Some(&b"TestID123"[..])
        );
        assert_eq!(get_tag_value(&tags, 0x0010, 0x0030), None);
    }

    #[test]
    fn test_get_tag_value_string() {
        let tags = vec![DicomTag {
            group: 0x0010,
            element: 0x0010,
            vr: "LO".to_string(),
            value_length: 8,
            value: b"TestName\0".to_vec(),
        }];

        assert_eq!(
            get_tag_value_string(&tags, 0x0010, 0x0010),
            Some("TestName".to_string())
        );
        assert_eq!(get_tag_value_string(&tags, 0x0010, 0x0020), None);
    }

    #[test]
    fn test_get_tag_value_u16() {
        let tags = vec![DicomTag {
            group: 0x0028,
            element: 0x0010,
            vr: "US".to_string(),
            value_length: 2,
            value: 64u16.to_le_bytes().to_vec(),
        }];

        assert_eq!(get_tag_value_u16(&tags, 0x0028, 0x0010), Some(64));
        assert_eq!(get_tag_value_u16(&tags, 0x0028, 0x0011), None);
    }

    #[test]
    fn test_create_test_dicom() {
        let dicom = create_test_dicom();

        assert!(dicom.has_preamble);
        assert!(dicom.magic_ok);
        assert_eq!(dicom.tags.len(), 4);
        assert_eq!(dicom.metadata.patient_name, Some("Test^P".to_string()));
        assert_eq!(dicom.metadata.patient_id, Some("12345678".to_string()));
        assert_eq!(dicom.metadata.rows, Some(64));
        assert_eq!(dicom.metadata.columns, Some(64));
        assert!(dicom.image_data.is_none());
    }

    #[test]
    fn test_extract_metadata() {
        let tags = vec![
            DicomTag {
                group: tags::PATIENT_NAME.0,
                element: tags::PATIENT_NAME.1,
                vr: "LO".to_string(),
                value_length: 8,
                value: b"Doe^John".to_vec(),
            },
            DicomTag {
                group: tags::ROWS.0,
                element: tags::ROWS.1,
                vr: "US".to_string(),
                value_length: 2,
                value: 128u16.to_le_bytes().to_vec(),
            },
            DicomTag {
                group: tags::COLUMNS.0,
                element: tags::COLUMNS.1,
                vr: "US".to_string(),
                value_length: 2,
                value: 256u16.to_le_bytes().to_vec(),
            },
        ];

        let metadata = extract_metadata(&tags).unwrap();

        assert_eq!(metadata.patient_name, Some("Doe^John".to_string()));
        assert_eq!(metadata.rows, Some(128));
        assert_eq!(metadata.columns, Some(256));
    }

    #[test]
    fn test_dicom_to_grayscale_8bit() {
        let image = DicomImage {
            width: 2,
            height: 2,
            depth: 8,
            samples_per_pixel: 1,
            photometric_interpretation: "MONOCHROME2".to_string(),
            pixel_data: vec![0, 128, 255, 64],
            window_center: None,
            window_width: None,
            rescale_intercept: None,
            rescale_slope: None,
        };

        let grayscale = dicom_to_grayscale(&image).unwrap();
        assert_eq!(grayscale, vec![0, 128, 255, 64]);
    }

    #[test]
    fn test_dicom_to_grayscale_16bit() {
        let image = DicomImage {
            width: 2,
            height: 1,
            depth: 16,
            samples_per_pixel: 1,
            photometric_interpretation: "MONOCHROME2".to_string(),
            pixel_data: vec![0, 0, 255, 255], // 0 and 65535 in little endian
            window_center: None,
            window_width: None,
            rescale_intercept: None,
            rescale_slope: None,
        };

        let grayscale = dicom_to_grayscale(&image).unwrap();
        assert_eq!(grayscale, vec![0, 255]);
    }

    #[test]
    fn test_apply_window_level() {
        let image = DicomImage {
            width: 4,
            height: 1,
            depth: 8,
            samples_per_pixel: 1,
            photometric_interpretation: "MONOCHROME2".to_string(),
            pixel_data: vec![0, 64, 128, 255],
            window_center: None,
            window_width: None,
            rescale_intercept: None,
            rescale_slope: None,
        };

        let windowed = apply_window_level(&image, 127.5, 255.0).unwrap();
        // Values should be scaled to 0-255 range based on window center/width
        assert_eq!(windowed.len(), 4);
    }

    #[test]
    fn test_apply_rescale() {
        let image = DicomImage {
            width: 2,
            height: 1,
            depth: 8,
            samples_per_pixel: 1,
            photometric_interpretation: "MONOCHROME2".to_string(),
            pixel_data: vec![100, 200],
            window_center: None,
            window_width: None,
            rescale_intercept: Some(10.0),
            rescale_slope: Some(2.0),
        };

        let rescaled = apply_rescale(&image).unwrap();
        assert_eq!(rescaled, vec![210.0, 410.0]); // (100*2+10, 200*2+10)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DicomError {
    pub message: String,
}

impl DicomError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

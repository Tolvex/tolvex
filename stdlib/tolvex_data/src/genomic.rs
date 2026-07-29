//! Genomic data formats support for FASTQ, VCF, and BAM files.
//!
//! This module provides comprehensive support for common genomic data formats:
//! - FASTQ: Sequencing reads with quality scores
//! - VCF: Variant Call Format for genomic variants
//! - BAM: Binary Alignment Map for sequence alignments
//!
//! # Examples
//!
//! ## FASTQ parsing
//! ```rust
//! use tolvex_data::genomic::{FastqRecord, parse_fastq};
//! let fastq_data = "@read1\nATCG\n+\nIIII\n";
//! let records = parse_fastq(fastq_data).unwrap();
//! assert_eq!(records.len(), 1);
//! assert_eq!(records[0].id, "read1");
//! assert_eq!(records[0].sequence, "ATCG");
//! ```
//!
//! ## VCF parsing
//! ```rust
//! use tolvex_data::genomic::{VcfRecord, parse_vcf};
//! let vcf_data = "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n1\t12345\t.\tA\tT\t99\tPASS\tDP=10\n";
//! let records = parse_vcf(vcf_data).unwrap();
//! assert_eq!(records.len(), 1);
//! assert_eq!(records[0].chromosome, "1");
//! assert_eq!(records[0].position, 12345);
//! ```

use std::collections::HashMap;

/// FASTQ record representing a single sequencing read
#[derive(Debug, Clone, PartialEq)]
pub struct FastqRecord {
    /// Read identifier (without @ symbol)
    pub id: String,
    /// DNA/RNA sequence
    pub sequence: String,
    /// Optional description after ID
    pub description: Option<String>,
    /// Quality scores (ASCII characters)
    pub quality: String,
}

/// VCF record representing a genomic variant
#[derive(Debug, Clone, PartialEq)]
pub struct VcfRecord {
    /// Chromosome name
    pub chromosome: String,
    /// 1-based position
    pub position: u64,
    /// Identifier (multiple separated by ;)
    pub id: String,
    /// Reference allele
    pub reference: String,
    /// Alternative alleles (comma-separated)
    pub alternative: String,
    /// Quality score (phred-scaled)
    pub quality: Option<f64>,
    /// Filter status (PASS or list of filters)
    pub filter: String,
    /// Info fields (semicolon-separated key=value pairs)
    pub info: HashMap<String, String>,
    /// Format fields (colon-separated)
    pub format: Option<String>,
    /// Sample data (for each sample)
    pub samples: Vec<HashMap<String, String>>,
}

/// BAM header information
#[derive(Debug, Clone, PartialEq)]
pub struct BamHeader {
    /// Header lines
    pub lines: Vec<String>,
    /// Reference sequences
    pub references: Vec<BamReference>,
    /// Read groups
    pub read_groups: Vec<BamReadGroup>,
}

/// BAM reference sequence
#[derive(Debug, Clone, PartialEq)]
pub struct BamReference {
    /// Reference name
    pub name: String,
    /// Reference length
    pub length: u64,
}

/// BAM read group
#[derive(Debug, Clone, PartialEq)]
pub struct BamReadGroup {
    /// Read group ID
    pub id: String,
    /// Sample name
    pub sample: Option<String>,
    /// Library name
    pub library: Option<String>,
    /// Additional fields
    pub fields: HashMap<String, String>,
}

/// BAM alignment record
#[derive(Debug, Clone, PartialEq)]
pub struct BamRecord {
    /// Query template name
    pub query_name: String,
    /// Reference sequence name
    pub reference_name: Option<String>,
    /// 1-based leftmost coordinate
    pub position: Option<u64>,
    /// Mapping quality
    pub mapping_quality: Option<u8>,
    /// CIGAR string
    pub cigar: String,
    /// Reference name of next mate/template
    pub next_reference_name: Option<String>,
    /// Position of next mate/template
    pub next_position: Option<u64>,
    /// Template length
    pub template_length: Option<i32>,
    /// Sequence
    pub sequence: String,
    /// Quality scores
    pub quality: Vec<u8>,
    /// Optional flags
    pub flags: BamFlags,
    /// Additional tags
    pub tags: HashMap<String, BamTagValue>,
}

/// BAM alignment flags
#[derive(Debug, Clone, PartialEq)]
pub struct BamFlags {
    pub read_paired: bool,
    pub read_proper_pair: bool,
    pub read_unmapped: bool,
    pub mate_unmapped: bool,
    pub read_reverse_strand: bool,
    pub mate_reverse_strand: bool,
    pub first_in_pair: bool,
    pub second_in_pair: bool,
    pub secondary_alignment: bool,
    pub read_fails_vendor_quality_check: bool,
    pub duplicate_read: bool,
    pub supplementary_alignment: bool,
}

/// BAM tag value types
#[derive(Debug, Clone, PartialEq)]
pub enum BamTagValue {
    String(String),
    Integer(i64),
    Float(f64),
    Character(char),
    Array(Vec<BamTagValue>),
}

/// Parse FASTQ format data
pub fn parse_fastq(data: &str) -> Result<Vec<FastqRecord>, GenomicError> {
    let mut records = Vec::new();
    let lines: Vec<&str> = data.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        if i + 3 >= lines.len() {
            return Err(GenomicError::new("Incomplete FASTQ record"));
        }

        let header_line = lines[i];
        let sequence_line = lines[i + 1];
        let plus_line = lines[i + 2];
        let quality_line = lines[i + 3];

        if !header_line.starts_with('@') {
            return Err(GenomicError::new("FASTQ record must start with @"));
        }

        if !plus_line.starts_with('+') {
            return Err(GenomicError::new("FASTQ quality line must start with +"));
        }

        let header = &header_line[1..];
        let mut parts = header.splitn(2, ' ');
        let id = parts.next().unwrap_or("").to_string();
        let description = parts.next().map(|s| s.to_string());

        // Validate sequence and quality lengths match
        if sequence_line.len() != quality_line.len() {
            return Err(GenomicError::new("Sequence and quality lengths must match"));
        }

        records.push(FastqRecord {
            id,
            sequence: sequence_line.to_string(),
            description,
            quality: quality_line.to_string(),
        });

        i += 4;
    }

    Ok(records)
}

/// Write FASTQ records to string
pub fn write_fastq(records: &[FastqRecord]) -> String {
    let mut output = String::new();

    for record in records {
        output.push('@');
        output.push_str(&record.id);
        if let Some(ref desc) = record.description {
            output.push(' ');
            output.push_str(desc);
        }
        output.push('\n');
        output.push_str(&record.sequence);
        output.push('\n');
        output.push('+');
        output.push('\n');
        output.push_str(&record.quality);
        output.push('\n');
    }

    output
}

/// Parse VCF (Variant Call Format) data
pub fn parse_vcf(data: &str) -> Result<Vec<VcfRecord>, GenomicError> {
    let mut records = Vec::new();

    for line in data.lines() {
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 8 {
            return Err(GenomicError::new("VCF record must have at least 8 fields"));
        }

        // Validate required fields are non-empty
        if parts[0].is_empty() {
            return Err(GenomicError::new("VCF CHROM field cannot be empty"));
        }
        if parts[1].is_empty() {
            return Err(GenomicError::new("VCF POS field cannot be empty"));
        }
        if parts[3].is_empty() || parts[3] == "." {
            return Err(GenomicError::new(
                "VCF REF field cannot be empty or missing",
            ));
        }
        if parts[4].is_empty() || parts[4] == "." {
            return Err(GenomicError::new(
                "VCF ALT field cannot be empty or missing",
            ));
        }

        let chromosome = parts[0].to_string();
        let position = parts[1]
            .parse()
            .map_err(|_| GenomicError::new("Invalid position"))?;
        let id = parts[2].to_string();
        let reference = parts[3].to_string();
        let alternative = parts[4].to_string();

        let quality = if parts[5] == "." {
            None
        } else {
            Some(
                parts[5]
                    .parse()
                    .map_err(|_| GenomicError::new("Invalid quality"))?,
            )
        };

        let filter = parts[6].to_string();
        let info_str = parts[7];

        // Parse INFO fields
        let mut info = HashMap::new();
        if !info_str.is_empty() && info_str != "." {
            for field in info_str.split(';') {
                if let Some((key, value)) = field.split_once('=') {
                    info.insert(key.to_string(), value.to_string());
                } else {
                    info.insert(field.to_string(), "true".to_string());
                }
            }
        }

        // Parse FORMAT and sample data if present
        let format = parts.get(8).map(|s| s.to_string());
        let mut samples = Vec::new();

        if let Some(format_str) = format.as_ref() {
            let format_fields: Vec<&str> = format_str.split(':').collect();

            if let Some(sample_data) = parts.get(9..) {
                for sample_line in sample_data {
                    let mut sample_data = HashMap::new();
                    let values: Vec<&str> = sample_line.split(':').collect();

                    for (i, field) in format_fields.iter().enumerate() {
                        if let Some(value) = values.get(i) {
                            sample_data.insert(field.to_string(), value.to_string());
                        }
                    }

                    samples.push(sample_data);
                }
            }
        }

        records.push(VcfRecord {
            chromosome,
            position,
            id,
            reference,
            alternative,
            quality,
            filter,
            info,
            format,
            samples,
        });
    }

    Ok(records)
}

/// Write VCF records to string
pub fn write_vcf(records: &[VcfRecord], header: &[String]) -> String {
    let mut output = String::new();

    // Write header
    for line in header {
        output.push_str(line);
        output.push('\n');
    }

    // Write records
    for record in records {
        output.push_str(&record.chromosome);
        output.push('\t');
        output.push_str(&record.position.to_string());
        output.push('\t');
        output.push_str(&record.id);
        output.push('\t');
        output.push_str(&record.reference);
        output.push('\t');
        output.push_str(&record.alternative);
        output.push('\t');

        match record.quality {
            Some(q) => output.push_str(&q.to_string()),
            None => output.push('.'),
        }
        output.push('\t');

        output.push_str(&record.filter);
        output.push('\t');

        // Write INFO fields
        if record.info.is_empty() {
            output.push('.');
        } else {
            let mut first = true;
            for (key, value) in &record.info {
                if !first {
                    output.push(';');
                }
                output.push_str(key);
                output.push('=');
                output.push_str(value);
                first = false;
            }
        }

        // Write FORMAT and sample data
        if let Some(ref format) = record.format {
            output.push('\t');
            output.push_str(format);

            for sample in &record.samples {
                output.push('\t');
                let mut first = true;
                for field in format.split(':') {
                    if !first {
                        output.push(':');
                    }
                    if let Some(value) = sample.get(field) {
                        output.push_str(value);
                    } else {
                        output.push('.');
                    }
                    first = false;
                }
            }
        }

        output.push('\n');
    }

    output
}

/// Parse BAM header from SAM header format
pub fn parse_bam_header(sam_header: &str) -> Result<BamHeader, GenomicError> {
    let mut lines = Vec::new();
    let mut references = Vec::new();
    let mut read_groups = Vec::new();

    for line in sam_header.lines() {
        if !line.starts_with('@') {
            continue;
        }

        lines.push(line.to_string());

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "@SQ" if parts.len() >= 3 => {
                let mut name = None;
                let mut length = None;

                for part in &parts[1..] {
                    if let Some((key, value)) = part.split_once(':') {
                        match key {
                            "SN" => name = Some(value.to_string()),
                            "LN" => length = value.parse().ok(),
                            _ => {}
                        }
                    }
                }

                if let (Some(name), Some(length)) = (name, length) {
                    references.push(BamReference { name, length });
                }
            }
            "@RG" if parts.len() >= 2 => {
                let mut id = None;
                let mut sample = None;
                let mut library = None;
                let mut fields = HashMap::new();

                for part in &parts[1..] {
                    if let Some((key, value)) = part.split_once(':') {
                        match key {
                            "ID" => id = Some(value.to_string()),
                            "SM" => sample = Some(value.to_string()),
                            "LB" => library = Some(value.to_string()),
                            _ => {
                                fields.insert(key.to_string(), value.to_string());
                            }
                        }
                    }
                }

                if let Some(id) = id {
                    read_groups.push(BamReadGroup {
                        id,
                        sample,
                        library,
                        fields,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(BamHeader {
        lines,
        references,
        read_groups,
    })
}

/// Create a simple FASTQ record for testing
pub fn create_test_fastq() -> FastqRecord {
    FastqRecord {
        id: "test_read".to_string(),
        sequence: "ATCGATCGATCG".to_string(),
        description: Some("test description".to_string()),
        quality: "IIIIIIIIIIII".to_string(),
    }
}

/// Create a simple VCF record for testing
pub fn create_test_vcf() -> VcfRecord {
    let mut info = HashMap::new();
    info.insert("DP".to_string(), "10".to_string());
    info.insert("AF".to_string(), "0.5".to_string());

    VcfRecord {
        chromosome: "1".to_string(),
        position: 12345,
        id: ".".to_string(),
        reference: "A".to_string(),
        alternative: "T".to_string(),
        quality: Some(99.0),
        filter: "PASS".to_string(),
        info,
        format: Some("GT:DP".to_string()),
        samples: vec![{
            let mut sample = HashMap::new();
            sample.insert("GT".to_string(), "0/1".to_string());
            sample.insert("DP".to_string(), "10".to_string());
            sample
        }],
    }
}

/// Calculate basic FASTQ statistics
pub fn fastq_stats(records: &[FastqRecord]) -> FastqStats {
    let mut total_bases = 0;
    let total_reads = records.len();
    let mut quality_sum: u64 = 0;
    let mut quality_count = 0;

    for record in records {
        total_bases += record.sequence.len();

        for q_char in record.quality.chars() {
            if let Some(q_score) = phred_score_from_char(q_char) {
                quality_sum += q_score as u64;
                quality_count += 1;
            }
        }
    }

    let avg_quality = if quality_count > 0 {
        quality_sum as f64 / quality_count as f64
    } else {
        0.0
    };

    FastqStats {
        total_reads,
        total_bases,
        average_read_length: total_bases.checked_div(total_reads).unwrap_or(0),
        average_quality: avg_quality,
    }
}

/// FASTQ statistics
#[derive(Debug, Clone, PartialEq)]
pub struct FastqStats {
    pub total_reads: usize,
    pub total_bases: usize,
    pub average_read_length: usize,
    pub average_quality: f64,
}

/// Convert Phred quality character to score
pub fn phred_score_from_char(c: char) -> Option<u8> {
    if c.is_ascii() && c >= '!' {
        Some(c as u8 - b'!')
    } else {
        None
    }
}

/// Convert Phred score to character
pub fn phred_score_to_char(score: u8) -> char {
    (score + b'!') as char
}

/// Calculate VCF statistics
pub fn vcf_stats(records: &[VcfRecord]) -> VcfStats {
    let total_variants = records.len();
    let mut snps = 0;
    let mut insertions = 0;
    let mut deletions = 0;
    let mut transitions = 0;
    let mut transversions = 0;

    for record in records {
        for alt in record.alternative.split(',') {
            if alt == "." {
                continue;
            }

            let ref_len = record.reference.len();
            let alt_len = alt.len();

            if ref_len == 1 && alt_len == 1 {
                snps += 1;

                // Check if transition or transversion
                if let (Some(ref_base), Some(alt_base)) =
                    (record.reference.chars().next(), alt.chars().next())
                {
                    if is_transition(ref_base, alt_base) {
                        transitions += 1;
                    } else {
                        transversions += 1;
                    }
                }
            } else if alt_len > ref_len {
                insertions += 1;
            } else if alt_len < ref_len {
                deletions += 1;
            }
        }
    }

    VcfStats {
        total_variants,
        snps,
        insertions,
        deletions,
        transitions,
        transversions,
    }
}

/// VCF statistics
#[derive(Debug, Clone, PartialEq)]
pub struct VcfStats {
    pub total_variants: usize,
    pub snps: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub transitions: usize,
    pub transversions: usize,
}

/// Check if a SNP is a transition (A<->G or C<->T)
fn is_transition(ref_base: char, alt_base: char) -> bool {
    matches!(
        (ref_base, alt_base),
        ('A', 'G') | ('G', 'A') | ('C', 'T') | ('T', 'C')
    )
}

/// Genomic error type
#[derive(Debug, Clone, PartialEq)]
pub struct GenomicError {
    pub message: String,
}

impl GenomicError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl std::fmt::Display for GenomicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Genomic error: {}", self.message)
    }
}

impl std::error::Error for GenomicError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fastq() {
        let fastq_data = "@read1 desc\nATCG\n+\nIIII\n@read2\nGCTA\n+\nHHHH\n";
        let records = parse_fastq(fastq_data).unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].id, "read1");
        assert_eq!(records[0].sequence, "ATCG");
        assert_eq!(records[0].description, Some("desc".to_string()));
        assert_eq!(records[0].quality, "IIII");

        assert_eq!(records[1].id, "read2");
        assert_eq!(records[1].sequence, "GCTA");
        assert_eq!(records[1].description, None);
        assert_eq!(records[1].quality, "HHHH");
    }

    #[test]
    fn test_write_fastq() {
        let records = vec![
            FastqRecord {
                id: "read1".to_string(),
                sequence: "ATCG".to_string(),
                description: Some("desc".to_string()),
                quality: "IIII".to_string(),
            },
            FastqRecord {
                id: "read2".to_string(),
                sequence: "GCTA".to_string(),
                description: None,
                quality: "HHHH".to_string(),
            },
        ];

        let output = write_fastq(&records);
        let expected = "@read1 desc\nATCG\n+\nIIII\n@read2\nGCTA\n+\nHHHH\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_vcf() {
        let vcf_data =
            "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n1\t12345\t.\tA\tT\t99\tPASS\tDP=10\n";
        let records = parse_vcf(vcf_data).unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].chromosome, "1");
        assert_eq!(records[0].position, 12345);
        assert_eq!(records[0].reference, "A");
        assert_eq!(records[0].alternative, "T");
        assert_eq!(records[0].quality, Some(99.0));
        assert_eq!(records[0].filter, "PASS");
        assert_eq!(records[0].info.get("DP"), Some(&"10".to_string()));
    }

    #[test]
    fn test_phred_score_conversion() {
        assert_eq!(phred_score_from_char('!'), Some(0));
        assert_eq!(phred_score_from_char('I'), Some(40));
        assert_eq!(phred_score_from_char('~'), Some(93)); // 126 - 33 = 93
        assert_eq!(phred_score_from_char('A'), Some(32)); // 'A' (65) - 33 = 32

        assert_eq!(phred_score_to_char(0), '!');
        assert_eq!(phred_score_to_char(40), 'I');
        assert_eq!(phred_score_to_char(93), '~'); // 93 + 33 = 126
    }

    #[test]
    fn test_fastq_stats() {
        let records = vec![
            FastqRecord {
                id: "read1".to_string(),
                sequence: "ATCG".to_string(),
                description: None,
                quality: "IIII".to_string(),
            },
            FastqRecord {
                id: "read2".to_string(),
                sequence: "GCTA".to_string(),
                description: None,
                quality: "HHHH".to_string(),
            },
        ];

        let stats = fastq_stats(&records);
        assert_eq!(stats.total_reads, 2);
        assert_eq!(stats.total_bases, 8);
        assert_eq!(stats.average_read_length, 4);
        assert_eq!(stats.average_quality, 39.5); // (4*40 + 4*39) / 8
    }

    #[test]
    fn test_vcf_stats() {
        let records = vec![
            create_test_vcf(), // SNP A->T (transversion)
        ];

        let stats = vcf_stats(&records);
        assert_eq!(stats.total_variants, 1);
        assert_eq!(stats.snps, 1);
        assert_eq!(stats.transitions, 0);
        assert_eq!(stats.transversions, 1);
    }

    #[test]
    fn test_is_transition() {
        assert!(is_transition('A', 'G'));
        assert!(is_transition('G', 'A'));
        assert!(is_transition('C', 'T'));
        assert!(is_transition('T', 'C'));

        assert!(!is_transition('A', 'C'));
        assert!(!is_transition('A', 'T'));
        assert!(!is_transition('G', 'C'));
        assert!(!is_transition('G', 'T'));
    }
}

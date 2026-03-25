//! Version management and semver enforcement

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

lazy_static! {
    static ref SEMVER_REGEX: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<pre>[0-9A-Za-z-\.]+))?(?:\+(?P<build>[0-9A-Za-z-\.]+))?$").unwrap();
}

#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Invalid semver: {0}")]
    InvalidSemver(String),
    #[error("Version conflict: {name} v{version} already exists")]
    VersionConflict { name: String, version: String },
    #[error("Pre-release not allowed: {0}")]
    PrereleaseNotAllowed(String),
    #[error("Breaking change in patch version: {0}")]
    BreakingPatch(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    pub build: Option<String>,
}

impl Version {
    pub fn parse(s: &str) -> Result<Self, VersionError> {
        let caps = SEMVER_REGEX
            .captures(s)
            .ok_or_else(|| VersionError::InvalidSemver(s.to_string()))?;

        let major = caps["major"]
            .parse()
            .map_err(|_| VersionError::InvalidSemver(s.to_string()))?;
        let minor = caps["minor"]
            .parse()
            .map_err(|_| VersionError::InvalidSemver(s.to_string()))?;
        let patch = caps["patch"]
            .parse()
            .map_err(|_| VersionError::InvalidSemver(s.to_string()))?;
        let pre = caps.name("pre").map(|m| m.as_str().to_string());
        let build = caps.name("build").map(|m| m.as_str().to_string());

        Ok(Self {
            major,
            minor,
            patch,
            pre,
            build,
        })
    }

    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }

    pub fn bump(&self, bump_type: BumpType) -> Self {
        match bump_type {
            BumpType::Major => Self {
                major: self.major + 1,
                minor: 0,
                patch: 0,
                pre: None,
                build: None,
            },
            BumpType::Minor => Self {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
                pre: None,
                build: None,
            },
            BumpType::Patch => Self {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
                pre: None,
                build: None,
            },
        }
    }

    pub fn is_compatible(&self, other: &Self) -> bool {
        // Compatible if same major and this version >= other
        self.major == other.major && self >= other
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.pre {
            write!(f, "-{}", pre)?;
        }
        if let Some(ref build) = self.build {
            write!(f, "+{}", build)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BumpType {
    Major,
    Minor,
    Patch,
}

pub struct VersionManager {
    versions: HashMap<String, Vec<Version>>,
}

impl VersionManager {
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    pub fn register_version(&mut self, name: &str, version: Version) -> Result<(), VersionError> {
        if !version.is_stable() {
            return Err(VersionError::PrereleaseNotAllowed(version.to_string()));
        }

        let entry = self
            .versions
            .entry(name.to_string())
            .or_insert_with(Vec::new);
        if entry.contains(&version) {
            return Err(VersionError::VersionConflict {
                name: name.to_string(),
                version: version.to_string(),
            });
        }

        entry.push(version.clone());
        entry.sort();
        Ok(())
    }

    pub fn latest_version(&self, name: &str) -> Option<&Version> {
        self.versions.get(name).and_then(|v| v.last())
    }

    pub fn list_versions(&self, name: &str) -> &[Version] {
        self.versions.get(name).map_or(&[], |v| v.as_slice())
    }

    pub fn suggest_next_version(&self, name: &str, bump_type: BumpType) -> Option<Version> {
        self.latest_version(name).map(|v| v.bump(bump_type))
    }

    pub fn check_compatibility(&self, name: &str, required: &Version) -> Result<(), VersionError> {
        let versions = self
            .versions
            .get(name)
            .ok_or_else(|| VersionError::InvalidSemver(format!("Package {} not found", name)))?;

        let compatible = versions.iter().any(|v| v.is_compatible(required));
        if !compatible {
            return Err(VersionError::InvalidSemver(format!(
                "No compatible version found for {} >= {}",
                name, required
            )));
        }

        Ok(())
    }
}

//! Registry client for formulary.tolvex.dev (stub)

use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Authentication required")]
    Auth,
    #[error("Version conflict: package {name} v{version} already exists")]
    VersionConflict { name: String, version: String },
    #[error("Security scan failed: {0}")]
    Security(String),
}

#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub fhir_version: Option<String>,
}

impl PackageMetadata {
    pub fn from_manifest(manifest: &crate::manifest::Manifest) -> Self {
        Self {
            name: manifest.name.clone(),
            version: manifest.version.clone(),
            description: manifest.description.clone(),
            authors: manifest.authors.clone(),
            license: manifest.license.clone(),
            dependencies: manifest
                .dependencies
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
            fhir_version: manifest.fhir.as_ref().and_then(|f| f.version.clone()),
        }
    }
}

pub struct RegistryClient {
    base_url: String,
    auth_token: Option<String>,
}

impl RegistryClient {
    pub fn new(base_url: impl Into<String>, auth_token: Option<String>) -> Self {
        Self {
            base_url: base_url.into(),
            auth_token,
        }
    }

    pub async fn check_version_exists(
        &self,
        _name: &str,
        _version: &str,
    ) -> Result<bool, RegistryError> {
        // Stub: always return false for now
        Ok(false)
    }

    pub async fn publish(
        &self,
        meta: &PackageMetadata,
        _tarball: &[u8],
    ) -> Result<(), RegistryError> {
        // Stub: simulate publishing
        eprintln!(
            "stub: publishing {} v{} to {}",
            meta.name, meta.version, self.base_url
        );
        Ok(())
    }

    pub async fn search(&self, _query: &str) -> Result<Vec<PackageMetadata>, RegistryError> {
        // Stub: return empty results
        Ok(vec![])
    }
}

pub async fn run_security_scan(package_dir: &std::path::Path) -> Result<(), RegistryError> {
    // Stub: always pass
    eprintln!("stub: security scan passed for {}", package_dir.display());
    Ok(())
}

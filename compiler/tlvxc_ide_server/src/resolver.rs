//! Basic dependency resolution system stub

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)] // Stub implementation - not all errors are used yet
pub enum ResolverError {
    #[error("Dependency not found: {name}")]
    DependencyNotFound { name: String },
    #[error("Version conflict: {name} requires {required} but {conflicting} is selected")]
    VersionConflict {
        name: String,
        required: String,
        conflicting: String,
    },
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Stub implementation
pub struct Dependency {
    pub name: String,
    pub version_req: String, // semver requirement
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Stub implementation
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[allow(dead_code)] // Stub implementation
pub struct DependencyResolver {
    packages: HashMap<String, Vec<PackageManifest>>,
}

#[allow(dead_code)] // Stub implementation
impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, manifest: PackageManifest) {
        let entry = self.packages.entry(manifest.name.clone()).or_default();
        entry.push(manifest);
    }

    pub fn resolve(
        &self,
        root_package: &str,
        root_version_req: Option<&str>,
    ) -> Result<Vec<PackageManifest>, ResolverError> {
        let mut resolved = HashMap::new();
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        self.resolve_recursive(
            root_package,
            root_version_req,
            &mut resolved,
            &mut visiting,
            &mut visited,
        )?;

        Ok(resolved.into_values().collect())
    }

    fn resolve_recursive(
        &self,
        name: &str,
        version_req: Option<&str>,
        resolved: &mut HashMap<String, PackageManifest>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), ResolverError> {
        let resolved_key = format!("{}@{}", name, version_req.unwrap_or("latest"));

        // If already processed in this dependency tree, we have a cycle
        if visiting.contains(name) {
            return Err(ResolverError::CircularDependency {
                cycle: visiting
                    .iter()
                    .chain(std::iter::once(&name.to_string()))
                    .cloned()
                    .collect(),
            });
        }

        // If already fully processed, skip
        if visited.contains(&resolved_key) {
            return Ok(());
        }

        visiting.insert(name.to_string());

        let manifest = self.find_best_match(name, version_req)?;
        let actual_key = format!("{}@{}", manifest.name, manifest.version);

        // Process dependencies first
        for dep in &manifest.dependencies {
            self.resolve_recursive(
                &dep.name,
                Some(&dep.version_req),
                resolved,
                visiting,
                visited,
            )?;
        }

        // Add this package to resolved
        resolved.insert(actual_key, manifest.clone());

        // Mark as fully processed and remove from visiting
        visited.insert(resolved_key);
        visiting.remove(name);

        Ok(())
    }

    fn find_best_match(
        &self,
        name: &str,
        version_req: Option<&str>,
    ) -> Result<PackageManifest, ResolverError> {
        let available =
            self.packages
                .get(name)
                .ok_or_else(|| ResolverError::DependencyNotFound {
                    name: name.to_string(),
                })?;

        if let Some(req) = version_req {
            // Stub: simple version matching (real implementation would use semver crate)
            let matching: Vec<_> = available
                .iter()
                .filter(|p| self.matches_version(&p.version, req))
                .cloned()
                .collect();

            if matching.is_empty() {
                return Err(ResolverError::VersionConflict {
                    name: name.to_string(),
                    required: req.to_string(),
                    conflicting: "no matching version".to_string(),
                });
            }

            // Pick the highest matching version
            matching
                .into_iter()
                .max_by(|a, b| a.version.cmp(&b.version))
                .ok_or_else(|| ResolverError::VersionConflict {
                    name: name.to_string(),
                    required: req.to_string(),
                    conflicting: "failed to select version".to_string(),
                })
        } else {
            // No version requirement: pick latest
            available
                .iter()
                .max_by(|a, b| a.version.cmp(&b.version))
                .cloned()
                .ok_or_else(|| ResolverError::DependencyNotFound {
                    name: name.to_string(),
                })
        }
    }

    fn matches_version(&self, version: &str, requirement: &str) -> bool {
        // Stub: very simple matching (real implementation would use semver)
        if requirement == "*" {
            return true;
        }
        if let Some(req_version) = requirement.strip_prefix("^") {
            return version.starts_with(req_version);
        }
        if let Some(req_version) = requirement.strip_prefix("~") {
            return version.starts_with(req_version);
        }
        version == requirement
    }

    pub fn generate_lockfile(&self, resolved: &[PackageManifest]) -> String {
        let mut lockfile = String::new();
        lockfile.push_str("# This file is automatically generated\n");
        lockfile.push('[');
        lockfile.push('[');
        lockfile.push_str("package]\n");
        lockfile.push_str("name = \"root\"\n");
        lockfile.push_str("version = \"0.1.0\"\n");
        lockfile.push('\n');

        for manifest in resolved {
            lockfile.push_str("[[package]]\n");
            lockfile.push_str(&format!("name = \"{}\"\n", manifest.name));
            lockfile.push_str(&format!("version = \"{}\"\n", manifest.version));
            lockfile.push('\n');
        }

        lockfile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_resolution() {
        let mut resolver = DependencyResolver::new();

        resolver.add_package(PackageManifest {
            name: "tolvex_data".to_string(),
            version: "0.1.6".to_string(),
            dependencies: vec![],
        });

        resolver.add_package(PackageManifest {
            name: "my_app".to_string(),
            version: "0.1.0".to_string(),
            dependencies: vec![Dependency {
                name: "tolvex_data".to_string(),
                version_req: "^0.1".to_string(),
            }],
        });

        let resolved = resolver.resolve("my_app", None).unwrap();
        assert_eq!(resolved.len(), 2);

        // Check that both packages are resolved (order doesn't matter)
        let package_names: Vec<String> = resolved.iter().map(|p| p.name.clone()).collect();
        assert!(package_names.contains(&"my_app".to_string()));
        assert!(package_names.contains(&"tolvex_data".to_string()));
    }
}

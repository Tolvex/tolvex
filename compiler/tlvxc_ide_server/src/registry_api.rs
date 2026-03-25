//! Stub API endpoints for package operations

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use super::registry_ui::{AppState, PackageInfo};
use crate::version::Version;

#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PublishRequest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub tarball: Vec<u8>, // Base64-encoded in real implementation
}

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route(
            "/packages",
            axum::routing::get(list_packages).post(publish_package),
        )
        .route("/packages/:name", axum::routing::get(get_package))
        .route(
            "/packages/:name/versions",
            axum::routing::get(list_versions),
        )
        .with_state(AppState::default())
}

async fn list_packages(State(state): State<AppState>) -> Json<Vec<PackageInfo>> {
    let packages = state.packages.read().await;
    let mut packages_vec: Vec<_> = packages.values().cloned().collect();
    packages_vec.sort_by_key(|p| p.name.clone());
    Json(packages_vec)
}

async fn get_package(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<PackageInfo>, StatusCode> {
    let packages = state.packages.read().await;
    packages
        .get(&name)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn list_versions(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let packages = state.packages.read().await;
    if packages.contains_key(&name) {
        // Stub: return only current version
        Ok(Json(vec!["0.1.6".to_string()]))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn publish_package(
    State(state): State<AppState>,
    Json(req): Json<PublishRequest>,
) -> Json<PublishResponse> {
    // Validate package name
    if req.name.is_empty()
        || !req
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Json(PublishResponse {
            success: false,
            message:
                "Invalid package name. Use only alphanumeric characters, underscores, and hyphens"
                    .to_string(),
        });
    }

    // Validate version format
    let version = match Version::parse(&req.version) {
        Ok(v) => v,
        Err(e) => {
            return Json(PublishResponse {
                success: false,
                message: format!("Invalid version format: {}", e),
            });
        }
    };

    // Reject pre-release versions
    if !version.is_stable() {
        return Json(PublishResponse {
            success: false,
            message: "Pre-release versions are not allowed".to_string(),
        });
    }

    // Validate tarball size (limit to 100MB for now)
    if req.tarball.len() > 100 * 1024 * 1024 {
        return Json(PublishResponse {
            success: false,
            message: "Package tarball too large (max 100MB)".to_string(),
        });
    }

    let mut packages = state.packages.write().await;

    // Check for version conflict
    if let Some(existing) = packages.get(&req.name) {
        if existing.version == req.version {
            return Json(PublishResponse {
                success: false,
                message: format!(
                    "Version {} already exists for package {}",
                    req.version, req.name
                ),
            });
        }
    }

    // Add to registry
    let info = PackageInfo {
        name: req.name.clone(),
        version: req.version.clone(),
        description: req.description,
        downloads: 0,
        tags: vec![],
    };
    packages.insert(req.name.clone(), info);

    Json(PublishResponse {
        success: true,
        message: format!("Published {} v{}", req.name, req.version),
    })
}

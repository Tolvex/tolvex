//! Minimal web UI scaffolding for the Tolvex package registry (formulary.tolvex.dev)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, Router},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Stub implementation
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Stub implementation
pub struct AppState {
    pub packages: Arc<RwLock<HashMap<String, PackageInfo>>>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut packages = HashMap::new();
        packages.insert(
            "tolvex_data".to_string(),
            PackageInfo {
                name: "tolvex_data".to_string(),
                version: "0.1.6".to_string(),
                description: Some("Healthcare data structures, FHIR, HL7, DICOM".to_string()),
                downloads: 1234,
                tags: vec!["data".to_string(), "fhir".to_string()],
            },
        );
        packages.insert(
            "tolvex_stats".to_string(),
            PackageInfo {
                name: "tolvex_stats".to_string(),
                version: "0.1.6".to_string(),
                description: Some("Statistical methods for clinical analysis".to_string()),
                downloads: 567,
                tags: vec!["stats".to_string(), "clinical".to_string()],
            },
        );
        Self {
            packages: Arc::new(RwLock::new(packages)),
        }
    }
}

#[allow(dead_code)] // Stub implementation
pub fn registry_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/packages", get(packages))
        .route("/packages/:name", get(package_detail))
        .route("/search", get(search))
        .with_state(AppState::default())
}

#[allow(dead_code)] // Stub implementation
async fn index() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

#[derive(Deserialize)]
#[allow(dead_code)] // Stub implementation
struct PackagesQuery {
    tag: Option<String>,
}

#[allow(dead_code)] // Stub implementation
async fn packages(
    Query(query): Query<PackagesQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let packages = state.packages.read().await;
    let mut packages_vec: Vec<_> = packages.values().cloned().collect();
    if let Some(tag) = query.tag {
        packages_vec.retain(|p| p.tags.contains(&tag));
    }
    packages_vec.sort_by_key(|p| p.name.clone());
    Json(packages_vec)
}

#[allow(dead_code)] // Stub implementation
async fn package_detail(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let packages = state.packages.read().await;
    match packages.get(&name) {
        Some(info) => Json(info.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, "Package not found").into_response(),
    }
}

#[derive(Deserialize)]
#[allow(dead_code)] // Stub implementation
struct SearchQuery {
    q: String,
}

#[allow(dead_code)] // Stub implementation
async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Validate search query
    if query.q.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Search query cannot be empty").into_response();
    }

    let packages = state.packages.read().await;
    let results: Vec<_> = packages
        .values()
        .filter(|p| {
            p.name.to_lowercase().contains(&query.q.to_lowercase())
                || p.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query.q.to_lowercase()))
                    .unwrap_or(false)
        })
        .cloned()
        .collect();
    Json(results).into_response()
}

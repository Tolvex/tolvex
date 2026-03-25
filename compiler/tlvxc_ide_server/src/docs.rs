//! Documentation generation for published packages

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    pub package_name: String,
    pub version: String,
    pub readme: Option<String>,
    pub modules: Vec<ModuleDoc>,
    pub examples: Vec<ExampleDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDoc {
    pub name: String,
    pub description: Option<String>,
    pub functions: Vec<FunctionDoc>,
    pub types: Vec<TypeDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDoc {
    pub name: String,
    pub signature: String,
    pub description: Option<String>,
    pub params: Vec<ParamDoc>,
    pub returns: Option<String>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDoc {
    pub name: String,
    pub kind: TypeKind,
    pub description: Option<String>,
    pub fields: Vec<FieldDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeKind {
    Struct,
    Enum,
    Alias,
    Union,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDoc {
    pub name: String,
    pub type_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    pub type_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDoc {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

pub struct DocsGenerator;

impl DocsGenerator {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate_from_source(
        &self,
        package_dir: &Path,
        package_name: &str,
        version: &str,
    ) -> Result<Documentation, DocsError> {
        // Stub: scan source files for documentation comments
        let mut modules = Vec::new();

        // Stub: parse main.tlvx or lib.tlvx
        let src_path = package_dir.join("src");
        if src_path.exists() {
            let _main_file = src_path.join("main.tlvx");
            let lib_file = src_path.join("lib.tlvx");

            if lib_file.exists() {
                modules.push(ModuleDoc {
                    name: package_name.to_string(),
                    description: Some(format!("Main module for {}", package_name)),
                    functions: vec![FunctionDoc {
                        name: "example_function".to_string(),
                        signature: "fn example_function(x: str) -> str".to_string(),
                        description: Some(
                            "An example function demonstrating documentation".to_string(),
                        ),
                        params: vec![ParamDoc {
                            name: "x".to_string(),
                            type_name: "str".to_string(),
                            description: Some("Input string".to_string()),
                        }],
                        returns: Some("Processed string".to_string()),
                        examples: vec!["let result = example_function(\"hello\");".to_string()],
                    }],
                    types: vec![TypeDoc {
                        name: "Patient".to_string(),
                        kind: TypeKind::Struct,
                        description: Some("Represents a patient record".to_string()),
                        fields: vec![
                            FieldDoc {
                                name: "id".to_string(),
                                type_name: "str".to_string(),
                                description: Some("Unique patient identifier".to_string()),
                            },
                            FieldDoc {
                                name: "name".to_string(),
                                type_name: "str".to_string(),
                                description: Some("Patient name".to_string()),
                            },
                        ],
                    }],
                });
            }
        }

        // Stub: read README.md if present
        let readme = std::fs::read_to_string(package_dir.join("README.md")).ok();

        Ok(Documentation {
            package_name: package_name.to_string(),
            version: version.to_string(),
            readme,
            modules,
            examples: vec![],
        })
    }

    pub fn render_html(&self, docs: &Documentation) -> Result<String, DocsError> {
        // Stub: generate simple HTML documentation
        let mut html = String::new();
        html.push_str(&format!(
            "<!DOCTYPE html><html><head><title>{} {}</title>",
            docs.package_name, docs.version
        ));
        html.push_str("<style>body { font-family: system-ui; max-width: 800px; margin: auto; padding: 1rem; }</style>");
        html.push_str("</head><body>");
        html.push_str(&format!("<h1>{} {}</h1>", docs.package_name, docs.version));

        if let Some(ref readme) = docs.readme {
            html.push_str("<section>");
            html.push_str("<h2>README</h2>");
            html.push_str("<pre>");
            html.push_str(readme);
            html.push_str("</pre>");
            html.push_str("</section>");
        }

        for module in &docs.modules {
            html.push_str("<section>");
            html.push_str(&format!("<h2>Module: {}</h2>", module.name));
            if let Some(ref desc) = module.description {
                html.push_str(&format!("<p>{}</p>", desc));
            }

            if !module.types.is_empty() {
                html.push_str("<h3>Types</h3>");
                for ty in &module.types {
                    html.push_str(&format!("<h4>{} ({:?})</h4>", ty.name, ty.kind));
                    if let Some(ref desc) = ty.description {
                        html.push_str(&format!("<p>{}</p>", desc));
                    }
                    if !ty.fields.is_empty() {
                        html.push_str("<ul>");
                        for field in &ty.fields {
                            html.push_str(&format!(
                                "<li>{}: {} - {}</li>",
                                field.name,
                                field.type_name,
                                field.description.as_deref().unwrap_or("")
                            ));
                        }
                        html.push_str("</ul>");
                    }
                }
            }

            if !module.functions.is_empty() {
                html.push_str("<h3>Functions</h3>");
                for func in &module.functions {
                    html.push_str(&format!("<h4>{}</h4>", func.name));
                    html.push_str(&format!("<code>{}</code>", func.signature));
                    if let Some(ref desc) = func.description {
                        html.push_str(&format!("<p>{}</p>", desc));
                    }
                    if !func.examples.is_empty() {
                        html.push_str("<h5>Examples</h5>");
                        for example in &func.examples {
                            html.push_str(&format!("<pre><code>{}</code></pre>", example));
                        }
                    }
                }
            }

            html.push_str("</section>");
        }

        html.push_str("</body></html>");
        Ok(html)
    }
}

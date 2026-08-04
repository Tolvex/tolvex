use std::collections::HashMap;

use crate::chart::InteractiveLineChart;
use crate::layout::{html_id, DashboardLayout};

/// Ties a `DashboardLayout` to the charts occupying its panels and renders
/// the result as a single self-contained HTML document (inline CSS grid +
/// inline SVG per panel) — the "web-based" runtime output of the dashboard.
#[derive(Debug, Clone)]
pub struct Dashboard {
    pub layout: DashboardLayout,
    charts: HashMap<String, InteractiveLineChart>,
}

impl Dashboard {
    pub fn new(layout: DashboardLayout) -> Self {
        Self {
            layout,
            charts: HashMap::new(),
        }
    }

    /// Attaches (or replaces) the chart shown in the panel with the given id.
    pub fn set_chart(&mut self, panel_id: impl Into<String>, chart: InteractiveLineChart) {
        self.charts.insert(panel_id.into(), chart);
    }

    pub fn chart(&self, panel_id: &str) -> Option<&InteractiveLineChart> {
        self.charts.get(panel_id)
    }

    pub fn render_html(&self) -> String {
        let css = self.layout.render_css_grid();
        let mut panels_html = String::new();
        for panel in self.layout.panels() {
            let svg = self
                .charts
                .get(&panel.id)
                .map(|c| c.render_svg())
                .unwrap_or_default();
            panels_html.push_str(&format!(
                "<div id=\"{}\" class=\"tolvex-panel\">{}</div>\n",
                html_id(&panel.id),
                svg
            ));
        }
        format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><style>{css}</style></head><body><div class=\"tolvex-dashboard\">\n{panels_html}</div></body></html>"
        )
    }
}

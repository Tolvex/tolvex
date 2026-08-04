#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub id: String,
    pub row: u32,
    pub col: u32,
    pub row_span: u32,
    pub col_span: u32,
}

impl Panel {
    pub fn new(id: impl Into<String>, row: u32, col: u32, row_span: u32, col_span: u32) -> Self {
        Self {
            id: id.into(),
            row,
            col,
            row_span,
            col_span,
        }
    }

    fn end_row(&self) -> u32 {
        self.row + self.row_span
    }

    fn end_col(&self) -> u32 {
        self.col + self.col_span
    }

    fn overlaps(&self, other: &Panel) -> bool {
        self.row < other.end_row()
            && other.row < self.end_row()
            && self.col < other.end_col()
            && other.col < self.end_col()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    ZeroSpan,
    OutOfBounds,
    Overlap(String),
    DuplicateId,
}

/// A grid-based dashboard layout. Panels are placed on a `rows` x `cols` grid;
/// placement is validated eagerly so overlapping or out-of-bounds panels are
/// rejected at `add_panel` time rather than surfacing as a rendering bug.
#[derive(Debug, Clone)]
pub struct DashboardLayout {
    pub rows: u32,
    pub cols: u32,
    panels: Vec<Panel>,
}

impl DashboardLayout {
    pub fn new(rows: u32, cols: u32) -> Self {
        Self {
            rows,
            cols,
            panels: Vec::new(),
        }
    }

    pub fn add_panel(&mut self, panel: Panel) -> Result<(), LayoutError> {
        if panel.row_span == 0 || panel.col_span == 0 {
            return Err(LayoutError::ZeroSpan);
        }
        if panel.end_row() > self.rows || panel.end_col() > self.cols {
            return Err(LayoutError::OutOfBounds);
        }
        for existing in &self.panels {
            if existing.id == panel.id {
                return Err(LayoutError::DuplicateId);
            }
            if existing.overlaps(&panel) {
                return Err(LayoutError::Overlap(existing.id.clone()));
            }
        }
        self.panels.push(panel);
        Ok(())
    }

    pub fn panels(&self) -> &[Panel] {
        &self.panels
    }

    /// Renders a CSS Grid container rule plus one `grid-area` placement rule
    /// per panel, keyed by an HTML-safe element id derived from the panel id.
    pub fn render_css_grid(&self) -> String {
        let mut css = format!(
            ".tolvex-dashboard {{ display: grid; grid-template-rows: repeat({}, 1fr); grid-template-columns: repeat({}, 1fr); gap: 8px; }}\n",
            self.rows, self.cols
        );
        for panel in &self.panels {
            css.push_str(&format!(
                "#{} {{ grid-row: {} / span {}; grid-column: {} / span {}; }}\n",
                html_id(&panel.id),
                panel.row + 1,
                panel.row_span,
                panel.col + 1,
                panel.col_span,
            ));
        }
        css
    }
}

pub fn html_id(id: &str) -> String {
    let mut out = String::with_capacity(id.len() + 6);
    out.push_str("panel-");
    for c in id.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            out.push(c);
        } else {
            out.push('-');
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
}

impl DataPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, label: None }
    }

    pub fn with_label(x: f64, y: f64, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            label: Some(label.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChartSeries {
    pub name: String,
    pub points: Vec<DataPoint>,
}

impl ChartSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            points: Vec::new(),
        }
    }

    pub fn push(&mut self, point: DataPoint) {
        self.points.push(point);
    }
}

/// A multi-series line chart that renders to interactive SVG: every data point
/// carries an embedded `<title>` tooltip, since SVG's native hover affordance is
/// how a stdlib primitive conveys interactivity without depending on a JS runtime.
#[derive(Debug, Clone)]
pub struct InteractiveLineChart {
    pub width: u32,
    pub height: u32,
    pub series: Vec<ChartSeries>,
}

impl InteractiveLineChart {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            series: Vec::new(),
        }
    }

    pub fn add_series(&mut self, series: ChartSeries) {
        self.series.push(series);
    }

    pub fn render_svg(&self) -> String {
        use std::fmt::Write as _;

        let w = self.width;
        let h = self.height;
        if w == 0 || h == 0 || self.series.iter().all(|s| s.points.is_empty()) {
            return format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\"></svg>"
            );
        }

        let bounds = self.bounds();
        let colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd"];

        let mut body = String::new();
        for (i, series) in self.series.iter().enumerate() {
            if series.points.is_empty() {
                continue;
            }
            let color = colors[i % colors.len()];

            let mut poly = String::new();
            let mut circles = String::new();
            for p in &series.points {
                let (sx, sy) = scale_point(p.x, p.y, bounds, w, h);
                if !poly.is_empty() {
                    poly.push(' ');
                }
                let _ = write!(poly, "{sx:.3},{sy:.3}");

                let tooltip = match &p.label {
                    Some(label) => escape_xml(label),
                    None => escape_xml(&format!("{}: ({:.3}, {:.3})", series.name, p.x, p.y)),
                };
                let _ = write!(
                    circles,
                    "<circle cx=\"{sx:.3}\" cy=\"{sy:.3}\" r=\"4\" fill=\"{color}\" opacity=\"0\"><title>{tooltip}</title></circle>"
                );
            }
            let _ = write!(
                body,
                "<polyline fill=\"none\" stroke=\"{color}\" stroke-width=\"1.5\" points=\"{poly}\" />"
            );
            body.push_str(&circles);
        }

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\">{body}</svg>"
        )
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for series in &self.series {
            for p in &series.points {
                x_min = x_min.min(p.x);
                x_max = x_max.max(p.x);
                y_min = y_min.min(p.y);
                y_max = y_max.max(p.y);
            }
        }
        (x_min, x_max, y_min, y_max)
    }
}

fn scale_point(x: f64, y: f64, bounds: (f64, f64, f64, f64), w: u32, h: u32) -> (f64, f64) {
    let (x_min, x_max, y_min, y_max) = bounds;
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;
    let sx = if x_range == 0.0 {
        w as f64 / 2.0
    } else {
        (x - x_min) / x_range * w as f64
    };
    let sy = if y_range == 0.0 {
        h as f64 / 2.0
    } else {
        h as f64 - (y - y_min) / y_range * h as f64
    };
    (sx, sy)
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

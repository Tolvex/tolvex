use tolvex_viz::binding::{bind_series, DataBinding};
use tolvex_viz::chart::{ChartSeries, DataPoint, InteractiveLineChart};
use tolvex_viz::dashboard::Dashboard;
use tolvex_viz::layout::{DashboardLayout, LayoutError, Panel};

#[test]
fn chart_render_svg_contains_polyline_and_tooltip() {
    let mut chart = InteractiveLineChart::new(200, 100);
    let mut series = ChartSeries::new("hr");
    series.push(DataPoint::new(0.0, 60.0));
    series.push(DataPoint::with_label(1.0, 90.0, "spike"));
    chart.add_series(series);

    let svg = chart.render_svg();
    assert!(
        svg.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"200\" height=\"100\">")
    );
    assert!(svg.contains("<polyline"));
    assert!(svg.contains("<title>spike</title>"));
}

#[test]
fn chart_empty_series_renders_empty_svg() {
    let chart = InteractiveLineChart::new(200, 100);
    let svg = chart.render_svg();
    assert_eq!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"200\" height=\"100\"></svg>"
    );
}

#[test]
fn layout_add_panel_rejects_overlap() {
    let mut layout = DashboardLayout::new(2, 2);
    layout
        .add_panel(Panel::new("a", 0, 0, 1, 2))
        .expect("first panel fits");

    let err = layout
        .add_panel(Panel::new("b", 0, 1, 1, 1))
        .expect_err("overlaps panel a");
    assert_eq!(err, LayoutError::Overlap("a".to_string()));
}

#[test]
fn layout_add_panel_rejects_out_of_bounds() {
    let mut layout = DashboardLayout::new(2, 2);
    let err = layout
        .add_panel(Panel::new("a", 0, 0, 3, 1))
        .expect_err("row_span exceeds grid rows");
    assert_eq!(err, LayoutError::OutOfBounds);
}

#[test]
fn layout_render_css_grid_places_panel() {
    let mut layout = DashboardLayout::new(2, 3);
    layout
        .add_panel(Panel::new("hr-chart", 0, 1, 1, 2))
        .expect("fits within grid");

    let css = layout.render_css_grid();
    assert!(css.contains("grid-template-rows: repeat(2, 1fr)"));
    assert!(css.contains("grid-template-columns: repeat(3, 1fr)"));
    assert!(css.contains("#panel-hr-chart { grid-row: 1 / span 1; grid-column: 2 / span 2; }"));
}

#[test]
fn binding_push_evicts_oldest_when_full() {
    let mut binding = DataBinding::with_capacity(3);
    binding.push(1.0);
    binding.push(2.0);
    binding.push(3.0);
    binding.push(4.0);

    assert_eq!(binding.len(), 3);
    assert_eq!(binding.snapshot(), vec![2.0, 3.0, 4.0]);
}

#[test]
fn binding_to_series_and_bind_series_updates_chart() {
    let mut binding = DataBinding::with_capacity(2);
    binding.push(10.0);
    binding.push(20.0);

    let mut chart = InteractiveLineChart::new(100, 50);
    bind_series(&mut chart, &binding, "hr");
    assert_eq!(chart.series.len(), 1);
    assert_eq!(chart.series[0].points.len(), 2);

    binding.push(30.0);
    bind_series(&mut chart, &binding, "hr");
    assert_eq!(chart.series.len(), 1, "rebinding replaces, not appends");
    assert_eq!(
        chart.series[0].points,
        vec![DataPoint::new(0.0, 20.0), DataPoint::new(1.0, 30.0)]
    );
}

#[test]
fn dashboard_render_html_embeds_panel_svg() {
    let mut layout = DashboardLayout::new(1, 1);
    layout
        .add_panel(Panel::new("hr", 0, 0, 1, 1))
        .expect("fits within grid");

    let mut dashboard = Dashboard::new(layout);
    let mut chart = InteractiveLineChart::new(50, 20);
    let mut series = ChartSeries::new("hr");
    series.push(DataPoint::new(0.0, 1.0));
    series.push(DataPoint::new(1.0, 2.0));
    chart.add_series(series);
    dashboard.set_chart("hr", chart);

    let html = dashboard.render_html();
    assert!(html.contains("<div id=\"panel-hr\" class=\"tolvex-panel\">"));
    assert!(html.contains("<svg"));
    assert!(html.contains(".tolvex-dashboard { display: grid;"));
}

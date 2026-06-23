use std::{cmp::Ordering, path::Path};

use plotters::prelude::*;

use crate::{population::PopulationSummaryRow, reports::ReportPaths};

#[derive(Debug, Copy, Clone)]
pub enum PopulationMetric {
    TargetCount,
    PercentTargetWithinGroup,
}

impl PopulationMetric {
    fn value(self, row: &PopulationSummaryRow) -> f64 {
        match self {
            Self::TargetCount => row.target_count as f64,
            Self::PercentTargetWithinGroup => row.percent_target_within_group,
        }
    }

    fn axis_label(self) -> &'static str {
        match self {
            Self::TargetCount => "Target-positive spectra",
            Self::PercentTargetWithinGroup => "Percent target-positive within group",
        }
    }

    fn file_suffix(self) -> &'static str {
        match self {
            Self::TargetCount => "target_count",
            Self::PercentTargetWithinGroup => "percent_target",
        }
    }
}

pub fn write_standard_population_figures(
    reports: &ReportPaths,
    stem: &str,
    title_root: &str,
    rows: &[PopulationSummaryRow],
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    render_top_population_chart(
        reports
            .figure(&format!("top_{stem}_by_{}.svg", PopulationMetric::TargetCount.file_suffix())),
        &format!("{title_root}: Top groups by target count"),
        rows,
        PopulationMetric::TargetCount,
        15,
        0,
    )?;

    render_top_population_chart(
        reports.figure(&format!(
            "top_{stem}_by_{}.svg",
            PopulationMetric::PercentTargetWithinGroup.file_suffix()
        )),
        &format!("{title_root}: Top groups by percent target"),
        rows,
        PopulationMetric::PercentTargetWithinGroup,
        15,
        30,
    )?;

    Ok(())
}

fn render_top_population_chart(
    path: impl AsRef<Path>,
    title: &str,
    rows: &[PopulationSummaryRow],
    metric: PopulationMetric,
    max_items: usize,
    min_total_count: usize,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut filtered: Vec<PopulationSummaryRow> = rows
        .iter()
        .filter(|row| row.total_count >= min_total_count)
        .filter(|row| !row.value.starts_with("TOTAL_"))
        .cloned()
        .collect();

    filtered
        .sort_by(|a, b| metric.value(b).partial_cmp(&metric.value(a)).unwrap_or(Ordering::Equal));

    filtered.truncate(max_items);
    filtered.reverse();

    if filtered.is_empty() {
        return Ok(());
    }

    let max_value = filtered.iter().map(|row| metric.value(row)).fold(0.0_f64, f64::max).max(1.0);

    let root = SVGBackend::new(path.as_ref(), (1400, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_count = filtered.len() as i32;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(280)
        .build_cartesian_2d(0f64..(max_value * 1.15), 0i32..y_count)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc(metric.axis_label())
        .y_labels(filtered.len())
        .y_label_style(("sans-serif", 18))
        .y_label_formatter(&|y| {
            usize::try_from(*y)
                .ok()
                .and_then(|index| filtered.get(index))
                .map(|row| row.value.clone())
                .unwrap_or_default()
        })
        .draw()?;

    for (idx, row) in filtered.iter().enumerate() {
        let y0 = idx as i32;
        let y1 = y0 + 1;
        let value = metric.value(row);

        chart.draw_series(std::iter::once(Rectangle::new(
            [(0.0, y0), (value, y1)],
            BLUE.mix(0.6).filled(),
        )))?;

        let label = match metric {
            PopulationMetric::TargetCount => {
                format!("{} ({:.2}%)", row.target_count, row.percent_target_within_group)
            }
            PopulationMetric::PercentTargetWithinGroup => {
                format!("{:.2}% (n={})", row.percent_target_within_group, row.target_count)
            }
        };

        chart.draw_series(std::iter::once(Text::new(
            label,
            (value + max_value * 0.01, y0),
            ("sans-serif", 16).into_font(),
        )))?;
    }

    root.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{population::PopulationSummaryRow, reports::ReportPaths};

    fn row(
        value: &str,
        total_count: usize,
        target_count: usize,
        percent_target_within_group: f64,
        percent_of_all_target: f64,
    ) -> PopulationSummaryRow {
        PopulationSummaryRow {
            value: value.to_string(),
            total_count,
            target_count,
            non_target_count: total_count - target_count,
            percent_target_within_group,
            percent_of_all_records: 0.0,
            percent_of_all_target,
            support_warning: String::new(),
        }
    }

    #[test]
    fn writes_standard_population_figures() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("report")).unwrap();

        let rows = vec![
            row("Class A", 100, 25, 25.0, 60.0),
            row("Class B", 200, 10, 5.0, 30.0),
            row("Class C", 50, 5, 10.0, 10.0),
        ];

        write_standard_population_figures(&reports, "npc_classes", "NPC classes", &rows).unwrap();

        let count_path = reports.figure("top_npc_classes_by_target_count.svg");
        let percent_path = reports.figure("top_npc_classes_by_percent_target.svg");

        assert!(count_path.exists());
        assert!(percent_path.exists());

        let count_svg = std::fs::read_to_string(count_path).unwrap();
        let percent_svg = std::fs::read_to_string(percent_path).unwrap();

        assert!(count_svg.contains("<svg"));
        assert!(percent_svg.contains("<svg"));
        assert!(count_svg.contains("NPC classes"));
        assert!(percent_svg.contains("NPC classes"));
    }

    #[test]
    fn skips_empty_population_figures_without_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("report")).unwrap();

        let rows = Vec::new();

        write_standard_population_figures(&reports, "npc_classes", "NPC classes", &rows).unwrap();
    }

    #[test]
    fn writes_figures_for_small_supported_rows_without_panicking() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("report")).unwrap();

        let rows = vec![
            row("Tiny but enriched", 42, 42, 100.0, 5.0),
            row("Large group", 42_000, 7_878, 18.76, 80.0),
        ];

        write_standard_population_figures(&reports, "npc_classes", "NPC classes", &rows).unwrap();

        assert!(reports.figure("top_npc_classes_by_target_count.svg").exists());

        assert!(reports.figure("top_npc_classes_by_percent_target.svg").exists());
    }
}

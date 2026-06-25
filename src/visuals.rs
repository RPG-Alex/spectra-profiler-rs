use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    path::Path,
};

use plotters::prelude::*;

use crate::{
    error::{Result, SpectraProfilerError},
    population::PopulationSummaryRow,
    reports::ReportPaths,
};
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
) -> Result<()> {
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
) -> Result<()> {
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
    root.fill(&WHITE).map_err(figure_error)?;

    let y_count = filtered.len() as i32;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(280)
        .build_cartesian_2d(0f64..(max_value * 1.15), 0i32..y_count)
        .map_err(figure_error)?;

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
        .draw()
        .map_err(figure_error)?;

    for (idx, row) in filtered.iter().enumerate() {
        let y0 = idx as i32;
        let y1 = y0 + 1;
        let value = metric.value(row);

        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, y0), (value, y1)],
                BLUE.mix(0.6).filled(),
            )))
            .map_err(figure_error)?;

        let label = match metric {
            PopulationMetric::TargetCount => {
                format!("{} ({:.2}%)", row.target_count, row.percent_target_within_group)
            }
            PopulationMetric::PercentTargetWithinGroup => {
                format!("{:.2}% (n={})", row.percent_target_within_group, row.target_count)
            }
        };

        chart
            .draw_series(std::iter::once(Text::new(
                label,
                (value + max_value * 0.01, y0),
                ("sans-serif", 16).into_font(),
            )))
            .map_err(figure_error)?;
    }

    root.present().map_err(figure_error)?;
    Ok(())
}

fn figure_error(error: impl Debug) -> SpectraProfilerError {
    SpectraProfilerError::FigureGeneration { message: format!("{error:?}") }
}

pub fn write_atom_count_distribution_figure(
    reports: &ReportPaths,
    target_element: &str,
    records_with_formula: usize,
    distribution: &BTreeMap<usize, usize>,
) -> Result<()> {
    render_atom_count_distribution_chart(
        reports.figure("target_atom_count_distribution.svg"),
        target_element,
        records_with_formula,
        distribution,
    )
}

fn render_atom_count_distribution_chart(
    path: impl AsRef<Path>,
    target_element: &str,
    records_with_formula: usize,
    distribution: &BTreeMap<usize, usize>,
) -> Result<()> {
    if distribution.is_empty() {
        return Ok(());
    }

    let max_count = distribution.values().copied().max().unwrap_or(1).max(1) as f64;
    let max_atom_count = distribution.keys().copied().max().unwrap_or(0) as i32;

    let root = SVGBackend::new(path.as_ref(), (1200, 800)).into_drawing_area();
    root.fill(&WHITE).map_err(figure_error)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{target_element} atom-count distribution"), ("sans-serif", 32))
        .margin(30)
        .x_label_area_size(60)
        .y_label_area_size(90)
        .build_cartesian_2d(0i32..(max_atom_count + 1), 0f64..(max_count * 1.15))
        .map_err(figure_error)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc(format!("Number of {target_element} atoms in formula"))
        .y_desc("Formula-bearing spectra")
        .x_labels((max_atom_count as usize + 1).min(20))
        .y_label_formatter(&|value| compact_count(*value as usize))
        .draw()
        .map_err(figure_error)?;

    chart
        .draw_series(distribution.iter().map(|(atom_count, record_count)| {
            let x0 = *atom_count as i32;
            let x1 = x0 + 1;
            let y = *record_count as f64;

            Rectangle::new([(x0, 0.0), (x1, y)], BLUE.mix(0.6).filled())
        }))
        .map_err(figure_error)?;

    let mut labeled_atom_counts = distribution
        .iter()
        .map(|(atom_count, record_count)| (*atom_count, *record_count))
        .collect::<Vec<_>>();

    labeled_atom_counts
        .sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));

    let labeled_atom_counts = labeled_atom_counts
        .into_iter()
        .take(15)
        .map(|(atom_count, _)| atom_count)
        .collect::<BTreeSet<_>>();

    for (atom_count, record_count) in distribution {
        if !labeled_atom_counts.contains(atom_count) {
            continue;
        }

        let percent = percent(*record_count, records_with_formula);

        chart
            .draw_series(std::iter::once(Text::new(
                format!("{} ({:.1}%)", compact_count(*record_count), percent),
                (*atom_count as i32, *record_count as f64 + max_count * 0.015),
                ("sans-serif", 16).into_font(),
            )))
            .map_err(figure_error)?;
    }

    root.present().map_err(figure_error)?;

    Ok(())
}

fn compact_count(count: usize) -> String {
    match count {
        1_000_000.. => format!("{:.1}M", count as f64 / 1_000_000.0),
        10_000.. => format!("{}k", count / 1_000),
        1_000.. => format!("{:.1}k", count as f64 / 1_000.0),
        _ => count.to_string(),
    }
}

fn percent(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        return 0.0;
    }

    numerator as f64 / denominator as f64 * 100.0
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

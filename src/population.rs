use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

const LOW_TOTAL_SUPPORT_THRESHOLD: usize = 30;
const LOW_TARGET_SUPPORT_THRESHOLD: usize = 10;

#[derive(Debug, Default)]
pub struct PopulationStats {
    total_count: usize,
    target_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PopulationSummaryRow {
    pub value: String,
    pub total_count: usize,
    pub target_count: usize,
    pub non_target_count: usize,
    pub percent_target_within_group: f64,
    pub percent_of_all_records: f64,
    pub percent_of_all_target: f64,
    pub support_warning: String,
}

pub type PopulationMap = BTreeMap<String, PopulationStats>;

pub fn increment_population(counts: &mut PopulationMap, key: &str, contains_target: bool) {
    let key = clean_group_value(key);
    let stats = counts.entry(key).or_default();

    stats.total_count += 1;

    if contains_target {
        stats.target_count += 1;
    }
}

pub fn increment_pipe_population(counts: &mut PopulationMap, value: &str, contains_target: bool) {
    if value.trim().is_empty() {
        increment_population(counts, "UNKNOWN", contains_target);
        return;
    }

    for part in split_pipe(value) {
        increment_population(counts, part, contains_target);
    }
}

pub fn summarize_population_map(
    counts: &PopulationMap,
    total_records: usize,
    total_target_records: usize,
) -> Vec<PopulationSummaryRow> {
    counts
        .iter()
        .map(|(value, stats)| {
            let non_target_count = stats.total_count.saturating_sub(stats.target_count);

            PopulationSummaryRow {
                value: value.clone(),
                total_count: stats.total_count,
                target_count: stats.target_count,
                non_target_count,
                percent_target_within_group: percent(stats.target_count, stats.total_count),
                percent_of_all_records: percent(stats.total_count, total_records),
                percent_of_all_target: percent(stats.target_count, total_target_records),
                support_warning: support_warning(stats),
            }
        })
        .collect()
}

pub fn write_population_map_csv(
    path: impl AsRef<Path>,
    counts: &PopulationMap,
    total_records: usize,
    total_target_records: usize,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path(path)?;

    let summary_rows = summarize_population_map(counts, total_records, total_target_records);

    for row in &summary_rows {
        writer.serialize(row)?;
    }

    let total_assignments = counts
        .values()
        .map(|stats| stats.total_count)
        .sum::<usize>();

    let total_target_assignments = counts
        .values()
        .map(|stats| stats.target_count)
        .sum::<usize>();

    writer.serialize(PopulationSummaryRow {
        value: "TOTAL_RECORDS".to_string(),
        total_count: total_records,
        target_count: total_target_records,
        non_target_count: total_records.saturating_sub(total_target_records),
        percent_target_within_group: percent(total_target_records, total_records),
        percent_of_all_records: 100.0,
        percent_of_all_target: 100.0,
        support_warning: String::new(),
    })?;

    writer.serialize(PopulationSummaryRow {
        value: "TOTAL_ASSIGNMENTS".to_string(),
        total_count: total_assignments,
        target_count: total_target_assignments,
        non_target_count: total_assignments.saturating_sub(total_target_assignments),
        percent_target_within_group: percent(total_target_assignments, total_assignments),
        percent_of_all_records: percent(total_assignments, total_records),
        percent_of_all_target: percent(total_target_assignments, total_target_records),
        support_warning: String::new(),
    })?;

    writer.flush()?;
    Ok(())
}

fn split_pipe(value: &str) -> impl Iterator<Item = &str> {
    value
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
}

fn clean_group_value(value: &str) -> String {
    let value = value.trim();

    if value.is_empty() || value == "None" {
        "UNKNOWN".to_string()
    } else {
        value.to_string()
    }
}

fn percent(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        return 0.0;
    }

    (numerator as f64 / denominator as f64) * 100.0
}

fn support_warning(stats: &PopulationStats) -> String {
    let mut warnings = Vec::new();

    if stats.total_count < LOW_TOTAL_SUPPORT_THRESHOLD {
        warnings.push("LOW_TOTAL_SUPPORT");
    }

    if stats.target_count == 0 {
        warnings.push("NO_TARGET_POSITIVES");
    } else if stats.target_count < LOW_TARGET_SUPPORT_THRESHOLD {
        warnings.push("LOW_TARGET_SUPPORT");
    }

    warnings.join("|")
}

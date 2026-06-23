use std::{collections::BTreeMap, path::Path};

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

    let total_assignments = counts.values().map(|stats| stats.total_count).sum::<usize>();

    let total_target_assignments = counts.values().map(|stats| stats.target_count).sum::<usize>();

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
    value.split('|').map(str::trim).filter(|part| !part.is_empty())
}

fn clean_group_value(value: &str) -> String {
    let value = value.trim();

    if value.is_empty() || value == "None" { "UNKNOWN".to_string() } else { value.to_string() }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_population_counts() {
        let mut counts = PopulationMap::new();

        increment_population(&mut counts, "Alkaloids", true);
        increment_population(&mut counts, "Alkaloids", false);
        increment_population(&mut counts, "Terpenoids", true);

        let alkaloids = counts.get("Alkaloids").unwrap();
        assert_eq!(alkaloids.total_count, 2);
        assert_eq!(alkaloids.target_count, 1);

        let terpenoids = counts.get("Terpenoids").unwrap();
        assert_eq!(terpenoids.total_count, 1);
        assert_eq!(terpenoids.target_count, 1);
    }

    #[test]
    fn empty_population_keys_become_unknown() {
        let mut counts = PopulationMap::new();

        increment_population(&mut counts, "", true);
        increment_population(&mut counts, "   ", false);

        let unknown = counts.get("UNKNOWN").unwrap();
        assert_eq!(unknown.total_count, 2);
        assert_eq!(unknown.target_count, 1);
    }

    #[test]
    fn pipe_population_splits_values() {
        let mut counts = PopulationMap::new();

        increment_pipe_population(&mut counts, "Alkaloids|Terpenoids", true);

        let alkaloids = counts.get("Alkaloids").unwrap();
        assert_eq!(alkaloids.total_count, 1);
        assert_eq!(alkaloids.target_count, 1);

        let terpenoids = counts.get("Terpenoids").unwrap();
        assert_eq!(terpenoids.total_count, 1);
        assert_eq!(terpenoids.target_count, 1);
    }

    #[test]
    fn population_summary_calculates_percentages() {
        let mut counts = PopulationMap::new();

        increment_population(&mut counts, "A", true);
        increment_population(&mut counts, "A", false);
        increment_population(&mut counts, "B", true);
        increment_population(&mut counts, "B", true);

        let rows = summarize_population_map(&counts, 4, 3);

        let a = rows.iter().find(|row| row.value == "A").unwrap();
        assert_eq!(a.total_count, 2);
        assert_eq!(a.target_count, 1);
        assert_eq!(a.non_target_count, 1);
        assert_eq!(a.percent_target_within_group, 50.0);
        assert_eq!(a.percent_of_all_records, 50.0);
        assert!((a.percent_of_all_target - 33.333333333333336).abs() < 0.0001);

        let b = rows.iter().find(|row| row.value == "B").unwrap();
        assert_eq!(b.total_count, 2);
        assert_eq!(b.target_count, 2);
        assert_eq!(b.non_target_count, 0);
        assert_eq!(b.percent_target_within_group, 100.0);
        assert_eq!(b.percent_of_all_records, 50.0);
        assert!((b.percent_of_all_target - 66.66666666666667).abs() < 0.0001);
    }

    #[test]
    fn csv_writer_writes_population_map() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("population.csv");

        let mut counts = PopulationMap::new();
        increment_population(&mut counts, "A", true);
        increment_population(&mut counts, "A", false);

        write_population_map_csv(&path, &counts, 2, 1).unwrap();

        let contents = std::fs::read_to_string(path).unwrap();

        assert!(contents.contains("value,total_count,target_count,non_target_count"));
        assert!(contents.contains("A,2,1,1"));
    }
}

use std::collections::BTreeMap;

use serde::Serialize;

use crate::{
    error::Result,
    population::{
        PopulationMap, increment_pipe_population, increment_population, summarize_population_map,
        write_population_map_csv,
    },
    records::LoadedDataset,
    reports::ReportPaths,
    visuals::{write_atom_count_distribution_figure, write_standard_population_figures},
};

pub fn profile_dataset(
    dataset: &LoadedDataset,
    target_element: &str,
    reports: &ReportPaths,
) -> Result<()> {
    let mut total_records = 0usize;
    let mut records_with_formula = 0usize;
    let mut records_with_target_element = 0usize;

    let mut target_atom_count_distribution = BTreeMap::<usize, usize>::new();

    let mut population_maps = BTreeMap::<String, PopulationMap>::new();

    for record in &dataset.records {
        total_records += 1;
        records_with_formula += 1;

        let target_atom_count = record.atom_count(target_element);
        *target_atom_count_distribution.entry(target_atom_count).or_default() += 1;

        let contains_target_element = record.contains_element(target_element);

        if contains_target_element {
            records_with_target_element += 1;
        }

        for (metadata_group, value) in &record.metadata {
            let counts = population_maps.entry(metadata_group.clone()).or_default();

            if value.contains('|') {
                increment_pipe_population(counts, value, contains_target_element);
            } else {
                increment_population(counts, value, contains_target_element);
            }
        }
    }

    write_summary_csv(
        reports,
        total_records,
        records_with_formula,
        records_with_target_element,
        target_element,
    )?;

    write_atom_count_distribution_csv(
        reports,
        target_element,
        records_with_formula,
        &target_atom_count_distribution,
    )?;

    write_atom_count_distribution_figure(
        reports,
        target_element,
        records_with_formula,
        &target_atom_count_distribution,
    )?;
    for (metadata_group, counts) in &population_maps {
        let stem = population_stem(metadata_group);

        write_population_outputs(
            reports,
            &stem,
            &format!("{target_element} by {metadata_group}"),
            counts,
            total_records,
            records_with_target_element,
        )?;
    }

    println!("Total records: {total_records}");
    println!("Records with formula: {records_with_formula}");
    println!("Records with {target_element}: {records_with_target_element}");

    Ok(())
}

fn population_stem(metadata_group: &str) -> String {
    metadata_group.to_ascii_lowercase().replace(' ', "_").replace('/', "_")
}

fn write_population_outputs(
    reports: &ReportPaths,
    stem: &str,
    title: &str,
    counts: &PopulationMap,
    total_records: usize,
    total_target_records: usize,
) -> Result<()> {
    write_population_map_csv(
        reports.table(&format!("contains_by_{stem}.csv")),
        counts,
        total_records,
        total_target_records,
    )?;

    let summary_rows = summarize_population_map(counts, total_records, total_target_records);

    write_standard_population_figures(reports, stem, title, &summary_rows)?;

    Ok(())
}

fn write_summary_csv(
    reports: &ReportPaths,
    total_records: usize,
    records_with_formula: usize,
    records_with_target_element: usize,
    target_element: &str,
) -> Result<()> {
    let mut writer = csv::Writer::from_path(reports.table("summary.csv"))?;

    writer.write_record(["metric", "value"])?;
    writer.write_record(["target_element".to_string(), target_element.to_string()])?;
    writer.write_record(["total_records".to_string(), total_records.to_string()])?;
    writer.write_record(["records_with_formula".to_string(), records_with_formula.to_string()])?;
    writer.write_record([
        "records_with_target_element".to_string(),
        records_with_target_element.to_string(),
    ])?;

    writer.flush()?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct AtomCountDistributionRow {
    atom_count: usize,
    record_count: usize,
    percent_of_formula_records: f64,
    contains_target: bool,
}

fn write_atom_count_distribution_csv(
    reports: &ReportPaths,
    target_element: &str,
    records_with_formula: usize,
    distribution: &BTreeMap<usize, usize>,
) -> Result<()> {
    let mut writer = csv::Writer::from_path(reports.table("target_atom_count_distribution.csv"))?;

    for (atom_count, record_count) in distribution {
        writer.serialize(AtomCountDistributionRow {
            atom_count: *atom_count,
            record_count: *record_count,
            percent_of_formula_records: percent(*record_count, records_with_formula),
            contains_target: *atom_count > 0,
        })?;
    }

    writer.flush()?;

    println!("Wrote atom-count distribution for {target_element}");

    Ok(())
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
    use crate::reports::ReportPaths;

    #[test]
    fn write_summary_csv_writes_expected_metrics() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("report")).unwrap();

        write_summary_csv(&reports, 100, 95, 12, "F").unwrap();

        let contents = std::fs::read_to_string(reports.table("summary.csv")).unwrap();

        assert!(contents.contains("metric,value"));
        assert!(contents.contains("target_element,F"));
        assert!(contents.contains("total_records,100"));
        assert!(contents.contains("records_with_formula,95"));
        assert!(contents.contains("records_with_target_element,12"));
    }
    #[test]
    fn write_atom_count_distribution_csv_writes_expected_rows() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("report")).unwrap();

        let mut distribution = BTreeMap::new();
        distribution.insert(0, 90);
        distribution.insert(1, 8);
        distribution.insert(2, 2);

        write_atom_count_distribution_csv(&reports, "F", 100, &distribution).unwrap();

        let contents =
            std::fs::read_to_string(reports.table("target_atom_count_distribution.csv")).unwrap();

        assert!(
            contents.contains("atom_count,record_count,percent_of_formula_records,contains_target")
        );
        assert!(contents.contains("0,90,90.0,false"));
        assert!(contents.contains("1,8,8.0,true"));
        assert!(contents.contains("2,2,2.0,true"));
    }
}

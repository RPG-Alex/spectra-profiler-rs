use std::collections::BTreeMap;

use mascot_rs::prelude::*;
use serde::Serialize;

use crate::{
    chemistry::atom_count_for_element,
    error::Result,
    metadata::{metadata_value, optional_debug_label},
    population::{
        PopulationMap, increment_pipe_population, increment_population, summarize_population_map,
        write_population_map_csv,
    },
    reports::ReportPaths,
    visuals::{write_atom_count_distribution_figure, write_standard_population_figures},
};

pub fn profile_dataset(
    spectra: &MGFVec<f64>,
    target_element: &str,
    reports: &ReportPaths,
) -> Result<()> {
    let mut total_records = 0usize;
    let mut records_with_formula = 0usize;
    let mut records_with_target_element = 0usize;

    let mut target_atom_count_distribution = BTreeMap::<usize, usize>::new();

    let mut by_npc_pathways = PopulationMap::new();
    let mut by_npc_superclasses = PopulationMap::new();
    let mut by_npc_classes = PopulationMap::new();
    let mut by_source_dataset = PopulationMap::new();
    let mut by_organism = PopulationMap::new();
    let mut by_ion_mode = PopulationMap::new();
    let mut by_source_instrument = PopulationMap::new();
    let mut by_library_quality = PopulationMap::new();

    for record in spectra.iter() {
        total_records += 1;

        let metadata = record.metadata();

        let source_dataset = metadata_value(metadata, "SOURCE_DATASET");
        let organism = metadata_value(metadata, "ORGANISM");
        let npc_pathways = metadata_value(metadata, "NPC_PATHWAYS");
        let npc_superclasses = metadata_value(metadata, "NPC_SUPERCLASSES");
        let npc_classes = metadata_value(metadata, "NPC_CLASSES");
        let library_quality = metadata_value(metadata, "LIBRARYQUALITY");

        let ion_mode = optional_debug_label(record.ion_mode());
        let source_instrument = optional_debug_label(record.source_instrument());

        let target_atom_count = if let Some(formula) = metadata.formula() {
            records_with_formula += 1;

            let atom_count = atom_count_for_element(formula, target_element);
            *target_atom_count_distribution.entry(atom_count).or_default() += 1;

            atom_count
        } else {
            0
        };

        let contains_target_element = target_atom_count > 0;

        if contains_target_element {
            records_with_target_element += 1;
        }

        increment_pipe_population(&mut by_npc_pathways, &npc_pathways, contains_target_element);
        increment_pipe_population(
            &mut by_npc_superclasses,
            &npc_superclasses,
            contains_target_element,
        );
        increment_pipe_population(&mut by_npc_classes, &npc_classes, contains_target_element);

        increment_population(&mut by_source_dataset, &source_dataset, contains_target_element);
        increment_population(&mut by_organism, &organism, contains_target_element);
        increment_population(&mut by_ion_mode, &ion_mode, contains_target_element);
        increment_population(
            &mut by_source_instrument,
            &source_instrument,
            contains_target_element,
        );
        increment_population(&mut by_library_quality, &library_quality, contains_target_element);
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

    write_population_outputs(
        reports,
        "npc_pathways",
        &format!("{target_element} by NPC pathways"),
        &by_npc_pathways,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "npc_superclasses",
        &format!("{target_element} by NPC superclasses"),
        &by_npc_superclasses,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "npc_classes",
        &format!("{target_element} by NPC classes"),
        &by_npc_classes,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "source_dataset",
        &format!("{target_element} by source dataset"),
        &by_source_dataset,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "organism",
        &format!("{target_element} by organism"),
        &by_organism,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "ion_mode",
        &format!("{target_element} by ion mode"),
        &by_ion_mode,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "source_instrument",
        &format!("{target_element} by source instrument"),
        &by_source_instrument,
        total_records,
        records_with_target_element,
    )?;

    write_population_outputs(
        reports,
        "library_quality",
        &format!("{target_element} by library quality"),
        &by_library_quality,
        total_records,
        records_with_target_element,
    )?;

    println!("Total records: {total_records}");
    println!("Records with formula: {records_with_formula}");
    println!("Records with {target_element}: {records_with_target_element}");

    Ok(())
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

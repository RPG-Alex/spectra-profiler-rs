use mascot_rs::prelude::*;

use crate::{
    chemistry::contains_element,
    error::Result,
    metadata::{metadata_value, optional_debug_label},
    population::{
        PopulationMap, increment_pipe_population, increment_population, summarize_population_map,
        write_population_map_csv,
    },
    reports::ReportPaths,
    visuals::write_standard_population_figures,
};

pub fn profile_dataset(
    spectra: &MGFVec<f64>,
    target_element: &str,
    reports: &ReportPaths,
) -> Result<()> {
    let mut total_records = 0usize;
    let mut records_with_formula = 0usize;
    let mut records_with_target_element = 0usize;

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

        let formula = metadata.formula().map(ToString::to_string).unwrap_or_default();

        if !formula.is_empty() {
            records_with_formula += 1;
        }

        let contains_target_element = contains_element(&formula, target_element);

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
}

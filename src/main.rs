use std::collections::btree_map::BTreeMap;
use std::fs;

use mascot_rs::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SpectrumMetadataRow {
    spectrum_id: String,
    name: String,
    formula: String,
    inchikey: String,
    source_dataset: String,
    organism: String,
    ion_mode: String,
    source_instrument: String,
    npc_superclasses: String,
    npc_classes: String,
    npc_pathways: String,
    chemont_kingdom: String,
    chemont_superclass: String,
    chemont_class: String,
    chemont_subclass: String,
    chemont_direct_parent: String,
    library_quality: String,
    contains_f: bool,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("reports")?;
    let loaded = MGFVec::<f64>::annotated_ms2()
        .target_directory("data")
        .verbose()
        .load()
        .await?;

    let spectra = loaded.spectra();

    println!("Loaded {} spectra", spectra.len());
    println!("Skipped {} malformed records", loaded.skipped_records());
    println!("Dataset path: {}", loaded.path().display());

    let mut writer = csv::Writer::from_path("reports/metadata_rows.csv")?;

    let mut total_records = 0usize;
    let mut records_with_formula = 0usize;
    let mut records_with_fluorine = 0usize;
    let mut source_counts = BTreeMap::<String, usize>::new();
    let mut npc_class_counts_for_f = BTreeMap::<String, usize>::new();
    let mut npc_pathway_counts_for_f = BTreeMap::<String, usize>::new();
    let mut npc_superclass_counts_for_f = BTreeMap::<String, usize>::new();

    for record in spectra.iter() {
        total_records += 1;
        let metadata = record.metadata();
        let formula = metadata
            .formula()
            .map(ToString::to_string)
            .unwrap_or_default();

        if !formula.is_empty() {
            records_with_formula += 1;
        }
        let contains_f = contains_element(&formula, "F");

        if contains_f {
            records_with_fluorine += 1;
        }
        let source_dataset = metadata_value(metadata, "SOURCE_DATASET");
        increment(&mut source_counts, &source_dataset);

        let npc_classes = metadata_value(metadata, "NPC_CLASSES");
        let npc_pathways = metadata_value(metadata, "NPC_PATHWAYS");
        let npc_superclasses = metadata_value(metadata, "NPC_SUPERCLASSES");

        if contains_f {
            increment_pipe_values(&mut npc_pathway_counts_for_f, &npc_pathways);
            increment_pipe_values(&mut npc_superclass_counts_for_f, &npc_superclasses);
            increment_pipe_values(&mut npc_class_counts_for_f, &npc_classes);
        }
        writer.serialize(SpectrumMetadataRow {
            spectrum_id: metadata_value(metadata, "SPECTRUMID"),
            name: metadata_value(metadata, "NAME"),
            formula,
            inchikey: metadata_value(metadata, "INCHIKEY"),
            source_dataset,
            organism: metadata_value(metadata, "ORGANISM"),
            ion_mode: format!("{:?}", record.ion_mode()),
            source_instrument: format!("{:?}", record.source_instrument()),
            npc_superclasses,
            npc_classes,
            npc_pathways,
            chemont_kingdom: metadata_value(metadata, "CHEMONT_KINGDOM"),
            chemont_superclass: metadata_value(metadata, "CHEMONT_SUPERCLASS"),
            chemont_class: metadata_value(metadata, "CHEMONT_CLASS"),
            chemont_subclass: metadata_value(metadata, "CHEMONT_SUBCLASS"),
            chemont_direct_parent: metadata_value(metadata, "CHEMONT_DIRECT_PARENT"),
            library_quality: metadata_value(metadata, "LIBRARYQUALITY"),
            contains_f,
        })?;
    }
    writer.flush()?;

    write_counts_csv("reports/source_dataset_counts.csv", &source_counts)?;
    write_counts_csv_with_totals(
        "reports/fluorine_npc_pathway_counts.csv",
        &npc_pathway_counts_for_f,
        records_with_fluorine,
    )?;

    write_counts_csv_with_totals(
        "reports/fluorine_npc_superclass_counts.csv",
        &npc_superclass_counts_for_f,
        records_with_fluorine,
    )?;

    write_counts_csv_with_totals(
        "reports/fluorine_npc_class_counts.csv",
        &npc_class_counts_for_f,
        records_with_fluorine,
    )?;

    println!("Total records: {total_records}");
    println!("Records with formula: {records_with_formula}");
    println!("Records with fluorine: {records_with_fluorine}");
    println!("Wrote reports/metadata_rows.csv");
    println!("Wrote reports/source_dataset_counts.csv");
    println!("Wrote reports/fluorine_npc_class_counts.csv");

    Ok(())
}

fn metadata_value(metadata: &MascotGenericFormatMetadata, key: &str) -> String {
    metadata
        .arbitrary_metadata_value(key)
        .unwrap_or_default()
        .to_string()
}

fn split_pipe(value: &str) -> impl Iterator<Item = &str> {
    value
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
}

fn increment(counts: &mut BTreeMap<String, usize>, key: &str) {
    let key = if key.is_empty() { "UNKNOWN" } else { key };
    *counts.entry(key.to_string()).or_insert(0) += 1;
}

fn write_counts_csv(
    path: &str,
    counts: &BTreeMap<String, usize>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["value", "count"])?;

    for (value, count) in counts {
        writer.write_record([value, &count.to_string()])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_counts_csv_with_totals(
    path: &str,
    counts: &BTreeMap<String, usize>,
    total_records: usize,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["value", "count"])?;
    for (value, count) in counts {
        writer.write_record([value, &count.to_string()])?;
    }
    let total_assignments = counts.values().sum::<usize>();
    writer.write_record(["TOTAL_F_SPECTA", &total_records.to_string()])?;
    writer.write_record(["TOTAL_NPC_ASSIGNMENTS", &total_assignments.to_string()])?;
    writer.flush()?;
    Ok(())
}

fn contains_element(formula: &str, target: &str) -> bool {
    formula_symbols(formula).any(|symbol| symbol == target)
}

fn formula_symbols(formula: &str) -> impl Iterator<Item = String> + '_ {
    let mut chars = formula.chars().peekable();

    std::iter::from_fn(move || {
        while let Some(ch) = chars.next() {
            if !ch.is_ascii_uppercase() {
                continue;
            }
            let mut symbol = String::from(ch);

            if let Some(next) = chars.peek() {
                if next.is_ascii_lowercase() {
                    symbol.push(*next);
                    chars.next();
                }
            }
            return Some(symbol);
        }
        None
    })
}

fn increment_pipe_values(counts: &mut BTreeMap<String, usize>, value: &str) {
    if value.trim().is_empty() {
        increment(counts, "UNKNOWN");
        return;
    }
    for part in split_pipe(value) {
        increment(counts, part);
    }
}

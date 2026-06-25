use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use flate2::read::GzDecoder;
use mascot_rs::prelude::*;
use molecular_formulas::prelude::ChemicalFormula;
use smiles_parser::{
    DatasetFetchOptions, SmilesDatasetRecordSource, PUBCHEM_SMILES, smiles::Smiles,
};

use crate::{
    chemistry::element_counts_in_formula,
    config::DatasetSource,
    error::{Result, SpectraProfilerError},
    metadata::{metadata_value, optional_debug_label},
    records::{LoadedDataset, MoleculeRecord},
};

pub async fn load_dataset(
    dataset_name: &str,
    source: &DatasetSource,
    cache_dir: &Path,
) -> Result<LoadedDataset> {
    match source {
        DatasetSource::AnnotatedMs2 => load_annotated_ms2(dataset_name, cache_dir).await,
        DatasetSource::LocalMgf(path) => load_local_mgf(dataset_name, path),
        DatasetSource::PubChemSmiles => load_pubchem_smiles(dataset_name, cache_dir),
        DatasetSource::LocalSmilesGz(path) => load_smiles_gz(dataset_name, path),
    }
}

async fn load_annotated_ms2(dataset_name: &str, cache_dir: &Path) -> Result<LoadedDataset> {
    let loaded = MGFVec::<f64>::annotated_ms2()
        .target_directory(cache_dir)
        .verbose()
        .load()
        .await
        .map_err(|source| SpectraProfilerError::DatasetLoad { source: source.into() })?;

    println!("Skipped {} malformed records", loaded.skipped_records());
    println!("Dataset path: {}", loaded.path().display());

    Ok(mgf_to_loaded_dataset(dataset_name, loaded.into_spectra()))
}

fn load_local_mgf(dataset_name: &str, path: &Path) -> Result<LoadedDataset> {
    let spectra = MGFVec::<f64>::from_path(path)
        .map_err(|source| SpectraProfilerError::DatasetLoad { source: source.into() })?;

    Ok(mgf_to_loaded_dataset(dataset_name, spectra))
}

fn mgf_to_loaded_dataset(dataset_name: &str, spectra: MGFVec<f64>) -> LoadedDataset {
    let records = spectra
        .iter()
        .enumerate()
        .filter_map(|(index, record)| {
            let formula = record.metadata().formula()?;

            let metadata = record.metadata();

            let mut groups = BTreeMap::new();

            groups.insert("Source dataset".to_string(), metadata_value(metadata, "SOURCE_DATASET"));
            groups.insert("Organism".to_string(), metadata_value(metadata, "ORGANISM"));
            groups.insert("NPC pathways".to_string(), metadata_value(metadata, "NPC_PATHWAYS"));
            groups.insert(
                "NPC superclasses".to_string(),
                metadata_value(metadata, "NPC_SUPERCLASSES"),
            );
            groups.insert("NPC classes".to_string(), metadata_value(metadata, "NPC_CLASSES"));
            groups
                .insert("Library quality".to_string(), metadata_value(metadata, "LIBRARYQUALITY"));
            groups.insert("Ion mode".to_string(), optional_debug_label(record.ion_mode()));
            groups.insert(
                "Source instrument".to_string(),
                optional_debug_label(record.source_instrument()),
            );

            Some(MoleculeRecord {
                id: record
                    .feature_id()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| format!("record-{index}")),
                element_counts: element_counts_in_formula(formula),
                metadata: groups,
                peak_count: Some(record.len()),
            })
        })
        .collect();

    LoadedDataset { name: dataset_name.to_string(), records }
}

fn load_smiles_gz(dataset_name: &str, path: &Path) -> Result<LoadedDataset> {
    let limit = record_limit();

    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let mut records = Vec::new();
    let mut skipped = 0usize;

    for line in reader.lines().take(limit.unwrap_or(usize::MAX)) {
        let line = line?;

        let Some((cid, smiles_text)) = line.split_once(char::is_whitespace) else {
            skipped += 1;
            continue;
        };

        let smiles_text = smiles_text.trim();

        let Ok(smiles) = smiles_text.parse::<Smiles>() else {
            skipped += 1;
            continue;
        };

        let formula: ChemicalFormula<u32, i32> = ChemicalFormula::from(&smiles);

        let mut metadata = BTreeMap::new();
        metadata.insert("Source dataset".to_string(), dataset_name.to_string());

        records.push(MoleculeRecord {
            id: cid.to_string(),
            element_counts: element_counts_in_formula(&formula),
            metadata,
            peak_count: None,
        });
    }

    println!("Loaded {} local SMILES records", records.len());
    println!("Skipped {skipped} local SMILES records");

    Ok(LoadedDataset { name: dataset_name.to_string(), records })
}

fn record_limit() -> Option<usize> {
    std::env::var("PROFILE_LIMIT")
        .or_else(|_| std::env::var("PUBCHEM_LIMIT"))
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
}

fn load_pubchem_smiles(dataset_name: &str, cache_dir: &Path) -> Result<LoadedDataset> {
    let limit = record_limit();

    let options = DatasetFetchOptions {
        cache_dir: Some(cache_dir.to_path_buf()),
        ..DatasetFetchOptions::default()
    };

    let pubchem_records = PUBCHEM_SMILES
        .iter_records_with_options(&options)
        .map_err(|source| SpectraProfilerError::DatasetLoad { source: source.into() })?;

    let mut records = Vec::new();
    let mut skipped = 0usize;

    for record in pubchem_records.take(limit.unwrap_or(usize::MAX)) {
        let record =
            record.map_err(|source| SpectraProfilerError::DatasetLoad { source: source.into() })?;

        let Ok(smiles) = record.smiles().parse::<Smiles>() else {
            skipped += 1;
            continue;
        };

        let formula: ChemicalFormula<u32, i32> = ChemicalFormula::from(&smiles);

        let mut metadata = BTreeMap::new();
        metadata.insert("Source dataset".to_string(), "PubChem".to_string());

        records.push(MoleculeRecord {
            id: record.id().to_string(),
            element_counts: element_counts_in_formula(&formula),
            metadata,
            peak_count: None,
        });
    }

    println!("Loaded {} PubChem records", records.len());
    println!("Skipped {skipped} PubChem records");

    Ok(LoadedDataset {
        name: dataset_name.to_string(),
        records,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::config::DatasetSource;

    #[tokio::test]
    async fn local_mgf_missing_path_returns_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let missing_path = temp_dir.path().join("missing.mgf");

        let result = load_dataset(
            "missing",
            &DatasetSource::LocalMgf(missing_path),
            &PathBuf::from("unused-cache-dir"),
        )
        .await;

        assert!(result.is_err());
    }
}

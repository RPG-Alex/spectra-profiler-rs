use std::path::Path;

use mascot_rs::prelude::*;

use crate::config::DatasetSource;

pub async fn load_dataset(
    source: &DatasetSource,
    cache_dir: &Path,
) -> std::result::Result<MGFVec<f64>, Box<dyn std::error::Error>> {
    match source {
        DatasetSource::AnnotatedMs2 => {
            let loaded = MGFVec::<f64>::annotated_ms2()
                .target_directory(cache_dir)
                .verbose()
                .load()
                .await?;

            println!("Skipped {} malformed records", loaded.skipped_records());
            println!("Dataset path: {}", loaded.path().display());

            Ok(loaded.into_spectra())
        }

        DatasetSource::LocalMgf(path) => {
            let spectra = MGFVec::<f64>::from_path(path)?;
            Ok(spectra)
        }
    }
}

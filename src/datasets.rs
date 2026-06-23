use std::path::Path;

use mascot_rs::prelude::*;

use crate::config::DatasetSource;

pub async fn load_dataset(
    source: &DatasetSource,
    cache_dir: &Path,
) -> std::result::Result<MGFVec<f64>, Box<dyn std::error::Error>> {
    match source {
        DatasetSource::AnnotatedMs2 => {
            let loaded =
                MGFVec::<f64>::annotated_ms2().target_directory(cache_dir).verbose().load().await?;

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
            &DatasetSource::LocalMgf(missing_path),
            &PathBuf::from("unused-cache-dir"),
        )
        .await;

        assert!(result.is_err());
    }
}

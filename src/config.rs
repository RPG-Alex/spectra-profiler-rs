use std::io;
use std::path::PathBuf;

use crate::chemistry::normalize_element_symbol;

#[derive(Debug, Clone)]
pub enum DatasetSource {
    AnnotatedMs2,
    LocalMgf(PathBuf),
}

#[derive(Debug, Clone)]
pub enum TargetSelection {
    One(String),
    AllObserved,
}

#[derive(Debug, Clone)]
pub struct ProfileConfig {
    pub dataset_name: String,
    pub dataset_source: DatasetSource,
    pub target_selection: TargetSelection,
    pub cache_dir: PathBuf,
    pub reports_root: PathBuf,
}

impl ProfileConfig {
    pub fn from_args() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let mut args = std::env::args().skip(1);

        let raw_target = args.next().unwrap_or_else(|| "F".to_string());

        let target_selection = if raw_target.eq_ignore_ascii_case("all") {
            TargetSelection::AllObserved
        } else {
            let target_element = normalize_element_symbol(&raw_target).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid element symbol: {raw_target}"),
                )
            })?;

            TargetSelection::One(target_element)
        };

        let dataset_path = args.next().map(PathBuf::from);

        let (dataset_name, dataset_source) = match dataset_path {
            Some(path) => {
                let dataset_name = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("local_mgf")
                    .to_string();

                (dataset_name, DatasetSource::LocalMgf(path))
            }
            None => ("annotated_ms2".to_string(), DatasetSource::AnnotatedMs2),
        };

        Ok(Self {
            dataset_name: dataset_name.clone(),
            dataset_source,
            target_selection,
            cache_dir: PathBuf::from("cache").join(&dataset_name),
            reports_root: PathBuf::from("reports").join(&dataset_name),
        })
    }

    pub fn report_dir_for(&self, target_element: &str) -> PathBuf {
        self.reports_root.join(target_element.to_ascii_lowercase())
    }
}

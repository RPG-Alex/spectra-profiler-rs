use std::{io, path::PathBuf};

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
        Self::from_iter(std::env::args().skip(1))
    }

    pub fn from_iter<I, S>(args: I) -> std::result::Result<Self, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut args = args.into_iter().map(Into::into);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_fluorine_and_annotated_ms2() {
        let config = ProfileConfig::from_iter(Vec::<String>::new()).unwrap();

        assert_eq!(config.dataset_name, "annotated_ms2");

        match config.target_selection {
            TargetSelection::One(element) => assert_eq!(element, "F"),
            TargetSelection::AllObserved => panic!("expected one target element"),
        }

        match config.dataset_source {
            DatasetSource::AnnotatedMs2 => {}
            DatasetSource::LocalMgf(_) => panic!("expected annotated_ms2 source"),
        }
    }

    #[test]
    fn parses_single_target_element() {
        let config = ProfileConfig::from_iter(["cl"]).unwrap();

        match config.target_selection {
            TargetSelection::One(element) => assert_eq!(element, "Cl"),
            TargetSelection::AllObserved => panic!("expected one target element"),
        }
    }

    #[test]
    fn parses_all_observed_target_selection() {
        let config = ProfileConfig::from_iter(["all"]).unwrap();

        match config.target_selection {
            TargetSelection::AllObserved => {}
            TargetSelection::One(element) => panic!("expected all, got {element}"),
        }
    }

    #[test]
    fn parses_local_mgf_path() {
        let config = ProfileConfig::from_iter(["F", "data/example_file.mgf"]).unwrap();

        assert_eq!(config.dataset_name, "example_file");

        match config.dataset_source {
            DatasetSource::LocalMgf(path) => {
                assert_eq!(path, PathBuf::from("data/example_file.mgf"));
            }
            DatasetSource::AnnotatedMs2 => panic!("expected local MGF source"),
        }
    }

    #[test]
    fn builds_report_directory_for_element() {
        let config = ProfileConfig::from_iter(["F"]).unwrap();

        assert_eq!(
            config.report_dir_for("Cl"),
            PathBuf::from("reports").join("annotated_ms2").join("cl")
        );
    }
}

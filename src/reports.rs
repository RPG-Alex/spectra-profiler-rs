use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ReportPaths {
    pub root: PathBuf,
    pub tables: PathBuf,
    pub figures: PathBuf,
}

impl ReportPaths {
    pub fn prepare(root: impl AsRef<Path>) -> std::io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        let tables = root.join("tables");
        let figures = root.join("figures");

        fs::create_dir_all(&tables)?;
        fs::create_dir_all(&figures)?;

        Ok(Self { root, tables, figures })
    }

    pub fn table(&self, filename: &str) -> PathBuf {
        self.tables.join(filename)
    }

    pub fn figure(&self, filename: &str) -> PathBuf {
        self.figures.join(filename)
    }

    pub fn readme(&self) -> PathBuf {
        self.root.join("README.md")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_report_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root = temp_dir.path().join("reports").join("annotated_ms2").join("f");

        let reports = ReportPaths::prepare(&root).unwrap();

        assert_eq!(reports.root, root);
        assert!(reports.tables.exists());
        assert!(reports.figures.exists());
    }

    #[test]
    fn builds_table_figure_and_readme_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("report")).unwrap();

        assert_eq!(reports.table("summary.csv"), reports.tables.join("summary.csv"));

        assert_eq!(reports.figure("plot.svg"), reports.figures.join("plot.svg"));

        assert_eq!(reports.readme(), reports.root.join("README.md"));
    }
}

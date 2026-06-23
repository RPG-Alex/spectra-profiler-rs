use std::fs;
use std::path::{Path, PathBuf};

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

        Ok(Self {
            root,
            tables,
            figures,
        })
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

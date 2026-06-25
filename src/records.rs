use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MoleculeRecord {
    #[allow(dead_code)]
    pub id: String,
    pub element_counts: BTreeMap<String, usize>,
    pub metadata: BTreeMap<String, String>,
    #[allow(dead_code)]
    pub peak_count: Option<usize>,
}

impl MoleculeRecord {
    pub fn contains_element(&self, element: &str) -> bool {
        self.atom_count(element) > 0
    }

    pub fn atom_count(&self, element: &str) -> usize {
        self.element_counts.get(element).copied().unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct LoadedDataset {
    #[allow(dead_code)]
    pub name: String,
    pub records: Vec<MoleculeRecord>,
}

impl LoadedDataset {
    pub fn len(&self) -> usize {
        self.records.len()
    }
}

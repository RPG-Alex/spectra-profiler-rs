use std::{fs::File, io::Write};

use crate::error::Result;

#[derive(Debug, Clone, Copy)]
pub(super) struct PopulationTable {
    pub(super) title: &'static str,
    pub(super) filename: &'static str,
}

pub(super) const POPULATION_TABLES: &[PopulationTable] = &[
    PopulationTable { title: "NPC pathways", filename: "contains_by_npc_pathways.csv" },
    PopulationTable { title: "NPC superclasses", filename: "contains_by_npc_superclasses.csv" },
    PopulationTable { title: "NPC classes", filename: "contains_by_npc_classes.csv" },
    PopulationTable { title: "Source dataset", filename: "contains_by_source_dataset.csv" },
    PopulationTable { title: "Organism", filename: "contains_by_organism.csv" },
    PopulationTable { title: "Ion mode", filename: "contains_by_ion_mode.csv" },
    PopulationTable { title: "Source instrument", filename: "contains_by_source_instrument.csv" },
    PopulationTable { title: "Library quality", filename: "contains_by_library_quality.csv" },
];

#[derive(Debug, Clone, Copy)]
pub(super) struct ReportSection {
    pub(super) stem: &'static str,
    pub(super) title: &'static str,
    pub(super) description: &'static str,
}

pub(super) const REPORT_SECTIONS: &[ReportSection] = &[
    ReportSection {
        stem: "npc_pathways",
        title: "NPC pathways",
        description: "Groups spectra by broad natural-product pathway annotations.",
    },
    ReportSection {
        stem: "npc_superclasses",
        title: "NPC superclasses",
        description: "Groups spectra by intermediate natural-product superclass annotations.",
    },
    ReportSection {
        stem: "npc_classes",
        title: "NPC classes",
        description: "Groups spectra by more specific natural-product class annotations.",
    },
    ReportSection {
        stem: "source_dataset",
        title: "Source dataset",
        description: "Groups spectra by the dataset or spectral-library source recorded in metadata.",
    },
    ReportSection {
        stem: "organism",
        title: "Organism",
        description: "Groups spectra by organism or source-organism metadata when available.",
    },
    ReportSection {
        stem: "ion_mode",
        title: "Ion mode",
        description: "Groups spectra by recorded ionization mode, such as positive or negative mode.",
    },
    ReportSection {
        stem: "source_instrument",
        title: "Source instrument",
        description: "Groups spectra by the instrument metadata associated with the source record.",
    },
    ReportSection {
        stem: "library_quality",
        title: "Library quality",
        description: "Groups spectra by the recorded quality label from the source library metadata.",
    },
];

pub(super) fn write_population_section(file: &mut File, section: &ReportSection) -> Result<()> {
    let table_path = format!("tables/contains_by_{}.csv", section.stem);
    let count_figure_path = format!("figures/top_{}_by_target_count.svg", section.stem);
    let percent_figure_path = format!("figures/top_{}_by_percent_target.svg", section.stem);

    writeln!(file)?;
    writeln!(file, "## {}", section.title)?;
    writeln!(file)?;
    writeln!(file, "{}", section.description)?;
    writeln!(file)?;
    writeln!(file, "[CSV table]({table_path})")?;
    writeln!(file)?;
    writeln!(file, "<table>")?;
    writeln!(file, "<tr>")?;
    writeln!(file, "<th>Top groups by target count</th>")?;
    writeln!(file, "<th>Top groups by percent target</th>")?;
    writeln!(file, "</tr>")?;
    writeln!(file, "<tr>")?;
    writeln!(
        file,
        "<td width=\"50%\"><img src=\"{count_figure_path}\" alt=\"{} by target count\" /></td>",
        section.title
    )?;
    writeln!(
        file,
        "<td width=\"50%\"><img src=\"{percent_figure_path}\" alt=\"{} by percent target\" /></td>",
        section.title
    )?;
    writeln!(file, "</tr>")?;
    writeln!(file, "</table>")?;

    Ok(())
}

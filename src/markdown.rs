use std::fs::File;
use std::io::Write;

use crate::reports::ReportPaths;

#[derive(Debug, Clone, Copy)]
struct ReportSection {
    stem: &'static str,
    title: &'static str,
    description: &'static str,
}

const REPORT_SECTIONS: &[ReportSection] = &[
    ReportSection {
        stem: "npc_pathways",
        title: "NPC pathways",
        description: "Natural-product pathway-level distribution for the target element.",
    },
    ReportSection {
        stem: "npc_superclasses",
        title: "NPC superclasses",
        description: "Natural-product superclass-level distribution for the target element.",
    },
    ReportSection {
        stem: "npc_classes",
        title: "NPC classes",
        description: "Natural-product class-level distribution for the target element.",
    },
    ReportSection {
        stem: "source_dataset",
        title: "Source dataset",
        description: "Distribution by original source dataset.",
    },
    ReportSection {
        stem: "organism",
        title: "Organism",
        description: "Distribution by organism/source organism metadata.",
    },
    ReportSection {
        stem: "ion_mode",
        title: "Ion mode",
        description: "Distribution by recorded ion mode.",
    },
    ReportSection {
        stem: "source_instrument",
        title: "Source instrument",
        description: "Distribution by recorded source instrument.",
    },
    ReportSection {
        stem: "library_quality",
        title: "Library quality",
        description: "Distribution by library quality metadata.",
    },
];

pub fn write_markdown_report(
    dataset_name: &str,
    target_element: &str,
    reports: &ReportPaths,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(reports.readme())?;

    writeln!(
        file,
        "# `{}` profile for `{}`",
        target_element, dataset_name
    )?;

    writeln!(file)?;
    writeln!(
        file,
        "This report summarizes how often the target element `{}` appears across metadata groups in `{}`.",
        target_element, dataset_name
    )?;

    writeln!(file)?;
    writeln!(file, "## Summary")?;
    writeln!(file)?;
    writeln!(file, "- [Summary table](tables/summary.csv)")?;
    writeln!(file, "- Tables are in [`tables/`](tables/)")?;
    writeln!(file, "- Figures are in [`figures/`](figures/)")?;

    writeln!(file)?;
    writeln!(file, "## How to read the figures")?;
    writeln!(file)?;
    writeln!(
        file,
        "- **Target count** shows which groups contribute the most target-positive spectra."
    )?;
    writeln!(
        file,
        "- **Percent target** shows which groups are most enriched for the target element."
    )?;
    writeln!(
        file,
        "- Small groups can look highly enriched, so check the linked CSV tables for support counts."
    )?;

    for section in REPORT_SECTIONS {
        write_section(&mut file, section)?;
    }

    Ok(())
}

fn write_section(
    file: &mut File,
    section: &ReportSection,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

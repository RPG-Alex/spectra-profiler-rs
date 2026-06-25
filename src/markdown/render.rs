use std::{fs::File, io::Write};

use super::data::{
    EnrichedGroupSummary, NumericSummary, TOP_ENRICHED_MIN_TOTAL_SUPPORT, WarningSummary,
};
use crate::error::Result;

pub(super) fn write_numeric_summary(file: &mut File, summary: &NumericSummary) -> Result<()> {
    writeln!(file)?;
    writeln!(file, "## Numeric summary")?;
    writeln!(file)?;
    writeln!(file, "| Metric | Value |")?;
    writeln!(file, "|---|---:|")?;
    writeln!(file, "| Total spectra | {} |", summary.total_spectra)?;
    writeln!(file, "| Positive count | {} |", summary.positive_count)?;
    writeln!(file, "| Negative count | {} |", summary.negative_count)?;
    writeln!(file, "| Positive percentage | {:.4}% |", summary.positive_percentage)?;

    Ok(())
}

pub(super) fn write_atom_count_distribution_section(
    file: &mut File,
    target_element: &str,
) -> Result<()> {
    writeln!(file)?;
    writeln!(file, "## Atom-count distribution")?;
    writeln!(file)?;
    writeln!(
        file,
        "This section shows how many formula-bearing spectra have exactly `k` atoms of `{target_element}`."
    )?;
    writeln!(file, "The `0` row represents formulas that do not contain `{target_element}`.")?;
    writeln!(file)?;
    writeln!(file, "[CSV table](tables/target_atom_count_distribution.csv)")?;
    writeln!(file)?;
    writeln!(
        file,
        "<img src=\"figures/target_atom_count_distribution.svg\" alt=\"{target_element} atom-count distribution\" />"
    )?;

    Ok(())
}

pub(super) fn write_top_enriched_groups(
    file: &mut File,
    groups: &[EnrichedGroupSummary],
) -> Result<()> {
    writeln!(file)?;
    writeln!(file, "## Top enriched groups")?;
    writeln!(file)?;
    writeln!(
        file,
        "This table compares **metadata groups** across all population-map tables. \
         A metadata group is one field/value pair, such as `NPC classes = Carboline alkaloids` \
         or `Ion mode = Positive`."
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "The table is sorted by **Positive %**, meaning the percentage of spectra inside that \
         group whose formulas contain the target element. Only groups with at least \
         `{TOP_ENRICHED_MIN_TOTAL_SUPPORT}` total spectra are included."
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "This table answers: **where is the target element unusually common?** \
         It does not necessarily show the groups with the largest absolute number of positives."
    )?;
    writeln!(file)?;

    if groups.is_empty() {
        writeln!(file, "No enriched groups met the minimum support threshold.")?;
        return Ok(());
    }

    writeln!(file, "| Metadata group | Value | Total | Positive | Positive % | % of positives |")?;
    writeln!(file, "|---|---|---:|---:|---:|---:|")?;

    for group in groups {
        writeln!(
            file,
            "| {} | {} | {} | {} | {:.2}% | {:.2}% |",
            group.metadata_group,
            group.value,
            group.total_count,
            group.target_count,
            group.percent_target_within_group,
            group.percent_of_all_target
        )?;
    }

    Ok(())
}

pub(super) fn write_warning_summary(file: &mut File, warnings: &[WarningSummary]) -> Result<()> {
    writeln!(file)?;
    writeln!(file, "## Low-support warning summary")?;
    writeln!(file)?;
    writeln!(
        file,
        "This section summarizes warning flags from the population-map CSV tables. \
         The `Count` column is the number of metadata-group rows with that warning, \
         not the number of spectra."
    )?;
    writeln!(file)?;
    writeln!(file, "Warning meanings:")?;
    writeln!(file)?;
    writeln!(file, "| Warning | Meaning |")?;
    writeln!(file, "|---|---|")?;
    writeln!(
        file,
        "| `LOW_TOTAL_SUPPORT` | The group has fewer than the minimum number of total spectra. |"
    )?;
    writeln!(
        file,
        "| `LOW_TARGET_SUPPORT` | The group has some target-positive spectra, but too few for confident interpretation. |"
    )?;
    writeln!(
        file,
        "| `NO_TARGET_POSITIVES` | The group has no spectra whose formulas contain the target element. |"
    )?;

    writeln!(file)?;

    if warnings.is_empty() {
        writeln!(file, "No low-support warnings were found in the population tables.")?;
        return Ok(());
    }

    writeln!(file, "| Warning | Count |")?;
    writeln!(file, "|---|---:|")?;

    for warning in warnings {
        writeln!(file, "| `{}` | {} |", warning.warning, warning.count)?;
    }

    Ok(())
}

pub(super) fn write_interpretation_guide(file: &mut File, target_element: &str) -> Result<()> {
    writeln!(file)?;
    writeln!(file, "## How to interpret this report")?;
    writeln!(file)?;
    writeln!(
        file,
        "This report treats each spectrum as **positive** when its molecular formula contains \
         the target element `{target_element}`. A spectrum is **negative** when its formula does \
         not contain `{target_element}`."
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "A **metadata group** means one metadata field and one value inside that field. \
         For example, in the `NPC classes` table, `Carboline alkaloids` is one group. \
         In the `Ion mode` table, `Positive` is one group."
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "The profiler compares the target-positive spectra against these groups to show \
         where the target element is common, rare, concentrated, or poorly supported."
    )?;
    writeln!(file)?;
    writeln!(file, "Important caveats:")?;
    writeln!(
        file,
        "- These reports are based on formula metadata, not direct spectral proof of the element."
    )?;
    writeln!(
        file,
        "- Some metadata fields can contain multiple pipe-separated values, so assignment counts \
         can be larger than the number of spectra."
    )?;
    writeln!(
        file,
        "- Highly enriched small groups can be interesting, but they should not be overinterpreted \
         without checking support counts."
    )?;

    Ok(())
}

pub(super) fn write_glossary_and_references(file: &mut File) -> Result<()> {
    writeln!(file)?;
    writeln!(file, "## Glossary and external references")?;
    writeln!(file)?;
    writeln!(file, "| Term | Meaning in this report | Reference |")?;
    writeln!(file, "|---|---|---|")?;
    writeln!(
        file,
        "| Molecular formula | Formula metadata used to decide whether a spectrum is target-positive. | [PubChem glossary - Molecular Formula](https://pubchem.ncbi.nlm.nih.gov/docs/glossary#section=Molecular-Formula) |"
    )?;
    writeln!(
        file,
        "| Target-positive spectrum | A spectrum whose molecular formula contains the selected target element. | Local report definition |"
    )?;
    writeln!(
        file,
        "| Metadata group | A group formed from one metadata field and one value, such as `NPC classes = Carboline alkaloids`. | Local report definition |"
    )?;
    writeln!(
        file,
        "| NPC pathways / superclasses / classes | Natural-product classification fields from NPClassifier-style annotations. | [NPClassifier](https://npclassifier.ucsd.edu/) |"
    )?;
    writeln!(
        file,
        "| ClassyFire taxonomy | Chemical taxonomy fields such as kingdom, superclass, class, subclass, and direct parent. | [ClassyFire paper](https://pmc.ncbi.nlm.nih.gov/articles/PMC5096306/) |"
    )?;
    writeln!(
        file,
        "| Source dataset | The dataset or library source from which the spectrum metadata originated. | [GNPS libraries](https://ccms-ucsd.github.io/GNPSDocumentation/gnpslibraries/) / [MassSpecGym](https://github.com/pluskal-lab/MassSpecGym) |"
    )?;
    writeln!(
        file,
        "| Enrichment | A group has high enrichment when a large percentage of spectra in that group are target-positive. | Local report definition |"
    )?;
    writeln!(
        file,
        "| Low support | A warning that a group has too few total spectra, too few target-positive spectra, or no target-positive spectra. | Local report definition |"
    )?;

    Ok(())
}

pub(super) fn write_report_links(file: &mut File) -> Result<()> {
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

    Ok(())
}

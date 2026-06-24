use std::{cmp::Ordering, collections::BTreeMap, fs::File, io::Write};

use serde::Deserialize;

use crate::{
    error::{Result, SpectraProfilerError},
    reports::ReportPaths,
};

const TOP_ENRICHED_LIMIT: usize = 8;
const TOP_ENRICHED_MIN_TOTAL_SUPPORT: usize = 30;

#[derive(Debug)]
struct NumericSummary {
    total_spectra: usize,
    positive_count: usize,
    negative_count: usize,
    positive_percentage: f64,
}

#[derive(Debug)]
struct EnrichedGroupSummary {
    metadata_group: String,
    value: String,
    total_count: usize,
    target_count: usize,
    percent_target_within_group: f64,
    percent_of_all_target: f64,
}

#[derive(Debug)]
struct WarningSummary {
    warning: String,
    count: usize,
}

#[derive(Debug)]
struct MarkdownReportSummary {
    numeric: NumericSummary,
    top_enriched_groups: Vec<EnrichedGroupSummary>,
    warning_summary: Vec<WarningSummary>,
}

#[derive(Debug, Deserialize)]
struct MetricCsvRow {
    metric: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct PopulationCsvRow {
    value: String,
    total_count: usize,
    target_count: usize,
    percent_target_within_group: f64,
    percent_of_all_target: f64,
    support_warning: String,
}

const POPULATION_TABLES: &[(&str, &str)] = &[
    ("NPC pathways", "contains_by_npc_pathways.csv"),
    ("NPC superclasses", "contains_by_npc_superclasses.csv"),
    ("NPC classes", "contains_by_npc_classes.csv"),
    ("Source dataset", "contains_by_source_dataset.csv"),
    ("Organism", "contains_by_organism.csv"),
    ("Ion mode", "contains_by_ion_mode.csv"),
    ("Source instrument", "contains_by_source_instrument.csv"),
    ("Library quality", "contains_by_library_quality.csv"),
];

fn read_markdown_report_summary(reports: &ReportPaths) -> Result<MarkdownReportSummary> {
    Ok(MarkdownReportSummary {
        numeric: read_numeric_summary(reports)?,
        top_enriched_groups: read_top_enriched_groups(reports)?,
        warning_summary: read_warning_summary(reports)?,
    })
}

fn read_numeric_summary(reports: &ReportPaths) -> Result<NumericSummary> {
    let mut reader = csv::Reader::from_path(reports.table("summary.csv"))?;
    let mut metrics = BTreeMap::new();

    for row in reader.deserialize::<MetricCsvRow>() {
        let row = row?;
        metrics.insert(row.metric, row.value);
    }

    let total_spectra = read_usize_metric(&metrics, "total_records")?;
    let positive_count = read_usize_metric(&metrics, "records_with_target_element")?;
    let negative_count = total_spectra.saturating_sub(positive_count);

    let positive_percentage =
        if total_spectra == 0 { 0.0 } else { positive_count as f64 / total_spectra as f64 * 100.0 };

    Ok(NumericSummary { total_spectra, positive_count, negative_count, positive_percentage })
}

fn read_usize_metric(metrics: &BTreeMap<String, String>, metric: &'static str) -> Result<usize> {
    let value = metrics.get(metric).ok_or(SpectraProfilerError::MissingSummaryMetric { metric })?;

    value
        .parse()
        .map_err(|_| SpectraProfilerError::InvalidSummaryMetric { metric, value: value.clone() })
}

fn read_top_enriched_groups(reports: &ReportPaths) -> Result<Vec<EnrichedGroupSummary>> {
    let mut groups = Vec::new();

    for (metadata_group, filename) in POPULATION_TABLES {
        for row in read_population_rows(reports, filename)? {
            if row.total_count < TOP_ENRICHED_MIN_TOTAL_SUPPORT || row.target_count == 0 {
                continue;
            }

            groups.push(EnrichedGroupSummary {
                metadata_group: (*metadata_group).to_string(),
                value: row.value,
                total_count: row.total_count,
                target_count: row.target_count,
                percent_target_within_group: row.percent_target_within_group,
                percent_of_all_target: row.percent_of_all_target,
            });
        }
    }

    groups.sort_by(|left, right| {
        right
            .percent_target_within_group
            .partial_cmp(&left.percent_target_within_group)
            .unwrap_or(Ordering::Equal)
            .then_with(|| right.target_count.cmp(&left.target_count))
            .then_with(|| right.total_count.cmp(&left.total_count))
    });

    groups.truncate(TOP_ENRICHED_LIMIT);

    Ok(groups)
}

fn read_warning_summary(reports: &ReportPaths) -> Result<Vec<WarningSummary>> {
    let mut counts = BTreeMap::<String, usize>::new();

    for (_, filename) in POPULATION_TABLES {
        for row in read_population_rows(reports, filename)? {
            for warning in row
                .support_warning
                .split(['|', ';', ','])
                .map(str::trim)
                .filter(|warning| !warning.is_empty())
                .filter(|warning| *warning != "NONE" && *warning != "OK")
            {
                *counts.entry(warning.to_string()).or_default() += 1;
            }
        }
    }

    Ok(counts.into_iter().map(|(warning, count)| WarningSummary { warning, count }).collect())
}

fn read_population_rows(reports: &ReportPaths, filename: &str) -> Result<Vec<PopulationCsvRow>> {
    let mut reader = csv::Reader::from_path(reports.table(filename))?;
    let rows =
        reader.deserialize::<PopulationCsvRow>().collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(rows)
}

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

pub fn write_markdown_report(
    dataset_name: &str,
    target_element: &str,
    reports: &ReportPaths,
) -> Result<()> {
    let mut file = File::create(reports.readme())?;

    let summary = read_markdown_report_summary(reports)?;

    writeln!(file, "# `{target_element}` profile for `{dataset_name}`")?;

    writeln!(file)?;
    writeln!(
        file,
        "This report summarizes how often the target element `{target_element}` appears across metadata groups in `{dataset_name}`.",
    )?;

    writeln!(file)?;

    write_interpretation_guide(&mut file, target_element)?;
    write_glossary_and_references(&mut file)?;
    write_numeric_summary(&mut file, &summary.numeric)?;
    write_top_enriched_groups(&mut file, &summary.top_enriched_groups)?;
    write_warning_summary(&mut file, &summary.warning_summary)?;

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

fn write_section(file: &mut File, section: &ReportSection) -> Result<()> {
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

fn write_numeric_summary(file: &mut File, summary: &NumericSummary) -> Result<()> {
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

fn write_top_enriched_groups(file: &mut File, groups: &[EnrichedGroupSummary]) -> Result<()> {
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

fn write_warning_summary(file: &mut File, warnings: &[WarningSummary]) -> Result<()> {
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

fn write_interpretation_guide(file: &mut File, target_element: &str) -> Result<()> {
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

fn write_glossary_and_references(file: &mut File) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reports::ReportPaths;

    fn write_summary_fixture(reports: &ReportPaths) {
        std::fs::write(
            reports.table("summary.csv"),
            [
                "metric,value",
                "target_element,F",
                "total_records,100",
                "records_with_formula,100",
                "records_with_target_element,10",
            ]
            .join("\n"),
        )
        .unwrap();
    }

    fn write_population_fixtures(reports: &ReportPaths) {
        let fixture = [
        "value,total_count,target_count,non_target_count,percent_target_within_group,percent_of_all_records,percent_of_all_target,support_warning",
        "Class A,50,10,40,20.0,50.0,100.0,NONE",
        "Tiny Class,5,5,0,100.0,5.0,50.0,LOW_TOTAL_SUPPORT",
        "Class B,45,0,45,0.0,45.0,0.0,NO_TARGET_POSITIVES",
    ]
    .join("\n");

        for filename in [
            "contains_by_npc_pathways.csv",
            "contains_by_npc_superclasses.csv",
            "contains_by_npc_classes.csv",
            "contains_by_source_dataset.csv",
            "contains_by_organism.csv",
            "contains_by_ion_mode.csv",
            "contains_by_source_instrument.csv",
            "contains_by_library_quality.csv",
        ] {
            std::fs::write(reports.table(filename), &fixture).unwrap();
        }
    }

    #[test]
    fn writes_markdown_report() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("f")).unwrap();

        write_summary_fixture(&reports);
        write_population_fixtures(&reports);

        write_markdown_report("annotated_ms2", "F", &reports).unwrap();

        let contents = std::fs::read_to_string(reports.readme()).unwrap();

        assert!(contents.contains("# `F` profile for `annotated_ms2`"));
        assert!(contents.contains("## Numeric summary"));
        assert!(contents.contains("| Total spectra | 100 |"));
        assert!(contents.contains("| Positive count | 10 |"));
        assert!(contents.contains("| Negative count | 90 |"));
        assert!(contents.contains("## Top enriched groups"));
        assert!(contents.contains("## Low-support warning summary"));
        assert!(contents.contains("tables/summary.csv"));
        assert!(contents.contains("figures/top_npc_classes_by_target_count.svg"));
        assert!(contents.contains("figures/top_npc_classes_by_percent_target.svg"));
    }

    #[test]
    fn read_numeric_summary_errors_when_required_metric_is_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("f")).unwrap();

        std::fs::write(
            reports.table("summary.csv"),
            ["metric,value", "target_element,F", "total_records,100"].join("\n"),
        )
        .unwrap();

        let error = read_numeric_summary(&reports).unwrap_err();

        assert!(matches!(
            error,
            SpectraProfilerError::MissingSummaryMetric { metric: "records_with_target_element" }
        ));
    }

    #[test]
    fn read_numeric_summary_errors_when_metric_is_not_numeric() {
        let temp_dir = tempfile::tempdir().unwrap();
        let reports = ReportPaths::prepare(temp_dir.path().join("f")).unwrap();

        std::fs::write(
            reports.table("summary.csv"),
            [
                "metric,value",
                "target_element,F",
                "total_records,not-a-number",
                "records_with_formula,100",
                "records_with_target_element,10",
            ]
            .join("\n"),
        )
        .unwrap();

        let error = read_numeric_summary(&reports).unwrap_err();

        assert!(matches!(
            error,
            SpectraProfilerError::InvalidSummaryMetric { metric: "total_records", .. }
        ));
    }
}

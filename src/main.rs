mod chemistry;
mod config;
mod cooccurence;
mod datasets;
mod error;
mod markdown;
mod metadata;
mod population;
mod profiler;
mod records;
mod reports;
mod visuals;

use std::collections::BTreeSet;

use config::{ProfileConfig, TargetSelection};
use cooccurence::write_cooccurrence_reports;
use datasets::load_dataset;
use markdown::write_markdown_report;
use profiler::profile_dataset;
use records::LoadedDataset;
use reports::{ReportPaths, write_reports_index};

use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ProfileConfig::from_args()?;

    println!("Dataset: {}", config.dataset_name);

    let dataset =
        load_dataset(&config.dataset_name, &config.dataset_source, &config.cache_dir).await?;

    println!("Loaded {} records", dataset.len());

    let target_elements = match &config.target_selection {
        TargetSelection::One(target_element) => vec![target_element.clone()],
        TargetSelection::AllObserved => observed_elements(&dataset),
    };

    println!("Target elements: {}", target_elements.join(", "));

    let cooccurrence_report_paths = ReportPaths::prepare(config.reports_root.join("cooccurrence"))?;

    println!(
        "Writing element co-occurrence reports to {}",
        cooccurrence_report_paths.root.display()
    );

    write_cooccurrence_reports(
        &config.dataset_name,
        &dataset,
        &cooccurrence_report_paths,
        &config.reports_root,
        &target_elements,
    )?;

    for target_element in target_elements {
        let report_dir = config.report_dir_for(&target_element);
        let report_paths = ReportPaths::prepare(&report_dir)?;

        println!("Profiling target element: {target_element}");
        println!("Report directory: {}", report_paths.root.display());

        profile_dataset(&dataset, &target_element, &report_paths)?;

        write_markdown_report(&config.dataset_name, &target_element, &report_paths)?;

        println!("Wrote reports to {}", report_paths.root.display());
    }

    write_reports_index("reports", "REPORTS.md")?;

    println!("Updated REPORTS.md");

    Ok(())
}

fn observed_elements(dataset: &LoadedDataset) -> Vec<String> {
    let mut elements = BTreeSet::new();

    for record in &dataset.records {
        for element in record.element_counts.keys() {
            elements.insert(element.clone());
        }
    }

    elements.into_iter().collect()
}

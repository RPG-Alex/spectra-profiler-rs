mod chemistry;
mod config;
mod cooccurence;
mod datasets;
mod error;
mod markdown;
mod metadata;
mod population;
mod profiler;
mod reports;
mod visuals;

use std::collections::BTreeSet;

use chemistry::element_symbols_in_formula;
use config::{ProfileConfig, TargetSelection};
use cooccurence::write_cooccurrence_reports;
use datasets::load_dataset;
use markdown::write_markdown_report;
use mascot_rs::prelude::*;
use profiler::profile_dataset;
use reports::{ReportPaths, write_reports_index};

use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ProfileConfig::from_args()?;

    println!("Dataset: {}", config.dataset_name);

    let spectra = load_dataset(&config.dataset_source, &config.cache_dir).await?;

    println!("Loaded {} spectra", spectra.len());

    let target_elements = match &config.target_selection {
        TargetSelection::One(target_element) => vec![target_element.clone()],
        TargetSelection::AllObserved => observed_elements(&spectra),
    };

    println!("Target elements: {}", target_elements.join(", "));

    let cooccurrence_report_paths = ReportPaths::prepare(config.reports_root.join("cooccurrence"))?;

    println!(
        "Writing element co-occurrence reports to {}",
        cooccurrence_report_paths.root.display()
    );

    write_cooccurrence_reports(
        &config.dataset_name,
        &spectra,
        &cooccurrence_report_paths,
        &config.reports_root,
        &target_elements,
    )?;

    for target_element in target_elements {
        let report_dir = config.report_dir_for(&target_element);
        let report_paths = ReportPaths::prepare(&report_dir)?;

        println!("Profiling target element: {target_element}");
        println!("Report directory: {}", report_paths.root.display());

        profile_dataset(&spectra, &target_element, &report_paths)?;

        write_markdown_report(&config.dataset_name, &target_element, &report_paths)?;

        println!("Wrote reports to {}", report_paths.root.display());
    }

    write_reports_index("reports", "REPORTS.md")?;

    println!("Updated REPORTS.md");

    Ok(())
}

fn observed_elements(spectra: &MGFVec<f64>) -> Vec<String> {
    let mut elements = BTreeSet::new();

    for record in spectra.iter() {
        let Some(formula) = record.metadata().formula() else {
            continue;
        };

        for element in element_symbols_in_formula(&formula.to_string()) {
            elements.insert(element);
        }
    }

    elements.into_iter().collect()
}

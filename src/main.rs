mod data;
mod graph;

use crate::data::{read_denning_csv, read_reproductive_csv, DenningPhenology, ReproductiveSuccess};
use plotters::prelude::*;
use std::collections::HashMap;
use chrono::Datelike;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let denning = read_denning_csv("data/Wolf_DenningPhenology_AK_CA.csv")?;
    let reproduction = read_reproductive_csv("data/Wolf_ReproductiveSuccess_AK_CA.csv")?;

    println!("📦 Loaded {} denning records", denning.len());
    println!("📦 Loaded {} reproductive records", reproduction.len());

    let temperature_impact = analyze_temperature_impact(&denning, &reproduction);
    let snow_cover_impact = analyze_snow_cover_impact(&denning, &reproduction);

    summarize_impact("🌡️ Temperature", &temperature_impact, "°C");
    summarize_impact("❄️ Snow Cover", &snow_cover_impact, "mm");

    identify_vulnerable_regions(&denning, &reproduction);
    cluster_denning_patterns(&denning);

    let network = graph::build_graph(&denning);
    let centrality = graph::compute_degree_centrality(&network);
    graph::print_top_central_packs(&centrality, 5);

    plot_denning_and_success(&denning, &reproduction)?;
    Ok(())
}

fn analyze_temperature_impact(
    denning_data: &[DenningPhenology],
    reproductive_data: &[ReproductiveSuccess],
) -> Vec<f64> {
    reproductive_data
        .iter()
        .filter_map(|r| match (r.summer_tmax, r.winter_tmax) {
            (Some(s), Some(w)) if denning_data.iter().any(|d| d.pack_id == r.pack_id) => Some((s - w) as f64),
            _ => None,
        })
        .collect()
}

fn analyze_snow_cover_impact(
    denning_data: &[DenningPhenology],
    reproductive_data: &[ReproductiveSuccess],
) -> Vec<f64> {
    reproductive_data
        .iter()
        .filter_map(|r| match (r.winter_swe, r.fall_prcp) {
            (Some(w), Some(f)) if denning_data.iter().any(|d| d.pack_id == r.pack_id) => Some(w as f64 - f as f64),
            _ => None,
        })
        .collect()
}

fn summarize_impact(label: &str, values: &[f64], unit: &str) {
    if values.is_empty() {
        println!("\n{} Impact: No data available.", label);
        return;
    }
    let avg = values.iter().sum::<f64>() / values.len() as f64;
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    println!("\n{} Impact Summary:", label);
    println!("  • Records: {}", values.len());
    println!("  • Avg Δ: {:.2}{}", avg, unit);
    println!("  • Min Δ: {:.2}{}", min, unit);
    println!("  • Max Δ: {:.2}{}", max, unit);
}

fn identify_vulnerable_regions(
    denning_data: &[DenningPhenology],
    reproductive_data: &[ReproductiveSuccess],
) {
    let vulnerable: Vec<_> = reproductive_data
        .iter()
        .filter(|r| r.success == 0)
        .filter_map(|r| {
            denning_data
                .iter()
                .find(|d| d.pack_id == r.pack_id)
                .map(|d| (r.pack_id, d.study.clone()))
        })
        .collect();

    println!("\n⚠️ Vulnerability Summary:");
    println!("  • Packs with 0 reproductive success: {}", vulnerable.len());
}

fn cluster_denning_patterns(data: &[DenningPhenology]) {
    let (mut early, mut late) = (0, 0);
    for d in data {
        if d.denning_doy < 120 {
            early += 1;
        } else {
            late += 1;
        }
    }
    println!("\n🧩 Denning Clustering Summary:");
    println!("  • Early denning (< DOY 120): {}", early);
    println!("  • Late denning (≥ DOY 120): {}", late);
}

fn plot_denning_and_success(
    denning_data: &[DenningPhenology],
    reproductive_data: &[ReproductiveSuccess],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut year_to_doy: HashMap<i32, Vec<u32>> = HashMap::new();
    let mut year_to_success: HashMap<i32, Vec<u32>> = HashMap::new();

    for d in denning_data {
        year_to_doy.entry(d.denning_date.year()).or_default().push(d.denning_doy.into());
    }
    for r in reproductive_data {
        year_to_success.entry(r.start_date.year()).or_default().push(r.success.into());
    }

    let mut avg_doy: Vec<_> = year_to_doy
        .iter()
        .map(|(y, v)| (*y, v.iter().sum::<u32>() / v.len() as u32))
        .collect();
    let mut avg_success: Vec<_> = year_to_success
        .iter()
        .map(|(y, v)| (*y, v.iter().sum::<u32>() / v.len() as u32))
        .collect();

    avg_doy.sort_by_key(|k| k.0);
    avg_success.sort_by_key(|k| k.0);

    let output_path = "output/denning_vs_success.png";
    std::fs::create_dir_all("output")?;
    let root = BitMapBackend::new(output_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_year = *avg_doy.first().map(|(y, _)| y).unwrap_or(&2000);
    let max_year = *avg_doy.last().map(|(y, _)| y).unwrap_or(&2025);

    let mut chart = ChartBuilder::on(&root)
        .caption("Average Denning DOY and Reproductive Success per Year", ("sans-serif", 22))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(min_year..max_year, 0u32..200u32)?;

    chart.configure_mesh().x_desc("Year").y_desc("Value").draw()?;

    chart
        .draw_series(LineSeries::new(avg_doy.clone(), &BLUE))?
        .label("Avg Denning DOY")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(avg_success.clone(), &RED))?
        .label("Avg Reproductive Success")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("\n📊 Saved visualization to `{}`", output_path);
    Ok(())
}


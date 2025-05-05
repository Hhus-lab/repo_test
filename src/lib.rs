// src/lib.rs
pub mod data;
pub mod graph;


pub fn analyze_temperature_impact(
    denning_data: &[data::DenningPhenology],
    reproductive_data: &[data::ReproductiveSuccess],
) -> Vec<f64> {
    reproductive_data
        .iter()
        .filter_map(|r| match (r.summer_tmax, r.winter_tmax) {
            (Some(s), Some(w)) if denning_data.iter().any(|d| d.pack_id == 
r.pack_id) => Some((s - w) as f64),
            _ => None,
        })
        .collect()
}

pub fn analyze_snow_cover_impact(
    denning_data: &[data::DenningPhenology],
    reproductive_data: &[data::ReproductiveSuccess],
) -> Vec<f64> {
    reproductive_data
        .iter()
        .filter_map(|r| match (r.winter_swe, r.fall_prcp) {
            (Some(w), Some(f)) if denning_data.iter().any(|d| d.pack_id == 
r.pack_id) => Some(w as f64 - f as f64),
            _ => None,
        })
        .collect()
}


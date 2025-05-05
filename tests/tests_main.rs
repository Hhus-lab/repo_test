use chrono::NaiveDate;
use wolf_project_210::data::{ReproductiveSuccess, DenningPhenology};
use wolf_project_210::{
    analyze_temperature_impact,
    analyze_snow_cover_impact,
    cluster_denning_patterns,
    plot_denning_and_success
};


use chrono::NaiveDate;
use wolf_project_210::data::{DenningPhenology, ReproductiveSuccess};

fn mock_denning_data() -> Vec<DenningPhenology> {
    vec![
        DenningPhenology {
            pack_id: String::from("PackA"),
            year: 2020,
            denning_date: NaiveDate::from_ymd(2020, 5, 10),
            latitude: 64.0,
            longitude: -147.0,
            snow_cover: Some(0.5),
            temperature: Some(2.0),
        },
        DenningPhenology {
            pack_id: String::from("PackB"),
            year: 2020,
            denning_date: NaiveDate::from_ymd(2020, 5, 15),
            latitude: 65.0,
            longitude: -148.0,
            snow_cover: Some(0.6),
            temperature: Some(3.0),
        },
    ]
}

fn mock_reproductive_data() -> Vec<ReproductiveSuccess> {
    vec![
        ReproductiveSuccess {
            pack_id: String::from("PackA"),
            year: 2020,
            start_date: NaiveDate::from_ymd(2020, 6, 1),
            end_date: NaiveDate::from_ymd(2020, 9, 1),
            success: true,
            pup_count: Some(4),
        },
        ReproductiveSuccess {
            pack_id: String::from("PackB"),
            year: 2020,
            start_date: NaiveDate::from_ymd(2020, 6, 5),
            end_date: NaiveDate::from_ymd(2020, 9, 5),
            success: false,
            pup_count: None,
        },
    ]
}



#[cfg(test)]
mod tests {
    use super::*;

    // Mock data
    fn mock_reproductive_data() -> Vec<ReproductiveSuccess> {
        vec![
            ReproductiveSuccess {
                uid: 1,
                study: "Study A".to_string(),
                longitude_study: 0.0,
                latitude_study: 0.0,
                pack_id: 1,
                start_date: NaiveDate::from_ymd(2020, 6, 1),
                end_date: NaiveDate::from_ymd(2020, 9, 1),
                success: 1,
                summer_prcp: Some(100),
                fall_prcp: Some(50),
                winter_swe: Some(200),
                fall_tmax: Some(10),
                summer_tmax: Some(20),
                winter_tmax: Some(5),
                ti_ndvi_prev1: Some(0.5),
                ti_ndvi: Some(0.6),
                annual_pdo: Some(1.0),
                annual_ao: Some(0.5),
                home_range_area: Some(100.0),
                denning_match_growing_season: Some(0.8),
            },
            // Add more mock records if needed
        ]
    }

    fn mock_denning_data() -> Vec<DenningPhenology> {
        vec![
            DenningPhenology {
                uid: 1,
                study: "Study A".to_string(),
                longitude_study: 0.0,
                latitude_study: 0.0,
                pack_id: 1,
                denning_date: NaiveDate::from_ymd(2020, 5, 10),
                denning_doy: 130,
                denned: 1,
                fall_tmax: Some(15),
                summer_tmax_prev1: Some(25),
                winter_tmax: Some(10),
                fall_prcp: Some(120),
                summer_prcp_prev1: Some(110),
                winter_swe: Some(250),
                ti_ndvi_prev1: Some(0.6),
                annual_pdo: Some(1.0),
                annual_ao: Some(0.5),
                sos_prev1: Some(0.5),
                los_prev1: Some(0.8),
                latitude_individual: 60.0,
            },
            // Add more mock records if needed
        ]
    }

    #[test]
    fn test_analyze_temperature_impact() {
        let denning_data = mock_denning_data();
        let reproductive_data = mock_reproductive_data();
        
        let temperature_impact = analyze_temperature_impact(&denning_data, &reproductive_data);
        
        assert!(!temperature_impact.is_empty(), "Temperature impact analysis returned no results");
    }

    #[test]
    fn test_analyze_snow_cover_impact() {
        let denning_data = mock_denning_data();
        let reproductive_data = mock_reproductive_data();

        let snow_cover_impact = analyze_snow_cover_impact(&denning_data, &reproductive_data);

        assert!(!snow_cover_impact.is_empty(), "Snow cover impact analysis returned no results");
    }
}

#[test]
fn test_cluster_denning_patterns() {
    let denning_data = mock_denning_data();
    let (early, late) = cluster_denning_patterns(&denning_data);
    assert!(early >= 0, "Early denning count should not be negative");
    assert!(late >= 0, "Late denning count should not be negative");
}

#[test]
fn test_plot_denning_and_success() {
    let denning_data = mock_denning_data();
    let reproductive_data = mock_reproductive_data();
    
    let result = plot_denning_and_success(&denning_data, &reproductive_data);
    
    assert!(result.is_ok(), "Plotting failed: {:?}", result.err());
    assert!(std::path::Path::new("output/denning_vs_success.png").exists(), "Output file not created");
}

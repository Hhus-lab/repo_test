use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

/// Struct for reproductive success data
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ReproductiveSuccess {
    pub uid: u32,
    pub study: String,
    pub longitude_study: f64,
    pub latitude_study: f64,
    pub pack_id: u32,

    #[serde(deserialize_with = "parse_date")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "parse_date")]
    pub end_date: NaiveDate,

    pub success: u8,

    #[serde(deserialize_with = "parse_optional_u32")]
    pub summer_prcp: Option<u32>,
    #[serde(deserialize_with = "parse_optional_u32")]
    pub fall_prcp: Option<u32>,
    #[serde(deserialize_with = "parse_optional_u32")]
    pub winter_swe: Option<u32>,

    #[serde(deserialize_with = "parse_optional_i8")]
    pub fall_tmax: Option<i8>,
    #[serde(deserialize_with = "parse_optional_i8")]
    pub summer_tmax: Option<i8>,
    #[serde(deserialize_with = "parse_optional_i8")]
    pub winter_tmax: Option<i8>,

    #[serde(rename = "tiNDVI_prev1", deserialize_with = "parse_optional_f64")]
    pub ti_ndvi_prev1: Option<f64>,
    #[serde(rename = "tiNDVI", deserialize_with = "parse_optional_f64")]
    pub ti_ndvi: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub annual_pdo: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub annual_ao: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub home_range_area: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub denning_match_growing_season: Option<f64>,
}

/// Struct for denning phenology data
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct DenningPhenology {
    pub uid: u32,
    pub study: String,
    pub longitude_study: f64,
    pub latitude_study: f64,
    pub pack_id: u32,

    #[serde(deserialize_with = "parse_date")]
    pub denning_date: NaiveDate,

    pub denning_doy: u16,
    pub denned: i8,

    #[serde(deserialize_with = "parse_optional_i8")]
    pub fall_tmax: Option<i8>,
    #[serde(deserialize_with = "parse_optional_i8")]
    pub summer_tmax_prev1: Option<i8>,
    #[serde(deserialize_with = "parse_optional_i8")]
    pub winter_tmax: Option<i8>,

    #[serde(deserialize_with = "parse_optional_u32")]
    pub fall_prcp: Option<u32>,
    #[serde(deserialize_with = "parse_optional_u32")]
    pub summer_prcp_prev1: Option<u32>,
    #[serde(deserialize_with = "parse_optional_u32")]
    pub winter_swe: Option<u32>,

    #[serde(rename = "tiNDVI_prev1", deserialize_with = "parse_optional_f64")]
    pub ti_ndvi_prev1: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub annual_pdo: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub annual_ao: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub sos_prev1: Option<f64>,
    #[serde(deserialize_with = "parse_optional_f64")]
    pub los_prev1: Option<f64>,

    pub latitude_individual: f64,
}

pub fn read_reproductive_csv(path: &str) -> Result<Vec<ReproductiveSuccess>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);

    let mut records = Vec::new();
    for (i, result) in csv_reader.deserialize::<ReproductiveSuccess>().enumerate() {
        match result {
            Ok(record) if record.has_no_missing_fields() => records.push(record),
            Ok(_) => eprintln!("Skipping line {} due to missing fields", i + 1),
            Err(e) => eprintln!("Error deserializing reproductive data (line {}): {}", i + 1, e),
        }
    }

    Ok(records)
}

pub fn read_denning_csv(path: &str) -> Result<Vec<DenningPhenology>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);

    let mut records = Vec::new();
    for (i, result) in csv_reader.deserialize::<DenningPhenology>().enumerate() {
        match result {
            Ok(record) if record.has_no_missing_fields() => records.push(record),
            Ok(_) => eprintln!("Skipping line {} due to missing fields", i + 1),
            Err(e) => eprintln!("Error deserializing denning data (line {}): {}", i + 1, e),
        }
    }

    Ok(records)
}

impl ReproductiveSuccess {
    pub fn has_no_missing_fields(&self) -> bool {
        self.uid != 0
            && !self.study.is_empty()
            && self.summer_prcp.is_some()
            && self.fall_prcp.is_some()
            && self.winter_swe.is_some()
            && self.fall_tmax.is_some()
            && self.summer_tmax.is_some()
            && self.winter_tmax.is_some()
            && self.ti_ndvi_prev1.is_some()
            && self.ti_ndvi.is_some()
            && self.annual_pdo.is_some()
            && self.annual_ao.is_some()
            && self.home_range_area.is_some()
            && self.denning_match_growing_season.is_some()
    }
}

impl DenningPhenology {
    pub fn has_no_missing_fields(&self) -> bool {
        self.uid != 0
            && !self.study.is_empty()
            && self.denning_doy != 0
            && self.denned != 0
            && self.fall_tmax.is_some()
            && self.summer_tmax_prev1.is_some()
            && self.winter_tmax.is_some()
            && self.fall_prcp.is_some()
            && self.summer_prcp_prev1.is_some()
            && self.winter_swe.is_some()
            && self.ti_ndvi_prev1.is_some()
            && self.annual_pdo.is_some()
            && self.annual_ao.is_some()
            && self.sos_prev1.is_some()
            && self.los_prev1.is_some()
            && self.latitude_individual != 0.0
    }
}

/// Parse optional f64 field (e.g., "NaN", "", "-9999")
fn parse_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim();

    // Handle more potential invalid cases explicitly
    if s.is_empty() || s == "-9999" || s.eq_ignore_ascii_case("na") || s.eq_ignore_ascii_case("nan") {
        Ok(None)
    } else {
        s.parse::<f64>()
            .map(Some)
            .map_err(|_| serde::de::Error::custom(format!("Invalid f64 value: {}", s)))
    }
}

/// Parse optional i8 field, rounding float to integer
fn parse_optional_i8<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim();

    // Handle invalid cases
    if s.is_empty() || s == "-9999" || s.eq_ignore_ascii_case("na") {
        Ok(None)
    } else {
        // Try to parse as f64, then round to i8
        s.parse::<f64>()
            .map(|f| Some(f.round() as i8)) // Round to nearest i8
            .map_err(|_| serde::de::Error::custom(format!("Invalid i8 value: {}", s)))
    }
}

/// Parse optional u32 field, rounding float to integer
fn parse_optional_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim();

    if s.is_empty() || s == "-9999" || s.eq_ignore_ascii_case("na") {
        Ok(None)
    } else {
        // Try to parse as f64, then round to u32
        s.parse::<f64>()
            .map(|f| Some(f.round() as u32)) // Round to nearest u32
            .map_err(|_| serde::de::Error::custom(format!("Invalid u32 value: {}", s)))
    }
}



/// Parse date
fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").map_err(serde::de::Error::custom)
}


use serde::{Deserialize, Serialize};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Deserialize, Serialize)]
pub struct OverpassResponse {
    pub elements: Vec<OverpassElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverpassElement {
    #[serde(rename = "type")]
    pub element_type: String,
    pub id: i64,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub nodes: Option<Vec<i64>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

pub fn get_roads(osm_type: &str, osm_id: i64) -> Result<OverpassResponse, Box<dyn std::error::Error>> {

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    spinner.set_message("Fetching roads from Overpass API...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let area_id = match osm_type {
        "relation" => osm_id + 3600000000,
        "way" => osm_id + 2400000000,
        _ => {
            spinner.finish_and_clear();
            return Err("Unsupported OSM type for area query".into());
        }
    };

    // Overpass query to get all roads in the area
    let query = format!(
        r#"[out:json];
        area({})->.searchArea;
        (
            way["highway"](area.searchArea);
        );
        out body;
        >;
        out skel qt;
        "#,
        area_id
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()?
        .json::<OverpassResponse>()?;

    spinner.finish_with_message("✓ Roads fetched successfully!");

    Ok(response)
}
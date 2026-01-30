use serde::Deserialize;
use urlencoding::encode;

#[derive(Debug, Deserialize)]
pub struct NominatimResult {
    pub place_id: u64,
    pub lat: String,
    pub lon: String,
    pub display_name: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub result_type: String,
    pub class: String,
    pub osm_id: i64,
    pub osm_type: String
}


pub fn call_nominatim(q: &str) -> Result<Vec<NominatimResult>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let encoded_query = encode(q);
    
    let resp = client
        .get(&format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json",
            encoded_query
        ))
        .header("User-Agent", "roads/0.0.1")
        .send()?;
    
    let results: Vec<NominatimResult> = resp.json()?;
    Ok(results)
}
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;

pub fn save_data<T: Serialize>(
    data: &T,
    display_name: &str,
    osm_id: i64,
    osm_type: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create data directory if it doesn't exist
    fs::create_dir_all("data")?;
    
    // Generate timestamp in milliseconds
    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis();
    
    // Sanitize display_name for filename (replace problematic characters)
    let sanitized_name = display_name
        .replace("/", "_")
        .replace("\\", "_")
        .replace(" ", "_")
        .replace(",", "");
    
    // Create filename
    let filename = format!(
        "data/{}_{}_{}_{}ms.json",
        sanitized_name,
        osm_id,
        osm_type,
        timestamp_ms
    );
    
    // Save to file with pretty formatting
    let json_content = serde_json::to_string_pretty(data)?;
    fs::write(&filename, json_content)?;
    
    Ok(filename)
}
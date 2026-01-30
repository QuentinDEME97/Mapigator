use crate::overpass::OverpassResponse;
use crate::benchmark::RoadStats;
use std::collections::{HashMap, HashSet};

/// Simple single-threaded implementation
pub fn analyze_roads_simple(data: &OverpassResponse) -> RoadStats {
    let mut stats = RoadStats::new();
    stats.total_roads = data.elements.len();
    
    let mut unique_names = HashSet::new();
    
    for element in &data.elements {
        if let Some(tags) = &element.tags {
            // Count highway types
            if let Some(highway_type) = tags.get("highway") {
                *stats.highway_types.entry(highway_type.clone()).or_insert(0) += 1;
            }
            
            // Count unique names
            if let Some(name) = tags.get("name") {
                unique_names.insert(name.clone());
            }
        }
        
        // For now, we don't have points data in the response
        // You'd need to fetch with "out geom;" to get actual points
        stats.points_per_road.insert(element.id, 0);
    }
    
    stats.unique_names = unique_names.len();
    stats
}

/// Parallel implementation using rayon
pub fn analyze_roads_parallel(data: &OverpassResponse) -> RoadStats {
    use rayon::prelude::*;
    use std::sync::Mutex;
    
    let stats = Mutex::new(RoadStats::new());
    let unique_names = Mutex::new(HashSet::new());
    
    data.elements.par_iter().for_each(|element| {
        if let Some(tags) = &element.tags {
            // Count highway types
            if let Some(highway_type) = tags.get("highway") {
                let mut stats = stats.lock().unwrap();
                *stats.highway_types.entry(highway_type.clone()).or_insert(0) += 1;
            }
            
            // Count unique names
            if let Some(name) = tags.get("name") {
                let mut names = unique_names.lock().unwrap();
                names.insert(name.clone());
            }
        }
    });
    
    let mut final_stats = stats.into_inner().unwrap();
    final_stats.total_roads = data.elements.len();
    final_stats.unique_names = unique_names.into_inner().unwrap().len();
    
    final_stats
}
use crate::overpass::OverpassResponse;
use crate::overpass::OverpassElement;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashSet, HashMap};

impl Node {
    pub fn new(id: i64, lat: f64, lon: f64) -> Self {
        Self { id, lat, lon }
    }
    
    // Haversine formula to calculate distance in meters
    pub fn distance_to(&self, other: &Node) -> f64 {
        let r = 6371.0; // Earth's radius in km
        
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();
        let delta_lat = (other.lat - self.lat).to_radians();
        let delta_lon = (other.lon - self.lon).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c * 100.00
    }
}

impl RoadGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
            adjacency: HashMap::new(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }
    
    pub fn add_link(&mut self, link: Link) {
        let link_idx = self.links.len();
        let origin_id = link.origin_id;
        
        self.links.push(link);
        self.adjacency
            .entry(origin_id)
            .or_insert_with(Vec::new)
            .push(link_idx);
    }
    
    pub fn get_outgoing_links(&self, node_id: i64) -> Vec<&Link> {
        self.adjacency
            .get(&node_id)
            .map(|indices| indices.iter().map(|&i| &self.links[i]).collect())
            .unwrap_or_default()
    }

    pub fn describe(&self) {
        println!("┌─ Road Graph Statistics");
        println!("│");
        println!("│  Nodes: {}", self.nodes.len());
        println!("│  Links: {}", self.links.len());
        println!("│");
        
        // Calculate average degree (outgoing links per node)
        let nodes_with_outgoing = self.adjacency.len();
        let avg_degree = if nodes_with_outgoing > 0 {
            self.links.len() as f64 / nodes_with_outgoing as f64
        } else {
            0.0
        };
        println!("│  Nodes with outgoing links: {}", nodes_with_outgoing);
        println!("│  Average outgoing degree: {:.2}", avg_degree);
        println!("│");
        
        // Find nodes with most connections
        if let Some((max_node_id, max_links)) = self.adjacency
            .iter()
            .max_by_key(|(_, links)| links.len()) {
            println!("│  Max outgoing links from single node: {} (node ID: {})", max_links.len(), max_node_id);
        }
        
        // Count highway types
        let mut highway_counts: HashMap<String, usize> = HashMap::new();
        for link in &self.links {
            if let Some(highway_type) = link.tags.get("highway") {
                *highway_counts.entry(highway_type.clone()).or_insert(0) += 1;
            }
        }
        
        println!("│");
        println!("│  Highway types:");
        let mut sorted_highways: Vec<_> = highway_counts.iter().collect();
        sorted_highways.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (highway_type, count) in sorted_highways.iter().take(10) {
            println!("│    - {}: {}", highway_type, count);
        }
        
        println!("└─");
    }
}
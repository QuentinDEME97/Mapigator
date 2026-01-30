use std::collections::HashMap;
use crate::overpass::OverpassResponse;
use crate::overpass::OverpassElement;

pub const DRIVABLE_HIGHWAYS: &[&str] = &[
    // Major roads
    "motorway",
    "trunk",
    "primary",
    "secondary",
    "tertiary",
    
    // Minor roads
    "unclassified",
    "residential",
    
    // Links (on/off ramps)
    "motorway_link",
    "trunk_link",
    "primary_link",
    "secondary_link",
    "tertiary_link",
    
    // Other drivable
    "living_street",
    "service",
    "road",  // unknown classification
];

// Node should be simple - just position data
#[derive(Debug, Clone)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
}

impl Node {
    pub fn new(id: i64, lat: f64, lon: f64) -> Self {
        Self { id, lat, lon }
    }
    
    // Haversine formula to calculate distance in km
    pub fn distance_to(&self, other: &Node) -> f64 {
        let r = 6371.0; // Earth's radius in km
        
        let lat1 = self.lat.to_radians();
        let lat2 = other.lat.to_radians();
        let delta_lat = (other.lat - self.lat).to_radians();
        let delta_lon = (other.lon - self.lon).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c
    }
}

// Link represents a directed edge between two nodes
#[derive(Debug, Clone)]
pub struct Link {
    pub origin_id: i64,
    pub dest_id: i64,
    pub distance: f64,
    pub way_id: i64,  // Which road this link belongs to
    pub tags: HashMap<String, String>,  // Road properties (name, type, etc.)
}

impl Link {
    pub fn new(origin: &Node, dest: &Node, way_id: i64, tags: HashMap<String, String>) -> Self {
        Self {
            origin_id: origin.id,
            dest_id: dest.id,
            distance: origin.distance_to(dest),
            way_id,
            tags,
        }
    }
}

// Graph structure to hold everything
#[derive(Debug)]
pub struct RoadGraph {
    pub nodes: HashMap<i64, Node>,
    pub links: Vec<Link>,
    // Adjacency list for quick lookups: node_id -> list of outgoing link indices
    pub adjacency: HashMap<i64, Vec<usize>>,
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

pub fn is_drivable(highway_type: &str) -> bool {
    DRIVABLE_HIGHWAYS.contains(&highway_type)
}

pub fn roads_to_graph(roads: OverpassResponse) -> RoadGraph {
  let mut road_graph = RoadGraph::new();

  let mut node_map: HashMap<i64, &OverpassElement> = HashMap::new();
  for element in &roads.elements {
      if element.element_type == "node" {
          node_map.insert(element.id, element);
      }
  }

  for (i, road) in roads.elements.iter().enumerate() {
      println!("Road #{}", i + 1);
      if let Some(tags) = &road.tags {
        if let Some(highway) = tags.get("highway") {
            if is_drivable(highway) {
                let is_oneway = tags.get("oneway")
                  .map(|v| v == "yes" || v == "1" || v == "true")
                  .unwrap_or(false);
              
              // Get the ordered list of node IDs
              if let Some(node_ids) = &road.nodes {
                  // Add all nodes to the graph
                  for &node_id in node_ids {
                      if let Some(&node_element) = node_map.get(&node_id) {
                          if let (Some(lat), Some(lon)) = (node_element.lat, node_element.lon) {
                              let node = Node::new(node_id, lat, lon);
                              road_graph.add_node(node);
                          }
                      }
                  }
                  
                  // Create links between consecutive nodes
                  for i in 0..node_ids.len() - 1 {
                        let origin_id = node_ids[i];
                        let dest_id = node_ids[i + 1];
                        
                        // Clone the nodes to avoid borrow conflict
                        let origin = road_graph.nodes.get(&origin_id).cloned();
                        let dest = road_graph.nodes.get(&dest_id).cloned();
                        
                        if let (Some(origin), Some(dest)) = (origin, dest) {
                            // Forward link (A -> B)
                            let link_forward = Link::new(
                                &origin,
                                &dest,
                                road.id,
                                tags.clone()
                            );
                            road_graph.add_link(link_forward);
                            
                            // If not oneway, add reverse link (B -> A)
                            if !is_oneway {
                                let link_reverse = Link::new(
                                    &dest,
                                    &origin,
                                    road.id,
                                    tags.clone()
                                );
                                road_graph.add_link(link_reverse);
                            }
                        }
                    }
              }
            }
        }
      }
      println!();
  }
  road_graph
}
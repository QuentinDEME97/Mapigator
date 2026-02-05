use crate::overpass::OverpassResponse;
use crate::overpass::OverpassElement;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashSet, HashMap};

// Node should be simple - just position data
#[derive(Debug, Clone)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
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

// Graph structure to hold everything
#[derive(Debug)]
pub struct OrientedGraph {
    pub nodes: HashMap<i64, Node>,
    pub links: Vec<Link>,
    // Adjacency list for quick lookups: node_id -> list of outgoing link indices
    pub adjacency: HashMap<i64, Vec<usize>>,
}

pub const INF: f64 = f64::INFINITY;

pub struct MatrixResult {
  pub dists: Vec<f64>,
  pub id_map: HashMap<i64, usize>,
  pub size: usize,
}
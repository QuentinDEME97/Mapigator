use std::collections::HashMap;

use crate::graph::edge::Edge;
use crate::graph::vertex::Vertex;

#[derive(Debug)]
pub struct OrientedGraph {
  pub vertices: HashMap<usize, Vertex>,
  pub edges: HashMap<usize, Edge>,
}

impl OrientedGraph {
  pub fn new() -> Self {
    Self {
      vertices: HashMap::new(),
      edges: HashMap::new(),
    }
  }

  pub fn add_vertex(&mut self, vertex: Vertex) {
    let id = vertex.id;
    self.vertices.insert(id, vertex);
  }

  pub fn add_edge(&mut self, edge: Edge) {
    let id = edge.id;
    self.edges.insert(id, edge);
  }
}

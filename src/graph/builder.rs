use crate::graph::edge::EdgeFactory;
use crate::graph::oriented_graph::OrientedGraph;
use crate::graph::utils::CoordinateType;
use crate::graph::vertex::VertexFactory;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
  #[error("Failed to read file: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Failed to parse OSM data: {0}")]
  ParseError(String),

  #[error("API request failed: {0}")]
  ApiError(#[from] reqwest::Error),

  #[error("Invalid edge index")]
  InvalidEdgeIndex,
}

pub trait GraphBuilder {
  fn build(self) -> Result<OrientedGraph, BuildError>;
  fn vertex_count(&self) -> usize;
  fn edge_count(&self) -> usize;
}

pub struct ManualGraphBuilder {
  vertex_factory: VertexFactory,
  edge_factory: EdgeFactory,
  graph: OrientedGraph,
  vertices: Vec<(f64, f64)>,
  edges: Vec<(usize, usize, Option<f64>)>,
  coordinates_type: CoordinateType,
}

impl ManualGraphBuilder {
  pub fn new(coordinate_type: CoordinateType) -> Self {
    Self {
      vertex_factory: VertexFactory::new(),
      edge_factory: EdgeFactory::new(),
      graph: OrientedGraph::new(),
      vertices: Vec::new(),
      edges: Vec::new(),
      coordinates_type: coordinate_type,
    }
  }

  pub fn add_vertex(mut self, coordinates: (f64, f64)) -> Self {
    self.vertices.push(coordinates);
    self
  }

  pub fn add_edge(mut self, from: usize, to: usize) -> Self {
    self.edges.push((from, to, Some(0.0)));
    self
  }

  pub fn with_vertices(mut self, vertices: Vec<(f64, f64)>) -> Self {
    self.vertices = vertices;
    self
  }

  pub fn with_edges(mut self, edges: Vec<(usize, usize, Option<f64>)>) -> Self {
    self.edges = edges;
    self
  }
}

impl GraphBuilder for ManualGraphBuilder {
  fn build(mut self) -> Result<OrientedGraph, BuildError> {
    let vertex_ids: Vec<usize> = self
      .vertices
      .iter()
      .map(|(x, y)| {
        let vertex = self
          .vertex_factory
          .create_vertex(None, (*x, *y), self.coordinates_type);
        let id = vertex.id;
        self.graph.add_vertex(vertex);
        id
      })
      .collect();

    for (from_idx, to_idx, weight) in self.edges {
      if from_idx >= vertex_ids.len() || to_idx >= vertex_ids.len() {
        return Err(BuildError::InvalidEdgeIndex);
      }
      self
        .graph
        .add_edge(vertex_ids[from_idx], vertex_ids[to_idx], Some(weight));
    }
    Ok(self.graph)
  }

  fn vertex_count(&self) -> usize {
    self.vertices.len()
  }

  fn edge_count(&self) -> usize {
    self.edges.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_graph_passing_vertices_then_edges() {
    let graph = ManualGraphBuilder::new(CoordinateType::XY)
      .add_vertex((0.0, 0.0))
      .add_vertex((0.0, 1.0))
      .add_edge(0, 1)
      .build()?;

    assert_eq!(graph.vertices.len(), 2);
    assert_eq!(graph.edges.len(), 1);
  }
}

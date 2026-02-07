use crate::graph::edge::EdgeFactory;
use crate::graph::oriented_graph::OrientedGraph;
use crate::graph::utils::{self, CoordinateType};
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

  pub fn add_edge(mut self, from: usize, to: usize, weight: Option<f64>) -> Self {
    self.edges.push((from, to, weight));
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
    let mut edge_factory = EdgeFactory::new();
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

      let mut computed_weight = weight.unwrap_or(0.0);
      if weight == Some(-1.0) || weight == None {
        // If weigth is some forced, we calculate it.
        let origin = self.graph.vertices[&from_idx].coordinates;
        let destination = self.graph.vertices[&to_idx].coordinates;
        if let Ok(dist) = utils::get_distance(self.coordinates_type, origin, destination) {
          computed_weight = dist;
        } else {
          // Handle the error, e.g., set a default or log it
          computed_weight = 0.0;
        }
      }

      self
        .graph
        .add_edge(edge_factory.create_edge(None, from_idx, to_idx, computed_weight, None));
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
  fn test_build_xy_graph_with_add_build() -> Result<(), Box<dyn std::error::Error>> {
    let graph_xy = ManualGraphBuilder::new(CoordinateType::XY)
      .add_vertex((0.0, 0.0))
      .add_vertex((0.0, 1.0))
      .add_edge(0, 1, None)
      .build()?;

    assert_eq!(graph_xy.vertices.len(), 2);
    assert_eq!(graph_xy.edges.len(), 1);

    assert_eq!(graph_xy.edges[&(0 as usize)].origin, 0);
    assert_eq!(graph_xy.edges[&(0 as usize)].destination, 1);

    Ok(())
  }

  #[test]
  fn test_build_xy_graph_with_list_build() -> Result<(), Box<dyn std::error::Error>> {
    let nodes = vec![(0.0, 0.0), (0.0, 1.0)];

    let edges = vec![(0, 1, None)];

    let graph_xy = ManualGraphBuilder::new(CoordinateType::XY)
      .with_vertices(nodes)
      .with_edges(edges)
      .build()?;

    assert_eq!(graph_xy.vertices.len(), 2);
    assert_eq!(graph_xy.edges.len(), 1);

    assert_eq!(graph_xy.edges[&(0 as usize)].origin, 0);
    assert_eq!(graph_xy.edges[&(0 as usize)].destination, 1);

    Ok(())
  }

  #[test]
  fn test_build_geo_graph_with_add_build() -> Result<(), Box<dyn std::error::Error>> {
    let graph_geo = ManualGraphBuilder::new(CoordinateType::GEO)
      .add_vertex((48.8566, 2.3522))
      .add_vertex((45.7640, 4.8357))
      .add_edge(0, 1, None)
      .build()?;

    assert_eq!(graph_geo.vertices.len(), 2);
    assert_eq!(graph_geo.edges.len(), 1);

    assert_eq!(graph_geo.edges[&(0 as usize)].origin, 0);
    assert_eq!(graph_geo.edges[&(0 as usize)].destination, 1);

    Ok(())
  }

  #[test]
  fn test_build_geo_graph_with_list_build() -> Result<(), Box<dyn std::error::Error>> {
    let nodes = vec![(48.8566, 2.3522), (45.7640, 4.8357)];

    let edges = vec![(0, 1, None)];

    let graph_geo = ManualGraphBuilder::new(CoordinateType::GEO)
      .with_vertices(nodes)
      .with_edges(edges)
      .build()?;

    assert_eq!(graph_geo.vertices.len(), 2);
    assert_eq!(graph_geo.edges.len(), 1);

    assert_eq!(graph_geo.edges[&(0 as usize)].origin, 0);
    assert_eq!(graph_geo.edges[&(0 as usize)].destination, 1);

    Ok(())
  }

  #[test]
  fn test_euclidian_distance_is_correct_with_add_build() -> Result<(), Box<dyn std::error::Error>> {
    let graph_xy = ManualGraphBuilder::new(CoordinateType::XY)
      .add_vertex((0.0, 0.0))
      .add_vertex((0.0, 1.0))
      .add_edge(0, 1, None)
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 1.0);

    Ok(())
  }

  #[test]
  fn test_euclidian_distance_is_correct_with_list_build() -> Result<(), Box<dyn std::error::Error>>
  {
    let nodes = vec![(0.0, 0.0), (0.0, 1.0)];

    let edges = vec![(0, 1, None)];

    let graph_xy = ManualGraphBuilder::new(CoordinateType::XY)
      .with_vertices(nodes)
      .with_edges(edges)
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 1.0);

    Ok(())
  }

  #[test]
  fn test_euclidian_distance_is_forced_with_add_build() -> Result<(), Box<dyn std::error::Error>> {
    let graph_xy = ManualGraphBuilder::new(CoordinateType::XY)
      .add_vertex((0.0, 0.0))
      .add_vertex((0.0, 1.0))
      .add_edge(0, 1, Some(23.0))
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 23.0);

    Ok(())
  }

  #[test]
  fn test_euclidian_distance_is_forced_with_list_build() -> Result<(), Box<dyn std::error::Error>> {
    let nodes = vec![(0.0, 0.0), (0.0, 1.0)];

    let edges = vec![(0, 1, Some(23.0))];

    let graph_xy = ManualGraphBuilder::new(CoordinateType::XY)
      .with_vertices(nodes)
      .with_edges(edges)
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 23.0);

    Ok(())
  }

  // GEO distances
  #[test]
  fn test_geo_distance_is_correct_with_add_build() -> Result<(), Box<dyn std::error::Error>> {
    let graph_xy = ManualGraphBuilder::new(CoordinateType::GEO)
      .add_vertex((49.09920, 0.28713))
      .add_vertex((49.09984, 0.28720))
      .add_edge(0, 1, None)
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 71.34700096697131);

    Ok(())
  }

  #[test]
  fn test_geo_distance_is_correct_with_list_build() -> Result<(), Box<dyn std::error::Error>> {
    let nodes = vec![(49.09920, 0.28713), (49.09984, 0.28720)];

    let edges = vec![(0, 1, None)];

    let graph_xy = ManualGraphBuilder::new(CoordinateType::GEO)
      .with_vertices(nodes)
      .with_edges(edges)
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 71.34700096697131);

    Ok(())
  }

  #[test]
  fn test_geo_distance_is_forced_with_add_build() -> Result<(), Box<dyn std::error::Error>> {
    let graph_xy = ManualGraphBuilder::new(CoordinateType::GEO)
      .add_vertex((49.09920, 0.28713))
      .add_vertex((49.09984, 0.28720))
      .add_edge(0, 1, Some(23.0))
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 23.0);

    Ok(())
  }

  #[test]
  fn test_geo_distance_is_forced_with_list_build() -> Result<(), Box<dyn std::error::Error>> {
    let nodes = vec![(49.09920, 0.28713), (49.09984, 0.28720)];

    let edges = vec![(0, 1, Some(23.0))];

    let graph_xy = ManualGraphBuilder::new(CoordinateType::GEO)
      .with_vertices(nodes)
      .with_edges(edges)
      .build()?;

    assert_eq!(graph_xy.edges[&(0 as usize)].weight, 23.0);

    Ok(())
  }
}

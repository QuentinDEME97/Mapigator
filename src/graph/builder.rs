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
}

impl ManualGraphBuilder {
  pub fn new() -> Self {
    Self {
      vertex_factory: VertexFactory::new(),
      edge_factory: EdgeFactory::new(),
      graph: OrientedGraph::new(),
      vertices: Vec::new(),
      edges: Vec::new(),
    }
  }

  pub fn add_vertex(mut self, lat: f64, lon: f64) -> Self {
      self.vertices.push((lat, lon));
      self
  }
  
  pub fn add_edge(mut self, from: usize, to: usize, weight: Option<f64>) -> Self {
      self.edges.push((from, to, weight));
      self
  }
}

impl GraphBuilder for ManualGraphBuilder {
  fn build(mut self) -> Result<OrientedGraph, BuildError> {
    let vertex_ids = self.vertices
      .iter()
      .map(|(lat, lon)| {
        let vertex = self.vertex_factory.create_vertex(*lat, *lon);
        let id = vertex.id;
        self.graph.add_vertex(vertex);
        id
      })
      .collect();

    for (from_idx, to_idx, weight) in self.edges {
      if from_idx >= vertex_ids.len() || to_idx >= vertex_ids.len() {
        return Err(BuildError::InvalidEdgeIndex)
      }
    self.graph.add_edge(
        vertex_ids[from_idx],
        vertex_ids[to_idx],
        weight,
      );
    }
    Ok(self.graph)
  }

  fn vertex_count(&self) -> usize {
    self.nodes.len()
  }
  
  fn edge_count(&self) -> usize {
    self.edges.len()
  }
}
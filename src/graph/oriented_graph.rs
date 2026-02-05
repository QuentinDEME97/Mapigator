#[derive(Debug)]
pub struct OrientedGraph {
  pub vertices: Vec<&Vertex>,
  pub edges: Vec<&Vertex>,
}
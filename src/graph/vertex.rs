#[derive(Debug)]
enum CoordinateType {
  GEO,
  XY
}

#[derive(Debug)]
pub struct Vertex {
  pub id: usize,
  pub base_id: Option<i64>,
  pub coordinates: (f64, f64),
  pub coordinate_type: CoordinateType,
  pub successors: Vec<usize>
}

pub struct VertexFactory {
  next_id: usize,
}

impl VertexFactory {
  pub fn new() -> Self {
    Self { next_id: 0}
  }

  pub fn create_vertex(&mut self, base_id: Option<i64>, coordinates: (f64, f64), coordinate_type: CoordinateType) -> Vertex {
    let id = self.next_id;
    self.next_id += 1;
    Vertex {
      id,
      base_id,
      coordinates,
      coordinate_type,
      successors: Vec::new(),
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_increments_id() {
        let mut factory = VertexFactory::new();
        
        let v1 = factory.create_vertex(None, (0.0,0.0), CoordinateType::XY);
        let v2 = factory.create_vertex(Some(2346237), (32.3, 48.9), CoordinateType::GEO);
        
        assert_eq!(v1.id, 0);
        assert_eq!(v2.id, 1);
    }

    #[test]
    fn test_factory_sets_base_id() {
        let mut factory = VertexFactory::new();
        
        let v1 = factory.create_vertex(None, (0.0,0.0), CoordinateType::XY);
        let v2 = factory.create_vertex(Some(2346237), (32.3, 48.9), CoordinateType::GEO);
        
        assert_eq!(v1.base_id, None);
        assert_eq!(v2.base_id, Some(2346237));
    }

    #[test]
    fn test_factory_starts_at_zero() {
        let factory = VertexFactory::new();
        assert_eq!(factory.next_id, 0);
    }
}
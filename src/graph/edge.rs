use std::collections::HashMap;

#[derive(Debug)]
pub struct Edge {
  pub id: usize,
  pub base_id: Option<i64>,
  pub origin: usize,
  pub destination: usize,
  pub weight: f64,
  pub tags: Option<HashMap<String, String>>,
}

struct EdgeFactory {
  next_id: usize
}

impl EdgeFactory {
  pub fn new() -> Self {
    Self { next_id: 0 }
  }

  pub fn create_edge(&mut self, base_id: Option<i64>, origin: usize, destination: usize, weight: f64, tags: Option<HashMap<String, String>>) -> Edge {
    let id: usize = self.next_id;
    self.next_id += 1;
    Edge { 
      id,
      base_id,
      origin,
      destination,
      weight,
      tags,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factory_increments_id() {
    let mut factory: EdgeFactory = EdgeFactory::new();

    let e1: Edge = factory.create_edge(None, 0, 1, 1.0, None);
    let e2: Edge = factory.create_edge(Some(222309), 0, 1, 1.0, Some(HashMap::from([("highway".to_string(), "footway".to_string()),])));
  
    assert_eq!(e1.id, 0);
    assert_eq!(e2.id, 1);
  }

  #[test]
  fn test_factory_sets_base_id() {
    let mut factory: EdgeFactory = EdgeFactory::new();

    let e1: Edge = factory.create_edge(None, 0, 1, 1.0, None);
    let e2: Edge = factory.create_edge(Some(222309), 0, 1, 1.0, Some(HashMap::from([("highway".to_string(), "footway".to_string()),])));
  
    assert_eq!(e1.base_id, None);
    assert_eq!(e2.base_id, Some(222309));
  }

  #[test]
    fn test_factory_starts_at_zero() {
        let factory = EdgeFactory::new();
        assert_eq!(factory.next_id, 0);
    }
}
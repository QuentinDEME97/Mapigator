```rust
  // Add nodes
  graph.add_node(Node::new(1, 49.18, -0.36));
  graph.add_node(Node::new(2, 49.19, -0.37));

  // Add link
  let origin = graph.nodes.get(&1).unwrap();
  let dest = graph.nodes.get(&2).unwrap();
  let link = Link::new(origin, dest, 12345, tags);
  graph.add_link(link);

  // Find outgoing roads from a node
  let outgoing = graph.get_outgoing_links(1);
```

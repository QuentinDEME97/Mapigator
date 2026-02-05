// use std::collections::HashMap;
// use crate::math::graph::{Node, Link, OrientedGraph};

// pub fn get_base_graph() -> OrientedGraph {
//     let mut graph = OrientedGraph::new();

//     // Add nodes
//     graph.add_node(Node::new(0, 49.18, -0.36));
//     graph.add_node(Node::new(1, 49.19, -0.35));
//     graph.add_node(Node::new(2, 49.20, -0.35));
//     graph.add_node(Node::new(3, 49.22, -0.34));
//     graph.add_node(Node::new(4, 49.21, -0.36));
//     graph.add_node(Node::new(5, 49.20, -0.37));
//     graph.add_node(Node::new(6, 49.20, -0.37));

//     // Add links
//     let connections = vec![
//         (0, 1, 1.00),
//         (1, 0, 1.00),
//         (1, 2, 2.00),
//         (2, 1, 2.00),
//         (2, 4, 4.00),
//         (2, 3, 3.00),
//         (3, 2, 3.00),
//         (3, 4, 2.00),
//         (3, 5, 5.00),
//         (5, 3, 8.00),
//         (5, 6, 10.00),
//         (5, 0, 12.00),
//         (6, 5, 10.00),
//         (6, 0, 7.00),
//         (0, 6, 7.00),
//         (0, 5, 12.00)
//     ];

//     // Create links
//     add_links_to_graph(&mut graph, connections);
//     graph
// }

// pub fn get_dumbbell_graph() -> OrientedGraph {
//     let mut graph = OrientedGraph::new();

//     // Add nodes
//     graph.add_node(Node::new(0, 49.18, -0.36));
//     graph.add_node(Node::new(1, 49.19, -0.35));
//     graph.add_node(Node::new(2, 49.20, -0.35));
//     graph.add_node(Node::new(3, 49.22, -0.34));
//     graph.add_node(Node::new(4, 49.21, -0.36));
//     graph.add_node(Node::new(5, 49.20, -0.37));

//     // Add links
//     let connections = vec![
//         (0, 1, 1.00),
//         (1, 0, 1.00),
//         (1, 2, 1.00),
//         (2, 1, 1.00),
//         (2, 0, 1.00),
//         (0, 2, 1.00),
//         (2, 3, 1.00), // |
//         (3, 2, 1.00), // |-> Bridge is here
//         (3, 4, 1.00),
//         (4, 3, 1.00),
//         (4, 5, 1.00),
//         (5, 4, 1.00),
//         (5, 3, 1.00),
//         (3, 5, 1.00),
//     ];

//     // Create links
//     add_links_to_graph(&mut graph, connections);
//     graph
// }

// pub fn get_trap_graph() -> OrientedGraph {
//     let mut graph = OrientedGraph::new();

//     // Add nodes
//     graph.add_node(Node::new(0, 49.18, -0.36));
//     graph.add_node(Node::new(1, 49.19, -0.35));
//     graph.add_node(Node::new(2, 49.20, -0.35));
//     graph.add_node(Node::new(3, 49.22, -0.34));
//     graph.add_node(Node::new(4, 49.21, -0.36));

//     // Add links
//     let connections = vec![
//         (0, 1, 1.00),
//         (1, 0, 1.00),
//         (1, 2, 1.00), // Trap
//         (2, 3, 1.00),
//         (3, 4, 1.00),
//         (4, 2, 1.00),
//     ];

//     // Create links
//     add_links_to_graph(&mut graph, connections);
//     graph
// }

// pub fn get_star_graph() -> OrientedGraph {
//     let mut graph = OrientedGraph::new();

//     // Add nodes
//     graph.add_node(Node::new(0, 49.18, -0.36));
//     graph.add_node(Node::new(1, 49.19, -0.35));
//     graph.add_node(Node::new(2, 49.20, -0.35));
//     graph.add_node(Node::new(3, 49.22, -0.34));
//     graph.add_node(Node::new(4, 49.21, -0.36)); // Is the HUB

//     let connections = vec![
//         (0, 4, 1.00),
//         (4, 0, 1.00),
//         (1, 4, 1.00),
//         (4, 1, 1.00),
//         (2, 4, 1.00),
//         (4, 2, 1.00),
//         (3, 4, 1.00),
//         (4, 3, 1.00),
//     ];

//     add_links_to_graph(&mut graph, connections);
//     graph
// }

// pub fn get_grid_graph() -> OrientedGraph {
//     let mut graph = OrientedGraph::new();

//     // Add nodes
//     graph.add_node(Node::new(0, 49.18, -0.36));
//     graph.add_node(Node::new(1, 49.19, -0.35));
//     graph.add_node(Node::new(2, 49.20, -0.35));
//     graph.add_node(Node::new(3, 49.22, -0.34));

//     let connections = vec![
//         (0, 1, 1.00),
//         (1, 0, 1.00),
//         (1, 2, 1.00),
//         (2, 1, 1.00),
//         (2, 3, 1.00),
//         (3, 2, 1.00),
//         (3, 0, 1.00),
//         (0, 3, 1.00),
//     ];

//     add_links_to_graph(&mut graph, connections);
//     graph
// }

// pub fn add_links_to_graph(graph: &mut OrientedGraph, connections: Vec<(i64, i64, f64)>) {
//     for (i, (src_id, dst_id, distance)) in connections.iter().enumerate() {
//         // On récupère les références aux noeuds (Attention au unwrap ici, 
//         // assure-toi que tes IDs dans 'connections' existent bien au dessus)
//         if let (Some(origin), Some(dest)) = (graph.nodes.get(src_id), graph.nodes.get(dst_id)) {
            
//             // Création de tags bidons pour le test
//             let mut tags = HashMap::new();
//             tags.insert("highway".to_string(), "residential".to_string());
            
//             // Création du lien (on utilise 'i' pour simuler un ID unique de lien)
//             let link = Link::with_fixed_distance(origin, dest, i as i64, tags, *distance);
            
//             graph.add_link(link);
//         }
//     }
// }
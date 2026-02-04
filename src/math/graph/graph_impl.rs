use std::collections::HashMap;

use rayon::prelude::*;

use crate::{math::graph::{INF, Link, MatrixResult, Node, OrientedGraph}, overpass::{DRIVABLE_HIGHWAYS, OverpassElement, OverpassResponse}};

impl Node {
  pub fn new(id: i64, lat: f64, lon: f64) -> Self {
      Self { id, lat, lon }
  }
  
  // Haversine formula to calculate distance in meters
  pub fn distance_to(&self, other: &Node) -> f64 {
      let r = 6371.0; // Earth's radius in km
      
      let lat1 = self.lat.to_radians();
      let lat2 = other.lat.to_radians();
      let delta_lat = (other.lat - self.lat).to_radians();
      let delta_lon = (other.lon - self.lon).to_radians();
      
      let a = (delta_lat / 2.0).sin().powi(2)
          + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
      let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
      
      r * c * 100.00
  }
}

impl Link {
    pub fn new(origin: &Node, dest: &Node, way_id: i64, tags: HashMap<String, String>) -> Self {
        Self {
            origin_id: origin.id,
            dest_id: dest.id,
            distance: origin.distance_to(dest),
            way_id,
            tags,
        }
    }
    
    pub fn with_fixed_distance(
        origin: &Node, 
        dest: &Node, 
        way_id: i64, 
        tags: HashMap<String, String>,
        distance: f64
    ) -> Self {
        Self {
            origin_id: origin.id,
            dest_id: dest.id,
            distance,
            way_id,
            tags,
        }
    }
}

impl OrientedGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
            adjacency: HashMap::new(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }
    
    pub fn add_link(&mut self, link: Link) {
        let link_idx = self.links.len();
        let origin_id = link.origin_id;
        
        self.links.push(link);
        self.adjacency
            .entry(origin_id)
            .or_insert_with(Vec::new)
            .push(link_idx);
    }
    
    pub fn get_outgoing_links(&self, node_id: i64) -> Vec<&Link> {
        self.adjacency
            .get(&node_id)
            .map(|indices| indices.iter().map(|&i| &self.links[i]).collect())
            .unwrap_or_default()
    }

    pub fn tarjan_seq(&self) -> Vec<Vec<i64>> {
        let mut index = 0;
        let mut stack = Vec::new();
        let mut on_stack = HashMap::new();
        let mut indices = HashMap::new();
        let mut lowlink = HashMap::new();
        let mut sccs = Vec::new();

        // On lance le DFS pour chaque nœud non visité
        for &node_id in self.nodes.keys() {
            if !indices.contains_key(&node_id) {
                self.strongconnect(
                    node_id,
                    &mut index,
                    &mut stack,
                    &mut on_stack,
                    &mut indices,
                    &mut lowlink,
                    &mut sccs,
                );
            }
        }
        sccs
    }

    fn strongconnect(
        &self,
        v: i64,
        index: &mut usize,
        stack: &mut Vec<i64>,
        on_stack: &mut HashMap<i64, bool>,
        indices: &mut HashMap<i64, usize>,
        lowlink: &mut HashMap<i64, usize>,
        sccs: &mut Vec<Vec<i64>>,
    ) {
        // Initialisation du nœud v
        indices.insert(v, *index);
        lowlink.insert(v, *index);
        *index += 1;
        stack.push(v);
        on_stack.insert(v, true);

        // Exploration des successeurs
        if let Some(links_indices) = self.adjacency.get(&v) {
            for &link_idx in links_indices {
                let w = self.links[link_idx].dest_id;
                
                if !indices.contains_key(&w) {
                    // Successeur non encore visité
                    self.strongconnect(w, index, stack, on_stack, indices, lowlink, sccs);
                    let new_low = std::cmp::min(lowlink[&v], lowlink[&w]);
                    lowlink.insert(v, new_low);
                } else if *on_stack.get(&w).unwrap_or(&false) {
                    // Le successeur est dans la pile (fait partie de la SCC courante)
                    let new_low = std::cmp::min(lowlink[&v], indices[&w]);
                    lowlink.insert(v, new_low);
                }
            }
        }

        // Si v est un nœud racine, on dépile pour extraire la SCC
        if lowlink[&v] == indices[&v] {
            let mut scc = Vec::new();
            while let Some(node) = stack.pop() {
                on_stack.insert(node, false);
                scc.push(node);
                if node == v { break; }
            }
            sccs.push(scc);
        }
    }

    pub fn floyd_warshall_seq(&self) -> MatrixResult {
      let (mut dists, id_map, n) = prepare_matrix(self);

      for k in 0..n {
        for i in 0..n {
          let dist_ik = dists[i * n + k];
          if dist_ik == INF { continue; }

          for j in 0..n {
            let dist_kj = dists[k * n + j];
            if dist_kj == INF { continue; }

            let new_dist = dist_ik + dist_kj;
            let idx = i * n + j;

            if new_dist < dists[idx] {
              dists[idx] = new_dist;
            }
          }
        }
      }
      MatrixResult { dists, id_map, size: n }
    }

    pub fn floyd_warshall_par(&self) -> MatrixResult {
        let (mut dists, id_map, n) = prepare_matrix(self);

        // Boucle K séquentielle (obligatoire)
        for k in 0..n {
            // Astuce Rust : On extrait la ligne K pour la lire sans conflit
            // pendant qu'on modifie le reste de la matrice en parallèle.
            let k_row_start = k * n;
            let k_row = dists[k_row_start..k_row_start + n].to_vec(); // Copie de la ligne k

            // On traite chaque ligne 'i' en parallèle
            // chunks_mut(n) découpe le gros vecteur en tranches de taille N (une ligne = une tranche)
            dists.par_chunks_mut(n)
                .enumerate() // On a besoin de savoir quel est l'index 'i'
                .for_each(|(i, row_i)| {
                    
                    // row_i est la ligne 'i' entière que ce thread doit mettre à jour
                    
                    // On récupère dist[i][k] (la distance pour aller au pivot)
                    // Note: row_i contient n éléments. L'élément k est à l'index k localement.
                    let dist_ik = row_i[k]; 

                    // Petite opti : si i ne peut pas aller à k, pas de raccourci possible
                    if dist_ik == INF { return; }

                    // Boucle J (interne à la ligne)
                    for j in 0..n {
                        let dist_kj = k_row[j]; // Lecture dans la copie de la ligne k

                        if dist_kj != INF {
                            let new_dist = dist_ik + dist_kj;
                            if new_dist < row_i[j] {
                                row_i[j] = new_dist;
                            }
                        }
                    }
                });
        }

        MatrixResult { dists, id_map, size: n }
    }

    pub fn describe(&self) {
        println!("┌─ Road Graph Statistics");
        println!("│");
        println!("│  Nodes: {}", self.nodes.len());
        println!("│  Links: {}", self.links.len());
        println!("│");
        
        // Calculate average degree (outgoing links per node)
        let nodes_with_outgoing = self.adjacency.len();
        let avg_degree = if nodes_with_outgoing > 0 {
            self.links.len() as f64 / nodes_with_outgoing as f64
        } else {
            0.0
        };
        println!("│  Nodes with outgoing links: {}", nodes_with_outgoing);
        println!("│  Average outgoing degree: {:.2}", avg_degree);
        println!("│");
        
        // Find nodes with most connections
        if let Some((max_node_id, max_links)) = self.adjacency
            .iter()
            .max_by_key(|(_, links)| links.len()) {
            println!("│  Max outgoing links from single node: {} (node ID: {})", max_links.len(), max_node_id);
        }
        
        // Count highway types
        let mut highway_counts: HashMap<String, usize> = HashMap::new();
        for link in &self.links {
            if let Some(highway_type) = link.tags.get("highway") {
                *highway_counts.entry(highway_type.clone()).or_insert(0) += 1;
            }
        }
        
        println!("│");
        println!("│  Highway types:");
        let mut sorted_highways: Vec<_> = highway_counts.iter().collect();
        sorted_highways.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (highway_type, count) in sorted_highways.iter().take(10) {
            println!("│    - {}: {}", highway_type, count);
        }
        
        println!("└─");
    }
}

impl MatrixResult {
  pub fn get_distance(&self, origin_id: i64, dest_id: i64) -> Option<f64> {
    let i = *self.id_map.get(&origin_id)?;
    let j = *self.id_map.get(&dest_id)?;
    Some(self.dists[i * self.size + j])
  }
}

pub fn prepare_matrix(graph: &OrientedGraph) -> (Vec<f64>, HashMap<i64, usize>, usize) {
    let mut id_map = HashMap::new();
    let mut index = 0;
    for &node_id in graph.nodes.keys() {
      id_map.insert(node_id, index);
      index += 1;
    }
    let n = index; // N size

    // Init matrix with INF
    let mut dists = vec![INF; n * n];

    // Set diag to 0
    for i in 0..n {
      dists[i * n + i] = 0.0;
    }

    // Fill with existing links
    for link in &graph.links {
      if let (Some(&u), Some(&v)) = (id_map.get(&link.origin_id), id_map.get(&link.dest_id)) {
        let idx = u * n + v;
        // Keep the min if there is multiple links between two nodes
        if (link.distance < dists[idx]) {
          dists[idx] = link.distance;
        }
      }
    }
    (dists, id_map, n)
  }

// TODO : Update with node_id mapping
pub fn display_matrix(matrix: &[f64], n: usize, id_map: &HashMap<i64, usize>) {
    // 1. Sécurité : Vérifier la cohérence
    if matrix.len() != n * n {
        println!("⚠️ Error : Vector size ({}) is not a matrix {}x{}", matrix.len(), n, n);
        return;
    }
    
    // 2. Limite d'affichage
    if n > 100 {
        println!("⚠️ Matrix too big to display ({}x{}). Limit : 100x100.", n, n);
        return;
    }
    
    // 3. Créer un vecteur trié des IDs pour un affichage cohérent
    let mut ids: Vec<i64> = id_map.keys().copied().collect();
    ids.sort();
    
    println!("┌ Matrix ({}x{}) :", n, n);
    
    // En-tête (Node IDs au lieu d'indices)
    print!("      "); // Espace pour l'index de ligne
    for &node_id in &ids {
        print!("{:>7} ", node_id); // Affiche le vrai ID du noeud
    }
    println!();
    println!("      {}", "─".repeat(n * 8)); // Ligne de séparation
    
    // Lignes de données
    for &row_id in &ids {
        let i = id_map[&row_id]; // Récupère l'index matrice du node_id
        
        // Index de ligne (avec le vrai ID)
        print!(" {:>4} │", row_id);
        
        // Valeurs de la ligne
        for &col_id in &ids {
            let j = id_map[&col_id]; // Récupère l'index matrice du node_id
            let val = matrix[i * n + j];
            
            if val == f64::INFINITY {
                print!("\x1b[90m{:>7}\x1b[0m ", "INF");
            } else if val == 0.0 {
                print!("\x1b[90m{:>7.1}\x1b[0m ", 0.0);
            } else {
                print!("{:>7.1} ", val);
            }
        }
        println!();
    }
    println!("└─────────────────────────────────");
}

pub fn roads_to_graph(roads: OverpassResponse) -> OrientedGraph {
  let mut road_graph = OrientedGraph::new();

  let mut node_map: HashMap<i64, &OverpassElement> = HashMap::new();
  for element in &roads.elements {
      if element.element_type == "node" {
          node_map.insert(element.id, element);
      }
  }

  for (i, road) in roads.elements.iter().enumerate() {
      println!("Road #{}", i + 1);
      if let Some(tags) = &road.tags {
        if let Some(highway) = tags.get("highway") {
            if is_drivable(highway) {
                let is_oneway = tags.get("oneway")
                  .map(|v| v == "yes" || v == "1" || v == "true")
                  .unwrap_or(false);
              
              // Get the ordered list of node IDs
              if let Some(node_ids) = &road.nodes {
                  // Add all nodes to the graph
                  for &node_id in node_ids {
                      if let Some(&node_element) = node_map.get(&node_id) {
                          if let (Some(lat), Some(lon)) = (node_element.lat, node_element.lon) {
                              let node = Node::new(node_id, lat, lon);
                              road_graph.add_node(node);
                          }
                      }
                  }
                  
                  // Create links between consecutive nodes
                  for i in 0..node_ids.len() - 1 {
                        let origin_id = node_ids[i];
                        let dest_id = node_ids[i + 1];
                        
                        // Clone the nodes to avoid borrow conflict
                        let origin = road_graph.nodes.get(&origin_id).cloned();
                        let dest = road_graph.nodes.get(&dest_id).cloned();
                        
                        if let (Some(origin), Some(dest)) = (origin, dest) {
                            // Forward link (A -> B)
                            let link_forward = Link::new(
                                &origin,
                                &dest,
                                road.id,
                                tags.clone()
                            );
                            road_graph.add_link(link_forward);
                            
                            // If not oneway, add reverse link (B -> A)
                            if !is_oneway {
                                let link_reverse = Link::new(
                                    &dest,
                                    &origin,
                                    road.id,
                                    tags.clone()
                                );
                                road_graph.add_link(link_reverse);
                            }
                        }
                    }
              }
            }
        }
      }
      println!();
  }
  road_graph
}

// TODO : Update this function to remove "service" if access is private
pub fn is_drivable(highway_type: &str) -> bool {    
    DRIVABLE_HIGHWAYS.contains(&highway_type)
}
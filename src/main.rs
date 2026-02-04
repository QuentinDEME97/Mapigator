mod nominatim;
mod overpass;
mod file;
mod benchmark;
mod analysis;
mod math;
mod utils;

use math::graph::{Node, Link, OrientedGraph};
use crate::math::graph::{MatrixResult, display_matrix, prepare_matrix};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = utils::get_trap_graph();

    // Vérification
    let outgoing = graph.get_outgoing_links(3);
    println!("Output of node 3 : {:?}", outgoing.len()); // Devrait afficher 3 (vers 1, 3 et 5)

    graph.describe();

    let (matrix, id_map, n) = prepare_matrix(&graph);

    // Affichage
    display_matrix(&matrix, n, &id_map);
    println!("{:?}", id_map);
    let floyd_matrix = graph.floyd_warshall_par();
    display_matrix(&floyd_matrix.dists, n, &id_map);

    let sccs = graph.tarjan_seq();
    println!("Graph has {} strongly connected component(s) :\n{:?}",sccs.len(), sccs);
    Ok(())
}

fn query_and_compute() -> Result<(), Box<dyn std::error::Error>> {
       let query = "Caen";

    println!("Searching location for '{}'...", query);

    let results = nominatim::call_nominatim(query)?;

    println!("Found {} result(s) !\n", results.len());
    
    for (i, result) in results.iter().enumerate() {
        println!("Result #{}", i + 1);
        println!("  {:<15} : {}", "osm_id", result.osm_id);
        println!("  {:<15} : {}", "osm_type", result.osm_type);
        println!("  {:<15} : {}", "place_id", result.place_id);
        println!("  {:<15} : {}", "Name", result.display_name);
        println!("  {:<15} : {}, {}", "Coordinates", result.lat, result.lon);
        println!("  {:<15} : {} ({})", "Type", result.result_type, result.class);
        println!();
    }

    if let Some(result) = results.first() {
        println!("Performing Overpass Request for result #1...\n");
        let roads = overpass::get_roads(&result.osm_type, result.osm_id)?;

        println!("Found {} roads !\n", roads.elements.len());

        let filename = file::save_data(
            &roads,
            &result.display_name,
            result.osm_id,
            &result.osm_type,
        )?;
        
        println!("✓ Saved to: {}\n", filename);

        // for (i, road) in roads.elements.iter().take(5).enumerate() {
        //     println!("Road #{}", i + 1);
        //     if let Some(tags) = &road.tags {
        //         println!("  {:<15} : {}", "Name", tags.get("name").unwrap_or(&"Unnamed".to_string()));
        //         println!("  {:<15} : {}", "Highway Type", tags.get("highway").unwrap_or(&"unknown".to_string()));
        //     }
        //     println!();
        // }

        // Benchmark different implementations
        println!("\n🔬 Running benchmarks...\n");
        
        let (bench1, stats1) = benchmark::bench("Simple Analysis", || {
            analysis::analyze_roads_simple(&roads)
        });
        bench1.print();
        
        let (bench2, stats2) = benchmark::bench("Parallel Analysis", || {
            analysis::analyze_roads_parallel(&roads)
        });
        bench2.print();
        
        // Print stats from one of them (they should be the same)
        stats1.print_summary();
        
        // Compare
        println!("\n📊 Performance Comparison:");
        let speedup = bench1.duration.as_secs_f64() / bench2.duration.as_secs_f64();
        println!("  Parallel is {:.2}x faster", speedup);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Importe tout ce qui est défini dans le fichier parent
    use std::collections::HashMap;

    #[test]
    fn test_floyd_warshall_topology() {
        let mut graph = OrientedGraph::new(); // Ou OrientedGraph selon votre nommage

        // 1. Add nodes
        // Les coordonnées importent peu ici car on force les distances
        graph.add_node(Node::new(0, 49.18, -0.36));
        graph.add_node(Node::new(1, 49.19, -0.35));
        graph.add_node(Node::new(2, 49.20, -0.35));
        graph.add_node(Node::new(3, 49.22, -0.34));
        graph.add_node(Node::new(4, 49.21, -0.36));
        graph.add_node(Node::new(5, 49.20, -0.37));
        graph.add_node(Node::new(6, 49.20, -0.37));

        // 2. Add links (Source, Dest, Distance Forcée)
        let connections = vec![
        (0, 1, 1.00),
        (1, 0, 1.00),
        (1, 2, 2.00),
        (2, 1, 2.00),
        (2, 4, 4.00),
        (2, 3, 3.00),
        (3, 2, 3.00),
        (3, 4, 2.00),
        (3, 5, 5.00),
        (5, 3, 8.00),
        (5, 6, 10.00),
        (5, 0, 12.00),
        (6, 5, 10.00),
        (6, 0, 7.00),
        (0, 6, 7.00),
        (0, 5, 12.00)
    ];

        for (i, (src_id, dst_id, distance)) in connections.iter().enumerate() {
            if let (Some(origin), Some(dest)) = (graph.nodes.get(src_id), graph.nodes.get(dst_id)) {
                let mut tags = HashMap::new();
                tags.insert("highway".to_string(), "test_road".to_string());
                
                // On utilise la méthode with_fixed_distance
                let link = Link::with_fixed_distance(origin, dest, i as i64, tags, *distance);
                graph.add_link(link);
            }
        }

        // --- Vérifications ---

        // Test basique de connectivité immédiate
        let outgoing_3 = graph.get_outgoing_links(3);
        println!("Output of node 3 : {} links", outgoing_3.len());
        // Node 3 va vers : 2, 4, 5. Donc 3 liens.
        assert_eq!(outgoing_3.len(), 3, "Node 3 devrait avoir 3 sorties");

        // Préparation et affichage (pour le debug visuel)
        let (matrix_init, id_map, n) = prepare_matrix(&graph);
        println!("\n--- Matrice Initiale (Liens directs) ---");
        display_matrix(&matrix_init, n, &id_map);

        // Exécution de l'algorithme parallèle
        let result = graph.floyd_warshall_par();
        
        println!("\n--- Résultat Floyd-Warshall ---");
        display_matrix(&result.dists, n, &id_map);

        // --- Assertions sur les plus courts chemins (Validation Mathématique) ---
        
        // Helper pour récupérer une distance depuis le résultat
        let get_dist = |u: i64, v: i64| -> f64 {
            result.get_distance(u, v).unwrap_or(f64::INFINITY)
        };

        // Cas 1 : Chemin direct 0 -> 1
        assert_eq!(get_dist(0, 1), 1.0);

        // Cas 2 : Chemin multi-sauts 0 -> 4
        // Trajet : 0 -> 1 (1.0) -> 2 (2.0) -> 4 (4.0) = Total 7.0
        // (Alternative via 5 est beaucoup plus longue)
        assert_eq!(get_dist(0, 4), 7.0, "Le chemin 0->4 devrait coûter 7.0");

        // Cas 3 : Un chemin qui remonte "à contre-sens" des IDs
        // Trajet 3 -> 0
        // Option A: 3->2(3) -> 1(2) -> 0(1) = 6.0
        // Option B: 3->5(5) -> 0(12) = 17.0
        assert_eq!(get_dist(3, 0), 6.0, "Le chemin 3->0 devrait passer par 2 et 1");

        // Cas 4 : Nœud 4 est un puits (Sink)
        // Il reçoit des liens mais n'en émet aucun vers les autres noeuds du set
        assert_eq!(get_dist(4, 0), f64::INFINITY, "Node 4 ne devrait pouvoir aller nulle part");
    }
}
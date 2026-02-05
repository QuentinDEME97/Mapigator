mod nominatim;
mod overpass;
mod file;
mod benchmark;
mod analysis;
mod utils;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut graph = utils::get_base_graph();

    // // Vérification
    // let outgoing = graph.get_outgoing_links(3);
    // println!("Output of node 3 : {:?}", outgoing.len()); // Devrait afficher 3 (vers 1, 3 et 5)

    // graph.describe();

    // let (matrix, id_map, n) = prepare_matrix(&graph);

    // // Affichage
    // display_matrix(&matrix, n, &id_map);
    // println!("{:?}", id_map);
    // let floyd_matrix = graph.floyd_warshall_par();
    // display_matrix(&floyd_matrix.dists, n, &id_map);

    // let sccs = graph.tarjan_seq();
    // println!("Graph has {} strongly connected component(s) :\n{:?}",sccs.len(), sccs);

    // let brandes = graph.brandes_betweenness_par();
    // println!("{:?}", brandes);
    let filename = "ecouche.json";
    // load_file_and_compute(filename);
    Ok(())
}

// fn load_file_and_compute(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let roads = file::load_file(filename)?;
//     let graph = graph::roads_to_graph(roads);

//     graph.describe();

//     // Floyd-Warshall
//     println!("\nRunning Floyd-Warshall (All-Pairs Shortest Path)...");
//     let floyd_matrix = graph.floyd_warshall_par();

//     // Tarjan
//     println!("\nRunning Tarjan (SCC)...");
//     let sccs = graph.tarjan_seq();
//     println!(" > Found {} strongly connected component(s).", sccs.len());
//     println!("{:?}", sccs);

//     // Brandes
//     println!("\nRunning Brandes (Betweenness Centrality)...");
//     let brandes = graph.brandes_betweenness_par();
    
//     // Affichage rapide du top 5 pour vérifier
//     let mut sorted_brandes: Vec<_> = brandes.iter().collect();
//     sorted_brandes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
//     println!(" > Top 5 central nodes:");
//     for (id, score) in sorted_brandes.iter().take(5) {
//         println!("   - Node {}: {:.2}", id, score);
//     }

//     Ok(())
// }

// fn query_city_and_compute() -> Result<(), Box<dyn std::error::Error>> {
//     let query = "Ecouché";
//     let m = MultiProgress::new();

//     // Style pour les tâches indéterminées (Recherche réseau, parsing...)
//     let spinner_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
//         .unwrap()
//         .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

//     // --- Étape 1 : Nominatim ---
//     let pb_net = m.add(ProgressBar::new_spinner());
//     pb_net.set_style(spinner_style.clone());
//     pb_net.set_message(format!("Searching location for '{}'...", query));
//     pb_net.enable_steady_tick(Duration::from_millis(100));

//     let results = nominatim::call_nominatim(query)?;
//     pb_net.finish_with_message(format!("Found {} result(s) for '{}'", results.len(), query));

//     // Affichage des détails (optionnel, on peut le garder)
//     for (i, result) in results.iter().enumerate() {
//         println!("Result #{} : {} ({})", i + 1, result.display_name, result.osm_type);
//     }

//     if let Some(result) = results.first() {
//         // --- Étape 2 : Overpass ---
//         let pb_overpass = m.add(ProgressBar::new_spinner());
//         pb_overpass.set_style(spinner_style.clone());
//         pb_overpass.set_message("Downloading roads from Overpass...");
//         pb_overpass.enable_steady_tick(Duration::from_millis(100));

//         let roads = overpass::get_roads(&result.osm_type, result.osm_id)?;
//         pb_overpass.finish_with_message(format!("Downloaded {} elements.", roads.elements.len()));

//         // --- Étape 3 : Sauvegarde ---
//         let pb_save = m.add(ProgressBar::new_spinner());
//         pb_save.set_style(spinner_style.clone());
//         pb_save.set_message("Saving data to file...");
//         pb_save.enable_steady_tick(Duration::from_millis(100));

//         let filename = file::save_data(
//             &roads,
//             &result.display_name,
//             result.osm_id,
//             &result.osm_type,
//         )?;
//         pb_save.finish_with_message(format!("Saved to {}", filename));

//         // --- Étape 4 : Construction du Graphe ---
//         println!("Building graph...");
//         let mut graph = roads_to_graph(roads);

//         // --- Étape 5 : Algorithmes avec barres de progression ---
//         // Floyd-Warshall
//         println!("\nRunning Floyd-Warshall (All-Pairs Shortest Path)...");
//         let floyd_matrix = graph.floyd_warshall_par();

//         // Tarjan
//         println!("\nRunning Tarjan (SCC)...");
//         let sccs = graph.tarjan_seq();
//         println!(" > Found {} strongly connected component(s).", sccs.len());

//         // Brandes
//         println!("\nRunning Brandes (Betweenness Centrality)...");
//         let brandes = graph.brandes_betweenness_par();
        
//         // Affichage rapide du top 5 pour vérifier
//         let mut sorted_brandes: Vec<_> = brandes.iter().collect();
//         sorted_brandes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
//         println!(" > Top 5 central nodes:");
//         for (id, score) in sorted_brandes.iter().take(5) {
//             println!("   - Node {}: {:.2}", id, score);
//         }
//     }
//     Ok(())
// }

// fn query_and_compute() -> Result<(), Box<dyn std::error::Error>> {
//     let query = "Caen";

//     println!("Searching location for '{}'...", query);

//     let results = nominatim::call_nominatim(query)?;

//     println!("Found {} result(s) !\n", results.len());
    
//     for (i, result) in results.iter().enumerate() {
//         println!("Result #{}", i + 1);
//         println!("  {:<15} : {}", "osm_id", result.osm_id);
//         println!("  {:<15} : {}", "osm_type", result.osm_type);
//         println!("  {:<15} : {}", "place_id", result.place_id);
//         println!("  {:<15} : {}", "Name", result.display_name);
//         println!("  {:<15} : {}, {}", "Coordinates", result.lat, result.lon);
//         println!("  {:<15} : {} ({})", "Type", result.result_type, result.class);
//         println!();
//     }

//     if let Some(result) = results.first() {
//         println!("Performing Overpass Request for result #1...\n");
//         let roads = overpass::get_roads(&result.osm_type, result.osm_id)?;

//         println!("Found {} roads !\n", roads.elements.len());

//         let filename = file::save_data(
//             &roads,
//             &result.display_name,
//             result.osm_id,
//             &result.osm_type,
//         )?;
        
//         println!("✓ Saved to: {}\n", filename);

//         // for (i, road) in roads.elements.iter().take(5).enumerate() {
//         //     println!("Road #{}", i + 1);
//         //     if let Some(tags) = &road.tags {
//         //         println!("  {:<15} : {}", "Name", tags.get("name").unwrap_or(&"Unnamed".to_string()));
//         //         println!("  {:<15} : {}", "Highway Type", tags.get("highway").unwrap_or(&"unknown".to_string()));
//         //     }
//         //     println!();
//         // }

//         // Benchmark different implementations
//         println!("\n🔬 Running benchmarks...\n");
        
//         let (bench1, stats1) = benchmark::bench("Simple Analysis", || {
//             analysis::analyze_roads_simple(&roads)
//         });
//         bench1.print();
        
//         let (bench2, stats2) = benchmark::bench("Parallel Analysis", || {
//             analysis::analyze_roads_parallel(&roads)
//         });
//         bench2.print();
        
//         // Print stats from one of them (they should be the same)
//         stats1.print_summary();
        
//         // Compare
//         println!("\n📊 Performance Comparison:");
//         let speedup = bench1.duration.as_secs_f64() / bench2.duration.as_secs_f64();
//         println!("  Parallel is {:.2}x faster", speedup);
//     }
//     Ok(())
// }
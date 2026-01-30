mod nominatim;
mod overpass;
mod file;
mod benchmark;
mod analysis;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Paris";

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
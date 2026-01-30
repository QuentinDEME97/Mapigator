use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub memory_usage: Option<usize>, // in bytes
}

impl BenchmarkResult {
    pub fn print(&self) {
        println!("┌─ Benchmark: {}", self.name);
        println!("│  ⏱️  Duration: {:?}", self.duration);
        println!("│  ⏱️  Duration (ms): {:.2}", self.duration.as_secs_f64() * 1000.0);
        if let Some(mem) = self.memory_usage {
            println!("│  💾 Memory: {} bytes ({:.2} MB)", mem, mem as f64 / 1_048_576.0);
        }
        println!("└─");
    }
}

/// Generic benchmark function that takes any closure and measures its execution
pub fn bench<F, T>(name: &str, mut func: F) -> (BenchmarkResult, T)
where
    F: FnMut() -> T,
{
    // Optional: measure memory before (requires nightly or external crate)
    // For now, we'll focus on time
    
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    
    let bench_result = BenchmarkResult {
        name: name.to_string(),
        duration,
        memory_usage: None, // We'll add this later if needed
    };
    
    (bench_result, result)
}

/// Benchmark and compare multiple implementations
pub fn bench_compare<F, T>(benchmarks: Vec<(&str, F)>) -> Vec<BenchmarkResult>
where
    F: FnMut() -> T,
{
    let mut results = Vec::new();
    
    for (name, mut func) in benchmarks {
        let (bench_result, _) = bench(name, func);
        bench_result.print();
        results.push(bench_result);
    }
    
    // Print comparison
    if results.len() > 1 {
        println!("\n📊 Comparison:");
        let fastest = results.iter().min_by_key(|r| r.duration).unwrap();
        for result in &results {
            let ratio = result.duration.as_secs_f64() / fastest.duration.as_secs_f64();
            println!("  {} : {:.2}x slower than fastest", result.name, ratio);
        }
    }
    
    results
}

/// Statistics helper for road analysis
#[derive(Debug)]
pub struct RoadStats {
    pub total_roads: usize,
    pub highway_types: HashMap<String, usize>,
    pub unique_names: usize,
    pub points_per_road: HashMap<i64, usize>, // road_id -> point_count
}

impl RoadStats {
    pub fn new() -> Self {
        Self {
            total_roads: 0,
            highway_types: HashMap::new(),
            unique_names: 0,
            points_per_road: HashMap::new(),
        }
    }
    
    pub fn print_summary(&self) {
        println!("\n📈 Road Statistics:");
        println!("  Total roads: {}", self.total_roads);
        println!("  Unique names: {}", self.unique_names);
        println!("  Highway types:");
        for (highway_type, count) in &self.highway_types {
            println!("    - {}: {}", highway_type, count);
        }
    }
}
<div align="center">

<!-- Add your logo here -->
<!-- ![Mapigator Logo](logo.png) -->

<img src="mapigator.webp" alt="drawing" width="200"/>

# Mapigator

**A high-performance road network analysis tool built in Rust**

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![OpenStreetMap](https://img.shields.io/badge/OpenStreetMap-7EBC6F?style=for-the-badge&logo=openstreetmap&logoColor=white)](https://www.openstreetmap.org/)

</div>

---

## 📖 Overview

Mapigator is a powerful command-line tool for analyzing road networks from OpenStreetMap data. It provides an intuitive terminal user interface (TUI) for scraping road data and performing detailed network analysis with optimized algorithms.

## ✨ Features

- 🗺️ **Data Scraping**: Search locations via Nominatim and fetch road networks from Overpass API
- 📊 **Network Analysis**: Build directed graphs from road networks with one-way street support
- ⚡ **Performance Benchmarking**: Compare single-threaded vs. parallel analysis implementations
- 🖥️ **Interactive TUI**: Navigate and analyze data through a clean terminal interface
- 💾 **Data Management**: Save and load road network data in JSON format
- 📈 **Statistics**: Comprehensive road network metrics and visualizations

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- Internet connection (for fetching OSM data)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/mapigator.git
cd mapigator

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## 🎮 Usage

### TUI Mode (Interactive)

```bash
cargo run
```

Navigate using:

- **Arrow keys**: Move through menus and lists
- **Enter**: Select options
- **ESC**: Go back to previous menu
- **Q**: Quit application

#### Scrap Mode

1. Select "Scrap" from main menu
2. Enter a location name (e.g., "Caen")
3. Browse search results
4. Press Enter to download road network data

#### Browse Mode

1. Select "Browse" from main menu
2. Navigate through saved data files
3. Press **S** for statistics
4. Press **B** for benchmark comparison

### Programmatic Usage

```rust
use mapigator::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load road network data
    let roads = file::load_file("data/your_file.json")?;

    // Convert to graph structure
    let graph = graph::roads_to_graph(roads);

    // Display statistics
    graph.describe();

    // Analyze with different methods
    let stats = analysis::analyze_roads_simple(&roads);
    stats.print_summary();

    Ok(())
}
```

## 📂 Project Structure

```
mapigator/
├── src/
│   ├── main.rs           # Application entry point
│   ├── tui.rs            # Terminal UI implementation
│   ├── nominatim.rs      # Location search via Nominatim API
│   ├── overpass.rs       # Road data fetching via Overpass API
│   ├── graph.rs          # Graph data structures (Node, Link, RoadGraph)
│   ├── analysis.rs       # Road network analysis algorithms
│   ├── benchmark.rs      # Performance benchmarking utilities
│   └── file.rs           # File I/O operations
├── data/                 # Stored road network data (JSON)
├── Cargo.toml
└── README.md
```

## 🛣️ Road Network Features

### Supported Road Types

Mapigator analyzes drivable roads including:

- Motorways, trunks, primary, secondary, tertiary roads
- Residential and unclassified roads
- Service roads and living streets
- Highway links (on/off ramps)

### Graph Properties

- **Nodes**: Intersections and waypoints with GPS coordinates
- **Links**: Directed edges between nodes with distance calculation
- **One-way support**: Proper handling of one-way streets
- **Distance calculation**: Haversine formula for accurate km distances

## 📊 Analysis Capabilities

### Statistics

- Total number of roads
- Unique street names
- Highway type distribution
- Node and link counts
- Network connectivity metrics

### Benchmarking

- Single-threaded analysis
- Parallel processing with Rayon
- Performance comparison and speedup metrics
- Execution time measurements

## 🛠️ Technical Stack

- **Language**: Rust 🦀
- **TUI Framework**: [ratatui](https://github.com/ratatui-org/ratatui)
- **Terminal**: [crossterm](https://github.com/crossterm-rs/crossterm)
- **HTTP Client**: [reqwest](https://github.com/seanmonstar/reqwest)
- **Serialization**: [serde](https://github.com/serde-rs/serde) & [serde_json](https://github.com/serde-rs/json)
- **Parallel Processing**: [rayon](https://github.com/rayon-rs/rayon)
- **Progress Indicators**: [indicatif](https://github.com/console-rs/indicatif)

## 🎯 Use Cases

- Urban planning and traffic analysis
- Route optimization research
- Road network connectivity studies
- OpenStreetMap data quality assessment
- Geographic information system (GIS) prototyping

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [OpenStreetMap](https://www.openstreetmap.org/) for map data
- [Nominatim](https://nominatim.org/) for geocoding services
- [Overpass API](https://overpass-api.de/) for OSM data queries
- The Rust community for excellent crates and tools

## 📧 Contact

Project Link: [https://github.com/yourusername/mapigator](https://github.com/yourusername/mapigator)

---

<div align="center">

Made with ❤️ and 🦀

</div>

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

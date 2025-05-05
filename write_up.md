# Wolf Reproductive Success and Denning Phenology Analysis

## Project Overview

This project investigates the relationship between environmental factors and 
reproductive success in wolves, focusing on denning phenology. Leveraging the NASA 
ABoVE dataset, we analyzed CSV-based field data to explore how variables such as 
timing, location, and cluster-based graph properties relate to wolf reproductive 
outcomes. The implementation is written in Rust, chosen for its performance and 
safety guarantees, and emphasizes data parsing, modularity, graph analysis, and 
visualizations.

## Goals and Objectives

The main objectives of this project were to:
- Parse and process CSV datasets containing denning and reproductive records.
- Construct a graph model representing the wolf data, allowing network-based 
analysis.
- Compute correlations and trends in reproductive success and denning behaviors.
- Apply centrality and clustering algorithms to detect ecological patterns.
- Visualize key relationships and trends using charts for interpretability.
- Adhere to modular and idiomatic Rust coding practices, including testing and 
documentation.

## Dataset Details

Two CSV datasets were used:
1. **Denning Data**: Includes wolf ID, pack, year, den entry/exit dates, and 
environmental tags.
2. **Reproductive Data**: Includes wolf ID, pack, year, litter size, and indicators 
of reproductive success.

Each dataset contained 500 entries, but the graph satisfies the size requirement 
for analysis. Parsing was implemented using the `csv` crate, and the data was 
cleaned and validated for missing fields and type mismatches.

## Code Structure and Modules

The Rust project follows a modular structure, separating responsibilities across 
three primary files:

### 1. **`data.rs`**
- Handles CSV parsing for both denning and reproductive datasets.
- Defines structured types (`DenningRecord`, `ReproductiveRecord`) and uses Rust's 
type safety to enforce schema integrity.
- Includes helper functions for loading, validating, and cleaning data.

### 2. **`graph.rs`**
- Builds a graph using the `petgraph` crate.
- Nodes represent individual wolves or den sites; edges indicate temporal, spatial, 
or reproductive connections.
- Calculates metrics such as degree centrality, connected components, and 
clustering coefficients.

### 3. **`main.rs`**
- Orchestrates the full analysis pipeline.
- Loads data, constructs the graph, computes metrics, and generates plots.
- Uses the `plotters` crate to create visualizations of:
  - Litter size vs. den entry dates.
  - Reproductive success trends by year.
  - Node centrality vs. reproductive outcomes.

## Graph Analysis

We modeled wolf denning interactions as a **temporal graph**, where:
- **Nodes** = wolves or dens.
- **Edges** = shared den use, temporal overlap, pack membership.

### Graph analysis techniques included:
- **Centrality**: Identifying key individuals or locations influencing reproductive 
success.
- **Clustering**: Detecting ecological hotspots and pack cohesion.
- **Subgraph Extraction**: Studying successful vs. unsuccessful reproductive 
histories.

## Visualizations

Visual outputs played a significant role in the analysis:
- **Line Charts**: Reproductive success across years, showing potential shifts in 
success rates.
- **Scatter Plots**: Litter size vs. den entry date, highlighting correlations 
between early denning and larger litters.
- **Bar Charts**: Pack-level success metrics and node centrality frequencies.

The charts were rendered using the `plotters` crate, embedded directly in the Rust 
runtime with PNG output support.

## Results and Findings

Key findings include:
- **Earlier den entry dates** correlated with increased reproductive success.
- **Packs with higher internal connectivity** (via graph edges) had more consistent 
reproductive outcomes.
- **Isolated nodes** (wolves without strong pack connections) showed lower litter 
sizes and success rates.
- **Clustering analysis** revealed that spatially close dens often had similar 
reproductive patterns, suggesting environmental drivers.

### **Denning Clustering Summary**:
- **Early denning (< DOY 120)**: 73 records
- **Late denning (≥ DOY 120)**: 151 records

### **Top 5 Packs by Degree Centrality**:
- **Pack 44** → degree 795
- **Pack 57** → degree 795
- **Pack 46** → degree 795
- **Pack 39** → degree 644
- **Pack 74** → degree 52

## Testing and Code Quality

We implemented unit tests to ensure the correctness of:
- Data loading
- Graph construction
- Metric calculations

Code follows idiomatic Rust practices:
- Use of iterators, match statements, and `Option/Result` handling.
- Separation of concerns across modules.
- Proper naming, documentation, and error handling throughout.

The project exceeds 150 lines of Rust code and includes structured commit history 
on GitHub, demonstrating consistent development practices.

## Conclusion

This project showcases how ecological data can be modeled and analyzed using Rust 
for high-performance insights. Through structured data parsing, graph-theoretic 
modeling, and visual exploration, we gained a deeper understanding of the 
environmental and social factors affecting wolf reproductive success.

### Future Directions
- Incorporating climate and vegetation data from the full NASA ABoVE archive.
- Applying machine learning models for prediction.
- Extending the graph model to simulate multi-year pack dynamics.

## Appendices

### **Crates Used**:
- `csv`, `petgraph`, `plotters`, `chrono`, `serde`, `serde_derive`

### **File Summary**:
- [NASA ABoVE 
Dataset](https://search.earthdata.nasa.gov/search/granules?p=C2143401778-ORNL_CLOUD&pg[0][v]=f&pg[0][gsk]=-start_date&g=G2143902954-ORNL_CLOUD&tl=1229255999.5!5!!&fsm0=Animals/Vertebrates&fst0=Biological%20Classification&lat=56.07421875&long=-100.546875)

### **File Descriptions**:
- **`src/main.rs`**: Project entry and orchestration.
- **`src/data.rs`**: CSV parsing and record definition.
- **`src/graph.rs`**: Graph building and metrics.
- **`output/`**: Generated PNG visualizations.

### **Output**:
- Skipping line 9 due to missing fields.
- Skipping line 32 due to missing fields.
- Skipping line 42 due to missing fields.



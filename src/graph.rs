//! Graph analysis module for wolf denning data
//! 
//! Builds a graph of wolf packs based on shared study areas and computes network centrality.

use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

use crate::data::DenningPhenology;

/// Struct representing a wolf pack node in the graph
/// Each node stores pack ID, study area, and location
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WolfNode {
    pub pack_id: u32,
    pub study: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Builds a graph from wolf denning data.
///
/// Nodes = packs  
/// Edges = packs in the same study area (co-location)
///
/// # Arguments
/// * `denning_data` - List of denning records
///
/// # Returns
/// * An undirected `Graph` of `WolfNode` with empty edge weights
pub fn build_graph(denning_data: &[DenningPhenology]) -> Graph<WolfNode, (), Undirected> {
    let mut graph = Graph::<WolfNode, (), Undirected>::new_undirected();
    let mut node_map = HashMap::new(); // Maps pack_id to graph node index

    // Add nodes for each unique pack
    for record in denning_data {
        if !node_map.contains_key(&record.pack_id) {
            let node = WolfNode {
                pack_id: record.pack_id,
                study: record.study.clone(),
                latitude: record.latitude_study,
                longitude: record.longitude_study,
            };
            let idx = graph.add_node(node);
            node_map.insert(record.pack_id, idx);
        }
    }

    // Group node indices by shared study area
    let mut study_map: HashMap<String, Vec<NodeIndex>> = HashMap::new();
    for record in denning_data {
        let idx = node_map[&record.pack_id];
        study_map.entry(record.study.clone()).or_default().push(idx);
    }

    // Fully connect nodes within each study area
    for nodes in study_map.values() {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                graph.add_edge(nodes[i], nodes[j], ());
            }
        }
    }

    graph
}

/// Computes degree centrality of each node in the graph.
///
/// # Arguments
/// * `graph` - Reference to the undirected graph
///
/// # Returns
/// * `HashMap<pack_id, degree>`
pub fn compute_degree_centrality(graph: &Graph<WolfNode, (), Undirected>) -> HashMap<u32, usize> {
    graph
        .node_indices()
        .map(|node| {
            let pack_id = graph[node].pack_id;
            let degree = graph.edges(node).count();
            (pack_id, degree)
        })
        .collect()
}

/// Prints the top N wolf packs ranked by degree centrality.
///
/// # Arguments
/// * `centrality` - Centrality scores
/// * `top_n` - Number of top entries to print
pub fn print_top_central_packs(centrality: &HashMap<u32, usize>, top_n: usize) {
    let mut ranked: Vec<_> = centrality.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nTop {} packs by degree centrality:", top_n);
    for (pack_id, degree) in ranked.into_iter().take(top_n) {
        println!("Pack {} → degree {}", pack_id, degree);
    }
}

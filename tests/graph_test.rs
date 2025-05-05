use wolf_project_210::graph::{build_graph, compute_degree_centrality};
use wolf_project_210::data::DenningPhenology;
use chrono::NaiveDate;


#[test]
fn test_build_graph() {
    let denning_data = mock_denning_data();
    let reproductive_data = mock_reproductive_data();

    
    assert!(graph.node_count() > 0, "Graph should have at least one node");
    assert!(graph.edge_count() >= 0, "Graph should have at least zero edges");
}

#[test]
fn test_compute_degree_centrality() {
    let denning_data = mock_denning_data();
    let graph = build_graph(&denning_data);
    let centrality = compute_degree_centrality(&graph);
    
    assert!(!centrality.is_empty(), "Centrality map should not be empty");
}

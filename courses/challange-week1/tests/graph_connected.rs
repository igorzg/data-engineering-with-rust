//! Task 4 tests: fully-connected check on undirected petgraph graphs.

use challange_week1::graph_connected::is_fully_connected;
use petgraph::graph::UnGraph;

fn undirected(nodes: usize, edges: &[(u32, u32)]) -> UnGraph<(), ()> {
    let mut graph = UnGraph::<(), ()>::new_undirected();
    for _ in 0..nodes {
        graph.add_node(());
    }
    for &(a, b) in edges {
        graph.add_edge(a.into(), b.into(), ());
    }
    graph
}

#[test]
fn empty_and_single_node_graphs_are_connected() {
    assert!(is_fully_connected(&undirected(0, &[])));
    assert!(is_fully_connected(&undirected(1, &[])));
}

#[test]
fn connected_graphs_return_true() {
    assert!(is_fully_connected(&undirected(4, &[(0, 1), (1, 2), (2, 3)])));
    assert!(is_fully_connected(&undirected(4, &[(0, 1), (1, 2), (2, 0), (2, 3)])));
}

#[test]
fn disconnected_graphs_return_false() {
    assert!(!is_fully_connected(&undirected(4, &[(0, 1), (2, 3)])));
    assert!(!is_fully_connected(&undirected(3, &[(0, 1)])));
    assert!(!is_fully_connected(&undirected(2, &[])));
}

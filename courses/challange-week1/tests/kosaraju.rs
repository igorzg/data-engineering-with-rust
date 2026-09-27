//! Task 5 tests: Kosaraju's strongly connected components.

use challange_week1::kosaraju::strongly_connected_components;
use petgraph::graph::{DiGraph, NodeIndex};

fn directed(nodes: usize, edges: &[(u32, u32)]) -> DiGraph<(), ()> {
    let mut graph = DiGraph::<(), ()>::new();
    for _ in 0..nodes {
        graph.add_node(());
    }
    for &(a, b) in edges {
        graph.add_edge(a.into(), b.into(), ());
    }
    graph
}

fn sorted_sizes(components: &[Vec<NodeIndex>]) -> Vec<usize> {
    let mut sizes: Vec<usize> = components.iter().map(Vec::len).collect();
    sizes.sort_unstable();
    sizes
}

#[test]
fn empty_graph_has_no_components() {
    assert!(strongly_connected_components(&directed(0, &[])).is_empty());
}

#[test]
fn acyclic_chain_yields_singleton_components() {
    let components = strongly_connected_components(&directed(3, &[(0, 1), (1, 2)]));
    assert_eq!(sorted_sizes(&components), vec![1, 1, 1]);
}

#[test]
fn cycle_is_one_component() {
    let components = strongly_connected_components(&directed(3, &[(0, 1), (1, 2), (2, 0)]));
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 3);
}

#[test]
fn chained_sccs_are_found() {
    // {0,1,2} -> {3,4} -> {5}
    let graph = directed(6, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 3), (4, 5)]);
    assert_eq!(sorted_sizes(&strongly_connected_components(&graph)), vec![1, 2, 3]);
}

#[test]
fn disjoint_cycles_are_separate_components() {
    let graph = directed(4, &[(0, 1), (1, 0), (2, 3), (3, 2)]);
    assert_eq!(sorted_sizes(&strongly_connected_components(&graph)), vec![2, 2]);
}

#[test]
fn self_loop_is_a_component_of_one() {
    let components = strongly_connected_components(&directed(1, &[(0, 0)]));
    assert_eq!(components, vec![vec![NodeIndex::new(0)]]);
}

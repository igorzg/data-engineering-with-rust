//! Task 5: Kosaraju's algorithm for strongly connected components (SCCs)
//! of a directed graph.
//!
//! Two nodes belong to the same SCC when each can reach the other. The
//! algorithm runs two depth-first passes:
//!
//! 1. **Finish order.** `DFS` on the graph, recording each node only *after*
//!    all nodes reachable from it have been recorded (post-order).
//! 2. **Component extraction.** Visit nodes in *decreasing* finish order
//!    while traversing the *transposed* graph. Walking each node's incoming
//!    edges is equivalent to walking the outgoing edges of the transpose,
//!    so the transpose is never materialised. Every tree rooted by this
//!    pass is exactly one SCC.

use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};

/// Runs Kosaraju's algorithm and returns the SCCs of `graph`.
///
/// Each inner `Vec` lists the member nodes of one strongly connected
/// component; the number of components equals the number of inner vectors.
#[must_use]
pub fn strongly_connected_components(graph: &DiGraph<(), ()>) -> Vec<Vec<NodeIndex>> {
    let node_count = graph.node_count();

    // Pass 1: post-order DFS on the original graph.
    let mut visited = vec![false; node_count];
    let mut finish_order: Vec<NodeIndex> = Vec::new();
    for node in graph.node_indices() {
        if !visited[node.index()] {
            record_finish_order(graph, node, &mut visited, &mut finish_order);
        }
    }

    // Pass 2: DFS on the transpose, in decreasing finish order.
    let mut in_component = vec![false; node_count];
    let mut components: Vec<Vec<NodeIndex>> = Vec::new();
    for &node in finish_order.iter().rev() {
        if !in_component[node.index()] {
            let mut component: Vec<NodeIndex> = Vec::new();
            extract_component(graph, node, &mut in_component, &mut component);
            components.push(component);
        }
    }

    components
}

/// First `DFS` pass: record `node` in `order` only after every node
/// reachable from it has been recorded (post-order = decreasing finish time).
fn record_finish_order(
    graph: &DiGraph<(), ()>,
    node: NodeIndex,
    visited: &mut [bool],
    order: &mut Vec<NodeIndex>,
) {
    visited[node.index()] = true;
    for next in graph.neighbors_directed(node, Direction::Outgoing) {
        if !visited[next.index()] {
            record_finish_order(graph, next, visited, order);
        }
    }
    order.push(node);
}

/// Second `DFS` pass: follow *incoming* edges (i.e. the transpose) and
/// collect everything reachable from `node` into `component`.
fn extract_component(
    graph: &DiGraph<(), ()>,
    node: NodeIndex,
    in_component: &mut [bool],
    component: &mut Vec<NodeIndex>,
) {
    in_component[node.index()] = true;
    component.push(node);
    for next in graph.neighbors_directed(node, Direction::Incoming) {
        if !in_component[next.index()] {
            extract_component(graph, next, in_component, component);
        }
    }
}

//! Task 4: determine whether a graph (petgraph, undirected) is fully
//! connected, i.e. every pair of nodes is joined by a path.

use petgraph::graph::Graph;
use petgraph::{Direction, Undirected};

/// Returns `true` if every node in `graph` is reachable from every other node.
///
/// The check runs a single depth-first search from an arbitrary node and
/// verifies that all nodes were visited. A graph with zero or one node is
/// connected by convention.
#[must_use]
pub fn is_fully_connected<N, E>(graph: &Graph<N, E, Undirected>) -> bool {
    let Some(root) = graph.node_indices().next() else {
        return true;
    };

    let mut visited = vec![false; graph.node_count()];
    let mut stack = vec![root];
    visited[root.index()] = true;

    while let Some(node) = stack.pop() {
        for neighbour in graph.neighbors_directed(node, Direction::Outgoing) {
            if !visited[neighbour.index()] {
                visited[neighbour.index()] = true;
                stack.push(neighbour);
            }
        }
    }

    visited.iter().all(|&seen| seen)
}

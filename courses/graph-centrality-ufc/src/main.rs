//!
//! Represents a fighter in a graph.
//!
//! # Fields
//!
//! * `name` - The name of the fighter as a `String`.
use std::fmt;
use petgraph::{Direction, EdgeDirection};
use petgraph::graph::{UnGraph, NodeIndex};

/// Represents a fighter in a combat scenario.
///
/// # Fields
///
/// * `name` - A `String` that holds the name of the fighter.
///
/// # Examples
///
/// ```
/// let fighter = Fighter {
///     name: String::from("Goku"),
/// };
/// println!("{:?}", fighter);
/// ```
#[derive(Debug)]
struct Fighter {
    name: String
}

impl Fighter {
    fn new(name: &str) -> Fighter {
        Fighter{
            name: name.to_string()
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Adds an edge between two specified nodes in the given graph with a weight of 1.0.
///
/// # Arguments
///
/// * `graph` - A mutable reference to an undirected graph (`UnGraph`) where each node holds a reference to a `Fighter`.
/// * `nodes` - A slice of node indices that are already present in the graph.
/// * `a` - The index of the first node in the `nodes` slice.
/// * `b` - The index of the second node in the `nodes` slice.
///
/// # Example
///
/// ```
/// use petgraph::prelude::*;
///
/// let mut graph = UnGraph::<&Fighter, f32>::new();
/// let fighter1 = Fighter { name: "John" };
/// let fighter2 = Fighter { name: "Jane" };
///
/// let node_a = graph.add_node(&fighter1);
/// let node_b = graph.add_node(&fighter2);
///
/// add_edge(&mut graph, &[node_a, node_b], 0, 1);
///
/// assert_eq!(graph.edge_count(), 1);
/// ```
///
/// # Panics
///
/// This function will panic if `a` or `b` are out of bounds for the `nodes` slice.
///
/// # Notes
///
/// Ensure that the nodes at indices `a` and `b` exist in the graph before calling this function.
fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

/// The main function to demonstrate the calculation of closeness centrality in an undirected graph.
///
///
/// # Dependencies
/// This code assumes the existence of a `Fighter` struct with a `name` field and an `add_edge` function that adds edges to the graph.
/// The `UnGraph` type and related functionality are part of the `petgraph` crate, which must be included in the project's dependencies.
///
/// # Example Output
/// ```
/// The closeness centrality of Dustin Poirier is 0.50
/// Dustin Poirier has a centrality of 0.50, implying they had less fights compared to Conor McGregor
/// --------------------
/// The closeness centrality of Khabib Nurmagomedov is 0.33
/// Khabib Nurmagomedov has a highest centrality of 0.33 as they have fought with the least number
/// --------------------
/// The closeness centrality of Jose Aldo is 0.33
/// Jose Aldo has a highest centrality of 0.33 as they have fought with the least number
/// --------------------
/// The closeness centrality of Conor McGregor is 0.25
/// Conor McGregor has the lowest centrality because he has fought with all other fighters in the group
/// --------------------
/// The closeness centrality of Nate Diaz is 0.50
/// Nate Diaz has a centrality of 0.50, implying they had less fights compared to Conor McGregor
/// --------------------
/// ```
///
/// # Usage
/// To run this program, ensure you have the `petgraph` crate included in your project's `Cargo.toml` file.
/// Then, execute the program using `cargo run`.
fn main() {
    let mut graph = UnGraph::new_undirected();
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz")
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter: &Fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 3, 0);
    add_edge(&mut graph, &fighter_nodes, 3, 2);
    add_edge(&mut graph, &fighter_nodes, 3, 4);
    add_edge(&mut graph, &fighter_nodes, 0, 4);
    add_edge(&mut graph, &fighter_nodes, 2, 4);

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness  = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the group",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor",
                name,
                closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has a highest centrality of {:.2} as they have fought with the least number",
                name,
                closeness
            ),
            _ => {}
        }
        println!("--------------------");
    }
}

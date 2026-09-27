use textwrap::fill;

struct PageRank {
    damping: f64,
    iterations: usize
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self { damping, iterations }
    }

    /// Calculates the PageRank of each node in a directed graph.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the struct containing the damping factor and number of iterations.
    /// * `graph` - A reference to a vector of vectors representing the directed graph. Each inner vector contains indices of nodes that the corresponding node has a link to.
    ///
    /// # Returns
    ///
    /// * A vector of f64 values representing the PageRank of each node in the graph.
    ///
    /// # Description
    ///
    /// This function computes the PageRank for all nodes in a given directed graph using an iterative approach. The graph is represented as an adjacency list where each node points to a list of other nodes it has a link to. The function initializes the ranks of all nodes equally and iteratively updates them based on the contributions from their incoming links, adjusted by the damping factor. After a specified number of iterations, it returns the final PageRank values for each node.
    ///
    /// # Example
    ///
    /// ```
    /// let graph = vec![
    ///     vec![1, 2], // Node 0 points to nodes 1 and 2
    ///     vec![2],    // Node 1 points to node 2
    ///     vec![],     // Node 2 has no outgoing links
    /// ];
    /// let pagerank = PageRank::new(0.85, 10);
    /// let ranks = pagerank.rank(&graph);
    /// println!("{:?}", ranks);
    /// ```
    ///
    /// # Note
    ///
    /// The damping factor is typically set between 0.8 and 0.9, with 0.85 being a common choice. It represents the probability of continuing along the random walk versus jumping to a random node in the graph.
    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        let n = graph.len();
        let mut ranks = vec![1.0 / (n as f64); n];
        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; n];

            for (node, edges) in graph.iter().enumerate() {
                let contribution = ranks[node] / (edges.len() as f64);
                for &edge in edges {
                    new_ranks[edge] += contribution
                }
            }

            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }
            ranks = new_ranks;
        }
        ranks
    }
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![0],
        vec![0, 3],
        vec![0],
        vec![0, 1]
    ];

    let names = vec!["ESPN", "NFN", "NBA", "UFC", "MLB"];

    let pagerank = PageRank::new(0.85, 100);

    let ranks = pagerank.rank(&graph);

    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRanke of {} is {:.8}", names[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page";

    println!("{}", fill(explanation, 78));
}

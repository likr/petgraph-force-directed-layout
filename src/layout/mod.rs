use petgraph::graph::Graph;
use super::graph::{Node, Edge};

pub fn layout(graph: Graph<Node, Edge>) {
    println!("{} {}", graph.node_count(), graph.edge_count());
}

extern crate adapton;

use std::collections::BinaryHeap;
use std::usize;

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn shortest_path(&self, source: NodeIndex, target: NodeIndex) -> Option<NodeIndex> {
        let mut dist_map: Vec<_> = (0..self.edges.len() + 1).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        dist_map[source] = 0;
        heap.push((0, source));

        while let Some((cost, position)) = heap.pop() {
            if position == target {
                return Some(cost);
            }

            if cost > dist_map[position] {
                continue;
            }

            let successors: Vec<NodeIndex> = self.successors(position).collect();
            for &neighbor in &successors[..] {
                if cost + 1 < dist_map[neighbor] {
                    heap.push((cost + 1, neighbor));
                    dist_map[neighbor] = cost + 1;
                }
            }
        }

        None
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors() {
        let mut graph = Graph::new();

        let n0 = graph.add_node();
        let n1 = graph.add_node();

        graph.add_edge(n0, n1);
        graph.add_edge(n1, n0);

        let successors: Vec<NodeIndex> = graph.successors(n0).collect();
        assert_eq!(&successors[..], &[n1]);
    }

    #[test]
    fn shortest_path() {
        let mut graph = Graph::new();

        let n0 = graph.add_node();
        let n1 = graph.add_node();
        let n2 = graph.add_node();

        graph.add_edge(n0, n2);
        graph.add_edge(n0, n1);
        graph.add_edge(n1, n2);

        let shortest_path_len = graph.shortest_path(n0, n2);
        assert_eq!(shortest_path_len, Some(1));
    }

    #[test]
    fn no_path() {
        let mut graph = Graph::new();

        let n0 = graph.add_node();
        let n1 = graph.add_node();

        let path_len_opt = graph.shortest_path(n0, n1);
        assert_eq!(path_len_opt, None);
    }
}

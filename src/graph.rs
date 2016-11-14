#![feature(associated_type_defaults)]
extern crate adapton;

use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use self::adapton::adapton_sigs::*;


// pub trait GraphT<A: Adapton,
//                  Node: Hash + Eq + Clone + Debug,
//                  NodeSet: SetT<A, Node> + Clone + Hash,
//                  NodeMap: MapT<A, Node, NodeSet>> {
//     type Graph: Hash + Eq + Clone + Debug;

//     // Intro forms:
//     fn empty(st: &mut A);
//     fn add_node(st: &mut A, graph: Self::Graph, node: Node) -> Self::Graph;
//     fn add_edge(st: &mut A, graph: Self::Graph, src: Node, tgt: Node) -> Self::Graph;
//     fn add_succs(st: &mut A, graph: Self::Graph, node: Node, succs: NodeSet) -> Self::Graph;

//     // Query forms:
//     fn get_succs(st: &mut A, graph: Self::Graph, node: Node) -> NodeSet;

//     // Other forms:
//     // fn rem_node(st:&mut A, graph:Self::Graph, node:NodeLab) -> Self::Graph;
//     // fn rem_edge(st:&mut A, graph:Self::Graph, edge:Self::Edge) -> Self::Graph;
// }

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub enum MyGraph<A: Adapton, Node> {
    G(Vec<Node>, Vec<(Node, Node)>),
    Rc(Rc<MyGraph<A, Node>>),
    Name(A::Name, Vec<Node>, Vec<(Node, Node)>),
    Art(Art<MyGraph<A, Node>, A::Loc>),
    FAIL,
}

pub trait MyGraphT<A: Adapton, Node: Hash + Eq + Clone + Debug> {
    type Graph: Hash + Eq + Clone + Debug;

    fn empty(st: &mut A) -> Self::Graph;

    fn add_node(st: &mut A, graph: Self::Graph, node: Node) -> Self::Graph;
    fn add_edge(st: &mut A, graph: Self::Graph, source: Node, target: Node) -> Self::Graph;
}

impl<A:Adapton+Debug+Hash+PartialEq+Eq+Clone
     ,Node:Debug+Hash+PartialEq+Eq+Clone>
    MyGraphT<A, Node>
    for MyGraph<A,Node> {

        type Graph = MyGraph<A,Node>;

        fn empty(st:&mut A) -> Self::Graph { MyGraph::G(Vec::new(), Vec::new()) }

        fn add_node(st: &mut A, graph: Self::Graph, node: Node) -> Self::Graph {
            match graph {
                MyGraph::G(mut nodes, edges) => {
                    nodes.push(node);
                    MyGraph::G(nodes, edges)
                },
                MyGraph::Rc(rc) =>
                    Self::add_node(st, (*rc).clone(), node),
                MyGraph::Name(nm, nodes, edges) =>
                    MyGraph::FAIL,
                MyGraph::Art(ref art) => {
                    let graph = st.force(art);
                    Self::add_node(st, graph, node)
                },
                MyGraph::FAIL => MyGraph::FAIL,
            }
        }

        fn add_edge(st: &mut A, graph: Self::Graph, source: Node, target: Node) -> Self::Graph {
// match graph {
//     MyGraph::G(nodes, edges) => {

//     }
// }
            MyGraph::FAIL
        }

    }

#[cfg(test)]
mod test {
    #[test]
    fn compiles() {}
}

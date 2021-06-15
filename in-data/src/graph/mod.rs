pub mod algo;
pub mod edge;
pub mod node;

use std::{cmp::max, fmt::{Write, Debug, self}, collections::BTreeMap, marker::PhantomData};
use std::ops::Deref;
use chrono::{DateTime, Utc};
use actix::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EdgeIdx(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeIdx(usize);

impl NodeIdx {
    pub fn max() -> Self { NodeIdx(usize::MAX) }

    pub fn idx(self) -> usize { self.0 }
}
impl EdgeIdx {
    pub fn max() -> Self { EdgeIdx(usize::MAX) }

    pub fn idx(self) -> usize { self.0 }
}

impl Default for NodeIdx {
    fn default() -> Self {
        Self::max()
    }
}
impl Default for EdgeIdx {
    fn default() -> Self {
        Self::max()
    }
}

#[derive(Clone, Debug)]
pub struct Node<N> {
    pub data: N,
    /// Next outgoing connected edge
    pub outgoing_next: Option<EdgeIdx>,
    /// Next incoming connected edge
    pub incoming_next: Option<EdgeIdx>,
}

#[derive(Clone, Debug)]
pub struct Edge<E> {
    pub data: E,
    pub start_next: Option<EdgeIdx>,
    pub end_next: Option<EdgeIdx>,
    pub start: NodeIdx,
    pub end: NodeIdx,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GraphKind {
    Directed,
    Undirected
}

#[derive(Clone)]
pub struct Graph<N, E>
where
    N: Clone
{
    kind: GraphKind,
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<E> Edge<E> {
    pub fn new_dir(data: E, start: NodeIdx, end: NodeIdx) -> Self {
        Edge {
            data,
            start, end,
            start_next: None,
            end_next: None,
        }
    }
    pub fn add_next_edges<N>(&mut self, start: &Node<N>, end: &Node<N>) {
        self.start_next = start.outgoing_next;
        self.end_next = end.incoming_next;
    }
    pub fn set_next_start_edge(&mut self, idx: EdgeIdx) {
        self.start_next = Some(idx);
    }
    pub fn set_next_end_edge(&mut self, idx: EdgeIdx) {
        self.end_next = Some(idx);
    }
}
impl<N> Node<N> {
    pub fn new(data: N) -> Self {
        Node { data, outgoing_next: None, incoming_next: None }
    }
    pub fn set_next_outgoing(&mut self, idx: EdgeIdx) {
        self.outgoing_next = Some(idx);
    }
    pub fn set_next_incoming(&mut self, idx: EdgeIdx) {
        self.incoming_next = Some(idx);
    }
}

impl<N, E> Graph<N, E>
where
    N: Clone
{
    pub fn new(kind: GraphKind) -> Self {
        Self { kind, nodes: Vec::new(), edges: Vec::new() }
    }
    pub fn with_capacity(kind: GraphKind, nodes: usize, edges: usize) -> Self {
        Self { kind, nodes: Vec::with_capacity(nodes), edges: Vec::with_capacity(edges) }
    }
    pub fn edge_num(&self) -> usize {
        self.edges.len()
    }
    pub fn node_num(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node_data(&self, idx: NodeIdx) -> &N {
        &self.nodes[idx.idx()].data
    }

    pub fn insert_node(&mut self, data: N) -> NodeIdx {
        let n = Node::new(data);
        let idx = NodeIdx(self.nodes.len());
        self.nodes.push(n);
        return idx;
    }

    pub fn insert_edge(&mut self, data: E, start: NodeIdx, end: NodeIdx, ) -> EdgeIdx {
        let idx = self.edges.len();
        let mut e = Edge::new_dir(data, start, end);
        if max(start.idx(), end.idx()) > self.nodes.len() {
            panic!("Graph::insert_edge: Provided (start, end) indices out of range.");
        } else if start == end {
            let mut n = &mut self.nodes[start.idx()];
            e.add_next_edges(&n, &n);
            n.set_next_outgoing(EdgeIdx(idx));
            n.set_next_incoming(EdgeIdx(idx));
        } else {
            let st = &mut self.nodes[start.idx()];
            st.set_next_outgoing(EdgeIdx(idx));
            let st_idx = st.outgoing_next;
            drop(st);
            let en = &mut self.nodes[end.idx()];
            en.set_next_incoming(EdgeIdx(idx));
            let en_idx = en.incoming_next;
            drop(en);
            if let Some(st_idx) = st_idx {
                e.set_next_start_edge(st_idx);
            }
            if let Some(en_idx) = en_idx {
                e.set_next_end_edge(en_idx);
            }
        }
        self.edges.push(e);
        EdgeIdx(self.edges.len())
    }

    pub fn set_edge(&mut self, start: NodeIdx, end: NodeIdx, data: E) -> EdgeIdx {
        if let Some(idx) = self.get_edge(start, end) {
            EdgeIdx(0)
        } else {
            EdgeIdx(0)
        }
    }

    pub fn get_node(&self, idx: NodeIdx) -> Option<&Node<N>> {
        if idx.idx() < self.nodes.len() {
            let n = &self.nodes[idx.idx()];
            return Some(n);
        } else {
            return None;

        }
    }

    pub fn get_edge(&self, st: NodeIdx, en: NodeIdx) -> Option<EdgeIdx> {
        match self.kind {
            GraphKind::Directed => if let Some(st_node) = self.nodes.get(st.idx()) {
                if let Some(next_outgoing_edge) = st_node.outgoing_next {
                    let mut e_idx = next_outgoing_edge;
                    while let Some(e) = self.edges.get(next_outgoing_edge.idx()) {
                        let e = &e;
                        if e.end == en {
                            return Some(e_idx);
                        }
                        if let Some(e_next_idx) = e.start_next {
                            e_idx = e_next_idx;
                        } else {
                            return None;
                        }
                    }
                    return None;
                } else {
                    return None;
                }
            } else {
                return None;
            }
            GraphKind::Undirected => self.get_edge_undirected(st, en),
        }
    }

    pub fn get_edge_undirected(&self, st: NodeIdx, en: NodeIdx) -> Option<EdgeIdx> {
        if let Some(st_node) = self.nodes.get(st.idx()) {
            if let Some(e_out) = st_node.outgoing_next {
                let mut e_idx = e_out;
                while let Some(e) = self.edges.get(e_idx.idx()) {
                    let e = &e;
                    if e.end == en  {
                        return Some(e_idx)
                    }
                    if let Some(e_next_idx) = e.start_next {
                        e_idx = e_next_idx;
                    } else {
                        return None
                    }

                }
            }
            if let Some(e_in) = st_node.incoming_next {
                let mut e_idx = e_in;
                while let Some(e) = self.edges.get(e_idx.idx()) {
                    let e = &e;
                    if e.start == en {
                        return Some(e_idx)
                    }
                    // TODO implement linked list structure and iterator for edge indices?
                    if let Some(e_next_idx) = e.end_next {
                        e_idx = e_next_idx;
                    } else {
                        return None
                    }
                }
            }
            return None;
        }
        return None;
    }

    pub fn get_neighbors(self, idx: NodeIdx) -> Vec<NodeIdx> {
        let n = &self.nodes[idx.idx()];
        let mut neighbors: Vec<NodeIdx> = Vec::new();
        neighbors
    }

    pub fn nodes(self) -> Vec<Node<N>> {
        self.nodes
    }

    pub fn edges(self) -> Vec<Edge<E>> {
        self.edges
    }
}

impl<N, E> fmt::Display for Graph<N, E>
where
    N: fmt::Debug + Clone,
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (node_no, edge_no) = (self.nodes.len(), self.edges.len());
        f.write_fmt(format_args!("directed: {:?}", &self.kind))?;
        f.write_fmt(format_args!("node_no: {}", &node_no.to_string()))?;
        f.write_fmt(format_args!("edge_no: {}", &edge_no.to_string()))?;
        f.write_fmt(format_args!("node_data: {:?}", &self.nodes.iter().map(|n| &n.data).collect::<Vec<&N>>()))?;
        f.write_fmt(format_args!("edge_data: {:?}", &self.edges.iter().map(|e| &e.data).collect::<Vec<&E>>()))?;
        Ok(())
    }
}

impl<N, E> fmt::Debug for Graph<N, E>
where
    N: fmt::Debug + Clone,
    E: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut g = f.debug_struct("Graph");
        let (node_no, edge_no) = (self.nodes.len(), self.edges.len());
        g.field("directed", &self.kind);
        g.field("node_no", &node_no.to_string());
        g.field("edge_no", &edge_no.to_string());
        g.field("node_data", &self.nodes.iter().map(|n| &n.data));
        g.field("edge_data", &self.edges.iter().map(|e| &e.data));
        Ok(())
    }
}



#[cfg(test)]
mod tests {

}

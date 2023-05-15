use std::collections::{HashSet};
use crate::edge::EdgeKey;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct NodeKey {
    pub version: String,
    pub state: String,
}

impl NodeKey {
    pub fn new(version: &str, state: &str) -> NodeKey {
        NodeKey {
            version: version.to_string(),
            state: state.to_string(),
        }
    }
}

pub struct Node {
    pub id: usize,
    pub key: NodeKey,
    pub uids: HashSet<String>,
    pub edge_ids_out: HashSet<EdgeKey>,
    pub edge_ids_in: HashSet<EdgeKey>,
}


impl Node {

    pub fn new(id: usize, key: NodeKey, uids: HashSet<String>) -> Node {
        Node {
            id,
            key,
            uids,
            edge_ids_out: HashSet::new(),
            edge_ids_in: HashSet::new(),
        }
    }

    pub fn add_edge_out(&mut self, edge: &EdgeKey) {
        self.edge_ids_out.insert(edge.clone());
    }

    pub fn add_edge_in(&mut self, edge: &EdgeKey) {
        self.edge_ids_in.insert(edge.clone());
    }
}
use std::collections::{HashSet};

use crate::node::NodeKey;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct EdgeKey {
    pub from_node: NodeKey,
    pub to_node: NodeKey,
}

impl EdgeKey {
    pub fn new(from_node: &NodeKey, to_node: &NodeKey) -> EdgeKey {
        EdgeKey {
            from_node: from_node.clone(),
            to_node: to_node.clone(),
        }
    }
}

pub struct Edge {
    pub id: usize,
    pub from_node: usize,
    pub to_node: usize,
    pub uids: HashSet<String>,
}

impl Edge {
    pub fn new(id: usize, from_node: usize, to_node: usize, uids: HashSet<String>) -> Edge {
        Edge {
            id,
            from_node,
            to_node,
            uids,
        }
    }
}



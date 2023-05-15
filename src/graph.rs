use std::collections::{HashMap, HashSet};
use crate::edge::{Edge, EdgeKey};
use crate::node::{Node, NodeKey};


pub struct Graph {
    pub node_key_to_id: HashMap<NodeKey, usize>,
    pub edge_key_to_id: HashMap<EdgeKey, usize>,

    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {

    pub fn new() -> Graph {
        Graph {
            node_key_to_id: HashMap::new(),
            edge_key_to_id: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn get_node(&self, key: &NodeKey) -> Option<&Node> {
        let node_id = self.node_key_to_id.get(key);
        match node_id {
            Some(id) => Some(&self.nodes[*id]),
            None => None,
        }
    }

    pub fn get_node_mut(&mut self, key: &NodeKey) -> &mut Node {
        let node_id = self.node_key_to_id.get(key);
        match node_id {
            Some(id) => &mut self.nodes[*id],
            None => panic!("Node does not exist"),
        }
    }

    pub fn get_edge_mut(&mut self, key: &EdgeKey) -> &mut Edge {
        let edge_id = self.edge_key_to_id.get(key);
        match edge_id {
            Some(id) => &mut self.edges[*id],
            None => panic!("Edge does not exist"),
        }
    }

    pub fn add_node(&mut self, key: NodeKey, uids: HashSet<String>) {
        let existing_node = self.get_node(&key);

        // Check if this node already exists
        if let Some(..) = existing_node {
            if existing_node.unwrap().uids != uids {
                panic!("Node already exists with different attributes")
            } else {
                return;
            }
        }

        // Create the node
        let node_id = self.nodes.len();
        self.nodes.push(Node::new(node_id, key.clone(), uids));
        self.node_key_to_id.insert(key.clone(), node_id);
    }

    pub fn get_edge(&self, edge_key: &EdgeKey) -> Option<&Edge> {
        let edge_id = self.edge_key_to_id.get(edge_key);
        match edge_id {
            Some(id) => Some(&self.edges[*id]),
            None => None,
        }
    }

    pub fn add_edge(&mut self, from_key: &NodeKey, to_key: &NodeKey, uids: HashSet<String>) {
        let edge_key = EdgeKey::new(from_key, to_key);
        let existing_edge = self.get_edge(&edge_key);

        // Check if this edge already exists
        if let Some(..) = existing_edge {
            if existing_edge.unwrap().uids != uids {
                panic!("Edge already exists with different attributes")
            } else {
                return;
            }
        }

        // Create the edge
        let from_node = self.get_node(from_key);
        let to_node = self.get_node(to_key);
        if from_node.is_none() || to_node.is_none() {
            panic!("Cannot create edge between non-existent nodes");
        }
        let from_node = from_node.unwrap();
        let to_node = to_node.unwrap();

        let edge_id = self.edges.len();
        self.edges.push(Edge::new(edge_id, from_node.id, to_node.id, uids));
        self.edge_key_to_id.insert(edge_key.clone(), edge_id);

        let from_node = self.get_node_mut(from_key);
        from_node.add_edge_out(&edge_key);

        let to_node = self.get_node_mut(to_key);
        to_node.add_edge_in(&edge_key);
    }
}
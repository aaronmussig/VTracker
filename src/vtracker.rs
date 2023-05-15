use std::collections::{HashMap, HashSet};
use std::string::ToString;

use crate::edge::EdgeKey;
use crate::graph::Graph;
use crate::model::{SankeyD3, SankeyLink, SankeyNode};
use crate::node::NodeKey;

pub struct VTracker {
    pub ver_to_idx: HashMap<String, usize>,
    pub idx_to_ver: Vec<String>,
    pub graph: Graph,

    pub uid_to_node: HashMap<String, HashSet<NodeKey>>,
    pub uid_to_edge: HashMap<String, HashSet<EdgeKey>>,

    pub str_na: String,
}

impl VTracker {
    pub fn new(versions: &Vec<String>, str_na: Option<&String>) -> VTracker {

        // Convert the versions into keys
        let mut ver_to_idx: HashMap<String, usize> = HashMap::new();
        for version in versions {
            ver_to_idx.insert(version.to_string(), ver_to_idx.len());
        }
        let idx_to_ver: Vec<String> = versions.clone();

        let str_not_available = match str_na {
            Some(str_na) => str_na.to_string(),
            None => "Not Present".to_string(),
        };

        VTracker {
            ver_to_idx,
            idx_to_ver,
            graph: Graph::new(),
            uid_to_node: HashMap::new(),
            uid_to_edge: HashMap::new(),
            str_na: str_not_available,
        }
    }

    fn add_uid_to_node(&mut self, uid: &str, key: &NodeKey) {
        let cur_set = self.uid_to_node.get_mut(uid);
        if cur_set.is_none() {
            self.uid_to_node.insert(uid.to_string(), HashSet::new());
        }
        self.uid_to_node.get_mut(uid).unwrap().insert(key.clone());
    }

    fn add_uid_to_edge(&mut self, uid: &str, key: &EdgeKey) {
        let cur_set = self.uid_to_edge.get_mut(uid);
        if cur_set.is_none() {
            self.uid_to_edge.insert(uid.to_string(), HashSet::new());
        }
        self.uid_to_edge.get_mut(uid).unwrap().insert(key.clone());
    }

    fn add_nodes(&mut self, uid: &str, ver_states: &HashMap<String, String>) -> Vec<NodeKey> {
        let mut out: Vec<NodeKey> = Vec::new();
        for version in &self.idx_to_ver {

            // Check if this uid appears in this version
            let key = match ver_states.get(version) {
                Some(state) => NodeKey::new(version, state),
                None => NodeKey::new(version, &self.str_na),
            };

            if self.graph.get_node(&key).is_none() {
                let uids: HashSet<String> = HashSet::from_iter(vec![uid.to_string()]);
                self.graph.add_node(key.clone(), uids);
            } else {
                let node = self.graph.get_node_mut(&key);
                node.uids.insert(uid.to_string());
            }
            out.push(key);
        }
        out
    }

    fn add_edges(&mut self, uid: &str, ver_states: &HashMap<String, String>) -> Vec<EdgeKey> {
        let mut out: Vec<EdgeKey> = Vec::new();
        for i in 0..&self.idx_to_ver.len() - 1 {
            let ver_from = &self.idx_to_ver[i];
            let ver_to = &self.idx_to_ver[i + 1];

            // Check if this uid appears in this or the next version
            let key_from = match ver_states.get(ver_from) {
                Some(k) => NodeKey::new(ver_from, k),
                None => NodeKey::new(ver_from, &self.str_na),
            };
            let key_to = match ver_states.get(ver_to) {
                Some(k) => NodeKey::new(ver_to, k),
                None => NodeKey::new(ver_to, &self.str_na),
            };

            // Create the edge associated with this key
            let edge_key = EdgeKey::new(&key_from, &key_to);

            if self.graph.get_edge(&edge_key).is_none() {
                let uids: HashSet<String> = HashSet::from_iter(vec![uid.to_string()]);
                self.graph.add_edge(&key_from, &key_to, uids);
            } else {
                let edge = self.graph.get_edge_mut(&edge_key);
                edge.uids.insert(uid.to_string());
            }
            out.push(edge_key);
        }
        out
    }

    pub fn add(&mut self, uid: &str, ver_states: &HashMap<String, String>) {
        let ver_states_keys: HashSet<&String> = HashSet::from_iter(ver_states.keys());
        let ver_to_idx_keys: HashSet<&String> = HashSet::from_iter(self.ver_to_idx.keys());

        // Sanity checking
        if ver_states_keys.difference(&ver_to_idx_keys).count() > 0 {
            panic!("Specified version which is not a part of this tracker.");
        }
        if self.uid_to_node.contains_key(uid) {
            panic!("Specified uid already exists in this tracker.");
        }

        // Iterate over each expected version
        let node_keys_to_add = self.add_nodes(uid, &ver_states);
        for node_key in &node_keys_to_add {
            self.add_uid_to_node(uid, node_key);
        }

        // Create each of the edges
        let edge_keys_to_add: Vec<EdgeKey> = self.add_edges(uid, &ver_states);
        for edge_key in &edge_keys_to_add {
            self.add_uid_to_edge(uid, edge_key);
        }
    }

    pub fn build_uid_paths(&self) -> (HashMap<String, HashSet<usize>>, HashMap<String, HashSet<usize>>) {
        let mut edges: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut nodes: HashMap<String, HashSet<usize>> = HashMap::new();

        for uid in self.uid_to_node.keys() {
            for edge_key in self.uid_to_edge.get(uid).unwrap() {
                let edge = self.graph.get_edge(edge_key).unwrap();
                if edges.get(uid).is_none() {
                    edges.insert(uid.to_string(), HashSet::new());
                }
                edges.get_mut(uid).unwrap().insert(edge.id);
            }

            for node_key in self.uid_to_node.get(uid).unwrap() {
                let node = self.graph.get_node(node_key).unwrap();
                if nodes.get(uid).is_none() {
                    nodes.insert(uid.to_string(), HashSet::new());
                }
                nodes.get_mut(uid).unwrap().insert(node.id);
            }
        }
        (nodes, edges)
    }

    pub fn get_sankey_nodes(&self, uid_travel_node_id: &HashMap<String, HashSet<usize>>, uid_travel_edge_id: &HashMap<String, HashSet<usize>>) -> Vec<SankeyNode> {
        let mut out_nodes: Vec<SankeyNode> = Vec::new();

        // Step 2: Calculate link highlighting
        for node in &self.graph.nodes {
            let ver = &node.key.version;
            let state = &node.key.state;

            let mut node_highlight_id: HashSet<usize> = HashSet::new();
            let mut link_highlight_id: HashSet<usize> = HashSet::new();

            for uid in node.uids.iter() {
                for node_uid in uid_travel_node_id.get(uid).unwrap() {
                    node_highlight_id.insert(*node_uid);
                }
                for edge_uid in uid_travel_edge_id.get(uid).unwrap() {
                    link_highlight_id.insert(*edge_uid);
                }
            }

            let mut node_highlight_id_sorted: Vec<usize> = Vec::from_iter(node_highlight_id);
            node_highlight_id_sorted.sort();

            let mut link_highlight_id_sorted: Vec<usize> = Vec::from_iter(link_highlight_id);
            link_highlight_id_sorted.sort();

            out_nodes.push(SankeyNode {
                col: ver.to_string(),
                id: node.id,
                link_highlight_id: Vec::from_iter(link_highlight_id_sorted),
                name: state.to_string(),
                node_highlight_id: Vec::from_iter(node_highlight_id_sorted),
                total: node.uids.len(),
            });
        }

        // Sort by id
        out_nodes.sort_by(|a, b| a.id.cmp(&b.id));
        out_nodes
    }

    pub fn get_sankey_edges(&self, uid_travel_node_id: &HashMap<String, HashSet<usize>>, uid_travel_edge_id: &HashMap<String, HashSet<usize>>) -> Vec<SankeyLink> {
        let mut out: Vec<SankeyLink> = Vec::new();

        // Create each of the edges in the sankey
        for edge in &self.graph.edges {
            let mut node_highlight_id: HashSet<usize> = HashSet::new();
            let mut link_highlight_id: HashSet<usize> = HashSet::new();

            for uid in &edge.uids {
                for node_uid in uid_travel_node_id.get(uid).unwrap() {
                    node_highlight_id.insert(*node_uid);
                }
                for edge_uid in uid_travel_edge_id.get(uid).unwrap() {
                    link_highlight_id.insert(*edge_uid);
                }
            }

            let mut node_highlight_id_sorted: Vec<usize> = Vec::from_iter(node_highlight_id);
            node_highlight_id_sorted.sort();

            let mut link_highlight_id_sorted: Vec<usize> = Vec::from_iter(link_highlight_id);
            link_highlight_id_sorted.sort();

            out.push(SankeyLink {
                id: edge.id,
                link_highlight_id: Vec::from_iter(link_highlight_id_sorted),
                node_highlight_id: Vec::from_iter(node_highlight_id_sorted),
                source: edge.from_node,
                target: edge.to_node,
                value: edge.uids.len(),
            })
        }

        // Sort by id
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }

    pub fn as_sankey_json(&self) -> SankeyD3 {

        // Compute the uid paths
        let (uid_travel_node_id, uid_travel_edge_id) = self.build_uid_paths();

        let out_nodes = self.get_sankey_nodes(&uid_travel_node_id, &uid_travel_edge_id);
        let out_edges = self.get_sankey_edges(&uid_travel_node_id, &uid_travel_edge_id);

        let out = SankeyD3 {
            nodes: out_nodes,
            links: out_edges,
        };

        out
    }
}


#[test]
fn test_v_tracker_new() {
    let versions: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let vt = VTracker::new(&versions, None);

    let mut ver_to_idx_expected: HashMap<String, usize> = HashMap::new();
    ver_to_idx_expected.insert("a".to_string(), 0);
    ver_to_idx_expected.insert("b".to_string(), 1);
    ver_to_idx_expected.insert("c".to_string(), 2);
    assert_eq!(vt.ver_to_idx, ver_to_idx_expected);

    let idx_to_ver_expected: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(vt.idx_to_ver, idx_to_ver_expected);

    assert_eq!(vt.uid_to_node.len(), 0);
    assert_eq!(vt.uid_to_edge.len(), 0);
}

fn test_init_v_tracker() -> VTracker {
    /*
        +--------------+--------+--------------+
       |     1        |   2    |      3       |
       +--------------+--------+--------------+
       | a: x         | a: x   | a: y         |
       |              | b: y,z |              |
       | Missing: y,z |        | Missing: x,z |
       +--------------+--------+--------------+
    */

    let versions: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut vt = VTracker::new(&versions, None);

    // Add x
    {
        let states = HashMap::from_iter(vec![
            ("1".to_string(), "a".to_string()),
            ("2".to_string(), "a".to_string()),
        ]);
        vt.add("x", &states);
    }

    // Add y
    {
        let states = HashMap::from_iter(vec![
            ("2".to_string(), "b".to_string()),
            ("3".to_string(), "a".to_string()),
        ]);
        vt.add("y", &states);
    }

    // Add z
    {
        let states = HashMap::from_iter(vec![
            ("2".to_string(), "b".to_string()),
        ]);
        vt.add("z", &states);
    }

    vt
}

#[test]
fn test_v_tracker_add() {
    let vt = test_init_v_tracker();

    // Create NodeKeys for later use
    let n_1a_key = NodeKey::new("1", "a");
    let n_2a_key = NodeKey::new("2", "a");
    let n_2b_key = NodeKey::new("2", "b");
    let n_1x_key = NodeKey::new("1", &vt.str_na);
    let n_3x_key = NodeKey::new("3", &vt.str_na);
    let n_3a_key = NodeKey::new("3", "a");

    // Check the nodes
    assert_eq!(vt.graph.nodes.len(), 6);

    // Check n_1_x
    {
        let edges_in: HashSet<EdgeKey> = HashSet::new();
        let edges_out: HashSet<EdgeKey> = vec![
            EdgeKey::new(&n_1a_key, &n_2a_key),
        ].into_iter().collect();
        let uids: HashSet<String> = vec!["x".to_string()].into_iter().collect();


        let node = vt.graph.get_node(&n_1a_key).unwrap();
        assert_eq!(node.uids, uids);
        assert_eq!(node.edge_ids_in, edges_in);
        assert_eq!(node.edge_ids_out, edges_out);
    }


    // Check n_1_yz
    {
        let edges_in: HashSet<EdgeKey> = HashSet::new();
        let edges_out: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_1x_key, &n_2b_key),
        ]);
        let uids: HashSet<String> = HashSet::from_iter(vec!["y".to_string(), "z".to_string()]);

        let node = vt.graph.get_node(&n_1x_key).unwrap();
        assert_eq!(node.uids, uids);
        assert_eq!(node.edge_ids_in, edges_in);
        assert_eq!(node.edge_ids_out, edges_out);
    }

    // Check n_2_x
    {
        let edges_in: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_1a_key, &n_2a_key),
        ]);
        let edges_out: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_2a_key, &n_3x_key),
        ]);
        let uids: HashSet<String> = HashSet::from_iter(vec!["x".to_string()]);

        let node = vt.graph.get_node(&n_2a_key).unwrap();
        assert_eq!(node.uids, uids);
        assert_eq!(node.edge_ids_in, edges_in);
        assert_eq!(node.edge_ids_out, edges_out);
    }

    // Check n_2_yz
    {
        let edges_in: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_1x_key, &n_2b_key),
        ]);
        let edges_out: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_2b_key, &n_3a_key),
            EdgeKey::new(&n_2b_key, &n_3x_key),
        ]);
        let uids: HashSet<String> = HashSet::from_iter(vec!["y".to_string(), "z".to_string()]);

        let node = vt.graph.get_node(&n_2b_key).unwrap();
        assert_eq!(node.uids, uids);
        assert_eq!(node.edge_ids_in, edges_in);
        assert_eq!(node.edge_ids_out, edges_out);
    }

    // Check n_3_y
    {
        let edges_in: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_2b_key, &n_3a_key),
        ]);
        let edges_out: HashSet<EdgeKey> = HashSet::new();
        let uids: HashSet<String> = HashSet::from_iter(vec!["y".to_string()]);

        let node = vt.graph.get_node(&n_3a_key).unwrap();
        assert_eq!(node.uids, uids);
        assert_eq!(node.edge_ids_in, edges_in);
        assert_eq!(node.edge_ids_out, edges_out);
    }

    // Check n_3_xz
    {
        let edges_in: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_2a_key, &n_3x_key),
            EdgeKey::new(&n_2b_key, &n_3x_key),
        ]);
        let edges_out: HashSet<EdgeKey> = HashSet::new();
        let uids: HashSet<String> = HashSet::from_iter(vec!["x".to_string(), "z".to_string()]);

        let node = vt.graph.get_node(&n_3x_key).unwrap();
        assert_eq!(node.uids, uids);
        assert_eq!(node.edge_ids_in, edges_in);
        assert_eq!(node.edge_ids_out, edges_out);
    }

    // Check the edges
    assert_eq!(vt.graph.edges.len(), 5);

    // e_1a_2a
    {
        let edge = vt.graph.get_edge(&EdgeKey::new(&n_1a_key, &n_2a_key)).unwrap();
        let uids: HashSet<String> = HashSet::from_iter(vec!["x".to_string()]);
        assert_eq!(edge.uids, uids);
    }

    // e_mis1_2b
    {
        let edge = vt.graph.get_edge(&EdgeKey::new(&n_1x_key, &n_2b_key)).unwrap();
        let uids: HashSet<String> = HashSet::from_iter(vec!["y".to_string(), "z".to_string()]);
        assert_eq!(edge.uids, uids);
    }

    // e_2a_3_mis
    {
        let edge = vt.graph.get_edge(&EdgeKey::new(&n_2a_key, &n_3x_key)).unwrap();
        let uids: HashSet<String> = HashSet::from_iter(vec!["x".to_string()]);
        assert_eq!(edge.uids, uids);
    }

    // e_2b_3_mis
    {
        let edge = vt.graph.get_edge(&EdgeKey::new(&n_2b_key, &n_3x_key)).unwrap();
        let uids: HashSet<String> = HashSet::from_iter(vec!["z".to_string()]);
        assert_eq!(edge.uids, uids);
    }

    // e_2b_3a
    {
        let edge = vt.graph.get_edge(&EdgeKey::new(&n_2b_key, &n_3a_key)).unwrap();
        let uids: HashSet<String> = HashSet::from_iter(vec!["y".to_string()]);
        assert_eq!(edge.uids, uids);
    }

    // Check the node indices
    // x
    {
        let node_idx: HashSet<NodeKey> = HashSet::from_iter(vec![n_1a_key.clone(), n_2a_key.clone(), n_3x_key.clone()]);
        assert_eq!(vt.uid_to_node.get("x").unwrap(), &node_idx);
    }

    // y
    {
        let node_idx: HashSet<NodeKey> = HashSet::from_iter(vec![n_1x_key.clone(), n_2b_key.clone(), n_3a_key.clone()]);
        assert_eq!(vt.uid_to_node.get("y").unwrap(), &node_idx);
    }

    // z
    {
        let node_idx: HashSet<NodeKey> = HashSet::from_iter(vec![n_1x_key.clone(), n_2b_key.clone(), n_3x_key.clone()]);
        assert_eq!(vt.uid_to_node.get("z").unwrap(), &node_idx);
    }

    // Check the edge indices
    // x
    {
        let edge_idx: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_1a_key, &n_2a_key),
            EdgeKey::new(&n_2a_key, &n_3x_key),
        ]);
        assert_eq!(vt.uid_to_edge.get("x").unwrap(), &edge_idx);
    }

    // y
    {
        let edge_idx: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_1x_key, &n_2b_key),
            EdgeKey::new(&n_2b_key, &n_3a_key),
        ]);
        assert_eq!(vt.uid_to_edge.get("y").unwrap(), &edge_idx);
    }

    // z
    {
        let edge_idx: HashSet<EdgeKey> = HashSet::from_iter(vec![
            EdgeKey::new(&n_1x_key, &n_2b_key),
            EdgeKey::new(&n_2b_key, &n_3x_key),
        ]);
        assert_eq!(vt.uid_to_edge.get("z").unwrap(), &edge_idx);
    }
}

#[test]
fn test_build_uid_paths() {
    let vt = test_init_v_tracker();

    // Create NodeKeys for later use
    let n_1a_key = NodeKey::new("1", "a");
    let n_2a_key = NodeKey::new("2", "a");
    let n_2b_key = NodeKey::new("2", "b");
    let n_1x_key = NodeKey::new("1", &vt.str_na);
    let n_3x_key = NodeKey::new("3", &vt.str_na);
    let n_3a_key = NodeKey::new("3", "a");

    let (nodes, edges) = vt.build_uid_paths();

    // Check node highlighting
    // x
    {
        let ids: HashSet<usize> = HashSet::from_iter(vec![
            vt.graph.get_node(&n_1a_key).unwrap().id,
            vt.graph.get_node(&n_2a_key).unwrap().id,
            vt.graph.get_node(&n_3x_key).unwrap().id,
        ]);
        assert_eq!(nodes.get("x").unwrap(), &ids);
    }

    // y
    {
        let ids: HashSet<usize> = HashSet::from_iter(vec![
            vt.graph.get_node(&n_1x_key).unwrap().id,
            vt.graph.get_node(&n_2b_key).unwrap().id,
            vt.graph.get_node(&n_3a_key).unwrap().id,
        ]);
        assert_eq!(nodes.get("y").unwrap(), &ids);
    }

    // z
    {
        let ids: HashSet<usize> = HashSet::from_iter(vec![
            vt.graph.get_node(&n_1x_key).unwrap().id,
            vt.graph.get_node(&n_2b_key).unwrap().id,
            vt.graph.get_node(&n_3x_key).unwrap().id,
        ]);
        assert_eq!(nodes.get("z").unwrap(), &ids);
    }

    // Check edge highlighting
    // x
    {
        let ids: HashSet<usize> = HashSet::from_iter(vec![
            vt.graph.get_edge(&EdgeKey::new(&n_1a_key, &n_2a_key)).unwrap().id,
            vt.graph.get_edge(&EdgeKey::new(&n_2a_key, &n_3x_key)).unwrap().id,
        ]);
        assert_eq!(edges.get("x").unwrap(), &ids);
    }

    // y
    {
        let ids: HashSet<usize> = HashSet::from_iter(vec![
            vt.graph.get_edge(&EdgeKey::new(&n_1x_key, &n_2b_key)).unwrap().id,
            vt.graph.get_edge(&EdgeKey::new(&n_2b_key, &n_3a_key)).unwrap().id,
        ]);
        assert_eq!(edges.get("y").unwrap(), &ids);
    }

    // z
    {
        let ids: HashSet<usize> = HashSet::from_iter(vec![
            vt.graph.get_edge(&EdgeKey::new(&n_1x_key, &n_2b_key)).unwrap().id,
            vt.graph.get_edge(&EdgeKey::new(&n_2b_key, &n_3x_key)).unwrap().id,
        ]);
        assert_eq!(edges.get("z").unwrap(), &ids);
    }
}

#[test]
fn test_as_sankey_json() {
    let vt = test_init_v_tracker();

    // Create NodeKeys for later use
    let n_1a_key = NodeKey::new("1", "a");
    let n_2a_key = NodeKey::new("2", "a");
    let n_2b_key = NodeKey::new("2", "b");
    let n_1x_key = NodeKey::new("1", &vt.str_na);
    let n_3x_key = NodeKey::new("3", &vt.str_na);
    let n_3a_key = NodeKey::new("3", "a");

    let (nodes, edges) = vt.build_uid_paths();

    // Get the node IDs
    let id_a1 = vt.graph.get_node(&n_1a_key).unwrap().id;
    let id_x1 = vt.graph.get_node(&n_1x_key).unwrap().id;
    let id_a2 = vt.graph.get_node(&n_2a_key).unwrap().id;
    let id_b2 = vt.graph.get_node(&n_2b_key).unwrap().id;
    let id_x3 = vt.graph.get_node(&n_3x_key).unwrap().id;
    let id_a3 = vt.graph.get_node(&n_3a_key).unwrap().id;

    // Create the D3 object
    let sankey_json = vt.as_sankey_json();

    // Create the expected
    let mut expected_nodes: Vec<SankeyNode> = Vec::new();

    // id_a1
    {
        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("x").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("x").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        expected_nodes.push(SankeyNode {
            id: id_a1,
            name: "a".to_string(),
            col: "1".to_string(),
            total: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_mis_1
    {
        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("y").unwrap().union(edges.get("z").unwrap()) {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("y").unwrap().union(nodes.get("z").unwrap()) {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        expected_nodes.push(SankeyNode {
            id: id_x1,
            name: vt.str_na.to_string(),
            col: "1".to_string(),
            total: 2,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_a2
    {
        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("x").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("x").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        expected_nodes.push(SankeyNode {
            id: id_a2,
            name: "a".to_string(),
            col: "2".to_string(),
            total: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_b2
    {
        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("y").unwrap().union(edges.get("z").unwrap()) {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("y").unwrap().union(nodes.get("z").unwrap()) {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        expected_nodes.push(SankeyNode {
            id: id_b2,
            name: "b".to_string(),
            col: "2".to_string(),
            total: 2,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_a3
    {
        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("y").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("y").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        expected_nodes.push(SankeyNode {
            id: id_a3,
            name: "a".to_string(),
            col: "3".to_string(),
            total: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_mis_3
    {
        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("x").unwrap().union(edges.get("z").unwrap()) {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("x").unwrap().union(nodes.get("z").unwrap()) {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        expected_nodes.push(SankeyNode {
            id: id_x3,
            name: vt.str_na.to_string(),
            col: "3".to_string(),
            total: 2,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // Sort the nodes by id
    expected_nodes.sort_by(|a, b| a.id.cmp(&b.id));

    // Test the nodes
    assert_eq!(sankey_json.nodes, expected_nodes);

    let mut edges_exp: Vec<SankeyLink> = Vec::new();

    // id_1
    {
        let id = vt.graph.get_edge(&EdgeKey::new(&n_1a_key, &n_2a_key)).unwrap().id;

        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("x").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("x").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        edges_exp.push(SankeyLink {
            id,
            source: id_a1,
            target: id_a2,
            value: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_2
    {
        let id = vt.graph.get_edge(&EdgeKey::new(&n_1x_key, &n_2b_key)).unwrap().id;

        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("y").unwrap().union(edges.get("z").unwrap()) {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("y").unwrap().union(nodes.get("z").unwrap()) {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        edges_exp.push(SankeyLink {
            id,
            source: id_x1,
            target: id_b2,
            value: 2,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_3
    {
        let id = vt.graph.get_edge(&EdgeKey::new(&n_2a_key, &n_3x_key)).unwrap().id;

        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("x").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("x").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        edges_exp.push(SankeyLink {
            id,
            source: id_a2,
            target: id_x3,
            value: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_4
    {
        let id = vt.graph.get_edge(&EdgeKey::new(&n_2b_key, &n_3a_key)).unwrap().id;

        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("y").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("y").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        edges_exp.push(SankeyLink {
            id,
            source: id_b2,
            target: id_a3,
            value: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // id_5
    {
        let id = vt.graph.get_edge(&EdgeKey::new(&n_2b_key, &n_3x_key)).unwrap().id;

        let mut cur_links: Vec<usize> = Vec::new();
        for link_id in edges.get("z").unwrap() {
            cur_links.push(*link_id);
        }
        cur_links.sort();

        let mut cur_nodes: Vec<usize> = Vec::new();
        for node_id in nodes.get("z").unwrap() {
            cur_nodes.push(*node_id);
        }
        cur_nodes.sort();

        edges_exp.push(SankeyLink {
            id,
            source: id_b2,
            target: id_x3,
            value: 1,
            link_highlight_id: cur_links,
            node_highlight_id: cur_nodes,
        });
    }

    // Sort edges_exp by the id
    edges_exp.sort_by(|a, b| a.id.cmp(&b.id));
    assert_eq!(sankey_json.links, edges_exp);
}
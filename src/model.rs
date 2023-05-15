#[derive(PartialEq, Debug)]
pub struct SankeyNode {
    pub col: String,
    pub id: usize,
    pub link_highlight_id: Vec<usize>,
    pub name: String,
    pub node_highlight_id: Vec<usize>,
    pub total: usize,
}

#[derive(PartialEq, Debug)]
pub struct SankeyLink {
    pub id: usize,
    pub link_highlight_id: Vec<usize>,
    pub node_highlight_id: Vec<usize>,
    pub source: usize,
    pub target: usize,
    pub value: usize,
}

#[derive(Debug)]
pub struct SankeyD3 {
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
}

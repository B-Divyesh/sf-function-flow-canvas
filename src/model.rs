use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceNode {
    pub id: String,
    pub name: String,
    pub detail: String,
    pub kind: u64,
    pub file: String,
    pub line: u64,
    pub column: u64,
    pub snippet: String,
    pub type_context: String,
    pub side: Side,
    pub depth: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Root,
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CallEdge {
    pub caller: String,
    pub callee: String,
    pub depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowCanvas {
    pub schema_version: u8,
    pub root_symbol: String,
    pub source_file: String,
    pub requested_depth: u8,
    pub server: String,
    pub nodes: Vec<SourceNode>,
    pub edges: Vec<CallEdge>,
    pub warnings: Vec<String>,
}

impl FlowCanvas {
    pub fn root(&self) -> Option<&SourceNode> {
        self.nodes.iter().find(|node| node.side == Side::Root)
    }
}

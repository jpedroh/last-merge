#[derive(Debug, Default)]
pub struct LogState<'a> {
    pub log: Vec<MergeChunk<'a>>,
    pub current_stable: ChunkData<'a>,
    pub current_unstable: ChunkData<'a>,
}

#[derive(Debug)]
pub enum MergeChunk<'a> {
    Stable(ChunkData<'a>),
    Unstable(ChunkData<'a>),
    UnorderedContextStart { node_kind: &'a str },
    UnorderedContextEnd { node_kind: &'a str },
}

#[derive(Debug, Default)]
pub struct ChunkData<'a> {
    pub left_nodes: Vec<&'a model::CSTNode<'a>>,
    pub base_nodes: Vec<&'a model::CSTNode<'a>>,
    pub right_nodes: Vec<&'a model::CSTNode<'a>>,
}

impl<'a> ChunkData<'a> {
    pub fn is_empty(&self) -> bool {
        self.left_nodes.is_empty() && self.base_nodes.is_empty() && self.right_nodes.is_empty()
    }
}

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub enum CSTNode<'a> {
    Terminal(Terminal<'a>),
    NonTerminal(NonTerminal<'a>),
}

impl CSTNode<'_> {
    pub fn id(&self) -> uuid::Uuid {
        match self {
            CSTNode::Terminal(terminal) => terminal.id,
            CSTNode::NonTerminal(non_terminal) => non_terminal.id,
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            CSTNode::Terminal(terminal) => terminal.kind,
            CSTNode::NonTerminal(non_terminal) => non_terminal.kind,
        }
    }

    pub fn contents(&self) -> String {
        match self {
            CSTNode::Terminal(node) => node.contents(),
            CSTNode::NonTerminal(node) => node.contents(),
        }
    }

    pub fn start_position(&self) -> Point {
        match self {
            CSTNode::Terminal(node) => node.start_position,
            CSTNode::NonTerminal(node) => node.start_position,
        }
    }

    pub fn end_position(&self) -> Point {
        match self {
            CSTNode::Terminal(node) => node.end_position,
            CSTNode::NonTerminal(node) => node.end_position,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, CSTNode::Terminal(_))
    }

    fn get_subtree_size(&self) -> usize {
        match self {
            CSTNode::Terminal(_) => 0,
            CSTNode::NonTerminal(node) => node
                .children
                .iter()
                .fold(node.children.len(), |acc, child| {
                    acc + child.get_subtree_size()
                }),
        }
    }

    pub fn get_tree_size(&self) -> usize {
        self.get_subtree_size() + 1
    }

    pub fn has_identifier(&self) -> bool {
        match self {
            CSTNode::Terminal(_) => true,
            CSTNode::NonTerminal(node) => node.get_identifier().is_some(),
        }
    }

    pub fn leading_white_space(&self) -> Option<&str> {
        match self {
            CSTNode::Terminal(node) => node.leading_white_space,
            CSTNode::NonTerminal(node) => node.leading_white_space,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Delimiters<'a> {
    start: &'a str,
    end: &'a str,
}

impl Delimiters<'_> {
    pub fn new<'a>(start: &'a str, end: &'a str) -> Delimiters<'a> {
        Delimiters { start, end }
    }

    pub fn start(&self) -> &str {
        self.start
    }

    pub fn end(&self) -> &str {
        self.end
    }
}

#[derive(Debug, Default, Clone)]
pub struct NonTerminal<'a> {
    pub id: uuid::Uuid,
    pub kind: &'a str,
    pub children: Vec<CSTNode<'a>>,
    pub start_position: Point,
    pub end_position: Point,
    pub are_children_unordered: bool,
    pub identifier: Option<Vec<&'a str>>,
    pub leading_white_space: Option<&'a str>,
    pub delimiters: Option<&'a Delimiters<'a>>,
}

impl PartialEq for NonTerminal<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for NonTerminal<'_> {}

impl PartialOrd for NonTerminal<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonTerminal<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for NonTerminal<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl NonTerminal<'_> {
    pub fn contents(&self) -> String {
        self.children.iter().fold(String::from(""), |acc, node| {
            format!("{} {}", acc, node.contents())
        })
    }

    pub fn get_identifier(&self) -> Option<&[&str]> {
        self.identifier.as_deref()
    }

    pub fn get_children(&self) -> &[CSTNode<'_>] {
        self.children.as_slice()
    }
}

impl<'a> TryFrom<&'a CSTNode<'a>> for &'a NonTerminal<'a> {
    type Error = &'static str;

    fn try_from(node: &'a CSTNode<'a>) -> Result<Self, Self::Error> {
        match node {
            CSTNode::NonTerminal(non_terminal) => Ok(non_terminal),
            CSTNode::Terminal(_) => Err("Cannot convert terminal to non-terminal"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Terminal<'a> {
    pub id: uuid::Uuid,
    pub kind: &'a str,
    pub value: &'a str,
    pub start_position: Point,
    pub end_position: Point,
    pub leading_white_space: Option<&'a str>,
}

impl PartialEq for Terminal<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Terminal<'_> {}

impl PartialOrd for Terminal<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Terminal<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Terminal<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Terminal<'_> {
    pub fn contents(&self) -> String {
        String::from(self.value)
    }

    pub fn get_identifier(&self) -> (&str, &str) {
        (self.kind, self.value)
    }
}

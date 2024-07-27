use std::fmt::Display;

use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum MergedCSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: std::borrow::Cow<'a, str>,
    },
    NonTerminal {
        kind: &'a str,
        children: Vec<MergedCSTNode<'a>>,
    },
    Conflict {
        left: Option<Box<MergedCSTNode<'a>>>,
        right: Option<Box<MergedCSTNode<'a>>>,
    },
}

impl<'a> From<&'a CSTNode<'a>> for MergedCSTNode<'a> {
    fn from(val: &'a CSTNode<'a>) -> Self {
        match val {
            CSTNode::Terminal(terminal) => terminal.into(),
            CSTNode::NonTerminal(NonTerminal { kind, children, .. }) => {
                MergedCSTNode::NonTerminal {
                    kind,
                    children: children.iter().map(|node| node.into()).collect(),
                }
            }
        }
    }
}

impl<'a> From<&'a Terminal<'a>> for MergedCSTNode<'a> {
    fn from(val: &'a Terminal<'a>) -> Self {
        MergedCSTNode::Terminal {
            kind: val.kind,
            value: std::borrow::Cow::Borrowed(val.value),
        }
    }
}

impl Display for MergedCSTNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergedCSTNode::Terminal { value, .. } => write!(f, "{}", value),
            MergedCSTNode::NonTerminal { children, .. } => {
                let result = children.iter().fold(String::new(), |acc, current| {
                    let mut result = acc.to_owned();
                    result.push(' ');
                    result.push_str(&current.clone().to_string());
                    result
                });

                write!(f, "{}", result)
            }
            MergedCSTNode::Conflict { left, right } => match (left, right) {
                (Some(left), Some(right)) => {
                    writeln!(f)?;
                    writeln!(f, "<<<<<<<<<")?;
                    writeln!(f, "{}", left)?;
                    writeln!(f, "=========")?;
                    writeln!(f, "{}", right)?;
                    writeln!(f, ">>>>>>>>>")
                }
                (Some(left), None) => {
                    writeln!(f)?;
                    writeln!(f, "<<<<<<<<<")?;
                    writeln!(f, "{}", left)?;
                    writeln!(f, "=========")?;
                    writeln!(f, ">>>>>>>>>")
                }
                (None, Some(right)) => {
                    writeln!(f)?;
                    writeln!(f, "<<<<<<<<<")?;
                    writeln!(f, "=========")?;
                    writeln!(f, "{}", right)?;
                    writeln!(f, ">>>>>>>>>")
                }
                (None, None) => unreachable!("Invalid conflict provided"),
            },
        }
    }
}

impl MergedCSTNode<'_> {
    pub fn has_conflict(&self) -> bool {
        match self {
            MergedCSTNode::NonTerminal { children, .. } => {
                children.iter().any(|child| child.has_conflict())
            }
            MergedCSTNode::Terminal { .. } => false,
            MergedCSTNode::Conflict { .. } => true,
        }
    }
}

use std::fmt::Display;

use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum MergedCSTNode<'a> {
    Terminal {
        kind: &'a str,
        value: std::borrow::Cow<'a, str>,
        leading_white_space: Option<&'a str>,
    },
    NonTerminal {
        kind: &'a str,
        children: Vec<MergedCSTNode<'a>>,
        leading_white_space: Option<&'a str>,
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
            CSTNode::NonTerminal(NonTerminal {
                kind,
                children,
                leading_white_space,
                ..
            }) => MergedCSTNode::NonTerminal {
                kind,
                children: children.iter().map(|node| node.into()).collect(),
                leading_white_space: leading_white_space.clone(),
            },
        }
    }
}

impl<'a> From<&'a Terminal<'a>> for MergedCSTNode<'a> {
    fn from(val: &'a Terminal<'a>) -> Self {
        MergedCSTNode::Terminal {
            kind: val.kind,
            value: std::borrow::Cow::Borrowed(val.value),
            leading_white_space: val.leading_white_space.clone(),
        }
    }
}

impl Display for MergedCSTNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergedCSTNode::Terminal { value, .. } => write!(f, "{value}"),
            MergedCSTNode::NonTerminal { children, .. } => {
                for current in children {
                    if let Some(leading) = current.leading_white_space() {
                        write!(f, "{leading}")?
                    }
                    write!(f, "{current}")?;
                }
                write!(f, "")
            }
            MergedCSTNode::Conflict { left, right } => match (left, right) {
                (Some(left), Some(right)) => {
                    writeln!(f)?;
                    writeln!(f, "<<<<<<<")?;
                    writeln!(f, "{left}")?;
                    writeln!(f, "=======")?;
                    writeln!(f, "{right}")?;
                    writeln!(f, ">>>>>>>")
                }
                (Some(left), None) => {
                    writeln!(f)?;
                    writeln!(f, "<<<<<<<")?;
                    writeln!(f, "{left}")?;
                    writeln!(f, "=======")?;
                    writeln!(f, ">>>>>>>")
                }
                (None, Some(right)) => {
                    writeln!(f)?;
                    writeln!(f, "<<<<<<<")?;
                    writeln!(f, "=======")?;
                    writeln!(f, "{right}")?;
                    writeln!(f, ">>>>>>>")
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

    pub fn leading_white_space(&self) -> Option<&str> {
        match self {
            MergedCSTNode::Terminal {
                leading_white_space,
                ..
            } => *leading_white_space,
            MergedCSTNode::NonTerminal {
                leading_white_space,
                ..
            } => *leading_white_space,
            MergedCSTNode::Conflict { .. } => None,
        }
    }

    pub fn raw_source_code(&self) -> Option<&str> {
        match self {
            MergedCSTNode::Terminal { value, .. } => Some(value),
            MergedCSTNode::NonTerminal {
                leading_white_space,
                ..
            } => *leading_white_space,
            MergedCSTNode::Conflict { .. } => None,
        }
    }
}

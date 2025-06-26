use matching::Matchings;
use model::cst_node::NonTerminal;

use crate::{MergeError, MergedCSTNode};

#[derive(Debug)]
pub enum MergeChunk<'a> {
    Stable(ChunkData<'a>),
    Unstable(ChunkData<'a>),
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

pub fn ordered_merge<'a>(
    left: &'a NonTerminal<'a>,
    right: &'a NonTerminal<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
    print_chunks: bool,
) -> Result<MergedCSTNode<'a>, MergeError> {
    // Nodes of different kind, early return
    if left.kind != right.kind {
        return Err(MergeError::NodesWithDifferentKinds(
            left.kind.to_string(),
            right.kind.to_string(),
        ));
    }

    let mut log: Vec<MergeChunk> = Vec::new();
    let mut current_stable = ChunkData::default();
    let mut current_unstable = ChunkData::default();

    let mut result_children =
        Vec::with_capacity(left.get_children().len() + right.get_children().len());

    let mut children_left_it = left.get_children().iter();
    let mut children_right_it = right.get_children().iter();

    let mut cur_left_option = children_left_it.next();
    let mut cur_right_option = children_right_it.next();

    while let (Some(cur_left), Some(cur_right)) = (cur_left_option, cur_right_option) {
        let matching_base_left = base_left_matchings.find_matching_for(cur_left);
        let matching_base_right = base_right_matchings.find_matching_for(cur_right);
        let left_matching_in_right = left_right_matchings.find_matching_for(cur_left);
        let right_matching_in_left = left_right_matchings.find_matching_for(cur_right);
        let has_bidirectional_matching_left_right =
            left_matching_in_right.is_some() && right_matching_in_left.is_some();

        match (
            has_bidirectional_matching_left_right,
            left_matching_in_right,
            matching_base_left,
            right_matching_in_left,
            matching_base_right,
        ) {
            (true, Some(_), Some(matching_base_left), Some(_), Some(_)) => {
                
                if print_chunks {
                    if !current_unstable.is_empty(){
                        log.push(MergeChunk::Unstable(std::mem::take(&mut current_unstable)));
                    }
                    current_stable.left_nodes.push(cur_left);
                    current_stable.base_nodes.push(matching_base_left.matching_node);
                    current_stable.right_nodes.push(cur_right);
                }
                
                result_children.push(crate::merge(
                    matching_base_left.matching_node,
                    cur_left,
                    cur_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                    print_chunks,
                )?);

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (true, Some(_), None, Some(_), None) => {

                if print_chunks {
                    if !current_unstable.is_empty() {
                        log.push(MergeChunk::Unstable(std::mem::take(&mut current_unstable)));
                    }
                    current_stable.left_nodes.push(cur_left);
                    current_stable.right_nodes.push(cur_right);
                }

                result_children.push(crate::merge(
                    cur_left,
                    cur_left,
                    cur_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                    print_chunks,
                )?);

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (true, Some(_), Some(matching_base_left), Some(_), None) => {

                if print_chunks {
                    if !current_unstable.is_empty() {
                        log.push(MergeChunk::Unstable(std::mem::take(&mut current_unstable)));
                    }
                    current_stable.left_nodes.push(cur_left);
                    current_stable.base_nodes.push(matching_base_left.matching_node);
                    current_stable.right_nodes.push(cur_right);
                }

                result_children.push(crate::merge(
                    matching_base_left.matching_node,
                    cur_left,
                    cur_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                    print_chunks,
                )?);

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (true, Some(_), None, Some(_), Some(matching_base_right)) => {

                if print_chunks {
                    if !current_unstable.is_empty() {
                        log.push(MergeChunk::Unstable(std::mem::take(&mut current_unstable)));
                    }
                    current_stable.left_nodes.push(cur_left);
                    current_stable.base_nodes.push(matching_base_right.matching_node);
                    current_stable.right_nodes.push(cur_right);
                }

                result_children.push(crate::merge(
                    matching_base_right.matching_node,
                    cur_left,
                    cur_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                    print_chunks,
                )?);

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, Some(_), Some(_), None, Some(matching_base_right)) => {

                if print_chunks {
                    if !current_stable.is_empty() { 
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable))); 
                    }
                    current_unstable.right_nodes.push(cur_right);
                }

                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(cur_right.into())),
                    });
                }

                cur_right_option = children_right_it.next();
            }
            (false, Some(_), Some(_), None, None) => {

                if print_chunks {
                    if !current_stable.is_empty() { 
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.right_nodes.push(cur_right);
                }

                result_children.push(cur_right.into());

                cur_right_option = children_right_it.next();
            }
            (false, Some(_), None, None, Some(matching_base_right)) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.right_nodes.push(cur_right);
                }

                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(cur_right.into())),
                    })
                }
                cur_right_option = children_right_it.next();
            }
            (false, Some(_), None, None, None) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.right_nodes.push(cur_right);
                }

                result_children.push(cur_right.into());
                cur_right_option = children_right_it.next();
            }
            (false, None, Some(matching_base_left), Some(_), Some(_)) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                }

                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.into())),
                        right: None,
                    });
                }

                cur_left_option = children_left_it.next();
            }
            (false, None, Some(matching_base_left), Some(_), None) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                }

                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.into())),
                        right: None,
                    })
                }
                cur_left_option = children_left_it.next();
            }
            (false, None, Some(matching_base_left), None, Some(matching_base_right)) => {
                
                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                    current_unstable.right_nodes.push(cur_right);
                    current_unstable.base_nodes.push(matching_base_left.matching_node);
                }

                
                match (
                    matching_base_left.is_perfect_match,
                    matching_base_right.is_perfect_match,
                ) {
                    (true, true) => {}
                    (true, false) => {
                        result_children.push(MergedCSTNode::Conflict {
                            left: Some(Box::new(cur_left.into())),
                            right: None,
                        });
                    }
                    (false, true) => {
                        result_children.push(MergedCSTNode::Conflict {
                            left: None,
                            right: Some(Box::new(cur_right.into())),
                        });
                    }
                    (false, false) => {
                        result_children.push(MergedCSTNode::Conflict {
                            left: Some(Box::new(cur_left.into())),
                            right: Some(Box::new(cur_right.into())),
                        });
                    }
                }

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, None, Some(matching_base_left), None, None) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                }

                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.into())),
                        right: Some(Box::new(cur_right.into())),
                    });
                } else {
                    result_children.push(cur_right.into());
                }

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, None, None, Some(_), Some(_)) => {

                if print_chunks{
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                }

                result_children.push(cur_left.into());
                cur_left_option = children_left_it.next();
            }
            (false, None, None, Some(_), None) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                }

                result_children.push(cur_left.into());
                cur_left_option = children_left_it.next();
            }
            (false, None, None, None, Some(matching_base_right)) => {

                if print_chunks {
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                    current_unstable.right_nodes.push(cur_right);
                }

                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.into())),
                        right: Some(Box::new(cur_right.into())),
                    })
                } else {
                    result_children.push(cur_left.into());
                }

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, None, None, None, None) => {

                if print_chunks{
                    if !current_stable.is_empty() {
                        log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
                    }
                    current_unstable.left_nodes.push(cur_left);
                    current_unstable.right_nodes.push(cur_right);
                }

                result_children.push(MergedCSTNode::Conflict {
                    left: Some(Box::new(cur_left.into())),
                    right: Some(Box::new(cur_right.into())),
                });

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (a, b, c, d, e) => {
                log::warn!(
                    "Reached an Invalid Matching Configuration. {} {} {} {} {}",
                    a,
                    b.is_some(),
                    c.is_some(),
                    d.is_some(),
                    e.is_some()
                );
                log::debug!(
                    "Involved nodes {} AND {}",
                    cur_left.contents(),
                    cur_right.contents()
                );
                log::debug!(
                    "Involved nodes parents {} AND {}",
                    left.contents(),
                    right.contents()
                );

                if cur_left.contents() == cur_right.contents() {
                    result_children.push(cur_left.into())
                } else {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.into())),
                        right: Some(Box::new(cur_right.into())),
                    })
                }

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
        }
    }

    if print_chunks {
        if !current_stable.is_empty() {
            log.push(MergeChunk::Stable(std::mem::take(&mut current_stable)));
        }
    }

    while let Some(cur_left) = cur_left_option {
        result_children.push(cur_left.into());
        cur_left_option = children_left_it.next();
    }

    while let Some(cur_right) = cur_right_option {
        result_children.push(cur_right.into());
        cur_right_option = children_right_it.next();
    }

    if print_chunks {
        if !current_unstable.is_empty() { 
            log.push(MergeChunk::Unstable(std::mem::take(&mut current_unstable))); 
        }

        println!("\n--- ORDERED CHUNK LOG ---");
        println!("===========================================================");

        // Helper para formatar a lista de nós de forma completa
        let format_node_list = |nodes: &Vec<&model::CSTNode>| -> String {
            if nodes.is_empty() {
                return "-".to_string();
            }

            let first_node = nodes.first().unwrap();
            let last_node = nodes.last().unwrap();
            
            let start_line = first_node.start_position().row + 1;
            let end_line = last_node.end_position().row + 1;
            
            let range = if start_line == end_line {
                format!("(L{})", start_line)
            } else {
                format!("(L{}-L{})", start_line, end_line)
            };
            
            const MAX_NODES_TO_SHOW: usize = 3;
            const MAX_CONTENT_LEN: usize = 20;

            // Itera para criar a descrição detalhada de cada nó
            let descriptions: Vec<String> = nodes.iter().map(|n| {
                // Limpa e trunca o conteúdo para legibilidade
                let mut content = n.contents().replace(['\n', '\r'], " ").trim().to_string();
                if content.len() > MAX_CONTENT_LEN {
                    content.truncate(MAX_CONTENT_LEN - 3);
                    content.push_str("...");
                }
                format!("{}: '{}'", n.kind(), content)
            }).take(MAX_NODES_TO_SHOW).collect();

            let mut summary = format!("[{}]", descriptions.join(", "));
            if nodes.len() > MAX_NODES_TO_SHOW {
                summary.push_str("...");
            }
            
            format!("{} nós {} {}", nodes.len(), range, summary)
        };

        for (i, chunk) in log.iter().enumerate() {
            match chunk {
                MergeChunk::Stable(data) => {
                    println!("-- stable chunk #{} --", i + 1);
                    let l_info = format_node_list(&data.left_nodes);
                    let b_info = format_node_list(&data.base_nodes);
                    let r_info = format_node_list(&data.right_nodes);
                    
                    // Aumentamos a largura da coluna para acomodar o texto extra
                    println!("    Left (L):  {:<85}", l_info);
                    println!("    Base (B):  {:<85}", b_info);
                    println!("    Right (R): {}", r_info);
                }
                MergeChunk::Unstable(data) => {
                    println!("-- unstable chunk #{} --", i + 1);
                    let l_info = format_node_list(&data.left_nodes);
                    let b_info = format_node_list(&data.base_nodes);
                    let r_info = format_node_list(&data.right_nodes);

                    println!("    Left (L):  {:<85}", l_info);
                    println!("    Base (B):  {:<85}", b_info);
                    println!("    Right (R): {}", r_info);
                }
            }
            println!("-----------------------------------------------------------"); 
        }
        println!("--- END ORDERED CHUNK LOG ---\n");
    }

    Ok(MergedCSTNode::NonTerminal {
        kind: left.kind,
        children: result_children,
    })
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, vec};

    use matching::{ordered, Matchings};
    use model::{cst_node::NonTerminal, cst_node::Terminal, CSTNode, Point};

    use crate::{MergeError, MergedCSTNode};

    use super::ordered_merge;

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side<'a>(
        base: &'a CSTNode<'a>,
        parent_a: &'a CSTNode<'a>,
        parent_b: &'a CSTNode<'a>,
        expected_merge: &'a MergedCSTNode<'a>,
    ) -> Result<(), MergeError> {
        let matchings_base_parent_a = ordered::calculate_matchings(base, parent_a);
        let matchings_base_parent_b = ordered::calculate_matchings(base, parent_b);
        let matchings_parents = ordered::calculate_matchings(parent_a, parent_b);

        let merged_tree = ordered_merge(
            parent_a.try_into().unwrap(),
            parent_b.try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
            false,
        )?;
        let merged_tree_swap = ordered_merge(
            parent_b.try_into().unwrap(),
            parent_a.try_into().unwrap(),
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
            false,
        )?;

        assert_eq!(expected_merge, &merged_tree);
        assert_eq!(expected_merge, &merged_tree_swap);

        Ok(())
    }

    fn assert_merge_output_is(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), MergeError> {
        let matchings_base_parent_a = ordered::calculate_matchings(base, parent_a);
        let matchings_base_parent_b = ordered::calculate_matchings(base, parent_b);
        let matchings_parents = ordered::calculate_matchings(parent_a, parent_b);

        let merged_tree = ordered_merge(
            parent_a.try_into().unwrap(),
            parent_b.try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
            false
        )?;

        assert_eq!(expected_merge, &merged_tree);

        Ok(())
    }

    #[test]
    fn it_merges_non_terminals_if_there_are_non_changes() -> Result<(), MergeError> {
        let tree = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = (&tree).into();

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &tree,
            &tree,
            &tree,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_both_left_and_right_add_the_same_things() -> Result<(), MergeError>
    {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });
        let parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });
        let expected_merge = (&parent).into();

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent,
            &parent,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_only_one_parent_adds_a_node_in_an_initially_empty_children_list(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let initially_empty_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let parent_that_added = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",
            children: vec![MergedCSTNode::Terminal {
                kind: "kind_a",
                value: Cow::Borrowed("value_a"),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &initially_empty_parent,
            &parent_that_added,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_only_one_parent_adds_a_node_in_non_empty_children_list(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let unchanged_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_that_added = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![
                MergedCSTNode::Terminal {
                    kind: "kind_a",
                    value: Cow::Borrowed("value_a"),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_b",
                    value: Cow::Borrowed("value_b"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &unchanged_parent,
            &parent_that_added,
            &merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_adds_a_node_and_removes_one_that_was_not_edited_in_the_other(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let changed_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let unchanged_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![MergedCSTNode::Terminal {
                kind: "kind_b",
                value: Cow::Borrowed("value_b"),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &changed_parent,
            &unchanged_parent,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_adds_a_node_and_removes_from_another_that_was_changed(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "another_subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let matchings_base_parent_a = ordered::calculate_matchings(&base, &parent_a);
        let matchings_base_parent_b = ordered::calculate_matchings(&base, &parent_b);
        let matchings_parents = ordered::calculate_matchings(&parent_a, &parent_b);

        let merged_tree = ordered_merge(
            (&parent_a).try_into().unwrap(),
            (&parent_b).try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
            false,
        )?;
        let merged_tree_swap = ordered_merge(
            (&parent_b).try_into().unwrap(),
            (&parent_a).try_into().unwrap(),
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
            false,
        )?;

        assert_eq!(
            MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "another_subtree",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_b",
                            value: Cow::Borrowed("value_b"),
                        }],
                    })),
                    right: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "subtree",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_c",
                            value: Cow::Borrowed("value_c"),
                        }],
                    })),
                },],
            },
            merged_tree
        );

        assert_eq!(
            MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "subtree",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_c",
                            value: Cow::Borrowed("value_c"),
                        }],
                    })),
                    right: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "another_subtree",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_b",
                            value: Cow::Borrowed("value_b"),
                        }],
                    })),
                },],
            },
            merged_tree_swap
        );

        Ok(())
    }

    #[test]
    fn if_both_parents_add_different_nodes_then_we_have_a_conflict() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: Some(Box::new(MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: Cow::Borrowed("value_a"),
                    })),
                    right: Some(Box::new(MergedCSTNode::Terminal {
                        kind: "kind_b",
                        value: Cow::Borrowed("value_b"),
                    })),
                }],
            },
        )
    }

    #[test]
    fn it_merges_when_one_parent_removes_a_node_that_was_not_changed_in_another_parent(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![MergedCSTNode::Terminal {
                kind: "kind_b",
                value: Cow::Borrowed("value_b"),
            }],
        };

        assert_merge_output_is(&base, &left, &right, &expected_merge)
    }

    #[test]
    fn it_detects_a_conflict_when_one_parent_removes_a_node_that_was_changed_in_another_parent(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_b",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_b",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_c",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_c",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_c",
                                value: Cow::Borrowed("value_c"),
                            }],
                        })),
                        right: None,
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: Cow::Borrowed("value_a"),
                    },
                ],
            },
        )?;

        assert_merge_output_is(
            &base,
            &right,
            &left,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_c",
                                value: Cow::Borrowed("value_c"),
                            }],
                        })),
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: Cow::Borrowed("value_a"),
                    },
                ],
            },
        )
    }

    #[test]
    fn it_merges_when_a_parent_adds_a_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let unchanged_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let changed_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![
                MergedCSTNode::Terminal {
                    kind: "kind_a",
                    value: Cow::Borrowed("value_a"),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_b",
                    value: Cow::Borrowed("value_b"),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_c",
                    value: Cow::Borrowed("value_c"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &unchanged_parent,
            &changed_parent,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_removes_and_add_a_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![MergedCSTNode::Terminal {
                kind: "kind_a",
                value: Cow::Borrowed("value_a"),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn it_conflicts_when_one_parent_removes_and_add_a_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_b",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_c",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_b",
                                value: Cow::Borrowed("value_c"),
                            }],
                        })),
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: Cow::Borrowed("value_a"),
                    },
                ],
            },
        )?;
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_b",
                                value: Cow::Borrowed("value_c"),
                            }],
                        })),
                        right: None,
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: Cow::Borrowed("value_a"),
                    },
                ],
            },
        )
    }

    #[test]
    fn it_merges_when_a_parent_adds_one_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![
                MergedCSTNode::Terminal {
                    kind: "kind_c",
                    value: Cow::Borrowed("value_c"),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_a",
                    value: Cow::Borrowed("value_a"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn it_does_not_detect_a_conflict_if_am_merging_two_subtrees_that_have_not_changed_mutually(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_c",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_c",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn it_detects_a_conflict_if_am_merging_two_subtrees_that_delete_a_node_that_was_changed_in_another_parent(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree_a",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_b",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_b",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree_b",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_c",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_c",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree_b",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            are_children_unordered: false,
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree_a",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "subtree_b",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_c",
                            value: Cow::Borrowed("value_c"),
                        }],
                    })),
                    right: None,
                }],
            },
        )?;
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: None,
                    right: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "subtree_b",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_c",
                            value: Cow::Borrowed("value_c"),
                        }],
                    })),
                }],
            },
        )
    }

    #[test]
    fn i_get_an_error_if_i_try_to_merge_nodes_of_different_kinds() {
        let kind_a = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            are_children_unordered: true,
            ..Default::default()
        };
        let kind_b = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            are_children_unordered: true,
            ..Default::default()
        };

        let matchings = Matchings::empty();
        let result = ordered_merge(&kind_a, &kind_b, &matchings, &matchings, &matchings, false);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MergeError::NodesWithDifferentKinds("kind_a".to_string(), "kind_b".to_string())
        );
    }
}

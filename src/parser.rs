use pest::{Parser};

use pest_derive::*;
use pest::iterators::Pair;

use crate::*;

#[derive(Parser)]
#[grammar = "/home/michael/code/rust/sgf-parser/parser.pest"]
struct SGFParser;

///
/// Parse input and return a `SgfGameTree`
///
pub fn parse(input: &str) -> Result<GameTree, SgfError> {
    let mut parse_roots = SGFParser::parse(Rule::game_tree, input).map_err(SgfError::parse_error)?;
    let game_tree = parse_roots.next().unwrap();
    let tree = parse_pair(game_tree);
    let game = create_game_tree(tree);
    Ok(game)
}

fn parse_sequence(sequence_nodes: Vec<ParserNode>) -> Vec<GameNode> {
    let mut nodes = vec![];
    for sequence_node in &sequence_nodes {
        if let ParserNode::Node(node_tokens) = sequence_node {
            let mut tokens: Vec<SgfToken> = vec![];
            node_tokens.iter().for_each(|t| {
                if let ParserNode::Token(token) = t {
                    tokens.push(token.clone());
                } else {
                    unreachable!("node parsing");
                }
            });
            nodes.push(GameNode {
                tokens
            });
        } else {
            unreachable!("Invalid sequence element");
        }
    }
    nodes
}

fn create_game_tree(parser_node: ParserNode) -> GameTree {
    if let ParserNode::GameTree(tree_nodes) = parser_node {
        let mut nodes: Vec<GameNode> = vec![];
        let mut variations: Vec<GameTree> = vec![];
        tree_nodes.into_iter().for_each(|node| {
            match node {
                ParserNode::Sequence(sequence_nodes) => {
                    nodes.extend(parse_sequence(sequence_nodes));
                },
                ParserNode::GameTree(_) => {
                    variations.push(create_game_tree(node.clone()));
                },
                _ => {
                    unreachable!("invalid game tree child");
                }
            }
        });
        GameTree {
            nodes,
            variations,
        }
    } else {
        unreachable!("invalid parser node");
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ParserNode<'a> {
    Token(SgfToken),
    Text(&'a str),
    Node(Vec<ParserNode<'a>>),
    Sequence(Vec<ParserNode<'a>>),
    GameTree(Vec<ParserNode<'a>>),
}

fn parse_pair(pair: Pair<Rule>) -> ParserNode {
    match pair.as_rule() {
        Rule::game_tree => {
            ParserNode::GameTree(pair.into_inner().map(|pair| {
                parse_pair(pair)
            }).collect())
        },
        Rule::sequence => {
            ParserNode::Sequence(pair.into_inner().map(|pair| {
                parse_pair(pair)
            }).collect())
        },
        Rule::node => {
            ParserNode::Node(pair.into_inner().map(|pair| {
                parse_pair(pair)
            }).collect())
        },
        Rule::property => {
            let mut pairs = pair.into_inner();
            let ident = if let ParserNode::Text(text) = parse_pair(pairs.next().unwrap()) {
                text
            } else {
                unreachable!("Property identifier should be a text string");
            };
            let value = if let ParserNode::Text(text) = parse_pair(pairs.next().unwrap()) {
                text
            } else {
                unreachable!("Property identifier should be a text string");
            };
            ParserNode::Token(SgfToken::from_pair(ident, value))
        },
        Rule::property_identifier => {
            ParserNode::Text(pair.as_str())
        },
        Rule::property_value => {
            let value = pair.as_str();
            let end = value.len() - 1;
            ParserNode::Text(&value[1..end])
        }
    }
}

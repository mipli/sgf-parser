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
pub fn parse(input: &str) -> Result<GameTree, ()> {

    println!("input: {:?}", input);
    let tmp = SGFParser::parse(Rule::game_tree, input);
    println!("result: {:?}", tmp);
    let game_tree = tmp.unwrap().next().unwrap();
    println!("AST: {:?}", game_tree);

    let tree = parse_pair(game_tree);
    println!("Parse AST: {:?}", tree);

    let game = create_game_nodes(tree);
    println!("Game tree: {:?}", game);
    Ok(game)
}

fn create_game_nodes(parser_node: ParserNode) -> GameTree {
    println!("parsing node: {:?}", parser_node);
    match parser_node {
        ParserNode::GameTree(tree_nodes) => {
            let mut nodes: Vec<GameNode> = vec![];
            let mut variations: Vec<GameTree> = vec![];
            tree_nodes.into_iter().for_each(|node| {
                match node {
                    ParserNode::Sequence(sequence_nodes) => {
                        for i in 0..sequence_nodes.len() {
                            match &sequence_nodes[i] {
                                ParserNode::Node(node_tokens) => {
                                    let mut tokens: Vec<SgfToken> = vec![];
                                    node_tokens.into_iter().for_each(|t| {
                                        match t {
                                            ParserNode::Token(token) => {
                                                tokens.push(token.clone());
                                            },
                                            _ => {
                                                unreachable!("node parsing");
                                            }
                                        }
                                    });
                                    nodes.push(GameNode {
                                        tokens
                                    });
                                }
                                _ => {
                                    unreachable!("Invalid sequence element");
                                }
                            }
                        }
                    },
                    ParserNode::GameTree(_) => {
                        variations.push(create_game_nodes(node.clone()));
                    },
                    _ => {
                        unreachable!("parsing game tree children");
                    }
                }
            });
            GameTree {
                nodes,
                variations,
            }
        },
        ParserNode::Sequence(sequence_nodes) => {
            unreachable!("seuqnce");
        },
        ParserNode::Node(_) => {
            unreachable!("node");
        },
        ParserNode::Text(_) => {
            unreachable!("text");
        },
        ParserNode::Token(_) => {
            unreachable!("token");
        },
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

fn parse_pair<'a>(pair: Pair<'a, Rule>) -> ParserNode<'a> {
    match pair.as_rule() {
        Rule::game_tree => {
            println!("game tree!");
            ParserNode::GameTree(pair.into_inner().map(|pair| {
                parse_pair(pair)
            }).collect())
        },
        Rule::sequence => {
            println!("sequence");
            ParserNode::Sequence(pair.into_inner().map(|pair| {
                parse_pair(pair)
            }).collect())
        },
        Rule::node => {
            println!("node, {:?}", pair);
            // let token = parse_pair(pair.into_inner().next().unwrap());
            // println!("node end");
            ParserNode::Node(pair.into_inner().map(|pair| {
                parse_pair(pair)
            }).collect())
            // ParserNode::Node(token.into())
        },
        Rule::property => {
            println!("property");
            let mut pairs = pair.into_inner();
            let ident = match parse_pair(pairs.next().unwrap()) {
                ParserNode::Text(text) => text,
                _ => {
                    unreachable!("Property identifier should be a text string");
                }
            };
            println!("got ident");
            let value = match parse_pair(pairs.next().unwrap()) {
                ParserNode::Text(text) => text,
                _ => {
                    unreachable!("Property value should be a text string");
                }
            };
            println!("got value");
            println!("property end");
            ParserNode::Token(SgfToken::from_pair(ident, value))
        },
        Rule::property_identifier => {
            println!("property_identifier: {}", pair.as_str());
            ParserNode::Text(pair.as_str())
        },
        Rule::property_value => {
            println!("property_value: {}", pair.as_str());
            let value = pair.as_str();
            let end = value.len() - 1;
            ParserNode::Text(&value[1..end])
        }
    }
}

/*
Collection = GameTree { GameTree }
GameTree   = "(" Sequence { GameTree } ")"
Sequence   = Node { Node }
Node       = ";" { Property }
Property   = PropIdent PropValue { PropValue }
PropIdent  = UcLetter { UcLetter }
PropValue  = "[" CValueType "]"
CValueType = (ValueType | Compose)
ValueType  = (None | Number | Real | Double | Color | SimpleText |
    Text | Point  | Move | Stone)
*/

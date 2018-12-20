use nom::types::CompleteStr;
use std::str::FromStr;

use crate::*;

#[allow(dead_code)]
fn str_to_coordinates(input: CompleteStr) -> Result<(u8, u8), std::string::ParseError> {
    if input.len() != 2 {
        return Ok((5, 5));
    }
    let coords = input
        .to_lowercase()
        .as_bytes()
        .iter()
        .map(convert_u8_to_coordinate)
        .take(2)
        .collect::<Vec<_>>();
    Ok((coords[0], coords[1]))
}

fn convert_u8_to_coordinate(c: &u8) -> u8 {
    let n = c - 96;
    if n >= 9 {
        n - 1
    } else {
        n
    }
}

#[allow(dead_code)]
fn str_to_integer(input: CompleteStr) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

macro_rules! create_string_parser {
    ($name:ident, $tag:expr, $token:expr) => {
        named!($name(CompleteStr) -> SgfToken,
        do_parse!(
            tag!($tag) >>
            value: parse_string_value >>
            ($token(value.to_string()))
        ));
    }
}

macro_rules! create_f32_parser {
    ($name:ident, $tag:expr, $token:expr) => {
        named!($name(CompleteStr) -> SgfToken,
        do_parse!(
            tag!($tag) >>
            value: parse_f32_value >>
            ($token(value))
        ));
    }
}

macro_rules! create_u32_parser {
    ($name:ident, $tag:expr, $token:expr) => {
        named!($name(CompleteStr) -> SgfToken,
        do_parse!(
            tag!($tag) >>
            value: parse_u32_value >>
            ($token(value))
        ));
    }
}

named!(parse_f32_value(CompleteStr) -> f32,
delimited!(tag!("["), call!(nom::float), tag!("]"))
);

named!(parse_u32_value(CompleteStr) -> u32,
delimited!(tag!("["), map_res!(nom::digit, |CompleteStr(s)| u32::from_str(s)), tag!("]"))
);

named!(parse_string_value(CompleteStr) -> CompleteStr,
do_parse!(
    tag!("[") >>
    value: take_until_and_consume!("]") >>
    (value)
));

create_string_parser!(parse_game_name, "GN", SgfToken::GameName);
create_string_parser!(parse_copyright, "CP", SgfToken::Copyright);
create_string_parser!(parse_event, "EV", SgfToken::Event);
create_string_parser!(parse_date, "DT", SgfToken::Date);
create_string_parser!(parse_place, "PC", SgfToken::Place);
create_string_parser!(parse_comment, "C", SgfToken::Comment);

create_f32_parser!(parse_komi, "KM", SgfToken::Komi);

create_u32_parser!(parse_size, "Size", SgfToken::Size);
create_u32_parser!(parse_time_limit, "TM", SgfToken::TimeLimit);

named!(parse_black_move(CompleteStr) -> SgfToken,
do_parse!(
    tag!("B") >>
    data: parse_string_value >>
    (SgfToken::Move(Move { stone: Stone::Black, coordinate: str_to_coordinates(data).unwrap() }))
));

named!(parse_black_time(CompleteStr) -> SgfToken,
do_parse!(
    tag!("BL") >>
    time: parse_u32_value >>
    (SgfToken::Time(Time { stone: Stone::Black, time: time }))
));

named!(parse_black_name(CompleteStr) -> SgfToken,
do_parse!(
    tag!("PB") >>
    value: parse_string_value >>
    (SgfToken::PlayerName(Player { stone: Stone::Black, name: value.to_string() }))
));

named!(parse_black_rank(CompleteStr) -> SgfToken,
do_parse!(
    tag!("BR") >>
    value: parse_string_value >>
    (SgfToken::PlayerRank(Rank { stone: Stone::Black, rank: value.to_string() }))
));

named!(parse_white_move(CompleteStr) -> SgfToken,
do_parse!(
    tag!("W") >>
    data: parse_string_value >>
    (SgfToken::Move(Move { stone: Stone::White, coordinate: str_to_coordinates(data).unwrap() }))
));

named!(parse_white_time(CompleteStr) -> SgfToken,
do_parse!(
    tag!("WL") >>
    time: parse_u32_value >>
    (SgfToken::Time(Time { stone: Stone::White, time: time }))
));

named!(parse_white_name(CompleteStr) -> SgfToken,
do_parse!(
    tag!("PW") >>
    value: parse_string_value >>
    (SgfToken::PlayerName(Player { stone: Stone::White, name: value.to_string() }))
));

named!(parse_white_rank(CompleteStr) -> SgfToken,
do_parse!(
    tag!("WR") >>
    value: parse_string_value >>
    (SgfToken::PlayerRank(Rank { stone: Stone::White, rank: value.to_string() }))
));

named!(parse_token(CompleteStr) -> SgfToken,
do_parse!(
    token: alt!(
        parse_game_name |
        parse_copyright |
        parse_event |
        parse_date |
        parse_place |
        parse_comment |
        parse_size |
        parse_time_limit |
        parse_black_move |
        parse_black_time |
        parse_black_name |
        parse_black_rank |
        parse_white_move |
        parse_white_time |
        parse_white_name |
        parse_white_rank |
        parse_komi) >>
    (token)
));

named!(parse_node(CompleteStr) -> SgfNode,
do_parse!(
    tag!(";") >>
    tokens: many1!(parse_token) >>
    (SgfNode {
        tokens,
        children: vec![],
    })
));

named!(parse_game_tree(CompleteStr) -> SgfGameTree,
do_parse!(
    root: delimited!(tag!("("), call!(parse_sgf_nodes), tag!(")")) >>
    (SgfGameTree {
        root
    })
));

named!(parse_game_trees(CompleteStr) -> Vec<SgfGameTree>,
    many1!(parse_game_tree)
);

fn parse_sgf_nodes(input: CompleteStr) -> Result<(CompleteStr, SgfNode), nom::Err<CompleteStr>> {
    let (output, mut node) = parse_node(input)?;
    let remainder = match parse_sgf_nodes(output) {
        Ok((rem, child)) => {
            node.children.push(child);
            rem
        }
        _ => output,
    };
    let remainder = match parse_game_trees(remainder) {
        Ok((rem, mut trees)) => {
            trees.drain(0..).for_each(|tree| { 
                node.children.push(tree.root);
            });
            rem
        }
        _ => remainder
    };

    Ok((remainder, node))
}

///
/// Parse input and return a `SgfGameTree`
///
pub fn parse(input: &str) -> Result<SgfGameTree, ()> {
    match parse_game_tree(CompleteStr(input)) {
        Ok((_, tree)) => Ok(tree),
        _ => Err(())
    }
}

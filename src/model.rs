#[derive(Debug, PartialEq)]
pub enum Stone {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub stone: Stone,
    pub coordinate: (u8, u8),
}

#[derive(Debug, PartialEq)]
pub struct Time {
    pub stone: Stone,
    pub time: u32,
}

#[derive(Debug, PartialEq)]
pub struct Player {
    pub stone: Stone,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Rank {
    pub stone: Stone,
    pub rank: String,
}

/// Enum describing all possible SGF Properties
#[derive(Debug, PartialEq)]
pub enum SgfToken {
    Move(Move),
    Time(Time),
    Komi(f32),
    PlayerName(Player),
    PlayerRank(Rank),
    Event(String),
    GameType(u32),
    Copyright(String),
    GameName(String),
    Place(String),
    Date(String),
    Size(u32),
    TimeLimit(u32),
    Comment(String),
}

/// A SGF Node, with information about the node and all possible children
#[derive(Debug, PartialEq)]
pub struct SgfNode {
    pub tokens: Vec<SgfToken>,
    pub children: Vec<SgfNode>,
}

/// Root game Tree
#[derive(Debug, PartialEq)]
pub struct SgfGameTree {
    pub root: SgfNode,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Stone {
    Black,
    White,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Move {
    pub stone: Stone,
    pub coordinate: (u8, u8),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Time {
    pub stone: Stone,
    pub time: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub stone: Stone,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rank {
    pub stone: Stone,
    pub rank: String,
}

/// Enum describing all possible SGF Properties
#[derive(Debug, PartialEq, Clone)]
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
    Unknown(String)
}

/// A SGF Node, with information about the node and all possible children
#[derive(Debug, PartialEq)]
pub struct SgfNode {
    pub tokens: Vec<SgfToken>,
    pub invalid: Vec<SgfToken>,
    pub children: Vec<SgfNode>,
}

/// Root game Tree
#[derive(Debug, PartialEq)]
pub struct SgfGameTree {
    pub root: SgfNode,
}

type BranchId = usize;

impl SgfGameTree {
    pub fn iter(&self) -> SgfGameTreeIterator {
        SgfGameTreeIterator {
            current: &self.root,
            next: Some(&self.root),
            branch: 0
        }
    }
}

pub struct SgfGameTreeIterator<'a> {
    current: &'a SgfNode,
    next: Option<&'a SgfNode>,
    branch: BranchId
}

impl<'a> Iterator for SgfGameTreeIterator<'a> {
    type Item = (&'a Vec<SgfToken>, &'a Vec<SgfToken>);

    fn next(&mut self) -> Option<(&'a Vec<SgfToken>, &'a Vec<SgfToken>)> {
        match self.next {
            None => None,
            Some(next) => {
                self.current = next;
                self.next = if self.current.children.is_empty() {
                    None
                } else {
                    Some(&self.current.children[self.branch])
                };
                Some((&self.current.tokens, &self.current.invalid))
            }
        }
    }
}

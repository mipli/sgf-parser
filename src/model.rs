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
    PlayerName(Player),
    PlayerRank(Rank),
    Komi(f32),
    Event(String),
    Copyright(String),
    GameName(String),
    Place(String),
    Date(String),
    Size(u32),
    TimeLimit(u32),
    Comment(String),
    Unknown((String, String))
}

impl SgfToken {
    pub fn from_pair(ident: &str, value: &str) -> SgfToken {
        let ident = ident.chars().filter(|c| c.is_uppercase()).collect::<String>();
        match ident.as_ref() {
            "B" => {
                SgfToken::Move(Move {
                    stone: Stone::Black,
                    coordinate: str_to_coordinates(value).unwrap()
                })
            },
            "BL" => {
                SgfToken::Time(Time {
                    stone: Stone::Black,
                    time: value.parse().unwrap()
                })
            },
            "PB" => {
                SgfToken::PlayerName(Player {
                    stone: Stone::Black,
                    name: value.to_string()
                })
            },
            "BR" => {
                SgfToken::PlayerRank(Rank {
                    stone: Stone::Black,
                    rank: value.to_string()
                })
            },
            "W" => {
                SgfToken::Move(Move {
                    stone: Stone::White,
                    coordinate: str_to_coordinates(value).unwrap()
                })
            },
            "WL" => {
                SgfToken::Time(Time {
                    stone: Stone::White,
                    time: value.parse().unwrap()
                })
            },
            "PW" => {
                SgfToken::PlayerName(Player {
                    stone: Stone::White,
                    name: value.to_string()
                })
            },
            "WR" => {
                SgfToken::PlayerRank(Rank {
                    stone: Stone::White,
                    rank: value.to_string()
                })
            },
            "KM" => {
                SgfToken::Komi(value.parse().expect("trying to unwrap komi value"))
            },
            "SZ" => {
                SgfToken::Size(value.parse().expect("trying to unwrap size value"))
            },
            "TM" => {
                SgfToken::TimeLimit(value.parse().expect("trying to unwrap time limit value"))
            },
            "EV" => {
                SgfToken::Event(value.to_string())
            },
            "C" => {
                SgfToken::Comment(value.to_string())
            },
            "GN" => {
                SgfToken::GameName(value.to_string())
            },
            "CR" => {
                SgfToken::Copyright(value.to_string())
            },
            "DT" => {
                SgfToken::Date(value.to_string())
            },
            "PC" => {
                SgfToken::Place(value.to_string())
            },
            _ => {
                SgfToken::Unknown((ident.to_string(), value.to_string()))
            }
        }
    }
}

fn str_to_coordinates(input: &str) -> Result<(u8, u8), std::string::ParseError> {
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

#[derive(Debug, PartialEq)]
pub struct SgfCollection {
    pub trees: Vec<GameTree>,
}

#[derive(Debug, PartialEq)]
pub struct GameNode {
    pub tokens: Vec<SgfToken>,
}

#[derive(Debug, PartialEq)]
pub struct GameTree {
    pub nodes: Vec<GameNode>,
    pub variations: Vec<GameTree>,
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Move {
    pub color: Color,
    pub coordinate: (u8, u8),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Time {
    pub color: Color,
    pub time: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub color: Color,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rank {
    pub color: Color,
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
    Unknown((String, String)),
    Invalid((String, String)),
}

impl SgfToken {
    pub fn from_pair(base_ident: &str, value: &str) -> SgfToken {
        let ident = base_ident.chars().filter(|c| c.is_uppercase()).collect::<String>();
        match ident.as_ref() {
            "B" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Move(Move {
                        color: Color::Black,
                        coordinate,
                    })
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "BL" => {
                if let Ok(time) = value.parse() {
                    SgfToken::Time(Time {
                        color: Color::Black,
                        time,
                    })
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "PB" => {
                SgfToken::PlayerName(Player {
                    color: Color::Black,
                    name: value.to_string()
                })
            },
            "BR" => {
                SgfToken::PlayerRank(Rank {
                    color: Color::Black,
                    rank: value.to_string()
                })
            },
            "W" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Move(Move {
                        color: Color::White,
                        coordinate,
                    })
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "WL" => {
                if let Ok(time) = value.parse() {
                    SgfToken::Time(Time {
                        color: Color::White,
                        time,
                    })
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "PW" => {
                SgfToken::PlayerName(Player {
                    color: Color::White,
                    name: value.to_string()
                })
            },
            "WR" => {
                SgfToken::PlayerRank(Rank {
                    color: Color::White,
                    rank: value.to_string()
                })
            },
            "KM" => {
                if let Ok(komi) = value.parse() {
                    SgfToken::Komi(komi)
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "SZ" => {
                if let Ok(size) = value.parse() {
                    SgfToken::Size(size)
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "TM" => {
                if let Ok(time) = value.parse() {
                    SgfToken::TimeLimit(time)
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
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
                SgfToken::Unknown((base_ident.to_string(), value.to_string()))
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
        .map(|&c| convert_u8_to_coordinate(c))
        .take(2)
        .collect::<Vec<_>>();
    Ok((coords[0], coords[1]))
}

fn convert_u8_to_coordinate(c: u8) -> u8 {
    let n = c - 96;
    if n >= 9 {
        n - 1
    } else {
        n
    }
}

#[derive(Debug, PartialEq)]
pub struct Collection {
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

impl Default for GameTree {
    fn default() -> Self {
        GameTree {
            nodes: vec![],
            variations: vec![]
        }
    }
}

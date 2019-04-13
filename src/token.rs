use crate::{SgfError, SgfErrorKind};

/// Indicates what color the token is related to
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

/// Enum describing all possible SGF Properties
#[derive(Debug, PartialEq, Clone)]
pub enum SgfToken {
    Add { color: Color, coordinate: (u8, u8) },
    Move { color: Color, coordinate: (u8, u8) },
    Time { color: Color, time: u32 },
    PlayerName { color: Color, name: String },
    PlayerRank { color: Color, rank: String },
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
    Square { coordinate: (u8, u8) },
    Triangle { coordinate: (u8, u8) },
    Label { label: String, coordinate: (u8, u8) },
}

impl SgfToken {
    /// Converts a `identifier` and `value` pair to a SGF token
    ///
    /// Returns `SgfToken::Unknown((identifier, value))` for tokens without a matching identifier
    ///
    /// Returns `SgfToken::Invalid((identifier, value))` for tokens with a matching identifier, but invalid value
    ///
    /// ```rust
    /// use sgf_parser::*;
    ///
    /// let token = SgfToken::from_pair("B", "aa");
    /// assert_eq!(token, SgfToken::Move { color: Color::Black, coordinate: (1, 1) });
    ///
    /// let token = SgfToken::from_pair("B", "not_coord");
    /// assert_eq!(token, SgfToken::Invalid(("B".to_string(), "not_coord".to_string())));
    ///
    /// let token = SgfToken::from_pair("FOO", "aa");
    /// assert_eq!(token, SgfToken::Unknown(("FOO".to_string(), "aa".to_string())));
    /// ```
    pub fn from_pair(base_ident: &str, value: &str) -> SgfToken {
        let ident = base_ident
            .chars()
            .filter(|c| c.is_uppercase())
            .collect::<String>();
        match ident.as_ref() {
            "LB" => {
                if value.len() >= 4 {
                    let (coord, label) = value.split_at(2);
                    if label.len() >= 2 {
                        if let Ok(coordinate) = str_to_coordinates(coord) {
                            return SgfToken::Label{
                                label: label[1..].to_string(),
                                coordinate,
                            };
                        }
                    }
                }
                SgfToken::Invalid((base_ident.to_string(), value.to_string()))
            },
            "SQ" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Square{
                        coordinate,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "TR" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Triangle{
                        coordinate,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "AB" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Add{
                        color: Color::Black,
                        coordinate,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "B" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Move{
                        color: Color::Black,
                        coordinate,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "BL" => {
                if let Ok(time) = value.parse() {
                    SgfToken::Time {
                        color: Color::Black,
                        time,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            },
            "PB" => SgfToken::PlayerName {
                color: Color::Black,
                name: value.to_string(),
            },
            "BR" => SgfToken::PlayerRank {
                color: Color::Black,
                rank: value.to_string(),
            },
            "AW" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Add{
                        color: Color::White,
                        coordinate,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            }
            "W" => {
                if let Ok(coordinate) = str_to_coordinates(value) {
                    SgfToken::Move {
                        color: Color::White,
                        coordinate,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            }
            "WL" => {
                if let Ok(time) = value.parse() {
                    SgfToken::Time {
                        color: Color::White,
                        time,
                    }
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            }
            "PW" => SgfToken::PlayerName {
                color: Color::White,
                name: value.to_string(),
            },
            "WR" => SgfToken::PlayerRank {
                color: Color::White,
                rank: value.to_string(),
            },
            "KM" => {
                if let Ok(komi) = value.parse() {
                    SgfToken::Komi(komi)
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            }
            "SZ" => {
                if let Ok(size) = value.parse() {
                    SgfToken::Size(size)
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            }
            "TM" => {
                if let Ok(time) = value.parse() {
                    SgfToken::TimeLimit(time)
                } else {
                    SgfToken::Invalid((base_ident.to_string(), value.to_string()))
                }
            }
            "EV" => SgfToken::Event(value.to_string()),
            "C" => SgfToken::Comment(value.to_string()),
            "GN" => SgfToken::GameName(value.to_string()),
            "CR" => SgfToken::Copyright(value.to_string()),
            "DT" => SgfToken::Date(value.to_string()),
            "PC" => SgfToken::Place(value.to_string()),
            _ => SgfToken::Unknown((base_ident.to_string(), value.to_string())),
        }
    }
}

impl Into<String> for &SgfToken {
    fn into(self) -> String {
        match self {
            SgfToken::Label{label, coordinate} => {
                let value = coordinate_to_str(*coordinate);
                format!("LB[{}:{}]", value, label)
            },
            SgfToken::Square{coordinate} => {
                let value = coordinate_to_str(*coordinate);
                format!("SQ[{}]", value)
            }
            SgfToken::Triangle{coordinate} => {
                let value = coordinate_to_str(*coordinate);
                format!("TR[{}]", value)
            }
            SgfToken::Add{color, coordinate} => {
                let token = match color {
                    Color::Black => "AB",
                    Color::White => "AW"
                };
                let value = coordinate_to_str(*coordinate);
                format!("{}[{}]", token, value)
            },
            SgfToken::Move{color, coordinate} => {
                let token = match color {
                    Color::Black => "B",
                    Color::White => "W"
                };
                let value = coordinate_to_str(*coordinate);
                format!("{}[{}]", token, value)
            },
            SgfToken::Time{color, time} => {
                let token = match color {
                    Color::Black => "BL",
                    Color::White => "WL"
                };
                format!("{}[{}]", token, time)
            },
            SgfToken::PlayerName{color, name} => {
                let token = match color {
                    Color::Black => "PB",
                    Color::White => "PW"
                };
                format!("{}[{}]", token, name)
            },
            SgfToken::PlayerRank{color, rank} => {
                let token = match color {
                    Color::Black => "BR",
                    Color::White => "WR"
                };
                format!("{}[{}]", token, rank)
            },
            SgfToken::Komi(komi) => format!("KM[{}]", komi),
            SgfToken::Size(size) => format!("SZ[{}]", size),
            SgfToken::TimeLimit(time) => format!("TM[{}]", time),
            SgfToken::Event(value) => format!("EV[{}]", value),
            SgfToken::Comment(value) => format!("C[{}]", value),
            SgfToken::GameName(value) => format!("GN[{}]", value),
            SgfToken::Copyright(value) => format!("CR[{}]", value),
            SgfToken::Date(value) => format!("DT[{}]", value),
            SgfToken::Place(value) => format!("PC[{}]", value),
            _ => panic!()
        }
    }
}

impl Into<String> for SgfToken {
    fn into(self) -> String {
        (&self).into()
    }
}

/// Converts goban coordinates to string representation
fn coordinate_to_str(coordinate: (u8, u8)) -> String {
    let conv = |n| {
        // skips 'I' as a valid coordinate
        n + if n >= 9 {
            97
        } else {
            96
        }
    };
    let x = conv(coordinate.0) as char;
    let y = conv(coordinate.1) as char;
    [x, y].iter().collect()
}

/// Converts a string describing goban coordinates to numeric coordinates
/// skips 'I' as a valid coordinate
fn str_to_coordinates(input: &str) -> Result<(u8, u8), SgfError> {
    if input.len() != 2 {
        return Err(SgfErrorKind::ParseError.into());
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

/// Converts a u8 char to numeric coordinates
fn convert_u8_to_coordinate(c: u8) -> u8 {
    let n = c - 96;
    // skips 'I' as a valid coordinate
    if n >= 9 {
        n - 1
    } else {
        n
    }
}


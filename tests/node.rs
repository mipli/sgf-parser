#[cfg(test)]
mod node_tests {
    use sgf_parser::*;

    #[test]
    fn can_convert_node_to_string() {
        let node = GameNode {
            tokens: vec![
                SgfToken::PlayerName {
                    color: Color::Black,
                    name: "black".to_string(),
                },
                SgfToken::PlayerName {
                    color: Color::White,
                    name: "white".to_string(),
                },
            ],
        };
        let string_node: String = node.into();
        assert_eq!(string_node, ";PB[black]PW[white]");
    }

    #[test]
    fn can_convert_node_with_multiple_of_same_property_to_string() {
        let node = GameNode {
            tokens: vec![
                SgfToken::Add {
                    color: Color::Black,
                    coordinate: (1, 1),
                },
                SgfToken::PlayerName {
                    color: Color::White,
                    name: "white".to_string(),
                },
                SgfToken::Add {
                    color: Color::Black,
                    coordinate: (2, 2),
                },
            ],
        };
        let string_node: String = node.into();
        assert_eq!(string_node, ";AB[aa][bb]PW[white]");
    }
}

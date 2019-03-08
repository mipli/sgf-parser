#[cfg(test)]
mod model_tests {
    use sgf_parser::*;

    #[test]
    fn can_get_unknown_nodes() {
        let tree: GameTree = parse("(;B[dc];W[ef]AB[23](;B[dd])(;AS[234]))").unwrap();
        let unknowns = tree.get_unknown_nodes();
        assert_eq!(unknowns.len(), 2);
        assert_eq!(
            *unknowns[0],
            GameNode {
                tokens: vec![
                    SgfToken::Move(Move {
                        color: Color::White,
                        coordinate: (5, 6)
                    }),
                    SgfToken::Unknown(("AB".to_string(), "23".to_string()))
                ]
            }
        );
        assert_eq!(
            *unknowns[1],
            GameNode {
                tokens: vec![SgfToken::Unknown(("AS".to_string(), "234".to_string()))]
            }
        );
    }

    #[test]
    fn can_get_invalid_nodes() {
        let tree: GameTree = parse("(;B[dc];W[foobar](;B[dd])(;B[234]))").unwrap();
        let unknowns = tree.get_invalid_nodes();
        assert_eq!(unknowns.len(), 2);
        assert_eq!(
            *unknowns[0],
            GameNode {
                tokens: vec![SgfToken::Invalid(("W".to_string(), "foobar".to_string()))]
            }
        );
        assert_eq!(
            *unknowns[1],
            GameNode {
                tokens: vec![SgfToken::Invalid(("B".to_string(), "234".to_string()))]
            }
        );
    }

    #[test]
    fn can_iterate_over_simple_tree() {
        let tree: GameTree = parse("(;B[dc];W[ef])").unwrap();
        let mut iter = tree.iter();

        assert_eq!(
            iter.next(),
            Some(&GameNode {
                tokens: vec![SgfToken::Move(Move {
                    color: Color::Black,
                    coordinate: (4, 3)
                })]
            })
        );
        assert_eq!(
            iter.next(),
            Some(&GameNode {
                tokens: vec![SgfToken::Move(Move {
                    color: Color::White,
                    coordinate: (5, 6)
                })]
            })
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_iterate_with_branch() {
        let tree: GameTree = parse("(;B[dc];W[ef](;B[aa])(;B[cc]))").unwrap();
        let mut iter = tree.iter();

        assert_eq!(
            iter.next(),
            Some(&GameNode {
                tokens: vec![SgfToken::Move(Move {
                    color: Color::Black,
                    coordinate: (4, 3)
                })]
            })
        );
        assert_eq!(
            iter.next(),
            Some(&GameNode {
                tokens: vec![SgfToken::Move(Move {
                    color: Color::White,
                    coordinate: (5, 6)
                })]
            })
        );
        assert_eq!(
            iter.next(),
            Some(&GameNode {
                tokens: vec![SgfToken::Move(Move {
                    color: Color::Black,
                    coordinate: (1, 1)
                })]
            })
        );
        assert_eq!(iter.next(), None);
    }
}

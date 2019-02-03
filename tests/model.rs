#[cfg(test)]
mod tests {
    use sgf_parser::*;

    #[test]
    fn can_iterate_over_simple_tree() {
        let tree: SgfGameTree = parse("(;B[dc];W[ef])").unwrap();
        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some((&vec![SgfToken::Move(
            Move {
                stone: Stone::Black,
                coordinate: (4, 3)
            }
        )], &vec![])));
        assert_eq!(iter.next(), Some((&vec![SgfToken::Move(
            Move {
                stone: Stone::White,
                coordinate: (5, 6)
            }
        )], &vec![])));
        assert_eq!(iter.next(), None);
    }
}

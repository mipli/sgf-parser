#[cfg(test)]
mod sgf_files_test {
    use sgf_parser::{Encoding, GameTree, SgfToken};
    #[test]
    fn parse_sfg() {
        let _g = sgf_parser::parse(include_str!("sgf/ShusakuvsInseki.sgf")).unwrap();
    }

    #[test]
    fn parse_iso_marked_sfg() {
        use std::fs::File;
        use std::io::Read;
        let mut file = File::open("tests/sgf/ShusakuvsInseki-iso.sgf").unwrap();
        let mut content = vec![];
        let _ = file.read_to_end(&mut content);

        // convert ISO-8859-1 byte string to UTF-8
        let sgf_string: String = content.iter().map(|&c| c as char).collect();

        let iso_game = sgf_parser::parse(&sgf_string).unwrap();
        assert!(has_iso_node(&iso_game));
        assert!(!has_utf_node(&iso_game));

        let output: String = iso_game.into();

        let utf_game = sgf_parser::parse(&output).unwrap();
        assert!(!has_iso_node(&utf_game));
        assert!(has_utf_node(&utf_game));
    }

    fn has_iso_node(tree: &GameTree) -> bool {
        tree.nodes[0].tokens.iter().any(|token| match token {
            SgfToken::Charset(Encoding::Other(enc)) if enc == "ISO-8859" => true,
            _ => false,
        })
    }

    fn has_utf_node(tree: &GameTree) -> bool {
        tree.nodes[0].tokens.iter().any(|token| match token {
            SgfToken::Charset(Encoding::UTF8) => true,
            _ => false,
        })
    }

    #[test]
    fn parse_sgf_with_empty_node() {
        let _g = sgf_parser::parse(include_str!("sgf/empty_node.sgf")).unwrap();
    }
}

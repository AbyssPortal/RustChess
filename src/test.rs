#[cfg(test)]
mod test {
    use crate::chess::chess::*;

    fn generic_test(moves: Vec<&str>, expected_output: &str) {
        let mut board = make_default_board();
        let mut buffer = Vec::<u8>::new();
        board.print_board(&mut buffer).unwrap();
        for move_text in moves {
            board
                .make_legal_move(
                    board.interpret_move(move_text).expect(
                        (String::from("move could not be interpreted: ") + move_text).as_str(),
                    ),
                )
                .expect("make_legal_move failed");
            board.print_board(&mut buffer).unwrap();
        }
        assert_eq!(
            String::from_utf8(buffer).expect("invalid utf8 printed"),
            expected_output
        );
    }

    #[test]
    fn kingside_castles_test() {
        let moves = vec!["e4", "e5", "Nf3", "Nf6", "Bc4", "Bc5", "O-O", "O-O"];
        let expected_out = "8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ♞ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ □ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ ♗ ■ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ ♘ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ♞ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ □ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ■ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ ♘ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ♝ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ♞ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ■ □ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ □ ■ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ ♘ ■ □ \n5 □ ■ ♗ ■ ♙ ■ □ ■ \n4 ■ □ ♝ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ♞ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ■ □ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ □ ■ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ ♘ ■ □ \n5 □ ■ ♗ ■ ♙ ■ □ ■ \n4 ■ □ ♝ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ♞ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ □ ♜ ♚ ■ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ■ ♖ ♔ □ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ ♘ ■ □ \n5 □ ■ ♗ ■ ♙ ■ □ ■ \n4 ■ □ ♝ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ♞ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ □ ♜ ♚ ■ \n  A B C D E F G H      Turn: White\n";
        generic_test(moves, expected_out)
    }

    #[test]
    fn check_test() {
        let moves = vec!["e4", "d5", "Ke2", "Bg4"];
        let expected_out = "8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♚ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ □ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ■ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ♗ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♚ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ □ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\nWhite is checked!\n";
        generic_test(moves, expected_out)
    }
    #[test]
    #[should_panic]
    fn check_test_try_illegal() {
        let moves = vec!["e4", "d5", "Ke2", "Bg4", "h3"];
        let expected_out = "8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♚ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ □ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ■ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ □ ♟ □ ♗ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♚ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ □ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\nWhite is checked!\n";
        generic_test(moves, expected_out)
    }

    #[test]
    fn queenside_castles_test() {
        let moves = vec!["d4","d5","Nc3","Nc6","Be3","Be6","Qd3","Qd6","O-O-O","O-O-O"];
        let expected_out = "8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ■ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ □ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ■ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ □ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ □ ■ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ■ ♝ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ■ □ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ □ ■ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ □ ♗ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ■ ♝ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ■ □ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ □ ■ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ □ ♗ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ♛ ♝ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ■ □ ■ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ □ ■ □ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ ♕ ♗ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ♛ ♝ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 ♜ ■ □ ■ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ □ ■ □ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ ♕ ♗ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ♛ ♝ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 □ ■ ♚ ♜ □ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ■ □ ♔ ♖ ■ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ■ ♙ ♙ ♙ ♙ \n6 ■ □ ♘ ♕ ♗ □ ■ □ \n5 □ ■ □ ♙ □ ■ □ ■ \n4 ■ □ ■ ♟ ■ □ ■ □ \n3 □ ■ ♞ ♛ ♝ ■ □ ■ \n2 ♟ ♟ ♟ □ ♟ ♟ ♟ ♟ \n1 □ ■ ♚ ♜ □ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n";
        generic_test(moves, expected_out)
    }

    #[test]
    fn fools_mate_test() {
        let moves = vec!["f3", "e5", "g4", "Qh4"];
        let expected_out = "8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ■ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ □ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ♟ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ □ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ■ □ ■ □ ■ □ \n3 □ ■ □ ■ □ ♟ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ □ ♟ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\n8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ■ □ ■ □ ♟ □ \n3 □ ■ □ ■ □ ♟ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ □ ■ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: Black\n8 ♖ ♘ ♗ □ ♔ ♗ ♘ ♖ \n7 ♙ ♙ ♙ ♙ □ ♙ ♙ ♙ \n6 ■ □ ■ □ ■ □ ■ □ \n5 □ ■ □ ■ ♙ ■ □ ■ \n4 ■ □ ■ □ ■ □ ♟ ♕ \n3 □ ■ □ ■ □ ♟ □ ■ \n2 ♟ ♟ ♟ ♟ ♟ □ ■ ♟ \n1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ \n  A B C D E F G H      Turn: White\nWhite is checkmated!\n";
        generic_test(moves, expected_out);
    }
}

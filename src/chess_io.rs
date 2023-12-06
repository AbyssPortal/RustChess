#[allow(dead_code)]
pub mod chess_io {

    //i fucking hate that i need to do this

    fn abc_to_position(letter: char) -> Option<usize> {
        return "abcdefghijlkmnopqrstuvwxyz".find(letter);
    }

    fn digit_to_position(letter: char) -> Option<usize> {
        return "123456789".find(letter);
    }

    use crate::chess::chess::*;
    impl Board {
        pub fn print_board(&self) {
            for i in (0..BOARD_SIZE).rev() {
                print!("{} ", i + 1);
                for j in 0..BOARD_SIZE {
                    match self.get_piece(i, j).unwrap() {
                        Some(piece) => {
                            print!("{} ", to_emoji(&piece))
                        }
                        None => {
                            print!(
                                "{}",
                                match (i + j) % 2 {
                                    1 => '🔲',
                                    0 => '⬛',
                                    _ => {
                                        panic!("unreachable");
                                    }
                                }
                            )
                        }
                    }
                }
                println!();
            }
            println!(
                "  A B C D E F G H      Turn: {}",
                self.get_turn().to_string()
            );
        }

        //interpert moves such as "Nf3" or "e4". cares for upper/lowercase.
        pub fn interpert_move(&self, move_text: &str) -> Result<ChessMove, AlgebraicChessError> {
            //TODO: completely incompatible with special cases like castling and promoting
            let mut chars = move_text.chars();
            let first_letter = chars.next();

            let move_data = match first_letter {
                None => {
                    return Err(AlgebraicChessError::ExpectedMoreError);
                }
                Some(letter) => {
                    match letter {
                        //TODO: add other pieces
                        _ => (
                            Piece {
                                kind: PieceKind::Pawn,
                                color: self.get_turn(),
                            },
                            digit_to_position(
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?,
                            )
                            .ok_or(AlgebraicChessError::UnexpectedCharError(letter))?,
                            abc_to_position(letter)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(letter))?,
                        ),
                    }
                }
            };

            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match self.get_piece(i, j) {
                        Ok(Some(piece)) => {
                            if piece == move_data.0 {
                                let potential_move = ChessMove {
                                    initial_row: i,
                                    initial_col: j,
                                    destination_row: move_data.1,
                                    destination_col: move_data.2,
                                };
                                if self
                                    .generate_moves(i, j)
                                    .expect("cannot error because of for")
                                    .contains(&potential_move)
                                {
                                    return Ok(potential_move);
                                }
                            }
                        }
                        Ok(None) => {}
                        Err(err) => {
                            panic!("{:?}", err);
                        }
                    }
                }
            }
            Err(AlgebraicChessError::IllegalMoveError)
        }
    }

    #[derive(Debug)]
    pub enum AlgebraicChessError {
        ExpectedMoreError,
        UnexpectedCharError(char),
        UnimplementedError,
        IllegalMoveError,
    }

    pub fn to_emoji(piece: &Piece) -> char {
        use Color::*;
        use PieceKind::*;
        match *piece {
            Piece {
                kind: Rook,
                color: Black,
            } => '♖',
            Piece {
                kind: Knight,
                color: Black,
            } => '♘',
            Piece {
                kind: Bishop,
                color: Black,
            } => '♗',
            Piece {
                kind: King,
                color: Black,
            } => '♔',
            Piece {
                kind: Queen,
                color: Black,
            } => '♕',
            Piece {
                kind: Pawn,
                color: Black,
            } => '♙',
            Piece {
                kind: Rook,
                color: White,
            } => '♜',
            Piece {
                kind: Knight,
                color: White,
            } => '♞',
            Piece {
                kind: Bishop,
                color: White,
            } => '♝',
            Piece {
                kind: King,
                color: White,
            } => '♚',
            Piece {
                kind: Queen,
                color: White,
            } => '♛',
            Piece {
                kind: Pawn,
                color: White,
            } => '♟',
            //♚♛♝♞♟♜♔♕♗♘♙♖
        }
    }
}

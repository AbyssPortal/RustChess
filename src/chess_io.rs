#[allow(dead_code)]
pub mod chess_io {

    use crate::chess::chess::*;
    use core::cmp::min;

    //i fucking hate that i need to do this

    fn abc_to_position(letter: char) -> Option<usize> {
        return "abcdefghijlkmnopqrstuvwxyz".find(letter);
    }

    fn digit_to_position(letter: char) -> Option<usize> {
        return "123456789".find(letter);
    }

    struct AlgebraicMoveData {
        moving_piece: Piece,
        destination_row: usize,
        destination_col: usize,
        origin_row: Option<usize>,
        origin_col: Option<usize>,
    }

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
                                "{} ",
                                match (i + j) % 2 {
                                    1 => '■',//'🔲',
                                    0 => '□',//'⬛',
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
            match (self.is_check, self.is_checkmate) {
                (_, Some(color)) => {
                    println!("{} is checkmated!", color.to_string())
                }
                (Some(color), None) => {
                    println!("{} is checked!", color.to_string())
                }
                (None, None) => {}
            }
        }

        //interpert moves such as "Nf3" or "e4". cares for upper/lowercase.
        pub fn interpert_move(&self, move_text: &str) -> Result<ChessMove, AlgebraicChessError> {
            use ChessMove::*;
            if move_text[0..min(move_text.len(), 5)].eq_ignore_ascii_case("O-O-O") {
                return Ok(Castling(Castles {
                    color: self.get_turn(),
                    side: CastleSide::QueenSide,
                }));
            } else if move_text[0..min(move_text.len(), 3)].eq_ignore_ascii_case("O-O") {
                return Ok(Castling(Castles {
                    color: self.get_turn(),
                    side: CastleSide::KingSide,
                }));
            }

            let mut chars = move_text
                .chars()
                .filter(|&c| c != 'x' && c != '+' && c != '#');
            //these have no meanings for the game  ^^^
            let first_letter = chars.next();

            let (move_data, required_promotion) = match first_letter {
                None => {
                    return Err(AlgebraicChessError::ExpectedMoreError);
                }
                Some(letter) => {
                    match letter {
                        //TODO: add other pieces
                        'N' | 'R' | 'B' | 'Q' | 'K' => (
                            {
                                let kind = match letter {
                                    'N' => PieceKind::Knight,
                                    'R' => PieceKind::Rook,
                                    'B' => PieceKind::Bishop,
                                    'Q' => PieceKind::Queen,
                                    'K' => PieceKind::King,
                                    _ => {
                                        panic!("inaccesible")
                                    }
                                };
                                let first =
                                    chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                                let second =
                                    chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                                self.generate_algebraic_move_data_from_squares(
                                    first,
                                    second,
                                    chars.next(),
                                    chars.next(),
                                    kind,
                                )?
                            },
                            None,
                        ),
                        _ => (
                            {
                                let second =
                                    chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                                match second.is_ascii_alphabetic() {
                                    true => self.generate_algebraic_move_data_from_squares(
                                        second,
                                        chars
                                            .next()
                                            .ok_or(AlgebraicChessError::ExpectedMoreError)?,
                                        None,
                                        None,
                                        PieceKind::Pawn,
                                    )?,
                                    false => self.generate_algebraic_move_data_from_squares(
                                        letter,
                                        second,
                                        chars.next(),
                                        chars.next(),
                                        PieceKind::Pawn,
                                    )?,
                                }
                            },
                            {
                                let mut reverse_iter = move_text
                                    .chars()
                                    .rev()
                                    .filter(|&c| c != 'x' && c != '+' && c != '#');
                                match (reverse_iter.next(), reverse_iter.next()) {
                                    (Some(letter), Some('=')) => kind_from_letter(letter),
                                    (_, _) => None,
                                }
                            },
                        ),
                    }
                }
            };

            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match self.get_piece(i, j) {
                        Ok(Some(piece)) => {
                            if piece == move_data.moving_piece {
                                match move_data.origin_row {
                                    Some(row) => {
                                        if row != i {
                                            continue;
                                        }
                                    }
                                    None => {}
                                }
                                match move_data.origin_col {
                                    Some(col) => {
                                        if col != j {
                                            continue;
                                        }
                                    }
                                    None => {}
                                }
                                let potential_move = match required_promotion {
                                    None => Normal(NormalChessMove {
                                        initial_row: i,
                                        initial_col: j,
                                        destination_row: move_data.destination_row,
                                        destination_col: move_data.destination_col,
                                    }),
                                    Some(kind) => Promotion(
                                        NormalChessMove {
                                            initial_row: i,
                                            initial_col: j,
                                            destination_row: move_data.destination_row,
                                            destination_col: move_data.destination_col,
                                        },
                                        kind,
                                    ),
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

        fn generate_algebraic_move_data_from_squares(
            &self,
            first: char,
            second: char,
            third: Option<char>,
            fourth: Option<char>,
            kind: PieceKind,
        ) -> Result<AlgebraicMoveData, AlgebraicChessError> {
            match third {
                None | Some('=') => Ok(AlgebraicMoveData {
                    moving_piece: Piece {
                        kind,
                        color: self.get_turn(),
                    },
                    destination_row: digit_to_position(second)
                        .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                    destination_col: abc_to_position(first)
                        .ok_or(AlgebraicChessError::UnexpectedCharError(first))?,
                    origin_row: None,
                    origin_col: None,
                }),
                Some(third_letter) => match fourth {
                    None | Some('=') => match first.is_ascii_alphabetic() {
                        true => Ok(AlgebraicMoveData {
                            moving_piece: Piece {
                                kind,
                                color: self.get_turn(),
                            },
                            destination_row: digit_to_position(third_letter)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(third_letter))?,
                            destination_col: abc_to_position(second)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                            origin_row: None,
                            origin_col: Some(
                                abc_to_position(first)
                                    .ok_or(AlgebraicChessError::UnexpectedCharError(first))?,
                            ),
                        }),
                        false => Ok(AlgebraicMoveData {
                            moving_piece: Piece {
                                kind,
                                color: self.get_turn(),
                            },
                            destination_row: digit_to_position(third_letter)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(third_letter))?,
                            destination_col: abc_to_position(second)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                            origin_row: Some(
                                digit_to_position(first)
                                    .ok_or(AlgebraicChessError::UnexpectedCharError(first))?,
                            ),
                            origin_col: None,
                        }),
                    },
                    Some(fourth_letter) => Ok(AlgebraicMoveData {
                        moving_piece: Piece {
                            kind,
                            color: self.get_turn(),
                        },
                        destination_row: digit_to_position(fourth_letter)
                            .ok_or(AlgebraicChessError::UnexpectedCharError(fourth_letter))?,
                        destination_col: abc_to_position(third_letter)
                            .ok_or(AlgebraicChessError::UnexpectedCharError(third_letter))?,
                        origin_row: Some(
                            digit_to_position(second)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                        ),
                        origin_col: Some(
                            abc_to_position(first)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(first))?,
                        ),
                    }),
                },
            }
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

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
            match self.is_check() {
                Some(color) => {
                    println!("{} is checked!", color.to_string())
                }
                None => {}
            }
        }

        //interpert moves such as "Nf3" or "e4". cares for upper/lowercase.
        pub fn interpert_move(&self, move_text: &str) -> Result<ChessMove, AlgebraicChessError> {
            use ChessMove::*;
            //TODO: completely incompatible with special cases like castling and promoting
            if move_text.to_uppercase() == "O-O" {
                return Ok(Castling(Castles{color: self.get_turn(), side: CastleSide::KingSide}))
            } 
            else if move_text.to_uppercase() == "O-O-O" {
                return Ok(Castling(Castles{color: self.get_turn(), side: CastleSide::QueenSide}))
            } 
            let mut chars = move_text.chars();
            let first_letter = chars.next();

            let move_data = match first_letter {
                None => {
                    return Err(AlgebraicChessError::ExpectedMoreError);
                }
                Some(letter) => {
                    match letter {
                        //TODO: add other pieces
                        'N' => {
                            let first =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            let second =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            self.generate_algebraic_move_data(first, second, PieceKind::Knight)?
                        }
                        'R' => {
                            let first =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            let second =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            self.generate_algebraic_move_data(first, second, PieceKind::Rook)?
                        }
                        'B' => {
                            let first =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            let second =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            self.generate_algebraic_move_data(first, second, PieceKind::Bishop)?
                        }
                        'Q' => {
                            let first =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            let second =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            self.generate_algebraic_move_data(first, second, PieceKind::Queen)?
                        }
                        'K' => {
                            let first =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            let second =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            self.generate_algebraic_move_data(first, second, PieceKind::King)?
                        }
                        _ => {
                            let second =
                                chars.next().ok_or(AlgebraicChessError::ExpectedMoreError)?;
                            self.generate_algebraic_move_data(letter, second, PieceKind::Pawn)?
                        }
                    }
                }
            };

            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match self.get_piece(i, j) {
                        Ok(Some(piece)) => {
                            if piece == move_data.0 {
                                let potential_move = Normal(NormalChessMove{
                                    initial_row: i,
                                    initial_col: j,
                                    destination_row: move_data.1,
                                    destination_col: move_data.2,
                                });
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

        fn generate_algebraic_move_data(
            &self,
            first: char,
            second: char,
            kind: PieceKind,
        ) -> Result<(Piece, usize, usize), AlgebraicChessError> {
            Ok((
                Piece {
                    kind,
                    color: self.get_turn(),
                },
                digit_to_position(second)
                    .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                abc_to_position(first).ok_or(AlgebraicChessError::UnexpectedCharError(first))?,
            ))
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

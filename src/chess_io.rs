#[allow(dead_code)]
pub mod chess_io {

    use crate::chess::chess::*;
    use text_io::read;

    //i fucking hate that i need to do this

    fn letter_to_position(letter: char) -> Option<usize> {
        return "abcdefgh".find(letter);
    }

    fn digit_to_position(letter: char) -> Option<usize> {
        return "123456789".find(letter);
    }
    fn position_to_letter(position: usize) -> char {
        match position {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        }
    }

    fn position_to_digit(position: usize) -> char {
        match position {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => ' ',
        }
    }

    fn kind_to_letter(kind: PieceKind) -> &'static str {
        match kind {
            PieceKind::Pawn => "",
            PieceKind::Knight => "N",
            PieceKind::Bishop => "B",
            PieceKind::Rook => "R",
            PieceKind::Queen => "Q",
            PieceKind::King => "K",
        }
    }

    struct AlgebraicMoveData {
        moving_piece: Piece,
        destination_row: usize,
        destination_col: usize,
        origin_row: Option<usize>,
        origin_col: Option<usize>,
    }

    impl ChessMove {
        pub fn name(&self, board: &Board) -> Result<String, BoardError> {
            let res = match *self {
                ChessMove::Normal(normal_move) => {
                    let kind = match board.get_piece(normal_move.initial_row, normal_move.initial_col)? {
                        Some(piece) => {
                            kind_to_letter(piece.kind)
                        }
                        None => {
                            return Err(BoardError::NoPieceError);
                        }
                    };
                    let a = position_to_letter(normal_move.initial_col);
                    let b = position_to_digit(normal_move.initial_row);
                    let c = position_to_letter(normal_move.destination_col);
                    let d = position_to_digit(normal_move.destination_row);
                    let takes = match board.get_piece(normal_move.destination_row, normal_move.destination_col)?.is_none() {
                        false => "x",
                        true => ""
                    };
                    format!("{}{}{}{}{}{}", kind,  a, b, takes, c, d)
                }
                Self::Castling(castles) => match castles.side {
                    CastleSide::KingSide => {
                        String::from("O-O")
                    },
                    CastleSide::QueenSide => {
                        String::from("O-O-O")
                    }
                },
                ChessMove::Promotion(normal_move, promotion_type) => {
                    let piece_name = kind_to_letter(promotion_type);
                    ChessMove::Normal(normal_move).name(board)? + "=" + piece_name
                }
            };
            Ok(res)
        }
    }

    impl Board {
        /// Prints the current state of the chess board to a given output stream.
        ///
        /// # Arguments
        ///
        /// * `output` - A mutable reference to an output stream that implements `std::io::Write`.
        ///
        /// # Returns
        ///
        /// * `Result<(), std::io::Error>` - Returns `Ok(())` if the board was successfully printed, or an `Err` with the associated `std::io::Error` if an error occurred.
        ///
        /// # Behavior
        ///
        /// This function iterates over the chess board in reverse row order (from 8 to 1) and column order (from A to H). For each cell, it prints the emoji representation of the piece if there is one, or a square emoji representing the color of the cell if there is no piece. The color of the square depends on whether the sum of the row and column indices is even or odd, creating a checkerboard pattern.
        ///
        /// After printing all cells in a row, it prints a newline character. After printing all rows, it prints the column labels (A to H) and the current turn.
        ///
        /// If the game is in checkmate, it prints a message stating which color is checkmated. If the game is in check but not checkmate, it prints a message stating which color is in check. If the game is neither in check nor checkmate, it does nothing.
        ///
        /// Finally, it flushes the output stream to ensure that all buffered output is actually written to the underlying stream. This is important for `Write` implementations that may buffer output for efficiency, such as file or network streams.
        ///
        /// # Panics
        ///
        /// This function panics if the sum of the row and column indices is not 0 or 1, which should be unreachable.
        ///
        /// # Examples
        ///
        /// ```
        /// let mut game = Board::new();
        /// let mut output = String::new();
        /// game.print_board(&mut output).unwrap();
        /// println!("{}", output);
        /// ```
        pub fn print_board<W>(&self, output: &mut W) -> Result<(), std::io::Error>
        where
            W: std::io::Write,
        {
            for i in (0..BOARD_SIZE).rev() {
                write!(output, "{} ", i + 1)?;
                for j in 0..BOARD_SIZE {
                    match self.get_piece(i, j).unwrap() {
                        Some(piece) => {
                            write!(output, "{} ", to_emoji(&piece))?;
                        }
                        None => {
                            write!(
                                output,
                                "{} ",
                                match (i + j) % 2 {
                                    1 => '■', //'🔲',
                                    0 => '□', //'⬛',
                                    _ => {
                                        panic!("unreachable");
                                    }
                                }
                            )?;
                        }
                    }
                }
                writeln!(output)?;
            }
            writeln!(
                output,
                "  A B C D E F G H      Turn: {}",
                self.get_turn().to_string()
            )?;
            match (self.is_check, self.is_checkmate) {
                (_, Some(game_end_condition)) => {
                    writeln!(output, "{}", game_end_condition.to_string())?;
                }
                (Some(color), None) => {
                    writeln!(output, "{} is checked!", color.to_string())?;
                }
                (None, None) => {}
            }
            output.flush()
        }

        //interpret moves such as "Nf3" or "e4". cares for upper/lowercase.
        pub fn interpret_move(&self, move_text: &str) -> Result<ChessMove, AlgebraicChessError> {
            use ChessMove::*;
            if move_text.to_ascii_uppercase().starts_with("O-O-O") {
                return Ok(Castling(Castles {
                    color: self.get_turn(),
                    side: CastleSide::QueenSide,
                }));
            } else if move_text.to_ascii_uppercase().starts_with("O-O") {
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
                                        panic!("inaccessible")
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
                                self.generate_algebraic_move_data_from_squares(
                                    letter,
                                    second,
                                    chars.next(),
                                    chars.next(),
                                    PieceKind::Pawn,
                                )?
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
                    destination_col: letter_to_position(first)
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
                            destination_col: letter_to_position(second)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                            origin_row: None,
                            origin_col: Some(
                                letter_to_position(first)
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
                            destination_col: letter_to_position(second)
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
                        destination_col: letter_to_position(third_letter)
                            .ok_or(AlgebraicChessError::UnexpectedCharError(third_letter))?,
                        origin_row: Some(
                            digit_to_position(second)
                                .ok_or(AlgebraicChessError::UnexpectedCharError(second))?,
                        ),
                        origin_col: Some(
                            letter_to_position(first)
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

    pub fn chess_game() {
        let mut def_board = Board::new();
        let mut output = std::io::stdout();

        while def_board.is_checkmate.is_none() {
            def_board.print_board(&mut output).unwrap();
            let move_string: String = read!();
            match def_board.interpret_move(&move_string) {
                Ok(chess_move) => match def_board.make_legal_move(chess_move) {
                    Err(BoardError::IllegalMoveError) => {
                        println!("That's illegal silly!");
                    }
                    Err(BoardError::NoPieceError) => {
                        println!("There's no piece there silly!");
                    }
                    Err(BoardError::OutOfBoundsError) => {
                        print!("That's out of bounds silly!");
                    }
                    Err(BoardError::WrongTurnError) => {
                        print!("Wrong turn silly!");
                    }
                    Ok(()) => {}
                },
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
        def_board.print_board(&mut output).unwrap();
    }
}

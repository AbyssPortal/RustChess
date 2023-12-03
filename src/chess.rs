#[allow(dead_code)]
pub mod chess {
    use core::panic;
    use std::vec;

    pub const BOARD_SIZE: usize = 8;

    #[derive(Copy, Clone, Debug)]
    pub struct Piece {
        pub kind: PieceKind,
        pub color: Color,
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Color {
        White,
        Black,
    }

    #[derive(Copy, Clone, Debug)]
    pub enum PieceKind {
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }

    #[derive(Debug)]
    pub struct Board {
        squares: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
        turn: Color,
        castle_rights: CastleRights,
    }

    #[derive(Debug)]
    pub enum BoardError {
        OutOfBoundsError,
    }

    #[derive(Debug)]

    pub struct ChessMove {
        initial_row: usize,
        initial_col: usize,
        destination_row: usize,
        destination_col: usize,
    }

    fn in_bounds(row: usize, col: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE
    }

    #[test]
    fn test_rook_moves() {
        let board = make_board_from_fen("8/8/8/4R3/8/8/8/8/ w KQkq - 0 1").unwrap();
        board.print_board();
        let res = board.generate_moves(4, 4);
        println!("{:?}", res.unwrap().unwrap().len());
    }

    impl Board {
        pub fn get_piece(&self, row: usize, col: usize) -> Result<Option<Piece>, BoardError> {
            if !in_bounds(row, col) {
                return Err(BoardError::OutOfBoundsError);
            }
            Ok(self.squares[row][col])
        }

        //returns a result (which errors in case of an error)
        //of an option (which is empty if there is no piece at the selected coordinates)
        //of a vector
        //of the moves
        fn generate_moves(
            &self,
            origin_row: usize,
            origin_col: usize,
        ) -> Result<Option<Vec<ChessMove>>, BoardError> {
            use PieceKind::*;
            let piece_option = self.get_piece(origin_row, origin_col)?;
            match piece_option {
                Some(piece) => {
                    let res: Vec<ChessMove> = match piece {
                        Piece { kind: Rook, color } => {
                            //right, up, left, down
                            let mut moves =
                                self.generate_linear_moves(origin_row, origin_col, 1, 0, color);
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, 0, 1, color),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, -1, 0, color),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, 0, -1, color),
                            );
                            moves
                        }
                        _ => {
                            panic!("dunno what this is");
                        }
                    };
                    return Ok(Some(res));
                }

                None => return Ok(None),
            }
        }
        fn generate_linear_moves(
            &self,
            origin_row: usize,
            origin_col: usize,
            offset_x: i32,
            offset_y: i32,
            color: Color,
        ) -> Vec<ChessMove> {
            let mut res: Vec<ChessMove> = Vec::new();
            for i in 1..BOARD_SIZE {
                let potential_move = ChessMove {
                    initial_row: origin_row,
                    initial_col: origin_col,
                    destination_row: ((origin_row as i32)+ (i as i32) * offset_x) as usize,
                    destination_col: ((origin_row as i32)+ (i as i32) * offset_y) as usize,
                };
                match self.get_piece(potential_move.destination_row, potential_move.destination_col) {
                    Ok(piece_option) => match piece_option {
                        Some(Piece {
                            kind: _,
                            color: other_color,
                        }) => {
                            if other_color != color {
                                res.push(potential_move)
                            }
                            break;
                        }
                        None => {}
                    },
                    Err(_) => {
                        break;
                    }
                }

                res.push(potential_move);

            }
            return res;
        }
    }

    #[derive(Debug)]
    struct CastleRights {
        white: OneSidedCastleRights,
        black: OneSidedCastleRights,
    }
    #[derive(Debug)]

    struct OneSidedCastleRights {
        king: bool,
        queen: bool,
    }

    #[derive(Debug)]
    pub enum FenError {
        //Content: the character in question
        UnrecognizedCharacter(char),
        RowUnfinished,
        NotEnoughLines,
        LineTooLong,
        TooManyLines,
        MetaDataError,
    }

    fn make_default_board() -> Board {
        let board =
            make_board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/ w KQkq - 0 1");
        match board {
            Ok(res) => {
                return res;
            }
            Err(_) => {
                panic!("make_default_board test failed horrifically.");
            }
        }
    }

    pub fn make_board_from_fen(fen: &str) -> Result<Board, FenError> {
        use Color::*;

        let mut string_split = fen.split(&['\\', '/']);

        let string_rows = string_split.clone().take(BOARD_SIZE);

        let mut rows = string_rows.map(|row| string_to_row(row.to_string()));

        let meta_data = string_split
            .nth(BOARD_SIZE)
            .ok_or(FenError::MetaDataError)?;

        let mut squares = [[None; BOARD_SIZE]; BOARD_SIZE];

        for row in squares.as_mut() {
            *row = match rows.next() {
                Some(some_row) => some_row?,
                None => return Err(FenError::NotEnoughLines),
            }
        }

        squares.reverse();

        let res = Board {
            squares: squares,
            turn: match meta_data.contains("w") {
                true => White,
                false => Black,
            },
            castle_rights: CastleRights {
                white: (OneSidedCastleRights {
                    king: meta_data.contains("K"),
                    queen: meta_data.contains("Q"),
                }),
                black: (OneSidedCastleRights {
                    king: meta_data.contains("k"),
                    queen: meta_data.contains("q"),
                }),
            },
        };

        Ok(res)
    }

    fn string_to_row(row_string: String) -> Result<[Option<Piece>; BOARD_SIZE], FenError> {
        use Color::*;
        use PieceKind::*;

        let mut res: [Option<Piece>; BOARD_SIZE] = [None; BOARD_SIZE];

        let mut letters = row_string.chars();

        let mut letter_option = letters.next();

        for square in res.iter_mut() {
            match letter_option {
                Some(c) => {
                    if c.is_ascii_alphabetic() {
                        let kind = match c.to_ascii_lowercase() {
                            'k' => King,
                            'n' => Knight,
                            'b' => Bishop,
                            'q' => Queen,
                            'p' => Pawn,
                            'r' => Rook,
                            _ => {
                                return Err(FenError::UnrecognizedCharacter(c));
                            }
                        };
                        let color = match c.is_uppercase() {
                            true => White,
                            false => Black,
                        };
                        *square = Some(Piece {
                            kind: kind,
                            color: color,
                        });
                        letter_option = letters.next();
                    } else if c.is_ascii_digit() {
                        //this is safe.
                        let number = c.to_digit(10).unwrap();
                        *square = None;
                        if number == 1 {
                            letter_option = letters.next();
                        } else {
                            //again, this is safe.
                            letter_option = char::from_digit(number - 1, 10)
                        }
                    } else {
                        return Err(FenError::UnrecognizedCharacter(c));
                    }
                }
                None => {
                    return Err(FenError::RowUnfinished);
                }
            }
        }

        Ok(res)
    }


}

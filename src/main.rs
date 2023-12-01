fn main() {
    println!("Among us");
}

#[allow(dead_code)]
mod chess {

    const BOARD_SIZE: usize = 8;

    #[derive(Copy, Clone)]
    struct Piece {
        kind: PieceKind,
        color: Color,
    }

    #[derive(Copy, Clone)]
    enum Color {
        White,
        Black,
    }

    #[derive(Copy, Clone)]
    enum PieceKind {
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }

    struct Board {
        squares: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
        turn: Color,
        castle_rights: CastleRights,
    }

    struct CastleRights {
        white: OneSidedCastleRights,
        black: OneSidedCastleRights,
    }

    struct OneSidedCastleRights {
        king: bool,
        queen: bool,
    }

    enum FenError {
        //Content: the character in question
        UnrecognizedCharacter(char),
        NotEnoughLines,
        LineTooLong,
        TooManyLines,
    }

    fn make_board_from_fen(fen: &str) -> Result<Board, FenError> {
        use Color::*;
        use PieceKind::*;
        let mut res = Board {
            squares: [[None; BOARD_SIZE]; BOARD_SIZE],
            turn: White,
            castle_rights: CastleRights {
                white: (OneSidedCastleRights {
                    king: (true),
                    queen: (true),
                }),
                black: (OneSidedCastleRights {
                    king: (true),
                    queen: (true),
                }),
            },
        };
        let mut row = BOARD_SIZE - 1;
        let mut col = 0;
        let mut done_with_pieces = false;
        for letter in fen.chars() {
            if !done_with_pieces {
                if letter.is_ascii_alphabetic() {
                    let color = match letter.is_ascii_uppercase() {
                        true => White,
                        false => Black,
                    };
                    let kind = match letter.to_ascii_lowercase() {
                        'r' => Rook,
                        'b' => Bishop,
                        'k' => King,
                        'n' => Knight,
                        'q' => Queen,
                        'p' => Pawn,
                        _ => return Err(FenError::UnrecognizedCharacter(letter)),
                    };
                    res.squares[row][col] = Some(Piece {
                        color: color,
                        kind: kind,
                    });
                    col += 1;
                    if col >= BOARD_SIZE {
                        if row == 0 {
                            done_with_pieces = true;
                        }
                        else {
                            row -= 1;
                        }
                    }
                }
                else {
                    match letter {
                        '/' | '\\' => {
                            if row == 0 {
                                done_with_pieces = true;
                            }
                            else {
                                row -= 1;
                            }
                        }
                        ' ' => (),

                        _ => return Err(FenError::UnrecognizedCharacter(letter)),
                    }
                }

            }
            else {
                    
            }
        }

        Ok(res)
    }
}

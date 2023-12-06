#[allow(dead_code)]
pub mod chess_io {
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
            println!("  A B C D E F G H      Turn: {}", self.get_turn().to_string());
        }
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

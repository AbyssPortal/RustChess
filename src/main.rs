use text_io::*;

mod chess;
mod chess_io;


fn main() {
    use chess::chess::*;
    let mut def_board = make_board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/ w KQkq - 0 1").unwrap();
    loop {
        def_board.print_board();
        match def_board
            .make_legal_move(ChessMove {
                initial_row: read!(),
                initial_col: read!(),
                destination_row: read!(),
                destination_col: read!(),
            }) {
                Err(BoardError::IllegalMoveError) => {println!("That's illegal silly!");}
                Err(BoardError::NoPieceError) => {println!("There's no piece there silly!");}
                Err(BoardError::OutOfBoundsError) => {print!("That's out of bounds silly!");}
                Ok(()) => {}
            }
    }
}

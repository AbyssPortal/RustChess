

use text_io::*;

mod test;
mod chess;
mod chess_io;

fn main() {
    use chess::chess::*;
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
                Ok(()) => {}
            },
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
    def_board.print_board(&mut output).unwrap();
}

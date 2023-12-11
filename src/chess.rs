#[allow(dead_code)]
pub mod chess {
    use core::panic;
    use std::fmt::Display;

    pub const BOARD_SIZE: usize = 8;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Piece {
        pub kind: PieceKind,
        pub color: Color,
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Color {
        White,
        Black,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = String::from(match self {
                Self::White => "White",
                Self::Black => "Black",
            });
            write!(f, "{}", str)
        }
    }

    impl Color {
        pub fn opposite(&self) -> Color {
            match self {
                Color::White => Color::Black,
                Color::Black => Color::White,
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PieceKind {
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }

    pub fn kind_from_letter(letter: char) -> Option<PieceKind> {
        use PieceKind::*;
        match letter.to_ascii_lowercase() {
            'k' => Some(King),
            'n' => Some(Knight),
            'b' => Some(Bishop),
            'q' => Some(Queen),
            'p' => Some(Pawn),
            'r' => Some(Rook),
            _ => None,
        }
    }

    #[derive(Debug, Clone)]
    pub struct Board {
        squares: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
        turn: Color,
        castle_rights: CastleRights,
        pub is_check: Option<Color>,
        pub is_checkmate: Option<Color>,
    }

    #[derive(Debug)]
    pub enum BoardError {
        OutOfBoundsError,
        IllegalMoveError,
        NoPieceError,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum CastleSide {
        KingSide,
        QueenSide,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Castles {
        pub color: Color,
        pub side: CastleSide,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]

    //TODO: turn into enum with moves such as castles and en passent
    pub enum ChessMove {
        Normal(NormalChessMove),
        Castling(Castles),
        Promotion(NormalChessMove, PieceKind),
    }
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct NormalChessMove {
        pub initial_row: usize,
        pub initial_col: usize,
        pub destination_row: usize,
        pub destination_col: usize,
    }

    fn in_bounds(row: usize, col: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE
    }

    #[test]
    fn test_rook_moves() {
        let board = make_board_from_fen("8/8/8/4R3/8/8/8/8/ w KQkq - 0 1").unwrap();
        board.print_board();
        let res = board.generate_moves(4, 4);
        println!("{:?}", res.unwrap().len());
    }

    struct PromotionIterator<I>
    where
        I: Iterator<Item = char>,
    {
        base_move: NormalChessMove,
        pieces_iterator: I,
    }

    impl<I> Iterator for PromotionIterator<I>
    where
        I: Iterator<Item = char>,
    {
        type Item = ChessMove;
        fn next(&mut self) -> Option<ChessMove> {
            match kind_from_letter(self.pieces_iterator.next()?)? {
            PieceKind::Pawn => 
            Some(ChessMove::Normal(self.base_move)),
            other => {
                Some(ChessMove::Promotion(
                    self.base_move,
                    other,
                ))
            }
        }
        }
    }

    impl Board {
        pub fn get_piece(&self, row: usize, col: usize) -> Result<Option<Piece>, BoardError> {
            if !in_bounds(row, col) {
                return Err(BoardError::OutOfBoundsError);
            }
            Ok(self.squares[row][col])
        }

        pub fn get_turn(&self) -> Color {
            return self.turn;
        }

        //the function used to make moves from outside the module.
        pub fn make_legal_move(&mut self, chess_move: ChessMove) -> Result<(), BoardError> {
            let res = self.make_legal_move_inner(chess_move);
            self.update_flags();
            return res;
        }

        fn update_flags(&mut self) {
            self.is_check = self.test_if_check();
            self.is_checkmate = self.test_if_checkmate();
        }

        //does not update check or checkmate flags.
        fn make_legal_move_inner(&mut self, chess_move: ChessMove) -> Result<(), BoardError> {
            match chess_move {
                ChessMove::Normal(normal_move) | ChessMove::Promotion(normal_move, _) => {
                    let piece_option =
                        self.get_piece(normal_move.initial_row, normal_move.initial_col)?;
                    match piece_option {
                        Some(Piece { kind: _, color }) => {
                            if color != self.turn {
                                return Err(BoardError::IllegalMoveError);
                            }
                        }
                        None => {
                            return Err(BoardError::NoPieceError);
                        }
                    }
                    let moves =
                        self.generate_moves(normal_move.initial_row, normal_move.initial_col)?;
                    if !moves.contains(&chess_move) {
                        return Err(BoardError::IllegalMoveError);
                    }
                }
                ChessMove::Castling(castle_move) => {
                    let row = match castle_move.color {
                        Color::White => 0,
                        Color::Black => BOARD_SIZE - 1,
                    };
                    match castle_move.side {
                        CastleSide::QueenSide => {
                            if self.squares[row][1].is_some()
                                || self.squares[row][2].is_some()
                                || self.squares[row][3].is_some()
                                || !self.castle_rights.at(self.turn, CastleSide::QueenSide)
                                || self.is_check.is_some()
                                || self.is_square_attacked(row, 3, self.turn)
                            {
                                return Err(BoardError::IllegalMoveError);
                            }
                        }
                        CastleSide::KingSide => {
                            if self.squares[row][5].is_some()
                                || self.squares[row][6].is_some()
                                || !self.castle_rights.at(self.turn, CastleSide::KingSide)
                                || self.is_check.is_some()
                                || self.is_square_attacked(row, 5, self.turn)
                            {
                                return Err(BoardError::IllegalMoveError);
                            }
                        }
                    }
                }
            }
            let old_board = self.clone();

            self.make_move(chess_move)?;
            if self.is_check_specific_color(self.turn.opposite()) {
                *self = old_board;
                return Err(BoardError::IllegalMoveError);
            }
            return Ok(());
        }

        fn make_move(&mut self, chess_move: ChessMove) -> Result<(), BoardError> {
            match chess_move {
                ChessMove::Normal(normal_move) => {
                    self.squares[normal_move.destination_row][normal_move.destination_col] =
                        self.squares[normal_move.initial_row][normal_move.initial_col];
                    self.squares[normal_move.initial_row][normal_move.initial_col] = None;
                    //if we're in the white row and moved the starting locations
                    //TODO: chess960 or smth

                    if normal_move.initial_row == 0
                        && (normal_move.initial_col == 4 || normal_move.initial_col == 0)
                    {
                        self.castle_rights.white.queen = false;
                    }
                    if normal_move.initial_row == 0
                        && (normal_move.initial_col == 4 || normal_move.initial_col == 7)
                    {
                        self.castle_rights.white.king = false;
                    }
                    if normal_move.initial_row == 7
                        && (normal_move.initial_col == 4 || normal_move.initial_col == 0)
                    {
                        self.castle_rights.black.queen = false;
                    }
                    if normal_move.initial_row == 7
                        && (normal_move.initial_col == 4 || normal_move.initial_col == 7)
                    {
                        self.castle_rights.black.king = false;
                    }
                }
                ChessMove::Castling(castle_move) => {
                    let row = match castle_move.color {
                        Color::White => 0,
                        Color::Black => BOARD_SIZE - 1,
                    };
                    match castle_move.side {
                        //TODO: it's illegal to castle through check
                        CastleSide::QueenSide => {
                            self.squares[row][0] = None;
                            self.squares[row][2] = Some(Piece {
                                kind: PieceKind::King,
                                color: castle_move.color,
                            });
                            self.squares[row][3] = Some(Piece {
                                kind: PieceKind::Rook,
                                color: castle_move.color,
                            });
                            self.squares[row][4] = None;
                        }
                        CastleSide::KingSide => {
                            self.squares[row][7] = None;
                            self.squares[row][6] = Some(Piece {
                                kind: PieceKind::King,
                                color: castle_move.color,
                            });
                            self.squares[row][5] = Some(Piece {
                                kind: PieceKind::Rook,
                                color: castle_move.color,
                            });
                            self.squares[row][4] = None;
                        }
                    }
                }
                ChessMove::Promotion(normal_move, kind) => {
                    self.squares[normal_move.destination_row][normal_move.destination_col] =
                        Some(Piece {
                            kind,
                            color: self.squares[normal_move.initial_row][normal_move.initial_col]
                                .ok_or(BoardError::NoPieceError)?
                                .color,
                        });
                    self.squares[normal_move.initial_row][normal_move.initial_col] = None;
                }
            }
            self.turn = self.turn.opposite();
            Ok(())
        }

        //returns a result (which errors in case of an error)
        //of an option (which is empty if there is no piece at the selected coordinates)
        //of a vector
        //of the moves
        pub fn generate_moves(
            &self,
            origin_row: usize,
            origin_col: usize,
        ) -> Result<Vec<ChessMove>, BoardError> {
            use ChessMove::*;
            use PieceKind::*;
            let piece_option = self.get_piece(origin_row, origin_col)?;
            match piece_option {
                Some(piece) => {
                    Ok(match piece {
                        Piece { kind: Rook, color } => {
                            //right, up, left, down
                            let mut moves = self.generate_linear_moves(
                                origin_row, origin_col, 1, 0, color, BOARD_SIZE,
                            );
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 0, 1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, -1, 0, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 0, -1, color, BOARD_SIZE,
                            ));
                            moves
                        }
                        Piece {
                            kind: Bishop,
                            color,
                        } => {
                            let mut moves = self.generate_linear_moves(
                                origin_row, origin_col, 1, 1, color, BOARD_SIZE,
                            );
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 1, -1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, -1, 1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, -1, -1, color, BOARD_SIZE,
                            ));
                            moves
                        }
                        Piece { kind: Queen, color } => {
                            //right, up, left, down
                            let mut moves = self.generate_linear_moves(
                                origin_row, origin_col, 1, 1, color, BOARD_SIZE,
                            );
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 1, -1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, -1, 1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, -1, -1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 1, 0, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 0, 1, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, -1, 0, color, BOARD_SIZE,
                            ));
                            moves.extend(self.generate_linear_moves(
                                origin_row, origin_col, 0, -1, color, BOARD_SIZE,
                            ));
                            moves
                        }
                        Piece { kind: King, color } => {
                            //right, up, left, down
                            let mut moves =
                                self.generate_linear_moves(origin_row, origin_col, 1, 1, color, 1);
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, 1, -1, color, 1),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, -1, 1, color, 1),
                            );
                            moves.extend(
                                self.generate_linear_moves(
                                    origin_row, origin_col, -1, -1, color, 1,
                                ),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, 1, 0, color, 1),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, 0, 1, color, 1),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, -1, 0, color, 1),
                            );
                            moves.extend(
                                self.generate_linear_moves(origin_row, origin_col, 0, -1, color, 1),
                            );
                            moves
                        }
                        Piece {
                            kind: Knight,
                            color,
                        } => {
                            let mut moves = Vec::<ChessMove>::new();
                            for i in (-2i32)..=2 {
                                for j in (-2i32)..=2 {
                                    if i.abs() == j.abs() || i == 0 || j == 0 {
                                        continue;
                                    }
                                    let chess_move = self.try_move(
                                        origin_row,
                                        origin_col,
                                        (i + origin_row as i32) as usize,
                                        (j + origin_col as i32) as usize,
                                        color,
                                    );
                                    match chess_move {
                                        Some(some_move) => {
                                            moves.push(Normal(some_move));
                                        }
                                        None => {}
                                    }
                                }
                            }
                            moves
                        }
                        Piece { kind: Pawn, color } => {
                            let mut moves = Vec::<ChessMove>::new();
                            match color {
                                Color::White => {
                                    if origin_row == 1 {
                                        match self.try_move_no_attack(
                                            origin_row,
                                            origin_col,
                                            origin_row + 2,
                                            origin_col,
                                        ) {
                                            Some(chess_move) => moves.extend(
                                                self.turn_move_to_promotion(chess_move, color),
                                            ),
                                            None => {}
                                        }
                                    }
                                    match self.try_move_no_attack(
                                        origin_row,
                                        origin_col,
                                        origin_row + 1,
                                        origin_col,
                                    ) {
                                        Some(chess_move) => moves
                                            .extend(self.turn_move_to_promotion(chess_move, color)),
                                        None => {}
                                    }
                                    match self.try_move_only_attack(
                                        origin_row,
                                        origin_col,
                                        origin_row + 1,
                                        origin_col + 1,
                                        color,
                                    ) {
                                        Some(chess_move) => moves
                                            .extend(self.turn_move_to_promotion(chess_move, color)),
                                        None => {}
                                    }

                                    match self.try_move_only_attack(
                                        origin_row,
                                        origin_col,
                                        origin_row + 1,
                                        (origin_col as i32 - 1i32) as usize, // This is ok! already handled by out of bounds check in function
                                        color,
                                    ) {
                                        Some(chess_move) => moves
                                            .extend(self.turn_move_to_promotion(chess_move, color)),
                                        None => {}
                                    }
                                }

                                Color::Black => {
                                    if origin_row == 6 {
                                        match self.try_move_no_attack(
                                            origin_row,
                                            origin_col,
                                            origin_row - 2,
                                            origin_col,
                                        ) {
                                            Some(chess_move) => moves.extend(
                                                self.turn_move_to_promotion(chess_move, color),
                                            ),
                                            None => {}
                                        }
                                    }
                                    match self.try_move_no_attack(
                                        origin_row,
                                        origin_col,
                                        (origin_row as i32 - 1i32) as usize, // This is ok! already handled by out of bounds check in function
                                        origin_col,
                                    ) {
                                        Some(chess_move) => moves
                                            .extend(self.turn_move_to_promotion(chess_move, color)),
                                        None => {}
                                    }
                                    match self.try_move_only_attack(
                                        origin_row,
                                        origin_col,
                                        (origin_row as i32 - 1i32) as usize, // This is ok! already handled by out of bounds check in function
                                        origin_col + 1,
                                        color,
                                    ) {
                                        Some(chess_move) => moves
                                            .extend(self.turn_move_to_promotion(chess_move, color)),
                                        None => {}
                                    }

                                    match self.try_move_only_attack(
                                        origin_row,
                                        origin_col,
                                        (origin_row as i32 - 1i32) as usize, // This is ok! already handled by out of bounds check in function
                                        (origin_col as i32 - 1) as usize,
                                        color,
                                    ) {
                                        Some(chess_move) => moves
                                            .extend(self.turn_move_to_promotion(chess_move, color)),
                                        None => {}
                                    }
                                }
                            }
                            moves
                        }
                    })
                }
                None => return Err(BoardError::NoPieceError),
            }
        }
        fn generate_linear_moves(
            &self,
            origin_row: usize,
            origin_col: usize,
            offset_x: i32,
            offset_y: i32,
            color: Color,
            max_distance: usize,
        ) -> Vec<ChessMove> {
            let mut res: Vec<ChessMove> = Vec::new();
            for i in 1..(max_distance + 1) {
                let potential_move = NormalChessMove {
                    initial_row: origin_row,
                    initial_col: origin_col,
                    destination_row: ((origin_row as i32) + (i as i32) * offset_x) as usize,
                    destination_col: ((origin_col as i32) + (i as i32) * offset_y) as usize,
                };
                match self.get_piece(
                    potential_move.destination_row,
                    potential_move.destination_col,
                ) {
                    Ok(piece_option) => match piece_option {
                        Some(Piece {
                            kind: _,
                            color: other_color,
                        }) => {
                            if other_color != color {
                                res.push(ChessMove::Normal(potential_move))
                            }
                            break;
                        }
                        None => {}
                    },
                    Err(_) => {
                        break;
                    }
                }

                res.push(ChessMove::Normal(potential_move));
            }
            return res;
        }

        fn try_move(
            &self,
            origin_row: usize,
            origin_col: usize,
            destination_row: usize,
            destination_col: usize,
            moving_color: Color,
        ) -> Option<NormalChessMove> {
            if !in_bounds(origin_row, origin_col) {
                return None;
            }
            if !in_bounds(destination_row, destination_col) {
                return None;
            }
            let taken_piece = self
                .get_piece(destination_row, destination_col)
                .unwrap_or(None);
            match taken_piece {
                Some(piece) => {
                    if piece.color == moving_color {
                        return None;
                    }
                }
                None => {}
            }
            Some(NormalChessMove {
                initial_row: origin_row,
                initial_col: origin_col,
                destination_row,
                destination_col,
            })
        }

        fn try_move_no_attack(
            &self,
            origin_row: usize,
            origin_col: usize,
            destination_row: usize,
            destination_col: usize,
        ) -> Option<NormalChessMove> {
            if !in_bounds(origin_row, origin_col) {
                return None;
            }
            if !in_bounds(destination_row, destination_col) {
                return None;
            }
            let taken_piece = self
                .get_piece(destination_row, destination_col)
                .unwrap_or(None);
            match taken_piece {
                Some(_) => {
                    return None;
                }
                None => {}
            }
            Some(NormalChessMove {
                initial_row: origin_row,
                initial_col: origin_col,
                destination_row,
                destination_col,
            })
        }

        fn try_move_only_attack(
            &self,
            origin_row: usize,
            origin_col: usize,
            destination_row: usize,
            destination_col: usize,
            moving_color: Color,
        ) -> Option<NormalChessMove> {
            if !in_bounds(origin_row, origin_col) {
                return None;
            }
            if !in_bounds(destination_row, destination_col) {
                return None;
            }
            let taken_piece = self
                .get_piece(destination_row, destination_col)
                .unwrap_or(None)?;

            if taken_piece.color == moving_color {
                return None;
            }

            Some(NormalChessMove {
                initial_row: origin_row,
                initial_col: origin_col,
                destination_row,
                destination_col,
            })
        }

        //color: the color of the moving piece. only used in generate_moves for pawn promotion
        fn turn_move_to_promotion(
            &self,
            chess_move: NormalChessMove,
            color: Color,
        ) -> PromotionIterator<std::str::Chars<'_>> {
            let promotion_row = match color {
                Color::White => 7,
                Color::Black => 0,
            };
            if chess_move.destination_row == promotion_row {
                PromotionIterator {
                    base_move: chess_move,
                    pieces_iterator: "rnbq".chars(),
                }
            } else {
                PromotionIterator {
                    base_move: chess_move,
                    pieces_iterator: "p".chars(),
                }
            }
        }

        //returns the color of the checkmated player
        fn test_if_checkmate(&self) -> Option<Color> {
            let mut hypothetical_board = self.clone();
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match hypothetical_board.generate_moves(i, j) {
                        Ok(moves) => {
                            for chess_move in moves {
                                if hypothetical_board.make_legal_move_inner(chess_move).is_ok() {
                                    return None;
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Some(self.turn)
        }

        //whether somebody is in check, if they are returns the color of the *checked* player.
        //(this will always be the color of the person who's turn it is because of how chess works).
        fn test_if_check(&self) -> Option<Color> {
            let turn_color = self.turn;
            match self.is_check_specific_color(turn_color) {
                true => Some(turn_color),
                false => None,
            }
        }

        //returns if color is checked
        fn is_check_specific_color(&self, color: Color) -> bool {
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match self.squares[i][j] {
                        Some(piece) => {
                            if piece.color == color {
                                continue;
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                    match self.generate_moves(i, j) {
                        Ok(moves) => {
                            for chess_move in moves {
                                match chess_move {
                                    ChessMove::Normal(normal_move) => {
                                        match self.squares[normal_move.destination_row]
                                            [normal_move.destination_col]
                                        {
                                            Some(Piece {
                                                kind: PieceKind::King,
                                                color,
                                            }) => {
                                                if color == color {
                                                    return true;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        //unreachable
                        Err(BoardError::NoPieceError) => {}
                        Err(err) => {
                            panic!("{:?} in is_check", err)
                        }
                    }
                }
            }
            false
        }

        //used for castling test. the given color is the one that may be attacked, not the attacker
        fn is_square_attacked(&self, row: usize, col: usize, color: Color) -> bool {
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    match self.squares[i][j] {
                        Some(Piece {
                            kind: _,
                            color: piece_color,
                        }) => {
                            if color == piece_color {
                                continue;
                            }
                            for chess_move in self.generate_moves(i, j).expect("bounds check") {
                                match chess_move {
                                    ChessMove::Normal(normal_move) => {
                                        if normal_move.destination_row == row
                                            && normal_move.destination_col == col
                                        {
                                            return true;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }
            false
        }
    }

    #[derive(Debug, Clone)]
    struct CastleRights {
        white: OneSidedCastleRights,
        black: OneSidedCastleRights,
    }

    impl CastleRights {
        fn at(&self, color: Color, side: CastleSide) -> &bool {
            use CastleSide::*;
            use Color::*;
            match (color, side) {
                (White, KingSide) => &self.white.king,
                (White, QueenSide) => &self.white.queen,
                (Black, KingSide) => &self.black.king,
                (Black, QueenSide) => &self.black.queen,
            }
        }
    }

    #[derive(Debug, Clone)]

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

        let mut res = Board {
            squares,
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
            is_check: None,
            is_checkmate: None,
        };

        res.update_flags();

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
                            kind,
                            color,
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

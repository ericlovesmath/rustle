use crate::board::*;

// TODO: Optimize on redundant storage of data, especially with
// edge cases like Castling and Capturing
#[derive(Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: u8,
    pub to: u8,
    // TODO: Detect what piece is being captured
    // Store in Masks a mapping from square to piece?
    pub capture: Option<u8>,
    pub castle: Option<Castle>,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(castle) = self.castle {
            return write!(
                f,
                "{}",
                match castle {
                    Castle::WhiteQueen => "White Queenside Castle",
                    Castle::WhiteKing => "White Kingside Castle",
                    Castle::BlackQueen => "Black Queenside Castle",
                    Castle::BlackKing => "Black Kingside Castle",
                }
            );
        }

        let piece = match self.piece {
            Piece::Pawn => "Pawn",
            Piece::Rook => "Rook",
            Piece::Knight => "Knight",
            Piece::Bishop => "Bishop",
            Piece::Queen => "Queen",
            Piece::King => "King",
        };
        let file_from = (b'A' + self.from % 8) as char;
        let rank_from = (b'8' - self.from / 8) as char;
        let file_to = (b'A' + self.to % 8) as char;
        let rank_to = (b'8' - self.to / 8) as char;
        let mut msg = format!(
            "{} {}{} to {}{}",
            piece, file_from, rank_from, file_to, rank_to,
        );
        if self.capture.is_some() {
            msg.push_str(" with capture");
        }
        write!(f, "{msg}")
    }
}

struct Masks {
    our_board: BitBoard,
    opp_board: BitBoard,
    block_board: BitBoard,
}

impl Board {
    pub fn apply(&self, mov: Move) -> Self {
        let mut board = self.clone();

        board.switch(self.turn, mov.piece, mov.from);
        board.switch(self.turn, mov.piece, mov.to);

        if let Some(s) = mov.capture {
            for piece in PIECES {
                if board.get(self.turn.switch(), piece, s) {
                    board.switch(self.turn.switch(), piece, s);
                }
            }
        }

        if let Some(castle) = mov.castle {
            match castle {
                Castle::WhiteQueen => {
                    board.switch(self.turn, Piece::Rook, A1);
                    board.switch(self.turn, Piece::Rook, D1);
                }
                Castle::WhiteKing => {
                    board.switch(self.turn, Piece::Rook, H1);
                    board.switch(self.turn, Piece::Rook, F1);
                }
                Castle::BlackQueen => {
                    board.switch(self.turn, Piece::Rook, A8);
                    board.switch(self.turn, Piece::Rook, D8);
                }
                Castle::BlackKing => {
                    board.switch(self.turn, Piece::Rook, H8);
                    board.switch(self.turn, Piece::Rook, F8);
                }
            }
        }

        // Update Castling Rights
        if mov.piece == Piece::King {
            board.castle_rights &= match self.turn {
                Sides::White => Castle::BlackKing as u8 | Castle::BlackQueen as u8,
                Sides::Black => Castle::WhiteKing as u8 | Castle::WhiteQueen as u8,
            }
        }
        if mov.piece == Piece::Rook {
            let disable = match (self.turn, mov.from) {
                (Sides::White, A1) => Castle::WhiteQueen as u8,
                (Sides::White, H1) => Castle::WhiteKing as u8,
                (Sides::Black, A8) => Castle::BlackQueen as u8,
                (Sides::Black, H8) => Castle::BlackKing as u8,
                _ => 0,
            };
            board.castle_rights &= 0b1111 ^ disable;
        }

        // Update En Passant square
        board.en_passant = None;
        if mov.piece == Piece::Pawn {
            if self.turn == Sides::White && mov.from - mov.to == 16 {
                board.en_passant = Some(mov.from - 8);
            } else if self.turn == Sides::Black && mov.to - mov.from == 16 {
                board.en_passant = Some(mov.from + 8);
            }
        }

        board.turn = board.turn.switch();
        board
    }

    pub fn moves(&self) -> Vec<Move> {
        self.pseudo_moves()
    }

    pub fn pseudo_moves(&self) -> Vec<Move> {
        let mut our_board = 0u64;
        for piece in PIECES {
            our_board |= self.state[self.turn as usize][piece as usize].0
        }

        let mut opp_board = 0u64;
        for piece in PIECES {
            opp_board |= self.state[self.turn as usize ^ 1][piece as usize].0
        }

        let masks = Masks {
            our_board: BitBoard(our_board),
            opp_board: BitBoard(opp_board),
            block_board: BitBoard(our_board | opp_board),
        };

        let mut moves = vec![];
        for piece in PIECES {
            for square in SQUARES {
                if self.get(self.turn, piece, square) {
                    let next = self.pseudo_moves_from(piece, square, &masks);
                    moves.extend_from_slice(next.as_slice());
                }
            }
        }

        // TODO: Castling, make this better

        match self.turn {
            Sides::White => {
                if self.castle_rights & Castle::WhiteQueen as u8 != 0
                    && masks.block_board.0 & (1u64 << B1 | 1u64 << C1 | 1u64 << D1) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E1,
                        to: C1,
                        capture: None,
                        castle: Some(Castle::WhiteQueen),
                    });
                }

                if self.castle_rights & Castle::WhiteKing as u8 != 0
                    && masks.block_board.0 & (1u64 << F1 | 1u64 << G1) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E1,
                        to: G1,
                        capture: None,
                        castle: Some(Castle::WhiteKing),
                    });
                }
            }
            Sides::Black => {
                if self.castle_rights & Castle::BlackQueen as u8 != 0
                    && masks.block_board.0 & (1u64 << B8 | 1u64 << C8 | 1u64 << D8) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E8,
                        to: C8,
                        capture: None,
                        castle: Some(Castle::BlackQueen),
                    });
                }

                if self.castle_rights & Castle::BlackKing as u8 != 0
                    && masks.block_board.0 & (1u64 << F8 | 1u64 << G8) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E8,
                        to: G8,
                        capture: None,
                        castle: Some(Castle::BlackKing),
                    });
                }
            }
        }

        // TODO: Pawn Promotion
        moves
    }

    fn pseudo_moves_from(&self, piece: Piece, square: u8, masks: &Masks) -> Vec<Move> {
        // TODO: Yield from generator
        // TODO: Pre-generate masks for each piece per square
        match piece {
            Piece::Pawn => self.pseudo_moves_pawn(square, masks),
            Piece::Rook => self.pseudo_moves_rook(square, masks),
            Piece::Knight => self.pseudo_moves_knight(square, masks),
            Piece::Bishop => self.pseudo_moves_bishop(square, masks),
            Piece::Queen => self.pseudo_moves_queen(square, masks),
            Piece::King => self.pseudo_moves_king(square, masks),
        }
    }

    fn pseudo_moves_pawn(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let mut moves = vec![];

        if self.turn == Sides::White {
            if let Some(s) = step(square, Direction::N) {
                if !masks.block_board.get(s) {
                    moves.push((s, None));
                    if (A2..=H2).contains(&square) {
                        if let Some(s) = step(s, Direction::N) {
                            if !masks.block_board.get(s) {
                                moves.push((s, None));
                            }
                        }
                    }
                }
            }
            for dir in [Direction::NW, Direction::NE] {
                if let Some(s) = step(square, dir) {
                    if masks.opp_board.get(s) {
                        moves.push((s, Some(s)));
                    } else if self.en_passant == Some(s) {
                        moves.push((s, Some(s + 8)));
                    }
                }
            }
        } else {
            if let Some(s) = step(square, Direction::S) {
                if !masks.block_board.get(s) {
                    moves.push((s, None));
                    if (A7..=H7).contains(&square) {
                        if let Some(s) = step(s, Direction::S) {
                            if !masks.block_board.get(s) {
                                moves.push((s, None));
                            }
                        }
                    }
                }
            }
            for dir in [Direction::SW, Direction::SE] {
                if let Some(s) = step(square, dir) {
                    if masks.opp_board.get(s) {
                        moves.push((s, Some(s)));
                    } else if self.en_passant == Some(s) {
                        moves.push((s, Some(s - 8)));
                    }
                }
            }
        }
        moves
            .into_iter()
            .map(|(square_to, capture)| Move {
                piece: Piece::Pawn,
                from: square,
                to: square_to,
                capture,
                castle: None,
            })
            .collect()
    }

    fn pseudo_moves_rook(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let dirs = [Direction::N, Direction::S, Direction::E, Direction::W];
        self.pseudo_moves_dirs(square, Piece::Rook, &dirs, masks)
    }

    fn pseudo_moves_bishop(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let dirs = [Direction::NE, Direction::SE, Direction::NW, Direction::SW];
        self.pseudo_moves_dirs(square, Piece::Bishop, &dirs, masks)
    }

    #[rustfmt::skip]
    fn pseudo_moves_queen(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let dirs = [
            Direction::N, Direction::S, Direction::E, Direction::W,
            Direction::NE, Direction::SE, Direction::NW, Direction::SW,
        ];
        self.pseudo_moves_dirs(square, Piece::Queen, &dirs, masks)
    }

    #[rustfmt::skip]
    fn pseudo_moves_king(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let dirs = [
            Direction::N, Direction::S, Direction::E, Direction::W,
            Direction::NE, Direction::SE, Direction::NW, Direction::SW,
        ];
        let mut moves = vec![];
        for dir in dirs {
            if let Some(sq) = step(square, dir) {
                if masks.opp_board.get(sq) {
                    moves.push((sq, Some(sq)));
                } else if !masks.our_board.get(sq) {
                    moves.push((sq, None));
                }
            }
        }
        moves
            .into_iter()
            .map(|(square_to, capture)| Move {
                piece: Piece::King,
                from: square,
                to: square_to,
                capture,
                castle: None,
            })
            .collect()
    }

    fn pseudo_moves_knight(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let dirs = [
            (Direction::NE, [Direction::N, Direction::E]),
            (Direction::NW, [Direction::N, Direction::W]),
            (Direction::SE, [Direction::S, Direction::E]),
            (Direction::SW, [Direction::S, Direction::W]),
        ];
        let mut moves = vec![];
        for (base, next) in dirs {
            if let Some(sq) = step(square, base) {
                for dir in next {
                    if let Some(sq) = step(sq, dir) {
                        if masks.opp_board.get(sq) {
                            moves.push((sq, Some(sq)));
                        } else if !masks.our_board.get(sq) {
                            moves.push((sq, None));
                        }
                    }
                }
            }
        }
        moves
            .into_iter()
            .map(|(square_to, capture)| Move {
                piece: Piece::Knight,
                from: square,
                to: square_to,
                capture,
                castle: None,
            })
            .collect()
    }

    fn pseudo_moves_dirs(
        &self,
        square: u8,
        piece: Piece,
        dirs: &[Direction],
        masks: &Masks,
    ) -> Vec<Move> {
        let mut moves = vec![];
        for dir in dirs {
            let mut curr = square;
            while let Some(next) = step(curr, *dir) {
                curr = next;
                if masks.opp_board.get(curr) {
                    moves.push((curr, Some(curr)));
                    break;
                }
                if masks.our_board.get(curr) {
                    break;
                }
                moves.push((curr, None));
            }
        }
        moves
            .into_iter()
            .map(|(square_to, capture)| Move {
                piece,
                from: square,
                to: square_to,
                capture,
                castle: None,
            })
            .collect()
    }
}

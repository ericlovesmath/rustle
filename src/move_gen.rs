use crate::board::*;

#[derive(Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: u8,
    pub to: u8,
    pub capture: bool,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
        if self.capture {
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

        if mov.capture {
            for piece in PIECES {
                if board.get(self.turn.switch(), piece, mov.to) {
                    board.switch(self.turn.switch(), piece, mov.to);
                }
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
        moves
    }

    fn pseudo_moves_from(&self, piece: Piece, square: u8, masks: &Masks) -> Vec<Move> {
        // TODO: Yield from generator
        // TODO: Add more rules like castling and enpassant and promotion
        // TODO: Pre-generate masks for each piece per square
        let moves = match piece {
            Piece::Pawn => self.pseudo_moves_pawn(square, masks),
            Piece::Rook => self.pseudo_moves_rook(square, masks),
            Piece::Knight => self.pseudo_moves_knight(square, masks),
            Piece::Bishop => self.pseudo_moves_bishop(square, masks),
            Piece::Queen => self.pseudo_moves_queen(square, masks),
            Piece::King => self.pseudo_moves_king(square, masks),
        };
        moves
            .into_iter()
            .map(|(square_to, capture)| Move {
                piece,
                from: square,
                to: square_to,
                capture,
            })
            .collect()
    }

    fn pseudo_moves_pawn(&self, square: u8, masks: &Masks) -> Vec<(u8, bool)> {
        let mut moves = vec![];
        if self.turn == Sides::White {
            if let Some(s) = step(square, Direction::N) {
                if !masks.block_board.get(s) {
                    moves.push((s, false));
                    if (A2..=H2).contains(&square) {
                        if let Some(s) = step(s, Direction::N) {
                            if !masks.block_board.get(s) {
                                moves.push((s, false));
                            }
                        }
                    }
                }
            }
            for dir in [Direction::NW, Direction::NE] {
                if let Some(s) = step(square, dir) {
                    if masks.opp_board.get(s) {
                        moves.push((s, true));
                    }
                }
            }
        } else {
            if let Some(s) = step(square, Direction::S) {
                if !masks.block_board.get(s) {
                    moves.push((s, false));
                    if (A7..=H7).contains(&square) {
                        if let Some(s) = step(s, Direction::S) {
                            if !masks.block_board.get(s) {
                                moves.push((s, false));
                            }
                        }
                    }
                }
            }
            for dir in [Direction::SW, Direction::SE] {
                if let Some(s) = step(square, dir) {
                    if masks.opp_board.get(s) {
                        moves.push((s, true));
                    }
                }
            }
        }
        moves
    }

    fn pseudo_moves_rook(&self, square: u8, masks: &Masks) -> Vec<(u8, bool)> {
        let dirs = [Direction::N, Direction::S, Direction::E, Direction::W];
        self.pseudo_moves_dirs(square, &dirs, masks)
    }

    fn pseudo_moves_bishop(&self, square: u8, masks: &Masks) -> Vec<(u8, bool)> {
        let dirs = [Direction::NE, Direction::SE, Direction::NW, Direction::SW];
        self.pseudo_moves_dirs(square, &dirs, masks)
    }

    #[rustfmt::skip]
    fn pseudo_moves_queen(&self, square: u8, masks: &Masks) -> Vec<(u8, bool)> {
        let dirs = [
            Direction::N, Direction::S, Direction::E, Direction::W,
            Direction::NE, Direction::SE, Direction::NW, Direction::SW,
        ];
        self.pseudo_moves_dirs(square, &dirs, masks)
    }

    #[rustfmt::skip]
    fn pseudo_moves_king(&self, square: u8, masks: &Masks) -> Vec<(u8, bool)> {
        let dirs = [
            Direction::N, Direction::S, Direction::E, Direction::W,
            Direction::NE, Direction::SE, Direction::NW, Direction::SW,
        ];
        let mut moves = vec![];
        for dir in dirs {
            if let Some(sq) = step(square, dir) {
                if masks.opp_board.get(sq) {
                    moves.push((sq, true));
                } else if !masks.our_board.get(sq) {
                    moves.push((sq, false));
                }
            }
        }
        moves
    }

    fn pseudo_moves_knight(&self, square: u8, masks: &Masks) -> Vec<(u8, bool)> {
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
                            moves.push((sq, true));
                        } else if !masks.our_board.get(sq) {
                            moves.push((sq, false));
                        }
                    }
                }
            }
        }
        moves
    }

    fn pseudo_moves_dirs(&self, square: u8, dirs: &[Direction], masks: &Masks) -> Vec<(u8, bool)> {
        let mut moves = vec![];
        for dir in dirs {
            let mut curr = square;
            while let Some(next) = step(curr, *dir) {
                curr = next;
                if masks.opp_board.get(curr) {
                    moves.push((curr, true));
                    break;
                }
                if masks.our_board.get(curr) {
                    break;
                }
                moves.push((curr, false));
            }
        }
        moves
    }
}

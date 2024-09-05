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
        let mut state = self.state.clone();
        let board = &mut state[self.turn as usize][mov.piece as usize];
        board.0 ^= 1u64 << mov.from;
        board.0 ^= 1u64 << mov.to;

        Self {
            state,
            turn: self.turn.switch(),
        }
    }
    pub fn moves(&self) -> Vec<Move> {
        // self.pseudo_moves(piece)
        let mut our_board = 0u64;
        for piece in PIECES {
            our_board |= self.state[self.turn as usize][piece as usize].0
        }
        //
        let mut opp_board = 0u64;
        for piece in PIECES {
            opp_board |= self.state[self.turn as usize ^ 1][piece as usize].0
        }
        let masks = Masks {
            our_board: BitBoard(our_board),
            opp_board: BitBoard(opp_board),
            block_board: BitBoard(our_board | opp_board),
        };

        // TODO: Yield from generator
        let mut moves = vec![];
        for piece in [Piece::Pawn] {
            for square in SQUARES {
                if self.state[self.turn as usize][piece as usize].flag(square) {
                    let next = self.pseudo_moves_from(piece, square, &masks);
                    moves.extend_from_slice(next.as_slice());
                }
            }
        }
        moves
    }

    fn pseudo_moves_from(&self, piece: Piece, square: u8, masks: &Masks) -> Vec<Move> {
        match piece {
            Piece::Pawn => self.pseudo_moves_pawn(square, masks),
            Piece::Rook => todo!(),
            Piece::Knight => todo!(),
            Piece::Bishop => todo!(),
            Piece::Queen => todo!(),
            Piece::King => todo!(),
        }
    }

    fn pseudo_moves_pawn(&self, square: u8, masks: &Masks) -> Vec<Move> {
        let mut moves = vec![];
        if self.turn == Sides::White {
            if let Some(s) = step(square, Direction::N) {
                if !masks.block_board.flag(s) {
                    moves.push((s, false));
                    if (A2..=H2).contains(&square) {
                        if let Some(s) = step(s, Direction::N) {
                            if !masks.block_board.flag(s) {
                                moves.push((s, false));
                            }
                        }
                    }
                }
            }
            if let Some(s) = step(square, Direction::NW) {
                if masks.opp_board.flag(s) {
                    moves.push((s, true));
                }
            }
            if let Some(s) = step(square, Direction::NE) {
                if masks.opp_board.flag(s) {
                    moves.push((s, true));
                }
            }
        } else {
            if let Some(s) = step(square, Direction::S) {
                if !masks.block_board.flag(s) {
                    moves.push((s, false));
                    if (A7..=H7).contains(&square) {
                        if let Some(s) = step(s, Direction::S) {
                            if !masks.block_board.flag(s) {
                                moves.push((s, false));
                            }
                        }
                    }
                }
            }
            if let Some(s) = step(square, Direction::SW) {
                if masks.opp_board.flag(s) {
                    moves.push((s, true));
                }
            }
            if let Some(s) = step(square, Direction::SE) {
                if masks.opp_board.flag(s) {
                    moves.push((s, true));
                }
            }
        }
        moves
            .into_iter()
            .map(|(sq, capture)| Move {
                piece: Piece::Pawn,
                from: square,
                to: sq,
                capture,
            })
            .collect()
    }
}

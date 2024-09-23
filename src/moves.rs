use crate::board::*;
use crate::board::Square::*;

// TODO: Optimize on redundant storage of data, especially with
// edge cases like Castling and Capturing
#[derive(Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    // TODO: Detect what piece is being captured
    // Store in Masks a mapping from square to piece?
    pub capture: Option<Square>,
    pub castle: Option<Castle>,
    pub promotion: Option<Piece>,
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
        let mut msg = format!(
            "{} {:?} to {:?}",
            piece, self.from, self.to
        );
        if self.capture.is_some() {
            msg.push_str(" with capture");
        }
        if self.promotion.is_some() {
            msg.push_str(" with promotion");
        }
        write!(f, "{msg}")
    }
}

struct Masks {
    our_board: BitBoard,
    opp_board: BitBoard,
    block_board: BitBoard,
}

impl GameState {
    pub fn apply(&self, mov: Move) -> Self {
        let mut game = self.clone();

        game.board_mut(self.turn, mov.piece).flip(mov.from);
        game.board_mut(self.turn, mov.piece).flip(mov.to);

        if let Some(s) = mov.capture {
            for piece in PIECES {
                let board = game.board_mut(self.turn.switch(), piece);
                if board.get(s) {
                    board.flip(s);
                }
            }
        }

        if let Some(castle) = mov.castle {
            let board = game.board_mut(self.turn, Piece::Rook);
            match castle {
                Castle::WhiteQueen => {
                    board.flip(A1);
                    board.flip(D1);
                }
                Castle::WhiteKing => {
                    board.flip(H1);
                    board.flip(F1);
                }
                Castle::BlackQueen => {
                    board.flip(A8);
                    board.flip(D8);
                }
                Castle::BlackKing => {
                    board.flip(H8);
                    board.flip(F8);
                }
            }
        }

        if let Some(piece) = mov.promotion {
            game.board_mut(self.turn, Piece::Pawn).flip(mov.to);
            game.board_mut(self.turn, piece).flip(mov.to);
        }

        // Update Castling Rights
        if mov.piece == Piece::King {
            game.castle_rights &= match self.turn {
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
            game.castle_rights &= 0b1111 ^ disable;
        }

        // Update En Passant square
        game.en_passant = None;
        if mov.piece == Piece::Pawn {
            if self.turn == Sides::White && mov.from as u8 - mov.to as u8 == 16 {
                game.en_passant = Some(Square::from(mov.from as u8 - 8));
            } else if self.turn == Sides::Black && mov.to as u8 - mov.from as u8 == 16 {
                game.en_passant = Some(Square::from(mov.from as u8 + 8));
            }
        }

        game.turn = game.turn.switch();
        game
    }

    pub fn moves(&self) -> Vec<Move> {
        self.pseudo_moves()
    }

    pub fn pseudo_moves(&self) -> Vec<Move> {
        let mut our_board = 0u64;
        for piece in PIECES {
            our_board |= self.board(self.turn, piece).0
        }

        let mut opp_board = 0u64;
        for piece in PIECES {
            opp_board |= self.board(self.turn.switch(), piece).0
        }

        let masks = Masks {
            our_board: BitBoard(our_board),
            opp_board: BitBoard(opp_board),
            block_board: BitBoard(our_board | opp_board),
        };

        let mut moves = vec![];
        for piece in PIECES {
            for square in SQUARES {
                if self.board(self.turn, piece).get(square) {
                    let next = self.pseudo_moves_from(piece, square, &masks);
                    moves.extend_from_slice(next.as_slice());
                }
            }
        }

        // TODO: Castling, make this better

        match self.turn {
            Sides::White => {
                if self.castle_rights & Castle::WhiteQueen as u8 != 0
                    && masks.block_board.0 & (1u64 << B1 as u8 | 1u64 << C1 as u8 | 1u64 << D1 as u8) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E1,
                        to: C1,
                        capture: None,
                        castle: Some(Castle::WhiteQueen),
                        promotion: None,
                    });
                }

                if self.castle_rights & Castle::WhiteKing as u8 != 0
                    && masks.block_board.0 & (1u64 << F1 as u8 | 1u64 << G1 as u8) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E1,
                        to: G1,
                        capture: None,
                        castle: Some(Castle::WhiteKing),
                        promotion: None,
                    });
                }
            }
            Sides::Black => {
                if self.castle_rights & Castle::BlackQueen as u8 != 0
                    && masks.block_board.0 & (1u64 << B8 as u8 | 1u64 << C8 as u8 | 1u64 << D8 as u8) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E8,
                        to: C8,
                        capture: None,
                        castle: Some(Castle::BlackQueen),
                        promotion: None,
                    });
                }

                if self.castle_rights & Castle::BlackKing as u8 != 0
                    && masks.block_board.0 & (1u64 << F8 as u8 | 1u64 << G8 as u8) == 0
                {
                    moves.push(Move {
                        piece: Piece::King,
                        from: E8,
                        to: G8,
                        capture: None,
                        castle: Some(Castle::BlackKing),
                        promotion: None,
                    });
                }
            }
        }

        moves
    }

    fn pseudo_moves_from(&self, piece: Piece, square: Square, masks: &Masks) -> Vec<Move> {
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

    fn pseudo_moves_pawn(&self, square: Square, masks: &Masks) -> Vec<Move> {
        let mut moves = vec![];

        if self.turn == Sides::White {
            if let Some(s) = square.step(Direction::N) {
                if !masks.block_board.get(s) {
                    moves.push((s, None));
                    if (A2..=H2).contains(&square) {
                        if let Some(s) = s.step(Direction::N) {
                            if !masks.block_board.get(s) {
                                moves.push((s, None));
                            }
                        }
                    }
                }
            }
            for dir in [Direction::NW, Direction::NE] {
                if let Some(s) = square.step(dir) {
                    if masks.opp_board.get(s) {
                        moves.push((s, Some(s)));
                    } else if self.en_passant == Some(s) {
                        moves.push((s, Some(Square::from(s as u8 + 8))));
                    }
                }
            }
        } else {
            if let Some(s) = square.step(Direction::S) {
                if !masks.block_board.get(s) {
                    moves.push((s, None));
                    if (A7..=H7).contains(&square) {
                        if let Some(s) = s.step(Direction::S) {
                            if !masks.block_board.get(s) {
                                moves.push((s, None));
                            }
                        }
                    }
                }
            }
            for dir in [Direction::SW, Direction::SE] {
                if let Some(s) = square.step(dir) {
                    if masks.opp_board.get(s) {
                        moves.push((s, Some(s)));
                    } else if self.en_passant == Some(s) {
                        moves.push((s, Some(Square::from(s as u8 - 8))));
                    }
                }
            }
        }

        let mut final_moves = vec![];

        for (to, capture) in moves.into_iter() {
            if (self.turn == Sides::White && to <= H8) || (self.turn == Sides::Black && to >= A1) {
                for piece in [Piece::Bishop, Piece::Knight, Piece::Rook, Piece::Queen] {
                    final_moves.push(Move {
                        piece: Piece::Pawn,
                        from: square,
                        to,
                        capture,
                        castle: None,
                        promotion: Some(piece),
                    });
                }
            } else {
                final_moves.push(Move {
                    piece: Piece::Pawn,
                    from: square,
                    to,
                    capture,
                    castle: None,
                    promotion: None,
                });
            }
        }

        final_moves
    }

    fn pseudo_moves_rook(&self, square: Square, masks: &Masks) -> Vec<Move> {
        let dirs = [Direction::N, Direction::S, Direction::E, Direction::W];
        self.pseudo_moves_dirs(square, Piece::Rook, &dirs, masks)
    }

    fn pseudo_moves_bishop(&self, square: Square, masks: &Masks) -> Vec<Move> {
        let dirs = [Direction::NE, Direction::SE, Direction::NW, Direction::SW];
        self.pseudo_moves_dirs(square, Piece::Bishop, &dirs, masks)
    }

    #[rustfmt::skip]
    fn pseudo_moves_queen(&self, square: Square, masks: &Masks) -> Vec<Move> {
        let dirs = [
            Direction::N, Direction::S, Direction::E, Direction::W,
            Direction::NE, Direction::SE, Direction::NW, Direction::SW,
        ];
        self.pseudo_moves_dirs(square, Piece::Queen, &dirs, masks)
    }

    #[rustfmt::skip]
    fn pseudo_moves_king(&self, square: Square, masks: &Masks) -> Vec<Move> {
        let dirs = [
            Direction::N, Direction::S, Direction::E, Direction::W,
            Direction::NE, Direction::SE, Direction::NW, Direction::SW,
        ];
        let mut moves = vec![];
        for dir in dirs {
            if let Some(sq) = square.step(dir) {
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
                promotion: None,
            })
            .collect()
    }

    fn pseudo_moves_knight(&self, square: Square, masks: &Masks) -> Vec<Move> {
        let dirs = [
            (Direction::NE, [Direction::N, Direction::E]),
            (Direction::NW, [Direction::N, Direction::W]),
            (Direction::SE, [Direction::S, Direction::E]),
            (Direction::SW, [Direction::S, Direction::W]),
        ];
        let mut moves = vec![];
        for (base, next) in dirs {
            if let Some(sq) = square.step(base) {
                for dir in next {
                    if let Some(sq) = sq.step(dir) {
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
                promotion: None,
            })
            .collect()
    }

    fn pseudo_moves_dirs(
        &self,
        square: Square,
        piece: Piece,
        dirs: &[Direction],
        masks: &Masks,
    ) -> Vec<Move> {
        let mut moves = vec![];
        for dir in dirs {
            let mut curr = square;
            while let Some(next) = curr.step(*dir) {
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
                promotion: None,
            })
            .collect()
    }
}

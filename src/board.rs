use std::fmt;

pub struct Board {
    state: [[BitBoard; 6]; 2],
    turn: Sides,
    castle_rights: u8,
    en_passant: Option<u8>,
}

impl Board {
    pub fn new() -> Self {
        use Square::*;
        let state = [
            [
                BitBoard::from(vec![A2, B2, C2, D2, E2, F2, G2, H2]),
                BitBoard::from(vec![A1, H1]),
                BitBoard::from(vec![B1, G1]),
                BitBoard::from(vec![C1, F1]),
                BitBoard::from(vec![D1]),
                BitBoard::from(vec![E1]),
            ],
            [
                BitBoard::from(vec![A7, B7, C7, D7, E7, F7, G7, H7]),
                BitBoard::from(vec![A8, H8]),
                BitBoard::from(vec![B8, G8]),
                BitBoard::from(vec![C8, F8]),
                BitBoard::from(vec![D8]),
                BitBoard::from(vec![E8]),
            ],
        ];

        Board {
            state,
            turn: Sides::White,
            castle_rights: 0,
            en_passant: None,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checks = [
            (Sides::White, Pieces::Pawn, 'P'),
            (Sides::White, Pieces::Rook, 'R'),
            (Sides::White, Pieces::Knight, 'N'),
            (Sides::White, Pieces::Bishop, 'B'),
            (Sides::White, Pieces::Queen, 'Q'),
            (Sides::White, Pieces::King, 'K'),
            (Sides::Black, Pieces::Pawn, 'p'),
            (Sides::Black, Pieces::Rook, 'r'),
            (Sides::Black, Pieces::Knight, 'n'),
            (Sides::Black, Pieces::Bishop, 'b'),
            (Sides::Black, Pieces::Queen, 'q'),
            (Sides::Black, Pieces::King, 'k'),
        ];

        let mut b = ['.'; 64];
        for (side, piece, repr) in checks {
            for x in 0..64 {
                if self.state[side as usize][piece as usize].0 & (1u64 << x) == (1u64 << x) {
                    b[x] = repr;
                }
            }
        }
        let mut s = String::new();
        for x in 0..64 {
            if x % 8 == 0 {
                s.push('\n');
            }
            s.push(b[x]);
        }
        write!(f, "{}", s)
    }
}

struct BitBoard(u64);

impl From<Vec<Square>> for BitBoard {
    fn from(squares: Vec<Square>) -> Self {
        let mut board = 0u64;
        for square in squares.iter() {
            board |= 1u64 << *square as u64;
        }
        BitBoard(board)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for x in 0..64 {
            if x % 8 == 0 {
                s.push('\n');
            }
            if self.0 & (1u64 << x) == (1u64 << x) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
        }
        write!(f, "{}", s)
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Sides {
    White,
    Black,
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[repr(usize)]
#[derive(Clone, Copy)]
#[rustfmt::skip]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

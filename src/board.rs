use std::fmt;

pub struct Board {
    pub state: [[BitBoard; 6]; 2],
    pub turn: Sides, // TODO: turn to 1 bit bool?
                     // castle_rights: u8,
                     // en_passant: Option<u8>,
                     // halfmoves: usize,
}

impl Board {
    pub fn new() -> Self {
        let state = [
            [
                BitBoard::from(vec![A2, B2, C2, D2, E2, F2, G2, H2]),
                BitBoard::from(vec![A1, H1]),
                BitBoard::from(vec![B1, G1]),
                BitBoard::from(vec![C1, F1]),
                // BitBoard::from(vec![D1]),
                BitBoard::from(vec![D1, E4]),
                BitBoard::from(vec![E1]),
            ],
            [
                BitBoard::from(vec![A7, B7, C7, D7, E7, F7, G7, H7]),
                BitBoard::from(vec![A8, H8]),
                BitBoard::from(vec![B8, G8]),
                // BitBoard::from(vec![C8, F8]),
                BitBoard::from(vec![C8, F8, D3]),
                BitBoard::from(vec![D8]),
                BitBoard::from(vec![E8]),
            ],
        ];

        // use CastleRights::*;
        // let castle_rights = WhiteQueen as u8 | WhiteKing as u8 | BlackQueen as u8 | BlackKing as u8;

        Board {
            state,
            turn: Sides::White,
            // castle_rights,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checks = [
            (Sides::White, Piece::Pawn, 'P'),
            (Sides::White, Piece::Rook, 'R'),
            (Sides::White, Piece::Knight, 'N'),
            (Sides::White, Piece::Bishop, 'B'),
            (Sides::White, Piece::Queen, 'Q'),
            (Sides::White, Piece::King, 'K'),
            (Sides::Black, Piece::Pawn, 'p'),
            (Sides::Black, Piece::Rook, 'r'),
            (Sides::Black, Piece::Knight, 'n'),
            (Sides::Black, Piece::Bishop, 'b'),
            (Sides::Black, Piece::Queen, 'q'),
            (Sides::Black, Piece::King, 'k'),
        ];

        let mut b = ['.'; 64];
        for (side, piece, repr) in checks {
            for x in 0..64 {
                // TODO: Convert repr[]'s to struct with consts to avoid casting
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

#[derive(Clone)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn flag(&self, square: u8) -> bool {
        let mask = 1u64 << square as u64;
        self.0 & mask == mask
    }
}

impl From<Vec<u8>> for BitBoard {
    fn from(squares: Vec<u8>) -> Self {
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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum CastleRights {
    WhiteQueen = 1u8 << 0,
    WhiteKing = 1u8 << 1,
    BlackQueen = 1u8 << 2,
    BlackKing = 1u8 << 3,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sides {
    White,
    Black,
}

impl Sides {
    pub fn switch(self) -> Self {
        match self {
            Sides::White => Sides::Black,
            Sides::Black => Sides::White,
        }
    }
}

pub const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Rook,
    Piece::Knight,
    Piece::Bishop,
    Piece::Queen,
    Piece::King,
];

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

macro_rules! generate_squares {
    ($($row:ident $col:expr),*) => {
        $(pub const $row: u8 = $col;)*
        pub const SQUARES: [u8; 64] = [
            $($row),*
        ];
    }
}

generate_squares! {
    A8 0,  B8 1,  C8 2,  D8 3,  E8 4,  F8 5,  G8 6,  H8 7,
    A7 8,  B7 9,  C7 10, D7 11, E7 12, F7 13, G7 14, H7 15,
    A6 16, B6 17, C6 18, D6 19, E6 20, F6 21, G6 22, H6 23,
    A5 24, B5 25, C5 26, D5 27, E5 28, F5 29, G5 30, H5 31,
    A4 32, B4 33, C4 34, D4 35, E4 36, F4 37, G4 38, H4 39,
    A3 40, B3 41, C3 42, D3 43, E3 44, F3 45, G3 46, H3 47,
    A2 48, B2 49, C2 50, D2 51, E2 52, F2 53, G2 54, H2 55,
    A1 56, B1 57, C1 58, D1 59, E1 60, F1 61, G1 62, H1 63
}

// TODO: Fix everything about this
#[rustfmt::skip]
pub enum Direction { N, E, S, W, NE, SE, NW, SW }

pub fn step(sq: u8, dir: Direction) -> Option<u8> {
    match dir {
        Direction::N if sq <= H8 => None,
        Direction::N => Some(sq - 8),
        Direction::S if sq >= A1 => None,
        Direction::S => Some(sq + 8),
        Direction::W if sq % 8 == 0 => None,
        Direction::W => Some(sq - 1),
        Direction::E if sq % 8 == 7 => None,
        Direction::E => Some(sq + 1),
        Direction::NE => {
            let sq = step(sq, Direction::N)?;
            step(sq, Direction::E)
        }
        Direction::SE => {
            let sq = step(sq, Direction::S)?;
            step(sq, Direction::E)
        }
        Direction::NW => {
            let sq = step(sq, Direction::N)?;
            step(sq, Direction::W)
        }
        Direction::SW => {
            let sq = step(sq, Direction::S)?;
            step(sq, Direction::W)
        }
    }
}

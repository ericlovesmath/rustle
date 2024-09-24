// TODO: Clean up and document the entire codebase because this is terrible
// TODO: Write Tests

#[derive(Clone)]
pub struct GameState {
    pub state: [[BitBoard; 6]; 2],
    pub turn: Sides, // TODO: turn to 1 bit bool?
    pub castle_rights: u8,
    pub en_passant: Option<Square>,
    // halfmoves: usize,
    // fullmoves: usize,
}

impl GameState {
    pub fn board(&self, side: Sides, piece: Piece) -> &BitBoard {
        &self.state[side as usize][piece as usize]
    }

    pub fn board_mut(&mut self, side: Sides, piece: Piece) -> &mut BitBoard {
        &mut self.state[side as usize][piece as usize]
    }
}

#[derive(Clone)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn get(&self, square: Square) -> bool {
        let mask = 1u64 << square as u64;
        self.0 & mask == mask
    }

    pub fn flip(&mut self, square: Square) {
        let mask = 1u64 << square as u64;
        self.0 ^= mask;
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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Castle {
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

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece { Pawn, Rook, Knight, Bishop, Queen, King }
// }

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

#[rustfmt::skip]
pub const SQUARES: [Square; 64] = {
    use Square::*;
    [
        A8, B8, C8, D8, E8, F8, G8, H8,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A1, B1, C1, D1, E1, F1, G1, H1,
    ]
};

impl From<u8> for Square {
    fn from(square: u8) -> Self {
        assert!(square < 64, "Invalid u8 attempted to be casted to Square");
        SQUARES[square as usize]
    }
}

impl From<Square> for String {
    fn from(sq: Square) -> Self {
        let file_from = (b'A' + sq as u8 % 8) as char;
        let rank_from = (b'8' - sq as u8 / 8) as char;
        format!("{file_from}{rank_from}")
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Direction { N, E, S, W, NE, SE, NW, SW }

impl Square {
    pub fn step(&self, dir: Direction) -> Option<Square> {
        use Square::*;
        let sq = *self as u8;
        match dir {
            Direction::N if sq <= H8 as u8 => None,
            Direction::N => Some(Square::from(sq - 8)),
            Direction::S if sq >= A1 as u8 => None,
            Direction::S => Some(Square::from(sq + 8)),
            Direction::W if sq % 8 == 0 => None,
            Direction::W => Some(Square::from(sq - 1)),
            Direction::E if sq % 8 == 7 => None,
            Direction::E => Some(Square::from(sq + 1)),
            Direction::NE => self.step(Direction::N)?.step(Direction::E),
            Direction::SE => self.step(Direction::S)?.step(Direction::E),
            Direction::NW => self.step(Direction::N)?.step(Direction::W),
            Direction::SW => self.step(Direction::S)?.step(Direction::W),
        }
    }
}

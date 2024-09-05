use crate::board::*;

impl From<String> for Board {
    fn from(fen: String) -> Self {
        let mut board = Board {
            state: [[BitBoard(0); 6]; 2],
            turn: Sides::White,
            castle_rights: 0,
        };

        // // use CastleRights::*;
        // // let castle_rights = WhiteQueen as u8 | WhiteKing as u8 | BlackQueen as u8 | BlackKing as u8;
        //
        // Board {
        //     state,
        //     turn: Sides::White,
        //     // castle_rights,
        // }

        let fen: Vec<&str> = fen.split_whitespace().collect();

        let mut x: i8 = 0;
        let mut y: u8 = 0;
        for c in fen[0].to_string().chars() {
            let sq = x as u8 + y * 8;
            match c {
                'P' => board.switch(Sides::White, Piece::Pawn, sq),
                'R' => board.switch(Sides::White, Piece::Rook, sq),
                'N' => board.switch(Sides::White, Piece::Knight, sq),
                'B' => board.switch(Sides::White, Piece::Bishop, sq),
                'Q' => board.switch(Sides::White, Piece::Queen, sq),
                'K' => board.switch(Sides::White, Piece::King, sq),
                'p' => board.switch(Sides::Black, Piece::Pawn, sq),
                'r' => board.switch(Sides::Black, Piece::Rook, sq),
                'n' => board.switch(Sides::Black, Piece::Knight, sq),
                'b' => board.switch(Sides::Black, Piece::Bishop, sq),
                'q' => board.switch(Sides::Black, Piece::Queen, sq),
                'k' => board.switch(Sides::Black, Piece::King, sq),
                '1'..='9' => x += c as i8 - '1' as i8,
                '/' => {
                    assert!(x == 8);
                    y += 1;
                    x = -1;
                }
                _ => panic!("Invalid FEN Position"),
            }
            x += 1;
        }
        assert!(x == 8 && y == 7);

        board.turn = match fen[1] {
            "w" => Sides::White,
            "b" => Sides::Black,
            _ => panic!("Invalid FEN Side"),
        };

        for c in fen[2].to_string().chars() {
            let right = match c {
                'K' => CastleRights::WhiteKing,
                'Q' => CastleRights::WhiteQueen,
                'k' => CastleRights::BlackKing,
                'q' => CastleRights::BlackQueen,
                _ => panic!("Invalid FEN Castle Rights"),
            };
            board.castle_rights |= right as u8;
        };

        board
    }
}

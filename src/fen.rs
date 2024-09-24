use crate::board::*;

impl From<String> for GameState {
    fn from(fen: String) -> Self {
        let mut game = GameState {
            state: std::array::from_fn(|_| std::array::from_fn(|_| BitBoard(0))),
            turn: Sides::White,
            castle_rights: 0,
            en_passant: None,
        };

        let fen: Vec<&str> = fen.split_whitespace().collect();

        let mut x: u8 = 0;
        let mut y: u8 = 0;
        for c in fen[0].to_string().chars() {
            let sq = Square::from(x + y * 8);
            x += 1;
            match c {
                'P' => game.board_mut(Sides::White, Piece::Pawn).flip(sq),
                'R' => game.board_mut(Sides::White, Piece::Rook).flip(sq),
                'N' => game.board_mut(Sides::White, Piece::Knight).flip(sq),
                'B' => game.board_mut(Sides::White, Piece::Bishop).flip(sq),
                'Q' => game.board_mut(Sides::White, Piece::Queen).flip(sq),
                'K' => game.board_mut(Sides::White, Piece::King).flip(sq),
                'p' => game.board_mut(Sides::Black, Piece::Pawn).flip(sq),
                'r' => game.board_mut(Sides::Black, Piece::Rook).flip(sq),
                'n' => game.board_mut(Sides::Black, Piece::Knight).flip(sq),
                'b' => game.board_mut(Sides::Black, Piece::Bishop).flip(sq),
                'q' => game.board_mut(Sides::Black, Piece::Queen).flip(sq),
                'k' => game.board_mut(Sides::Black, Piece::King).flip(sq),
                '1'..='9' => x += c as u8 - b'1',
                '/' => {
                    x = 0;
                    y += 1;
                }
                _ => panic!("Invalid FEN Position"),
            }
        }

        game.turn = match fen[1] {
            "w" => Sides::White,
            "b" => Sides::Black,
            _ => panic!("Invalid FEN Side"),
        };

        if fen[2] != "-" {
            for c in fen[2].to_string().chars() {
                let right = match c {
                    'K' => Castle::WhiteKing,
                    'Q' => Castle::WhiteQueen,
                    'k' => Castle::BlackKing,
                    'q' => Castle::BlackQueen,
                    _ => panic!("Invalid FEN Castle Rights"),
                };
                game.castle_rights |= right as u8;
            }
        }

        if fen[3] != "-" {
            let mut chars = fen[3].chars();
            let file = chars.next().unwrap() as u8 - b'a';
            let row = b'8' - chars.next().unwrap() as u8;
            let square = Square::from(row * 8 + file);
            game.en_passant = Some(square);
        }

        // TODO: Halfmove and Fullmove

        game
    }
}

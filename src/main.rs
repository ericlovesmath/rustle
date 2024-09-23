mod board;
mod fen;
mod moves;

use board::{GameState, Piece, Sides, Square};
use macroquad::prelude::*;

#[macroquad::main("rustle")]
async fn main() {
    let chess_font = macroquad::text::load_ttf_font("./static/chess.ttf")
        .await
        .ok();

    let mut game = GameState::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    // let mut board = Board::from("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1".to_string());
    // let board = Board::from("r3k2r/ppp1pppp/8/2PpP3/4PP2/8/PPPPPPPP/R3K2R w KQkq d6 0 1".to_string());

    let mut curr = game.clone();
    let mut moves = game.moves();
    let mut index = 0;

    loop {
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let size = (screen_height() - offset_y * 2.) / 8f32;
        for x in 0..8 {
            for y in 0..8 {
                draw_rectangle(
                    offset_x + x as f32 * size,
                    offset_y + y as f32 * size,
                    size,
                    size,
                    if (x + y) % 2 == 1 { BROWN } else { BEIGE },
                );

                let checks = [
                    (Piece::Pawn, 'o'),
                    (Piece::Rook, 't'),
                    (Piece::Knight, 'm'),
                    (Piece::Bishop, 'v'),
                    (Piece::Queen, 'w'),
                    (Piece::King, 'l'),
                ];

                for (side, color) in [(Sides::White, WHITE), (Sides::Black, BLACK)] {
                    for (piece, text) in checks {
                        if curr.board(side, piece).get(Square::from(x + y * 8)) {
                            let text_params = TextParams {
                                font_size: (size * 0.80) as u16,
                                font: chess_font.as_ref(),
                                color,
                                ..Default::default()
                            };

                            let dim = measure_text(
                                &text.to_string(),
                                chess_font.as_ref(),
                                text_params.font_size,
                                1.0,
                            );

                            draw_text_ex(
                                &text.to_string(),
                                offset_x + x as f32 * size + (size - dim.width) / 2.0,
                                offset_y + y as f32 * size + (size + dim.height) / 2.0,
                                text_params,
                            );
                        }
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::Left) {
            index = (index + moves.len() - 1) % moves.len();
            println!("{}", moves[index]);
            curr = game.apply(moves[index].clone());
        }
        if is_key_pressed(KeyCode::Right) {
            index = (index + 1) % moves.len();
            println!("{}", moves[index]);
            curr = game.apply(moves[index].clone());
        }
        if is_key_pressed(KeyCode::Space) {
            game = game.apply(moves[rand::gen_range(0, moves.len())].clone());
            curr = game.clone();
            moves = game.moves();
            index = 0;
        }
        next_frame().await;
    }
}

mod board;
mod move_gen;

use board::{Board, Piece, Sides};
use macroquad::prelude::*;

#[macroquad::main("rustle")]
async fn main() {
    let chess_font = macroquad::text::load_ttf_font("./static/chess.ttf")
        .await
        .ok();

    let board = Board::new();
    let mut curr = board.clone();
    let moves = board.moves();
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
                        if curr.state[side as usize][piece as usize].flag(x + y * 8) {
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
            curr = board.apply(moves[index].clone());
        }
        if is_key_pressed(KeyCode::Right) {
            index = (index + 1) % moves.len();
            println!("{}", moves[index]);
            curr = board.apply(moves[index].clone());
        }
        next_frame().await;
    }
}

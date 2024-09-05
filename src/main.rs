mod board;
mod move_gen;

use board::Board;

fn main() {
    let board = Board::new();
    println!("From: {}", board);

    let moves = board.moves();
    for m in moves.iter() {
        println!("{}", m);
    }
}

mod board;
use board::Board;

fn main() {
    println!("Hello, world!");

    let mut game = board::Board::new(15);

    game.fill_board();
    game.print();
}

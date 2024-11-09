mod board;
use board::Board;

fn main() {
    println!("Hello, world!");

    let mut game = board::Board::new(20);

    game.fill_board();
    game.print();

    for _ in 1..10 {
        println!();
        game.generate_next();
        game.print();
    }
}

use std::{thread, time};

fn main() {
    let one_sec = time::Duration::from_millis(1000);

    let width = 50;
    let height = 20;

    // TODO: add support to load initial state from a file as well as save a state
    let mut board = gol::Board::build(width, height, 1);
    board.pretty_print();
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let b2 = board.next_board_state();
        b2.pretty_print();
        board = b2;
        thread::sleep(one_sec);
    }
    
}
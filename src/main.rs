fn main() {
    let board = gol::Board::build(10, 8, 1);

    board.pretty_print();
}

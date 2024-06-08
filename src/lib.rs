use rand;

#[derive(PartialEq, Clone)]
enum State {
    Alive,
    Dead,
}
#[derive(Clone)]
struct Cell {
    x: usize,
    y: usize,
    state: State,
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    data: Vec<Cell>,
}

impl Board {
    pub fn build(width: usize, height: usize, board_type: u8) -> Self {
        let data = vec![
            Cell {
                x: 0,
                y: 0,
                state: State::Dead,
            };
            width * height
        ];

        let mut board = Board { width, height, data };
        if board_type == 0 {
            board.init_dead_state();
        } else {
            board.init_random_state();
        }
        
        board
    }

    fn init_random_state(&mut self) -> () {
        for i in 0..self.width {
            for j in 0..self.height {
                self.data[j * self.width + i].x = i;
                self.data[j * self.width + i].y = j;
                if rand::random() {
                    self.data[j * self.width + i].state = State::Dead;
                } else {
                    self.data[j * self.width + i].state = State::Alive;
                }
            }
        }
    }

    fn init_dead_state(&mut self) -> () {
        for i in 0..self.width {
            for j in 0..self.height {
                self.data[j * self.width + i].x = i;
                self.data[j * self.width + i].y = j;
                self.data[j * self.width + i].state = State::Dead;

            }
        }
    }

    pub fn pretty_print(&self) {
        print!("+");
        for _i in 0..(self.height *2 + 1) {
            print!("-");
        }
        println!("+ ");

        for i in 0..self.width {
            print!("| ");
            for j in 0..self.height {
                if self.data[j * self.width + i].state == State::Alive {
                    print!("# ");
                } else {
                    print!("  ");
                }
            }
            println!("| ");
        }

        print!("+");
        for _i in 0..(self.height *2 + 1) {
            print!("-");
        }
        println!("+ ");
    }

/*     pub fn next_board_state(&mut self) -> () {
        let _new_state = Board::build(self.width, self.height, 0);
    }

    fn get_num_neighbours(&self, index: usize) -> u8 {
        let count: u8 = 0;
        count
    }
*/
}
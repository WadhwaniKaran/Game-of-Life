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
                let coord = j * self.width + i;
                self.data[coord].x = i;
                self.data[coord].y = j;
                if rand::random() {
                    self.data[coord].state = State::Dead;
                } else {
                    self.data[coord].state = State::Alive;
                }
            }
        }
    }

    fn init_dead_state(&mut self) -> () {
        for i in 0..self.width {
            for j in 0..self.height {
                let coord = j * self.width + i;
                self.data[coord].x = i;
                self.data[coord].y = j;
                self.data[coord].state = State::Dead;

            }
        }
    }

    pub fn pretty_print(&self) {
        print!("+");
        for _i in 0..(self.width) {
            print!("-");
        }
        println!("+");

        for j in 0..self.height {
            print!("|");
            for i in 0..self.width {
                if self.data[j * self.width + i].state == State::Alive {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }

        print!("+");
        for _i in 0..(self.width) {
            print!("-");
        }
        println!("+ ");
    }

    pub fn next_board_state(&self) -> Board {
        let mut new_state = Board::build(self.width, self.height, 0);
        for i in 0..self.width {
            for j in 0..self.height {
                let alive = self.num_alive_neighbours(i, j);
                let coord = j * self.width + i;
                if self.data[coord].state == State::Alive && alive <= 1 {
                    new_state.data[coord].state = State::Dead;
                } else if self.data[coord].state == State::Alive && alive <= 3 {
                    new_state.data[coord].state = State::Alive;
                } else if self.data[coord].state == State::Alive && alive > 3 {
                    new_state.data[coord].state = State::Dead;
                } else if self.data[coord].state == State::Dead && alive == 3 {
                    new_state.data[coord].state = State::Alive;
                }
            }
        }
        new_state 
    }

    fn num_alive_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        let arr_x = self.get_horizontals(x); // stores the x coords
        for i in 0..2 {
            if self.data[y*self.width+arr_x[i]].state == State::Alive {
                count += 1;
            }
        }
        let arr_y = self.get_verticals(y); // stores the y coords
        for i in 0..2 {
            if self.data[arr_y[i]*self.width+x].state == State::Alive {
                count += 1;
            }
        }
        for i in 0..2 {
            for j in 0..2 {
                if self.data[arr_y[j]*self.width+arr_x[i]].state == State::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_horizontals(&self, x: usize) -> [usize; 2] {
        let tx1 = (x / self.width) as i32;
        let tx2 = (x % self.width) as i32;

        let cx1 = ((tx2 + 1).rem_euclid(self.width as i32)) + tx1 * self.width as i32;
        let cx2 = ((tx2 - 1).rem_euclid(self.width as i32)) + tx1 * self.width as i32;
        [cx1 as usize, cx2 as usize]
    }

    fn get_verticals(&self, y: usize) -> [usize; 2] {
        let ty1 = (y / self.height) as i32;
        let ty2 = (y % self.height) as i32;

        let cy1 = ((ty2 + 1).rem_euclid(self.height as i32)) + ty1 * self.height as i32;
        let cy2 = ((ty2 - 1).rem_euclid(self.height as i32)) + ty1 * self.height as i32;
        [cy1 as usize, cy2 as usize]
    }
    
    #[allow(dead_code)]
    fn compare_boards(&self, other: &Board) -> bool {
        for i in 0..self.width {
            for j in 0..self.height {
                let coord = j * self.width + i;
                if self.data[coord].state != other.data[coord].state {
                    return false;
                }
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_state() { // dead state stays dead
        let b1 = Board::build(3, 3, 0);
        let b2 = b1.next_board_state();
        assert_eq!(b1.compare_boards(&b2), true);
    }
    #[test]
    fn some_state() { // cell with exact 3 neighbours should become alive
        let mut b1 = Board::build(5, 5, 0);
        b1.data[0].state = State::Alive;
        b1.data[5].state = State::Alive;
        b1.data[6].state = State::Alive;

        let b2 = b1.next_board_state();

        let mut b2_actual = Board::build(5, 5, 0);
        b2_actual.data[0].state = State::Alive;
        b2_actual.data[1].state = State::Alive;
        b2_actual.data[5].state = State::Alive;
        b2_actual.data[6].state = State::Alive;
        b1.pretty_print();
        b2.pretty_print();
        b2_actual.pretty_print();
        assert_eq!(b2_actual.compare_boards(&b2), true);
    }
} 
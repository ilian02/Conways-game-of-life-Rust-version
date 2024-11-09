use rand::Rng;

#[warn(dead_code)]
  
#[derive(Debug, Clone)]
enum Cell {
    Alive,
    Dead,
    None
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
    generation: i32,
    size: usize
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board { cells: vec![vec![Cell::Dead; size]; size], generation : 0, size }
    }

    pub fn print(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.cells[i][j] {
                    Cell::Alive => print!("*"),
                    Cell::Dead => print!("."),
                    Cell::None => print!("e") 
                }
            }
            println!();
        }
    }

    pub fn fill_board(&mut self) {
        let mut rng = rand::thread_rng();
        
        for i in 0..self.size {
            for j in 0..self.size {
                let random_bool: bool = rng.gen();
                match random_bool {
                    true => self.cells[i][j] = Cell::Alive,
                    false => self.cells[i][j] = Cell::Dead
                }
            }
            println!();
        }
    }

    pub fn generate_next(&self) {
        todo!();
    }
}

use rand::Rng;

#[warn(dead_code)]
  
#[derive(Debug, Clone, PartialEq, Copy)]
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
        }
    }

    pub fn generate_next(&mut self) {
        self.generation += 1;

        let mut new_matrix = vec![vec![Cell::Dead; self.size]; self.size];

        for i in 0..self.size {
            for j in 0..self.size {
                new_matrix[i][j] = self.get_new_cell_state(i, j);
            }
        }

        self.cells = new_matrix;
    }

    fn get_new_cell_state(&self, i:usize, j:usize) -> Cell{

        let mut counter = 0;

        // top right
        if i > 0 && j > 0 {
            counter += (self.cells[i - 1][j - 1] == Cell::Alive) as i32;
        }

        // top middle
        if i > 0 {
            counter += (self.cells[i - 1][j] == Cell::Alive) as i32;
        }

        // top left
        if i > 0 && j < self.size - 1 {
            counter += (self.cells[i - 1][j + 1] == Cell::Alive) as i32;
        }

        // middle right
        if j > 0 {
            counter += (self.cells[i][j - 1] == Cell::Alive) as i32;
        }

        // middle left
        if j < self.size - 1 {
            counter += (self.cells[i][j + 1] == Cell::Alive) as i32;
        }

        // bottom right
        if i < self.size - 1 && j > 0 {
            counter += (self.cells[i + 1][j - 1] == Cell::Alive) as i32;
        }

        // bottom middle
        if i < self.size - 1 {
            counter += (self.cells[i + 1][j] == Cell::Alive) as i32;
        }

        // bottom left
        if i < self.size - 1 && j < self.size - 1 {
            counter += (self.cells[i + 1][j + 1] == Cell::Alive) as i32;
        }

        match (self.cells[i][j], counter) {
            (Cell::Alive, 2) => return Cell::Alive,
            (Cell::Alive, 3) => return Cell::Alive,
            (Cell::Dead, 3) => return Cell::Alive,
            (_,_) => return Cell::Dead
        }
    }
}

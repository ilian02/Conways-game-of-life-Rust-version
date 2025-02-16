use std::collections::HashSet;

use rand::Rng;

pub struct ConwaysGame {
    pub grid: Vec<Vec<bool>>,
    pub grid_width: usize,
    pub grid_height: usize,
    pub alive_history: HashSet<(usize, usize)>,
    pub points: usize
}

impl ConwaysGame {
    pub fn new(grid_width: usize, grid_height: usize) -> Self {
        let grid = vec![vec![false; grid_width]; grid_height];
        Self { grid, grid_width, grid_height, alive_history: HashSet::new(), points: 0}
    }

    pub fn update_grid(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let live_neighbors = self.live_neighbor_count(x, y);
                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (true, 2) | (true, 3) => true,  // Survive
                    (false, 3) => true,             // Reproduce
                    _ => false,                    // Die
                };
                if new_grid[y][x] && self.alive_history.insert((y, x)) {
                    self.points += 3;
                } else if new_grid[y][x] {
                    self.points += 1;
                }
            }
        }
        self.grid = new_grid;
    }

    pub fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx).rem_euclid(self.grid_width as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(self.grid_height as isize) as usize;
                if self.grid[ny][nx] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.grid_width && y < self.grid_height {
            self.grid[y][x] = !self.grid[y][x];
        }
    }

    pub fn toggle_random(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.grid_height {
            for j in 0..self.grid_width {
                self.grid[i][j] = rng.gen_bool(0.5);
            }   
        } 
    }

    pub fn print_cells(&self) {
        for i in 0..self.grid_height {
            for j in 0..self.grid_width {
                print!("{}",self.grid[i][j]);
            }
            println!();
        } 
    }
}
use ggez::*;
use std::{fs, io};
use std::path::Path;
use std::time::Duration;
use ggez::input::keyboard::KeyCode;

pub mod game;
use game::ConwaysGame;

enum GameState {
    Menu,
    Versus,
    Freeplay
}

struct State {
    game_state: GameState,
    last_update: Duration,
    camera_x: usize,
    camera_y: usize,
    view_width: usize,
    view_height: usize,
    per_row: usize,
    paused: bool,
    game: ConwaysGame,
    current_gen: usize,
    max_gen: usize,
    game_ended: bool,
    starting_max: usize,
    currently_places: usize,
    started: bool
}


impl State {
    fn draw_grid(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let mut mesh_builder = graphics::MeshBuilder::new();
        let square_size = self.view_width / self.per_row;

        for y in self.camera_y..(self.camera_y + self.per_row).min(self.game.grid_height) {
            for x in self.camera_x..(self.camera_x + self.per_row).min(self.game.grid_width) {
                let color = if self.game.grid[y][x] {
                    graphics::Color::BLACK
                } else {
                    graphics::Color::WHITE
                };

                let rect = graphics::Rect::new(
                    (x - self.camera_x) as f32 * square_size as f32,
                    (y - self.camera_y) as f32 * square_size as f32,  
                    square_size as f32, 
                    square_size as f32
                );

                let _ = mesh_builder.rectangle(graphics::DrawMode::fill(), rect, color);
            }
        }

        let mesh = graphics::Mesh::from_data(ctx, mesh_builder.build());
        canvas.draw(&mesh, graphics::DrawParam::default());

        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for State {

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let keyboard = ctx.keyboard.clone();
        if keyboard.is_key_pressed(KeyCode::Right) {
            if self.camera_x + self.per_row < self.game.grid_width {
                self.camera_x = self.camera_x + 1;
            }
        }
        if keyboard.is_key_pressed(KeyCode::Left) {
            if self.camera_x > 0 {
                self.camera_x = self.camera_x - 1;
            }
        }
        if keyboard.is_key_pressed(KeyCode::Down) {
            
            if self.camera_y + self.per_row < self.game.grid_height {
                self.camera_y = self.camera_y + 1;
            }
        }
        if keyboard.is_key_pressed(KeyCode::Up) {
            if self.camera_y > 0 {
                self.camera_y = self.camera_y - 1;
            }
        }
        if keyboard.is_key_just_released(KeyCode::Space) {
            
            self.started = true;

            if !self.game_ended {
                self.paused = !self.paused;
            }
        }

        if keyboard.is_key_pressed(KeyCode::P) {
            if self.per_row < self.game.grid_width - 2 {
                self.per_row = self.per_row + 1;
            }
        }

        if keyboard.is_key_pressed(KeyCode::M) {
            if self.per_row > 5 { 
                self.per_row = self.per_row - 1;
            }
        }

        if keyboard.is_key_pressed(KeyCode::Q) {
            ctx.request_quit();
        }

        
        let now = ctx.time.time_since_start();

        if !self.paused {
            match &mut self.game_state {
                GameState::Freeplay => {
                    if now - self.last_update >= Duration::from_millis(100) {
                        self.game.update_grid();
                        self.last_update = now;
                    }
                }
                GameState::Versus => {        
                    if now - self.last_update >= Duration::from_millis(100) {
                        self.game.update_grid();
                        self.last_update = now;
                        if self.current_gen < self.max_gen {
                            self.current_gen += 1;
                        } else {
                            self.paused = true;
                            self.game_ended = true;
                            println!("Maximum rounds reaches:\n points: {}", self.game.points);
                        }
                    }
                }
                _ => {

                }
            }
        } else {
            if ctx.mouse.button_just_pressed(event::MouseButton::Left) {
                let x_coord = ctx.mouse.position().x;
                let y_coord = ctx.mouse.position().y;

                let x_cell = (x_coord as usize) / (self.view_width / self.per_row) + self.camera_x;
                let y_cell = (y_coord as usize) / (self.view_height / self.per_row) + self.camera_y;

                match self.game_state {
                    GameState::Freeplay => {
                        self.game.toggle_cell(x_cell, y_cell);
                    }
                    GameState::Versus => {
                        if !self.started {
                            self.game.toggle_cell(x_cell, y_cell);
                            if self.game.grid[y_cell][x_cell] && self.currently_places < self.starting_max {
                                self.currently_places += 1;
                            } else {
                                self.currently_places -= 1;
                            }
                            println!("Currently places: {}, {} left", self.currently_places, self.starting_max - self.currently_places);
                        }
                        
                    }
                    _ => {

                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);

        match self.game_state {
            GameState::Freeplay => {
                self.draw_grid(ctx, &mut canvas)?;
            }
            GameState::Versus => { 
                self.draw_grid(ctx, &mut canvas)?;
            }
            _ => {

            }
        }

        canvas.finish(ctx)
    }

}


fn print_rules() {
    println!("Conways game of life is a cellular automaton, meaning it is played without a player.");
    println!("There are only 4 rules which every cell follows");
    println!("1 -> Any live cell with fewer than two live neighbours dies, as if by underpopulation.");
    println!("2 -> Any live cell with two or three live neighbours lives on to the next generation.");
    println!("3 -> Any live cell with more than three live neighbours dies, as if by overpopulation.");
    println!("4 -> Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.");
}

fn versus_menu(game: &mut State) {

    let options = [(16, 30, 30), (16, 100, 30), (32, 150, 45), (64, 200, 60)];

    loop {
    println!("In the versus mode you can pick your level and see how many points you get:\nEach first time a cell is alive gives 3 points and after that you get 1 point for them");
    println!("Level 1: start 16 with 30 generations and 30x30 board");
    println!("Level 2: start 16 with 100 generations and 30x30 board");
    println!("Level 3: start 32 with 150 generations and 45x45 board");
    println!("Level 4: start 64 with 200 generations and 60x60 board");
    println!("Input number of level");
    print!("> ");

    let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim().to_string();

    match input.trim().parse::<usize>() {
        Ok(num) if (1..=4).contains(&num) => {
            if !input.is_empty() && ["1", "2", "3", "4"].contains(&input.as_str()) {
                game.starting_max = options[num - 1].0;
                game.max_gen = options[num - 1].1;

                let conway_board = ConwaysGame::new(options[num - 1].2, options[num - 1].2);
                game.game = conway_board;
                break;
            } else {
                println!("Invalid choice, please enter a valid option.");
            }
        },
            _ => println!("Invalid choice, please enter a number between 1 and 4."),
        }
    }
}

fn freeplay_menu(game: &mut State) {
    loop {
        println!("1) Empty board");
        println!("2) Random 50/50 board");
        println!("3) Load file");
        print!("> ");

        let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();

        match input {
            "1" => {
                game.game = ConwaysGame::new(100,100);
                return
            },
            "2" => {
                game.game = ConwaysGame::new(100,100);
                game.game.toggle_random();
                return
            },
            "3" => {
                if read_file(game) {
                    return;
                }
            },
            _ => {
                println!("Invalid command. Try again\n> ")
            }
        }

    }
}

fn read_file(game: &mut State) -> bool {
    println!("Enter relative path of file:\n> ");
    let mut file_name_input = String::new();
    io::stdin().read_line(&mut file_name_input).expect("Failed to read input");

    let file_name = file_name_input.trim();

    if !Path::new(file_name).exists() {
        println!("Error: File '{}' does not exist.", file_name);
        return false;
    }


    let content = fs::read_to_string(file_name).unwrap();
    let mut lines = content.lines();

    let width: usize = lines.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let height: usize = lines.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    game.game = ConwaysGame::new(width, height);

    if width == 0 || height == 0 {
        println!("Error: Invalid width or height!");
        return false;
    }

    let mut grid: Vec<Vec<char>> = Vec::new();
    for row_index in 0..height {
        if let Some(line) = lines.next() {
            if line.len() != width {
                println!("Error: Row {} length ({}) does not match width ({})!", row_index, line.len(), width);
                return false;
            }

            let row_symbols: Vec<char> = line.chars().collect();
            grid.push(row_symbols);
        } else {
            println!("Error: Not enough rows in the file!");
            return false;
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &symbol) in row.iter().enumerate() {
            match symbol {
                '0' => game.game.grid[y][x] = false,
                '1' => game.game.grid[y][x] = true,
                _ => {
                    println!("Invalid element in grid");
                    return false;
                }
            }
        }
    }


    return true;
}


pub fn main() {

    println!("Conways game of life: the game");
    print_rules();
    println!("Pick mode: \n 1) Freeplay\n 2) Versus");

    
    let answer: usize;
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(value) => { answer = value; break },
            Err(_) => println!("Invalid input, please enter a valid usize."),
        }
    }
    
    let freeplay_conway = ConwaysGame::new(10, 10);

    let mut state = State {
        game_state: GameState::Menu,
        last_update: Duration::ZERO,
        camera_x: 0,
        camera_y: 0,
        view_height: 600,
        view_width: 600,
        per_row: 15,
        paused: true,
        game: freeplay_conway,
        current_gen: 0,
        max_gen: 50,
        game_ended: false,
        starting_max: 16,
        currently_places: 0,
        started: false
    };
    match answer {
        1 => {
            println!("You picked freeplay");
            state.game_state = GameState::Freeplay;
            freeplay_menu(&mut state);
        }
        2 => {
            println!("You picked versus");
            versus_menu(&mut state);
            state.game_state = GameState::Versus;
        }
        _ => {println!("Error picking.")}
    }

    let c = ggez::conf::Conf {
        window_mode: ggez::conf::WindowMode {
            width: 600.0,
            height: 600.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let (ctx, event_loop) = ContextBuilder::new("Conways Game", "The one and only")
    .default_conf(c)
    .build()
    .unwrap();
    event::run(ctx, event_loop, state)
}
    
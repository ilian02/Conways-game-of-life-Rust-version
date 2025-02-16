use ConwaysGame::game::ConwaysGame;


#[test]
fn test_new_game() {
    let game = ConwaysGame::new(5, 5);
    assert_eq!(game.grid_width, 5);
    assert_eq!(game.grid_height, 5);
    assert_eq!(game.grid.len(), 5);
    assert!(game.grid.iter().all(|row| row.len() == 5));
    assert!(game.grid.iter().all(|row| row.iter().all(|&cell| !cell)));
}

#[test]
fn test_toggle_cell() {
    let mut game = ConwaysGame::new(5, 5);
    game.toggle_cell(2, 2);
    assert!(game.grid[2][2]); // Cell should be alive

    game.toggle_cell(2, 2);
    assert!(!game.grid[2][2]); // Cell should be dead again
    }

#[test]
fn test_live_neighbor_count() {
    let mut game = ConwaysGame::new(5, 5);
    game.grid[1][1] = true;
    game.grid[1][2] = true;
    game.grid[2][1] = true;
    assert_eq!(game.live_neighbor_count(2, 2), 3);
}

#[test]
fn test_update_grid() {
    let mut game = ConwaysGame::new(5, 5);
    game.grid[1][1] = true;
    game.grid[1][2] = true;
    game.grid[2][1] = true;

    game.update_grid();

    assert!(game.grid[2][2]); // This cell should become alive (3 neighbors)
    assert!(game.grid[1][1]); // This cell should stay alive (2 neighbors)
}

#[test]
fn test_toggle_random() {
    let mut game = ConwaysGame::new(5, 5);
    game.toggle_random();
        
    let alive_count = game.grid.iter().flatten().filter(|&&cell| cell).count();
    assert!(alive_count > 0 && alive_count < 25); // Ensure some cells are toggled
}
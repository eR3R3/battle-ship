use std::io::{self, Write};
use rand::{random_range, Rng};

const BOARD_SIZE: usize = 10;

struct Board{
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize,usize)>,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState{
    Empty,
    Ship,
    Hit,
    Miss
}

impl Board{
    fn new()->Self{
        Board{
            grid:[[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }

    fn place_ship(&mut self, size: usize){
        let mut rng = rand::rng();
        loop{
            let row = rng.random_range(0..BOARD_SIZE);
            let col = rng.random_range(0..BOARD_SIZE);
            let direction = rng.random::<bool>();
            if self.can_place_ship( row, col,size, direction){
                for i in 0..size {
                    let (row, col) = if direction { (row, col + i) } else { (row + i, col) };
                    self.grid[row][col] = CellState::Ship;
                    self.ships.push((row, col));
                }
                break;
            } else {
                continue;
            }
        }
    }

    #[allow(unused_variables)]
    fn can_place_ship(&self, row:usize, col:usize, size:usize, direction: bool)->bool{
        if direction{
            if col + size > BOARD_SIZE{ return false }
            for i in 0..size{
                if self.grid[row][col+i]!=CellState::Empty { return false }
            }
        } else {
            if row + size > BOARD_SIZE { return false }
            for i in 0..size{
                if self.grid[row+i][col]!=CellState::Empty { return false }
            }
        }
        true
    }

    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col]{
            CellState::Empty => {
                self.grid[row][col]=CellState::Miss;
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col]=CellState::Hit;
                CellState::Hit
            },
            _ => CellState::Miss
        }
    }

    fn display(&self, hide_ships: bool){
        print!("   ");
        for i in 0..BOARD_SIZE {
            print!("{} ", i);
        }
        println!("");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{} ", i);
            for cell in row {
                match cell {
                    CellState::Empty => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A1} ");
                        }
                    }
                    CellState::Ship => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A0} ");
                        }
                    }
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),
                    CellState::Miss => print!("\x1b[36m \u{00B7} \x1b[0m"),
                }
            }
            println!();
        }
    }

    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|&(r, c)| self.grid[r][c] == CellState::Hit)
    }
}

fn main() {
    // Initialize the game board for the player and the opponent
    let mut player_board = Board::new();
    let mut opponent_board = Board::new();

    // Place ships on the boards
    player_board.place_ship(5); // Aircraft Carrier
    player_board.place_ship(4); // Battleship
    player_board.place_ship(3); // Cruiser
    player_board.place_ship(3); // Submarine
    player_board.place_ship(2); // Destroyer

    opponent_board.place_ship(5); // Similarly place ships for the opponent
    opponent_board.place_ship(4);
    opponent_board.place_ship(3);
    opponent_board.place_ship(3);
    opponent_board.place_ship(2);

    // Main game loop
    loop {
        // Clear the screen for a fresh display of the game board each turn
        print!("\x1b[2J\x1b[1;1H");

        // Display the player's board and the opponent's board
        println!("\x1b[1;37mYour Board:\x1b[0m");
        player_board.display(false); // Display player's board with ships visible
        println!("\x1b[1;37mOpponent's Board:\x1b[0m");
        opponent_board.display(true); // Display opponent's board with ships hidden

        // Player's turn: prompt for input and process the firing result
        let (player_row, player_col) = get_player_input(); // Get coordinates from the player
        let result = opponent_board.fire(player_row, player_col);
        match result {
            CellState::Miss => println!("\x1b[36mYou missed!\x1b[0m"),
            CellState::Hit => println!("\x1b[31mYou hit a ship!\x1b[0m"),
            _ => (), // No action needed for other states
        }
        println!("Press Enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");

        // Check if all opponent ships have been sunk
        if opponent_board.is_game_over() {
            println!("\x1b[1;32mCongratulations! You sank all of your opponent's ships!\x1b[0m");
            break; // End the game loop if the game is over
        }

        // Opponent's turn: simulate opponent move (could be AI-controlled in future enhancements)
        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("\x1b[36mOpponent missed!\x1b[0m"),
            CellState::Hit => println!("\x1b[31mOpponent hit one of your ships!\x1b[0m"),
            _ => (), // No action needed for other states
        }
        println!("Press Enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");

        // Check if all player ships have been sunk
        if player_board.is_game_over() {
            println!("\x1b[1;31mOh no! All of your ships have been sunk!\x1b[0m");
            break; // End the game loop if the game is over
        }
    }
}

// Function to get player input for firing
fn get_player_input() -> (usize, usize) {
    loop {
        print!("\x1b[1;37mEnter coordinates to fire (row, col): \x1b[0m");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input is typed
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coordinates: Vec<usize> = input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid input"))
            .collect();
        if coordinates.len() == 2 && coordinates[0] < BOARD_SIZE && coordinates[1] < BOARD_SIZE {
            return (coordinates[0], coordinates[1]); // Return valid coordinates
        } else {
            println!("\x1b[1;31mInvalid input. Please enter row and column numbers separated by a comma.\x1b[0m");
        }
    }
}

// Function to generate a random move for the opponent
fn generate_opponent_move() -> (usize, usize) {
    let mut rng = rand::rng(); // Use a random number generator for move selection
    (rng.random_range(0..BOARD_SIZE), random_range(0..BOARD_SIZE)) // Return a random row and column
}
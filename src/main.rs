struct Game {
    pieces: [[usize; 3]; 3]
}

impl Game {
    fn new() -> Game {
        Game {
            pieces: [[0, 0, 0],
                     [0, 0, 0],
                     [0, 0, 0]]
            // 1 on the board indicates an X, 2 on the board indicates an O.
        }
    }

    fn print_board(&self) {
        println!("------------");

        let mut counter : u32 = 1; // This is for displaying the box indexes to the user.

        // Matching piece numbers to letters and displaying them to make the board.
        for elem in self.pieces {
            let mut first : char = '_';
            match elem[0] {
                1 => first = 'X',
                2 => first = 'Y',
                _ => {}
            }

            let mut second : char = '_';
            match elem[1] {
                1 => second = 'X',
                2 => second = 'Y',
                _ => {}
            }

            let mut third : char = '_';
            match elem[2] {
                1 => third = 'X',
                2 => third = 'Y',
                _ => {}
            }

            for _i in 0..3 {
                print!("{}   ", counter);
                counter += 1;
            }
            println!("");

            println!("{} | {} | {}", first, second, third);
            println!("------------");
        }
    }

    fn game_iteration(&mut self) -> &str {
        print!("\x1B[2J");
        self.print_board();

        println!("Player X please input your first placement: ");
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Error reading");
        let x = x.trim().parse::<i32>();
        let mut coords : (i32, i32) = (0, 0);
        match x.unwrap() {
            1 => coords = (0, 0),
            2 => coords = (1, 0),
            3 => coords = (2, 0),
            4 => coords = (0, 1),
            5 => coords = (1, 1),
            6 => coords = (2, 1),
            7 => coords = (0, 2),
            8 => coords = (1, 2),
            9 => coords = (2, 2),
            _ => ()
        }
        
        print!("\x1B[2J");
        self.pieces[coords.1 as usize][coords.0 as usize] = 1 as usize;

        self.print_board();
        // Check if anyone has won yet.
        let finished = self.check_win_lose();
        match finished.as_str() {
            "X" => { return "X wins!" },
            "Y" => { return "Y wins!" },
            _ => ()
        }

        println!("Player O please input your first placement: ");
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Error reading");
        let x = x.trim().parse::<i32>();
        let mut coords : (i32, i32) = (0, 0);
        match x.unwrap() {
            1 => coords = (0, 0),
            2 => coords = (1, 0),
            3 => coords = (2, 0),
            4 => coords = (0, 1),
            5 => coords = (1, 1),
            6 => coords = (2, 1),
            7 => coords = (0, 2),
            8 => coords = (1, 2),
            9 => coords = (2, 2),
            _ => ()
        }

        print!("\x1B[2J");
        self.pieces[coords.1 as usize][coords.0 as usize] = 2 as usize;

        self.print_board();
        // Check if anyone has won yet.
        let finished = self.check_win_lose();
        match finished.as_str() {
            "X" => { return "X wins!" },
            "Y" => { return "Y wins!" },
            _ => { return "ITERATE" }
        }

        /*
        // Inputting coordinates for the X player (X, Y coords).
        // Read coords first and parse them into usize values.
        println!("Player X please input your first coordinate (X coord): ");

        let mut first_x_coord = String::new();
        std::io::stdin().read_line(&mut first_x_coord).expect("Error reading");
        let first_x_coord = first_x_coord.trim().parse::<usize>();

        println!("Player X please input your second coordinate (Y coord): ");

        let mut second_x_coord = String::new();
        std::io::stdin().read_line(&mut second_x_coord).expect("Error reading");
        let second_x_coord = second_x_coord.trim().parse::<usize>();

        // Assign the X value (1) to the specified coords.
        self.pieces[second_x_coord.unwrap()][first_x_coord.unwrap()] = 1 as usize;

        // Print the board to the user.
        self.print_board();

        // Check if anyone has won yet.
        let finished = self.check_win_lose();
        match finished.as_str() {
            "X" => { return "X wins!" },
            "Y" => { return "Y wins!" },
            _ => {}
        }


        // Inputting coordinates for the O player (X, Y coords).
        // Read coords first and parse them into usize values.
        println!("Player O please input your first coordinate (X coord): ");

        let mut first_o_coord = String::new();
        std::io::stdin().read_line(&mut first_o_coord).expect("Error reading");
        let first_o_coord = first_o_coord.trim().parse::<usize>();

        println!("Player O please input your second coordinate (Y coord): ");

        let mut second_o_coord = String::new();
        std::io::stdin().read_line(&mut second_o_coord).expect("Error reading");
        let second_o_coord = second_o_coord.trim().parse::<usize>();

        // Assign the O value (2) to the specified coords.
        self.pieces[second_o_coord.unwrap()][first_o_coord.unwrap()] = 2 as usize;

        // Print the board to the user.
        self.print_board();

        // Check if anyone has won yet.
        let finished = self.check_win_lose();
        match finished.as_str() {
            "X" => { return "X wins!" },
            "Y" => { return "Y wins!" },
            _ => { return "ITERATE" }
        }
        */
    }

    fn check_win_lose(&self) -> String {
        // Check if anyone has a horizontal win.
        for elem in self.pieces {
            if elem[0] == elem[1] && elem[1] == elem[2] {
                match elem[0] {
                    1 => return "X".to_string(),
                    2 => return "Y".to_string(),
                    _ => return "ERROR".to_string()
                }
            }
        }

        // Check if anyone has a vertical win.
        for elem in 0..3 {
            if self.pieces[0][elem] == self.pieces[1][elem] && self.pieces[1][elem] == self.pieces[2][elem] {
                match self.pieces[0][elem] {
                    1 => return "X".to_string(),
                    2 => return "Y".to_string(),
                    _ => return "ERROR".to_string()
                }
            }
        }

        // Check if anyone has a diagonal win.
        if self.pieces[0][0] == self.pieces[1][1] && self.pieces[1][1] == self.pieces[2][2] {
            match self.pieces[0][0] {
                1 => return "X".to_string(),
                2 => return "Y".to_string(),
                _ => return "ERROR".to_string()
            }
        }
        if self.pieces[0][2] == self.pieces[1][1] && self.pieces[1][1] == self.pieces[2][0] {
            match self.pieces[0][2] {
                1 => return "X".to_string(),
                2 => return "Y".to_string(),
                _ => return "ERROR".to_string()
            }
        }

        "No dice".to_string()
    }
}

fn main() {
    // Construct our game manager.
    let mut game_mgr = Game::new();

    // We loop the game until someone wins ("check_win_lose" used in "game_iteration").
    loop {
        // Store the result of "game_iteration" to check if the game has concluded.
        let return_code = game_mgr.game_iteration();

        // Check what the "return_code" value and respond...
        match return_code {
            "ITERATE" => (),                                // If "ITERATE" has been returned, just iterate over the code again a.k.a run another game tick.
            _ => { println!("{}", return_code); return; }   // All other values will mean a player has won, which means 
                                                            // we just need to print it and return from main (end the program).
        }
    }
}
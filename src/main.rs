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
            println!("{} {} {}", first, second, third);
        }
    }

    fn game_iteration(&mut self) -> &str {
        println!("Player X please input your first coordinate (X coord): ");

        let mut first_x_coord = String::new();
        std::io::stdin().read_line(&mut first_x_coord).expect("Error reading");
        let first_x_coord = first_x_coord.trim().parse::<u32>();

        println!("Player X please input your second coordinate (Y coord): ");

        let mut second_x_coord = String::new();
        std::io::stdin().read_line(&mut second_x_coord).expect("Error reading");
        let second_x_coord = second_x_coord.trim().parse::<i32>();

        self.pieces[second_x_coord.unwrap() as usize][first_x_coord.unwrap() as usize] = 1 as usize;

        self.print_board();
        let finished = self.check_win_lose();
        match finished.as_str() {
            "X" => { return "X wins!" },
            "Y" => { return "Y wins!" },
            _ => {}
        }

        println!("Player O please input your first coordinate (X coord): ");

        let mut first_o_coord = String::new();
        std::io::stdin().read_line(&mut first_o_coord).expect("Error reading");
        let first_o_coord = first_o_coord.trim().parse::<u32>();

        println!("Player O please input your second coordinate (Y coord): ");

        let mut second_o_coord = String::new();
        std::io::stdin().read_line(&mut second_o_coord).expect("Error reading");
        let second_o_coord = second_o_coord.trim().parse::<i32>();

        self.pieces[second_o_coord.unwrap() as usize][first_o_coord.unwrap() as usize] = 2 as usize;

        self.print_board();
        let finished = self.check_win_lose();
        match finished.as_str() {
            "X" => { return "X wins!" },
            "Y" => { return "Y wins!" },
            _ => { return "ITERATE" }
        }
    }

    fn check_win_lose(&self) -> String {
        // Check horizontal winnings
        for elem in self.pieces {
            if elem[0] == elem[1] && elem[1] == elem[2] {
                match elem[0] {
                    1 => return "X".to_string(),
                    2 => return "Y".to_string(),
                    _ => return "ERROR".to_string()
                }
            }
        }

        // Check vertical winnings
        for iter in 0..3 {
            if self.pieces[0][iter] == self.pieces[1][iter] && self.pieces[1][iter] == self.pieces[2][iter] {
                match self.pieces[0][iter] {
                    1 => return "X".to_string(),
                    2 => return "Y".to_string(),
                    _ => return "ERROR".to_string()
                }
            }
        }

        "No dice".to_string()
    }
}

fn main() {
    let mut game_mgr = Game::new();

    loop {
        let return_code = game_mgr.game_iteration();
        match return_code {
            "ITERATE" => (),
            _ => { println!("{}", return_code); return; }
        }
    }
}
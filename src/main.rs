use std::io;
use std::process;
use Mark::Empty;
use rand::Rng;

#[derive(Copy, Clone)]
enum PlayerType{
    Human,
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq,Copy,Clone)]
enum Mark {
    X,
    O,
    Empty,
}

struct Game {
    grid: Vec<Mark>,
    currentplayer: Player,
    nextplayer: Player,
}


impl Game {
    /// Returns the mark at the appropriate x, y position.
    fn get_mark(&self, x: usize, y: usize) -> Mark {
        if x > 2 {
            panic!("X out of bounds: {}", x);
        }

        if y > 2 {
            panic!("Y out of bounds: {}", y);
        }
        match self.grid.get(x + y * 3) {
            Some(m) => return *m,
            _ => {panic!("Something went wrong accessing the grid: X: {} Y: {}", x, y)}
        }
    }

    /// Sets the mark on the square. If a mark already exists on the square, the function returns false.
    fn set_mark(&mut self, x: usize, y: usize, mark: Mark) -> bool {
        if self.get_mark(x, y) == Empty {
            self.grid[x + y * 3] = mark;
            true
        }
        else {
            false
        }        
    }

    /// Returns true if either side won, false if neither side has won.
    fn won(&self) -> bool {
        // Horizontal win conditions
        if self.get_mark(0,0) == self.get_mark(1,0) && 
           self.get_mark(1,0) == self.get_mark(2,0) && 
           self.get_mark(0,0) != Mark::Empty {
            return true    
        }
        
        if self.get_mark(0,1) == self.get_mark(1,1) && 
           self.get_mark(1,1) == self.get_mark(2,1) &&
           self.get_mark(0,1) != Mark::Empty {
            return true
        }
        
        if self.get_mark(0,2) == self.get_mark(1,2) &&
           self.get_mark(1,2) == self.get_mark(2,2) &&
           self.get_mark(0,2) != Mark::Empty {
                    return true
        }
    
        // Vertical win conditions
        if self.get_mark(0,0) == self.get_mark(0,1) &&
           self.get_mark(0,1) == self.get_mark(0,2) &&
           self.get_mark(0,0) != Mark::Empty {
                    return true
        }

        if self.get_mark(1,0) == self.get_mark(1,1) &&
           self.get_mark(1,1) == self.get_mark(1,2) &&
           self.get_mark(1,0) != Mark::Empty {
                    return true
        }

        if self.get_mark(2,0) == self.get_mark(2,1) &&
           self.get_mark(2,1) == self.get_mark(2,2) &&
           self.get_mark(2,0) != Mark::Empty {
                    return true
        }

        // Diagonal win conditions
        if self.get_mark(0,0) == self.get_mark(1,1) &&
           self.get_mark(1,1) == self.get_mark(2,2) &&
           self.get_mark(0,0) != Mark::Empty {
                    return true
        }

        if self.get_mark(0,2) == self.get_mark(1,1) &&
           self.get_mark(1,1) == self.get_mark(2,0) &&
           self.get_mark(0,2) != Mark::Empty {
                return true
        }

        // Everything else failed, so nobody's won yet.
        return false
    }


    /// Returns true if the game is drawn.
    fn is_draw(&self) -> bool {
        for i in self.grid.as_slice() {
            if *i == Mark::Empty {
                return false;
            }
        }
        true
    }

    fn take_turn(&mut self) {
        match &self.currentplayer.ptype {
            PlayerType::Human => self.human_turn(),
            PlayerType::Easy => self.easy_turn(),
            PlayerType::Medium => self.med_turn(),
            PlayerType::Hard => self.hard_turn(),
        }
        self.print_grid();
    }

    fn human_turn(&self) {
        panic!("Human interface not yet implemented");
    }

    fn easy_turn(&mut self) {
        loop {
            let mut rng = rand::thread_rng();
            let x: usize = rng.gen_range(0, 3);
            let y: usize = rng.gen_range(0, 3);
            if self.set_mark(x, y, self.currentplayer.mark) {
                break;
            }
        }
    }

    fn med_turn(&self) {
        panic!("Medium AI not yet implemented.");
    }

    fn hard_turn(&self) {
        panic!("Hard AI not yet implemented.");
    }

    fn print_mark(&self, x: usize, y: usize) -> char {
        match self.get_mark(x, y) {
            Mark::X => 'X',
            Mark::O => 'O',
            Empty => ' ',
        }
    }

    fn print_grid(&self) {
        println!("+-+-+-+");
        println!("|{}|{}|{}|", self.print_mark(0,2), self.print_mark(1,2), self.print_mark(2,2));
        println!("+-+-+-+");
        println!("|{}|{}|{}|", self.print_mark(0,1), self.print_mark(1,1), self.print_mark(2,1));
        println!("+-+-+-+");
        println!("|{}|{}|{}|", self.print_mark(0,0), self.print_mark(1,0), self.print_mark(2,0));
        println!("+-+-+-+");
    }
}

#[derive(Copy, Clone)]
struct Player{
    ptype: PlayerType,
    mark: Mark,
}

fn print_menu() {
    println!("Type 'start [] []' where [] can be 'Human', 'Easy', 'Medium', and 'Hard'.");
}

fn menu() -> (PlayerType, PlayerType) {
    loop {
        print_menu();
        println!("Input command:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let mitem: Vec<&str> = input.split_whitespace().collect();

                match mitem[0].to_lowercase().as_str() {
                    "help" | "menu" | "?" => print_menu(),
                    "exit" | "quit" | "stop" => process::exit(0),
                    "start" => {
                        let p1 = match mitem[1].to_lowercase().trim() {
                            "human" | "player" | "user" => PlayerType::Human,
                            "easy" => PlayerType::Easy,
                            "medium" => PlayerType::Medium,
                            "hard" => PlayerType::Hard,
                            _ => continue,
                        };
                        let p2 = match mitem[2].to_lowercase().trim() {
                            "human" => PlayerType::Human,
                            "easy" => PlayerType::Easy,
                            "medium" => PlayerType::Medium,
                            "hard" => PlayerType::Hard,
                            _ => continue,
                        };
                        return (p1, p2)
                    }
                    _ => continue
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn main() {
    loop {
        let (cp, np) = menu();

        let mut game = Game {
            grid: vec![Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty],
            currentplayer: Player {ptype: cp, mark: Mark::X},
            nextplayer: Player {ptype: np, mark: Mark::O},
        };

        game.print_grid();

        while (game.won() == false) && (game.is_draw() == false) {
            if game.currentplayer.mark == Mark::X {
                println!("\nX Player turn:");
            }
            else
            { 
                println!("\nO Player's turn:");
            }

            game.take_turn();
            
            // TODO: Figure out why mem::swap didn't work here.
            let player_holder: Player = game.currentplayer;
            game.currentplayer = game.nextplayer;
            game.nextplayer = player_holder
        }

        if game.is_draw() {
            println!("The game is drawn!");
        }
        if game.won() {
            if game.nextplayer.mark == Mark::X {
                println!("The game is won by X!")
            }
            if game.nextplayer.mark == Mark::O {
                println!("The game is won by O!")
            }
        }
    }
}
// the number guessing game from chapter 2 of the book of rust
// but modified to use structs and other cool stuff that I learned
// yeah something like that

use std::io;
use std::cmp::Ordering;
use std::ops::Sub;
use rand::Rng;
use std::time::Instant;

enum GameEnd {
    WIN(i32),
    LOSS,
}

struct GameStats {
    end: GameEnd,
    number: u32, // the secret number
    lives_left: i32, // 0 if loss
    round_length: u128, // time taken for round
}

#[derive(Debug)]
struct AllTimeStats {
    games_played: i32,
    wins: i32,
    avg_guesses: f64,
    win_rate: f64,
    avg_round_length: u128,
}

impl AllTimeStats {
    fn add_game(&mut self, game: &GameStats){
        self.avg_round_length = ((self.avg_round_length * self.games_played as u128) + game.round_length) / (self.games_played as u128 + 1 as u128);

        
        // from the book of rust:
        /*
            "Using if let means less typing, less indentation, and less boilerplate code.
            However, you lose the exhaustive checking that match enforces.
            Choosing between match and if let depends on what you’re doing in your
            particular situation and whether gaining conciseness is an appropriate
            trade-off for losing exhaustive checking."
        */
        // what conciseness? do they really think I'm going to be writing concise code?

        // try and figure out wtf this does
        self.avg_guesses = if let GameEnd::WIN(guesses) = game.end {
            (self.avg_guesses * (self.wins as f64) + (guesses as f64)) / (self.wins as f64 + 1.)
        } else {
            self.avg_guesses
        };

        // oh that's what they meant by "conciseness"
        if let GameEnd::WIN(_) = game.end { self.wins += 1; }
    
        self.games_played += 1;
        self.win_rate = (self.wins as f64) / (self.games_played as f64);
    }
}

fn randint() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // // can't use the `rand` crate because no wifi is like that
    // let secret_number = 6;

    secret_number
}

fn run_game(max_lives: i32) -> GameStats {
    let rand = randint();
    let mut lives = max_lives;
    let start_time = Instant::now();

    // YOU CAN RETURN VALUES FROM LOOPS??
    // I FREAKIN LOVE THiS LANGUAGE
    let game: GameStats = loop {
        println!("Guess the number!");
        println!("What's your guess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("pwease enter a valid number :(");
                continue;
            }
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&rand) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                break GameStats {
                    end: GameEnd::WIN(max_lives - lives),
                    round_length: Instant::now().sub(start_time).as_millis(),
                    lives_left: lives,
                    number: rand
                };
            },
        }

        lives -= 1;
        if lives == 0 {
            print!("out of lives... ");
            break GameStats {
                end: GameEnd::LOSS,
                lives_left: 0,
                round_length: Instant::now().sub(start_time).as_millis(),
                number: rand
            };
        }else {
            println!("{lives} lives left!");
        }
    };


    println!("{}", match game.end { GameEnd::WIN(_) => "You win!", _ => "Next time!"});
    println!("Match stats:");
    println!("\tNumber to guess: {0}", game.number);
    println!("\tRound length: {0}s", game.round_length / 1_000);
    println!("\tLives left: {0}", game.lives_left);

    game
}

fn main() {
    let mut stats: AllTimeStats = AllTimeStats {
        games_played: 0,
        wins: 0,
        avg_guesses: 0.,
        win_rate: 0.0,
        avg_round_length: 0,
    };
    const MAX_LIVES: i32 = 8; // some of my spotlight search math suggests that this is the number of guesses needed to do binary search on the range 1-100

    loop {
        let game: GameStats = run_game(MAX_LIVES);

        stats.add_game(&game);

        println!("play again? (y/n)");
        let mut ans: String = String::new();

        io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line");

        if ans.to_uppercase().trim() != "Y"{
            break;
        }
    };

    // dbg!(stats);
    println!(
        "Out of {} game{}, you won {} (that's {}%!)",
        stats.games_played,
        if stats.games_played == 1 { "" } else { "s" },
        stats.wins,
        (stats.win_rate * 100.) as i32
    );
    println!(
        "When you won, it took you an average of {} guesses and {} seconds to guess the number.",
        stats.avg_guesses,
        stats.avg_round_length as f64 / 1_000.
    );
}

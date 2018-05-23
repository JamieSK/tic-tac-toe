extern crate noughts_and_crosses;

use noughts_and_crosses::*;

use std::io;

fn main() {
    let mut game = Game::new();
    println!("Welcome to Noughts and Crosses.");

    loop {
        match game.state() {
            State::InPlay => {
                println!("\n{}", game.to_string());
                let input = take_input(game.player);
                match game.play(input) {
                    Err(e) => println!("{}", e),
                    _ => {},
                }
            },
            State::Stalemate => {
                println!("\nOh, stalemate, boring.");
                break;
            },
            State::Won(player) => {
                println!("\n{}", game.to_string());
                println!("{}, you've won!", player);
                break;
            },
        }
    }
}

fn take_input(player: Player) -> usize {
    println!("Where would you like to play, {}?", player);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failure to read line.");
    let input_number: usize = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => take_input(player),
    };
    input_number
}

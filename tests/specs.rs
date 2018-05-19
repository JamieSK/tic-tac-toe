extern crate Noughts_and_Crosses;

use Noughts_and_Crosses::Game;

#[test]
fn it_starts_blank() {
    let game = Game::new();
    assert_eq!(" | | \n | | \n | | \n", game.to_string());
}

#[test]
fn can_play_as_x() {
    let mut game = Game::new();
    game.play(1).unwrap();
    assert_eq!("X| | \n | | \n | | \n", game.to_string());
}

#[test]
fn can_play_as_o() {
    let mut game = Game::new();
    game.play(1).unwrap();
    game.play(2).unwrap();
    assert_eq!("X|O| \n | | \n | | \n", game.to_string());
}

#[test]
fn cannot_play_in_occupied_cell() {
    let mut game = Game::new();
    game.play(1).unwrap();
    assert_eq!(game.play(1), Err("Occupied cell."));
}
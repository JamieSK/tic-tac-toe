extern crate Noughts_and_Crosses;

use Noughts_and_Crosses::Game;
use Noughts_and_Crosses::Player;

#[test]
fn it_starts_blank() {
    let game = Game::new();
    assert_eq!(" | | \n | | \n | | \n", game.to_string());
}

#[test]
fn can_play_as_x() {
    let mut game = Game::new();
    game.play(1);
    assert_eq!("X| | \n | | \n | | \n", game.to_string());
}

#[test]
fn can_play_as_o() {
    let mut game = Game::new();
    game.play(1);
    game.play(2);
    assert_eq!("X|O| \n | | \n | | \n", game.to_string());
}
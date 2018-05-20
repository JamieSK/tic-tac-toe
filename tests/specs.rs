extern crate noughts_and_crosses;

use noughts_and_crosses::Game;
use noughts_and_crosses::State;
use noughts_and_crosses::Player;

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

#[test]
fn can_play_in_all_cells() {
    let mut game = Game::new();
    game.play(1).unwrap();
    game.play(2).unwrap();
    game.play(3).unwrap();
    game.play(6).unwrap();
    game.play(4).unwrap();
    game.play(7).unwrap();
    game.play(5).unwrap();
    game.play(9).unwrap();
    game.play(8).unwrap();
    assert_eq!("X|O|X\nX|X|O\nO|X|O\n", game.to_string());
}

#[test]
fn unplayed_game_is_in_unfinished_state() {
    let mut game = Game::new();
    assert_eq!(State::InPlay, game.state());
}

#[test]
fn full_board_with_no_winner_is_stalemate() {
    let mut game = Game::new();
    game.play(1).unwrap();
    game.play(2).unwrap();
    game.play(3).unwrap();
    game.play(6).unwrap();
    game.play(4).unwrap();
    game.play(7).unwrap();
    game.play(5).unwrap();
    game.play(9).unwrap();
    game.play(8).unwrap();
    assert_eq!(State::Stalemate, game.state());
}

#[test]
fn can_win_horizontally_on_top_row() {
    let mut game = Game::new();
    game.play(1).unwrap();
    game.play(4).unwrap();
    game.play(2).unwrap();
    game.play(5).unwrap();
    game.play(3).unwrap();
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_horizontally_on_middle_row() {
    let mut game = Game::new();
    game.play(4).unwrap();
    game.play(1).unwrap();
    game.play(5).unwrap();
    game.play(2).unwrap();
    game.play(6).unwrap();
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_horizontally_on_bottom_row() {
    let mut game = Game::new();
    game.play(7).unwrap();
    game.play(1).unwrap();
    game.play(8).unwrap();
    game.play(2).unwrap();
    game.play(9).unwrap();
    assert_eq!(State::Won(Player::X), game.state());
}
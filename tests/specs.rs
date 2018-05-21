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
    let game = setup_game_and_play_in_cells(vec![1]);
    assert_eq!("X| | \n | | \n | | \n", game.to_string());
}

#[test]
fn can_play_as_o() {
    let game = setup_game_and_play_in_cells(vec![1, 2]);
    assert_eq!("X|O| \n | | \n | | \n", game.to_string());
}

#[test]
fn cannot_play_in_occupied_cell() {
    let mut game = setup_game_and_play_in_cells(vec![1]);
    assert_eq!(game.play(1), Err("Occupied cell."));
}

#[test]
fn can_play_in_all_cells() {
    let game = setup_game_and_play_in_cells(vec![1, 2, 3, 6, 4, 7, 5, 9, 8]);
    assert_eq!("X|O|X\nX|X|O\nO|X|O\n", game.to_string());
}

#[test]
fn unplayed_game_is_in_unfinished_state() {
    let mut game = Game::new();
    assert_eq!(State::InPlay, game.state());
}

#[test]
fn full_board_with_no_winner_is_stalemate() {
    let mut game = setup_game_and_play_in_cells(vec![1, 2, 3, 6, 4, 7, 5, 9, 8]);
    assert_eq!(State::Stalemate, game.state());
}

#[test]
fn can_win_horizontally_on_top_row() {
    let mut game = setup_game_and_play_in_cells(vec![1, 4, 2, 5, 3]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_horizontally_on_middle_row() {
    let mut game = setup_game_and_play_in_cells(vec![4, 1, 5, 2, 6]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_horizontally_on_bottom_row() {
    let mut game = setup_game_and_play_in_cells(vec![7, 1, 8, 2, 9]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_vertically_on_left_column() {
    let mut game = setup_game_and_play_in_cells(vec![1, 2, 4, 5, 7]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_vertically_on_middle_column() {
    let mut game = setup_game_and_play_in_cells(vec![2, 1, 5, 4, 8]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_vertically_on_right_column() {
    let mut game = setup_game_and_play_in_cells(vec![3, 1, 6, 4, 9]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_diagonally_from_top_left() {
    let mut game = setup_game_and_play_in_cells(vec![1, 2, 5, 3, 9]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_diagonally_from_bottom_left() {
    let mut game = setup_game_and_play_in_cells(vec![7, 2, 5, 1, 3]);
    assert_eq!(State::Won(Player::X), game.state());
}

#[test]
fn can_win_as_o() {
    let mut game = setup_game_and_play_in_cells(vec![7, 1, 4, 2, 8, 3]);
    assert_eq!(State::Won(Player::O), game.state());
}

#[test]
fn cannot_play_out_of_board() {
    let mut game = Game::new();
    assert_eq!(Err("Off the board."), game.play(0));
}

fn setup_game_and_play_in_cells(cells: Vec<usize>) -> Game {
    let mut game = Game::new();
    for cell in cells {
        game.play(cell).unwrap();
    }
    game
}
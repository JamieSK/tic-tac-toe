use std::fmt;

#[derive(Copy, Clone)]
pub enum Player {
    X,
    O,
    None,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                Player::X => "X",
                Player::O => "O",
                Player::None => " ",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Won,
    Stalemate,
    InPlay,
}

pub struct Game {
    player: Player,
    board: [Player; 9],
    total_turns: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::X,
            board: [Player::None; 9],
            total_turns: 0,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}|{}|{}\n{}|{}|{}\n{}|{}|{}\n",
                self.board[0], self.board[1], self.board[2],
                self.board[3], self.board[4], self.board[5],
                self.board[6], self.board[7], self.board[8])
    }

    pub fn play(&mut self, cell: usize) -> Result<&str, &str> {
        match self.board[cell - 1] {
            Player::X | Player::O => Err("Occupied cell."),
            Player::None => {
                self.board[cell - 1] = self.player;
                self.total_turns += 1;
                swap_player(self);
                Ok("Played a turn")
            }
        }
    }

    pub fn state(& self) -> State {
        if self.total_turns > 8 {
            State::Stalemate
        } else {
            State::InPlay
        }
    }
}

fn swap_player(game: &mut Game) {
    game.player = match game.player {
        Player::X => Player::O,
        Player::O => Player::X,
        Player::None => Player::None,
    }
}

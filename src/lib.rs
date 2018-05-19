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

pub struct Game {
    player: Player,
    board: [Player; 9],
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::X,
            board: [Player::None; 9],
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}|{}|{}\n{}|{}|{}\n{}|{}|{}\n",
                self.board[0], self.board[1], self.board[2],
                self.board[3], self.board[4], self.board[5],
                self.board[6], self.board[7], self.board[8])
    }

    pub fn play(&mut self, cell: usize) {
        self.board[cell - 1] = self.player;
        swap_player(self);
    }

}

fn swap_player(game: &mut Game) {
    game.player = match game.player {
        Player::X => Player::O,
        Player::O => Player::X,
        Player::None => Player::None,
    }
}

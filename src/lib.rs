use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum State {
    Won(Player),
    Stalemate,
    InPlay,
}

pub struct Game {
    player: Player,
    board: [Player; 9],
    total_turns: u8,
    winner: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::X,
            board: [Player::None; 9],
            total_turns: 0,
            winner: Player::None,
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

    pub fn state(&mut self) -> State {
        self.set_winner();
        if self.winner != Player::None {
            State::Won(self.winner)
        } else if self.total_turns > 8 {
            State::Stalemate
        } else {
            State::InPlay
        }
    }

    fn set_winner(&mut self) {
        let lines: Vec<Vec<usize>> = vec![
            vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8],
            vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8],
            vec![0, 4, 8], vec![6, 4, 2],
        ];

        for line in lines {
            let first_cell = line[0];
            if self.is_winning_line(line) {
                self.winner = self.board[first_cell];
            }
        }
    }

    fn is_winning_line(&self, cells: Vec<usize>) -> bool {
        let mut cells_iter = cells.iter();
        let first: Player = self.board[*cells_iter.next().unwrap()];

        if first == Player::None {
            return false
        }

        for cell in cells_iter {
            if self.board[*cell] != first {
                return false;
            }
        }

        return true;
    }
}

fn swap_player(game: &mut Game) {
    game.player = match game.player {
        Player::X => Player::O,
        Player::O => Player::X,
        Player::None => Player::None,
    }
}

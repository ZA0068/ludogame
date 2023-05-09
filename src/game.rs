mod game {
    use board::Board;
    use players::Player;
    pub struct Game {
        players: Vec<Player>,
        board: Board,
    }

    impl Game {
        pub fn new_default() -> Game {
            Game {
                players: (0..4).map(Player::new).collect(),
                board: Board::new(),
            }
        }

        pub fn new_with_players(players: i8) -> Game {
            Game {
                players: (0..players).map(Player::new).collect(),
                board: Board::new(),
            }
        }

        pub fn new_with_flags(player1: bool, player2: bool, player3: bool, player4: bool) -> Game {
            let mut players = Vec::new();
            if player1 {
                players.push(Player::new(0));
            }
            if player2 {
                players.push(Player::new(1));
            }
            if player3 {
                players.push(Player::new(2));
            }
            if player4 {
                players.push(Player::new(3));
            }
            Game {
                players,
                board: Board::new(),
            }
        }

        pub fn player(&mut self, id: u8) -> &mut Player {
            &mut self.players[id as usize]
        }

        pub fn players(&mut self) -> &mut Vec<Player> {
            &mut self.players
        }

        pub fn board(&self) -> &Board {
            &self.board
        }
    }
}

pub use game::Game;

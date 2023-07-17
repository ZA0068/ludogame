mod game {
    pub use board::Board;
    use dice::Dice;
    use iplayers::{Behavior, IPlayer, Playstyle};
    use std::{cell::RefCell, rc::Rc};

    #[derive(Clone, Debug, PartialEq)]
    pub struct Game {
        iplayers: Vec<IPlayer>,
        board: Rc<RefCell<Board>>,
        dice: Dice,
    }

    impl Game {
        pub fn new() -> Self {
            let board = Rc::new(RefCell::new(Board::new()));
            let dice = Dice::default();
            Self {
                iplayers: vec![
                    IPlayer::new(0),
                    IPlayer::new(1),
                    IPlayer::new(2),
                    IPlayer::new(3),
                ],
                board,
                dice,
            }
        }

        pub fn reset_game(&mut self) {
            self.board.borrow_mut().reset();
            self.iplayers.iter_mut().for_each(|iplayer| {
                iplayer.reset_first_round();
            });
        }

        pub fn setup_game(&mut self) {
            for iplayer in &mut self.iplayers {
                iplayer.setup_board(self.board.clone());
            }
        }

        pub fn get_board(&self) -> Rc<RefCell<Board>> {
            self.board.clone()
        }

        pub fn reset_scores(&mut self) {
            self.iplayers.iter_mut().for_each(|iplayer| {
                iplayer.reset_scores();
            });
        }

        pub fn start_game(&mut self, total_games: u16) {
            self.reset_scores();
            for _ in 0..total_games {
                self.play_game();
            }
        }

        fn play_game(&mut self) {
            self.beginning();
            self.run();
            self.reset_game();
        }

        pub fn iplayer(&mut self, id: i8) -> &mut IPlayer {
            let index = self.iplayers.iter().position(|player| player.player().id() == id).unwrap();
            &mut self.iplayers[index]
        }

        pub fn give_iplayer_a_playstyle(&mut self, id: i8, playstyle: Playstyle) {
            self.iplayers[id as usize].set_playstyle(playstyle);
        }

        pub fn set_iplayer(&mut self, id: i8, iplayer: &mut IPlayer) {
            self.iplayer(id).substitute(iplayer);
        }

        pub fn get_iplayer(&mut self, id: i8, iplayer: &mut IPlayer) {
            iplayer.substitute(self.iplayer(id));
        }

        pub fn run(&mut self) {
            loop {
                for player_idx in 0..4 {
                    self.play_turn(player_idx);
                    if self.has_player_won(player_idx) {
                        return;
                    }
                    self.next_turn(player_idx);
                }
            }
        }

        fn play_turn(&mut self, player_idx: usize) {
            self.iplayers[player_idx].my_turn();
            self.iplayers[player_idx].play(false);
        }

        fn next_turn(&mut self, player_idx: usize) {
            self.iplayers[player_idx]
                .clone()
                .give_dice(&mut self.iplayers[(player_idx + 1) % 4]);
        }

        fn has_player_won(&mut self, player_idx: usize) -> bool {
            if self.iplayers[player_idx].player().is_finished() {
                self.iplayers[player_idx].win();
                return true;
            }
            false
        }

        pub fn beginning(&mut self) {
            let mut scores: Vec<(i8, i32)> = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
            self.roll_dice_for_players(&mut scores);
            while self.has_ties(&scores) {
                self.adjust_scores_for_tied_players(&mut scores);
            }
            self.sort_players_by_scores(&scores);
            self.iplayers[0].take_dice(self.dice.clone());
        }

        fn has_ties(&self, scores: &[(i8, i32)]) -> bool {
            let max_score = self.get_max_score(scores);
            scores
                .iter()
                .filter(|&(_, score)| *score == max_score)
                .count()
                > 1
        }

        fn get_max_score(&self, scores: &[(i8, i32)]) -> i32 {
            scores.iter().map(|(_, score)| *score).max().unwrap_or(0)
        }

        fn roll_dice_for_players(&mut self, scores: &mut [(i8, i32)]) {
            for (idx, iplayer) in self.iplayers.iter_mut().enumerate() {
                iplayer.take_dice(self.dice.clone());
                iplayer.roll_dice();
                scores[idx].1 += iplayer.player().get_dice_number() as i32;
            }
        }

        fn adjust_scores_for_tied_players(&mut self, scores: &mut [(i8, i32)]) {
            loop {
                self.roll_dice_for_players(scores);
                let (tied_players, _max_score) = self.get_tied_players(scores);
                if tied_players.len() == 1 {
                    break;
                }
                for &player_id in &tied_players {
                    if let Some((_id, _player_score)) =
                        scores.iter_mut().find(|(id, _)| *id == player_id)
                    {}
                }
            }
        }

        fn get_tied_players(&self, scores: &[(i8, i32)]) -> (Vec<i8>, i32) {
            let max_score = self.get_max_score(scores);
            let tied_players: Vec<_> = scores
                .iter()
                .filter_map(|(id, score)| if *score == max_score { Some(*id) } else { None })
                .collect();
            (tied_players, max_score)
        }

        fn sort_players_by_scores(&mut self, scores: &[(i8, i32)]) {
            self.iplayers.sort_by_key(|p| {
                let player_score = scores
                    .iter()
                    .find(|(id, _)| *id == p.player().id())
                    .unwrap()
                    .1;
                -player_score // use negative to sort in descending order
            });
        }
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use game::Game;

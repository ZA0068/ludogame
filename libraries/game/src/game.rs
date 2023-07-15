mod game {
    use board::Board;
    use dice::Dice;
    use players::{Act, Player};
    use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
    use std::{cell::RefCell, rc::Rc};
    use iplayers::IPlayer;

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

        pub fn reset(&mut self)
        {
            let board = Rc::new(RefCell::new(Board::new()));
            let dice = Dice::default();
            self.iplayers = vec![
                IPlayer::new(0),
                IPlayer::new(1),
                IPlayer::new(2),
                IPlayer::new(3),
            ];
        }


        // fn play_game(&mut self, genetic_action: &[Act]) -> Result<i8, Box<dyn std::error::Error>> {
        //     loop {
        //         self.iplayers[0].genetic(genetic_action.to_vec());
        //         self.iplayers[2].safe();
        //         self.iplayers[1].fast();
        //         self.iplayers[3].aggressive();

        //         match self
        //             .iplayers
        //             .iter_mut()
        //             .find(|iplayer| iplayer.player.is_finished())
        //         {
        //             Some(iplayer) => return Ok(iplayer.player.id()),
        //             None => continue,
        //         }
        //     }
        // }

        // pub fn first_round(&mut self) {
        //     let mut rng = rand::thread_rng();
        //     for iplayer in self.iplayers {
        //         let mut roll_count = 0;
        //         while roll_count < 3 {
        //             if iplayer.roll_dice() == 6 {
        //                 iplayer.free_piece(rng.gen_range(0..4));
        //                 break;
        //             }
        //             roll_count += 1;
        //         }
        //     }
        // }

        // pub fn beginning(&mut self) {
        //     let mut scores = Vec::new();

        //     for (index, iplayer) in self.iplayers.iter_mut().enumerate() {
        //         scores.push((index, iplayer.player.roll_dice()));
        //     }

        //     scores.sort_by(|a, b| b.1.cmp(&a.1));

        //     let mut scores_with_duplicates = scores.clone();

        //     let mut duplicate_scores = std::collections::HashSet::new();
        //     for &(_, score) in &scores_with_duplicates {
        //         if self.count_score_occurrences(&scores_with_duplicates, score) > 1 {
        //             duplicate_scores.insert(score);
        //         }
        //     }
        //     scores_with_duplicates.retain(|&(_, score)| !duplicate_scores.contains(&score));

        //     let new_order: Vec<usize> = scores_with_duplicates
        //         .iter()
        //         .map(|&(index, _)| index)
        //         .collect();

        //     self.iplayers.sort_by_key(|iplayer| {
        //         new_order
        //             .iter()
        //             .position(|&x| x == iplayer.player.id() as usize)
        //             .unwrap_or(0)
        //     });
        // }

        fn count_score_occurrences(&self, scores: &[(usize, i8)], score: i8) -> usize {
            scores.iter().filter(|&(_, s)| *s == score).count()
        }
    }

    impl Default for Game {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use game::Game;
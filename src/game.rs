mod game {
    use std::{cell::RefCell, rc::Rc};

    use board::Board;
    use players::{Player, Act};
    use dice::Dice;
    use pieces::Piece;
    use std::cmp::Reverse;
    use rand::Rng;

    static AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Goal,
        Act::Skip,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];
    static FAST_AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Goal,
        Act::Skip,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];
    static SAFE_ACTIONS: [Act; 10] = [
        Act::Join,
        Act::Safe,
        Act::Goal,
        Act::Move,
        Act::Kill,
        Act::Skip,
        Act::Free,
        Act::Leave,
        Act::Nothing,
        Act::Die,
    ];
    static FAST_ACTIONS: [Act; 10] = [
        Act::Goal,
        Act::Skip,
        Act::Leave,
        Act::Join,
        Act::Move,
        Act::Kill,
        Act::Free,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

#[derive(Clone)]
    pub struct IPlayer {
        player: Player,
    }

    impl IPlayer {
        pub fn new(player: Player) -> Self {
            IPlayer { player }
        }
    }

    pub struct Game {
        iplayers: Vec<IPlayer>,
        board: Board,
        dice: Dice,
    }

    trait Playstyles {
        fn aggressive(&mut self);
        fn fast(&mut self);
        fn random(&mut self);
        fn safe(&mut self);
        fn fast_aggressive(&mut self);
        fn genetic(&mut self);
    }


    impl Playstyles for IPlayer {
        fn aggressive(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(AGGRO_ACTIONS.to_vec(), true);
        }
        fn fast(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(SAFE_ACTIONS.to_vec(), true);
        }
        fn random(&mut self) {
            self.player.my_turn();
            self.player.random_play(FAST_ACTIONS.to_vec());
        }
        fn safe(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(SAFE_ACTIONS.to_vec(), false);
        }
        fn fast_aggressive(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(FAST_AGGRO_ACTIONS.to_vec(), true);
        }
        fn genetic(&mut self) {
            self.player.my_turn();
            self.player.ordered_play(FAST_ACTIONS.to_vec(), true);
        }
    }


    impl Game{
        pub fn new() -> Self {
            let board = Rc::new(RefCell::new(Board::new()));
            let dice = Rc::new(RefCell::new(Dice::new()));
            let player0 = Player::new(0, board.clone(), Some(dice.clone()));
            let player1 = Player::new(1, board.clone(), Some(dice.clone()));
            let player2 = Player::new(2, board.clone(), Some(dice.clone()));
            let player3 = Player::new(3, board.clone(), Some(dice.clone()));
            let board = board.borrow().to_owned();
            let dice = dice.borrow().to_owned();
            Self {
                iplayers: vec![IPlayer::new(player0), IPlayer::new(player1), IPlayer::new(player2), IPlayer::new(player3)],
                board,
                dice,
            }
        }

        pub fn start_the_game(&mut self) {

            self.beginning();


            let mut turn = 0;
            loop {
                let iplayer = &mut self.iplayers[turn];
                iplayer.aggressive();
                turn = (turn + 1) % 4;
                if iplayer.player.is_finished() {
                    break;
                }
            }
        }

        pub fn first_round(&mut self) {
            let mut rng = rand::thread_rng();
            for iplayer in &mut self.iplayers {
                let mut roll_count = 0;
                while roll_count < 3 {
                    if iplayer.player.roll_dice() == 6 {
                        iplayer.player.free_piece(rng.gen_range(0..4));
                        break;
                    }
                    roll_count += 1;
                }
            }
        }

        pub fn beginning(&mut self) {
            let mut scores = Vec::new();
        
            for (index, iplayer) in self.iplayers.iter_mut().enumerate() {
                scores.push((index, iplayer.player.roll_dice()));
            }
        
            scores.sort_by(|a, b| b.1.cmp(&a.1));
        
            let mut scores_with_duplicates = scores.clone();
        
            let mut duplicate_scores = std::collections::HashSet::new();
            for &(_, score) in &scores_with_duplicates {
                if self.count_score_occurrences(&scores_with_duplicates, score) > 1 {
                    duplicate_scores.insert(score);
                }
            }
            scores_with_duplicates.retain(|&(_, score)| !duplicate_scores.contains(&score));
            
            
        
            let new_order: Vec<usize> = scores_with_duplicates.iter().map(|&(index, _)| index).collect();
        
            self.iplayers.sort_by_key(|iplayer| {
                new_order.iter().position(|&x| x == iplayer.player.id() as usize).unwrap_or(0)
            });
        }    

        fn count_score_occurrences(&self, scores: &[ (usize, i8) ], score: i8) -> usize {
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

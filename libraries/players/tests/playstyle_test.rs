// use board::Board;
// use dice::Dice;
// use players::{Act, Player};
// use std::{cell::RefCell, rc::Rc};

// #[cfg(test)]
// mod playstyle_tests {
//     use super::*;

//     static ACTIONS: [Act; 10] = [
//         Act::Move,
//         Act::Free,
//         Act::Kill,
//         Act::Join,
//         Act::Leave,
//         Act::Die,
//         Act::Goal,
//         Act::Safe,
//         Act::Skip,
//         Act::Nothing,
//     ];

//     static AGGRO_ACTIONS: [Act; 10] = [
//         Act::Kill,
//         Act::Move,
//         Act::Join,
//         Act::Free,
//         Act::Goal,
//         Act::Skip,
//         Act::Leave,
//         Act::Safe,
//         Act::Nothing,
//         Act::Die,
//     ];

//     #[test]
//     fn aggressive_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

//         let mut aggressive_player = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut random_player = Player::new(2, board, Some(dice));
//         let take_closest = false;
//         loop {
//             random_player.my_turn();
//             random_player.random_play(ACTIONS.to_vec());
//             if random_player.is_finished() {
//                 println!("random_player wins");
//                 break;
//             }
//             aggressive_player.my_turn();
//             aggressive_player.ordered_play(AGGRO_ACTIONS.to_vec(), take_closest);
//             if aggressive_player.is_finished() {
//                 println!("aggressive_player wins");
//                 break;
//             }
//         }
//         assert!(aggressive_player.is_finished() || random_player.is_finished());
//     }

//     static FAST_AGGRO_ACTIONS: [Act; 10] = [
//         Act::Kill,
//         Act::Goal,
//         Act::Skip,
//         Act::Move,
//         Act::Join,
//         Act::Free,
//         Act::Leave,
//         Act::Safe,
//         Act::Nothing,
//         Act::Die,
//     ];

//     #[test]
//     fn fast_aggressive_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

//         let mut fast_aggressive_player = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut random_player = Player::new(3, board, Some(dice));
//         let take_closest = true;
//         loop {
//             random_player.my_turn();
//             random_player.random_play(ACTIONS.to_vec());
//             if random_player.is_finished() {
//                 println!("random_player wins");
//                 break;
//             }

//             fast_aggressive_player.my_turn();
//             fast_aggressive_player.ordered_play(FAST_AGGRO_ACTIONS.to_vec(), take_closest);
//             if fast_aggressive_player.is_finished() {
//                 println!("fast aggressive_player wins");
//                 break;
//             }
//         }
//         assert!(fast_aggressive_player.is_finished() || random_player.is_finished());
//     }

//     static SAFE_ACTIONS: [Act; 10] = [
//         Act::Join,
//         Act::Safe,
//         Act::Goal,
//         Act::Move,
//         Act::Kill,
//         Act::Skip,
//         Act::Free,
//         Act::Leave,
//         Act::Nothing,
//         Act::Die,
//     ];

//     #[test]
//     fn safest_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

//         let mut safe_player = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut random_player = Player::new(3, board, Some(dice));
//         let take_closest = true;
//         loop {
//             random_player.my_turn();
//             random_player.random_play(ACTIONS.to_vec());
//             if random_player.is_finished() {
//                 println!("random_player wins");
//                 break;
//             }
//             safe_player.my_turn();
//             safe_player.ordered_play(SAFE_ACTIONS.to_vec(), take_closest);
//             if safe_player.is_finished() {
//                 println!("safe player wins");
//                 break;
//             }
//         }
//         assert!(safe_player.is_finished() || random_player.is_finished());
//     }
//     static FAST_ACTIONS: [Act; 10] = [
//         Act::Goal,
//         Act::Skip,
//         Act::Leave,
//         Act::Join,
//         Act::Move,
//         Act::Kill,
//         Act::Free,
//         Act::Safe,
//         Act::Nothing,
//         Act::Die,
//     ];

//     #[test]
//     fn fastest_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

//         let mut fastest_player = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut random_player = Player::new(2, board, Some(dice));
//         let take_closest = true;
//         loop {
//             random_player.my_turn();
//             random_player.random_play(ACTIONS.to_vec());
//             if random_player.is_finished() {
//                 println!("random_player wins");
//                 break;
//             }
//             fastest_player.my_turn();
//             fastest_player.ordered_play(FAST_ACTIONS.to_vec(), take_closest);
//             if fastest_player.is_finished() {
//                 println!("fastest player wins");
//                 break;
//             }
//         }
//         assert!(fastest_player.is_finished() || random_player.is_finished());
//     }

//     #[test]
//     #[ignore]
//     fn final_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut fastest_player = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut safe_player = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut fast_aggressive_player = Player::new(2, board.clone(), Some(dice.clone()));
//         let mut aggressive_player = Player::new(3, board, Some(dice));
//         let take_closest = true;

//         let mut winrate: Vec<f32> = vec![0.0; 4];

//         for _ in 0..1000 {
//             loop {
//                 fastest_player.my_turn();
//                 fastest_player.ordered_play(FAST_ACTIONS.to_vec(), take_closest);
//                 if fastest_player.is_finished() {
//                     println!("fastest_player wins");
//                     winrate[0] += 1.0;
//                     break;
//                 }
//                 safe_player.my_turn();
//                 safe_player.ordered_play(SAFE_ACTIONS.to_vec(), take_closest);
//                 if safe_player.is_finished() {
//                     println!("safe player wins");
//                     winrate[1] += 1.0;
//                     break;
//                 }

//                 fast_aggressive_player.my_turn();
//                 fast_aggressive_player.ordered_play(FAST_AGGRO_ACTIONS.to_vec(), take_closest);
//                 if fast_aggressive_player.is_finished() {
//                     println!("fast aggressive_player wins");
//                     winrate[2] += 1.0;
//                     break;
//                 }

//                 aggressive_player.my_turn();
//                 aggressive_player.ordered_play(AGGRO_ACTIONS.to_vec(), take_closest);
//                 if aggressive_player.is_finished() {
//                     println!("aggressive_player wins");
//                     winrate[3] += 1.0;
//                     break;
//                 }
//             }
//             assert!(
//                 fastest_player.is_finished()
//                     || safe_player.is_finished()
//                     || fast_aggressive_player.is_finished()
//                     || aggressive_player.is_finished()
//             );
//             fastest_player.reset();
//             safe_player.reset();
//             fast_aggressive_player.reset();
//             aggressive_player.reset();
//         }
//         println!("fastest_player winrate: {}", winrate[0] / 100.0);
//         println!("safe player winrate: {}", winrate[1] / 100.0);
//         println!("fast aggressive_player winrate: {}", winrate[2] / 100.0);
//         println!("aggressive_player winrate: {}", winrate[3] / 100.0);
//     }
// }

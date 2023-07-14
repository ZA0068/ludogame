use board::Board;
use dice::Dice;
use players::{Act, Player};
use std::{cell::RefCell, rc::Rc};



mod multiplayer_tests {
    use super::*;

    static ACTIONS: [Act; 10] = [
        Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Starjump,
        Act::Nothing,
    ];

    #[test]
    #[ignore = "long test"]
    fn player_0_vs_1_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player_0 = Player::new(0, board.clone());
        let mut player1 = Player::new(1, board);
        
            player1.take_dice(dice);
            loop {
                player1.my_turn();
                player1.play_random(ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player1.give_dice(&mut player_0);
            player_0.my_turn();
            player_0.play_random(ACTIONS.to_vec());
            if player_0.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player_0.give_dice(&mut player1);
        }
        assert!(player_0.is_finished() || player1.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_0_vs_1_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player_0 = Player::new(0, board.clone());
        let mut player1 = Player::new(1, board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            player1.take_dice(dice.clone());
            loop {
                player1.my_turn();
                player1.play_random(ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 0 wins");
                winrates[0] += 1.0;
                break;
            }
            player1.give_dice(&mut player_0);
            player_0.my_turn();
            player_0.play_random(ACTIONS.to_vec());
            if player_0.is_finished() {
                println!("Player 1 wins");
                winrates[1] += 1.0;
                break;
            }
            player_0.give_dice(&mut player1);
        }
        assert!(player_0.is_finished() || player1.is_finished());
        board.borrow_mut().reset();
    }
    println!("Player 0 winrate: {}", winrates[0] / (1.0 * max_iter as f32));
    println!("Player 1 winrate: {}", winrates[1] / (1.0 * max_iter as f32));
    }

    #[test]
    #[ignore = "long test"]
    fn player_0_vs_2_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0, board.clone());
        let mut player2 = Player::new(2, board);
        
            player2.take_dice(dice);
            loop {
                player2.my_turn();
                player2.play_random(ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player2.give_dice(&mut player0);
            player0.my_turn();
            player0.play_random(ACTIONS.to_vec());
            if player0.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player0.give_dice(&mut player2);
        }
        assert!(player0.is_finished() || player2.is_finished());
    }


    #[test]
    #[ignore = "very long test"]
    fn player_0_vs_2_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0, board.clone());
        let mut player2 = Player::new(2, board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            player2.take_dice(dice.clone());
            loop {
                player2.my_turn();
                player2.play_random(ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 0 wins");
                winrates[0] += 1.0;
                break;
            }
            player2.give_dice(&mut player0);
            player0.my_turn();
            player0.play_random(ACTIONS.to_vec());
            if player0.is_finished() {
                println!("Player 2 wins");
                winrates[1] += 1.0;
                break;
            }
            player0.give_dice(&mut player2);
        }
        assert!(player0.is_finished() || player2.is_finished());
        board.borrow_mut().reset();
    }
    println!("Player 0 winrate: {}", winrates[0] / (1.0 * max_iter as f32));
    println!("Player 2 winrate: {}", winrates[1] / (1.0 * max_iter as f32));
    }

    #[test]
    #[ignore = "long test"]
    fn player_0_vs_3_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0, board.clone());
        let mut player3 = Player::new(3, board);
        
            player3.take_dice(dice);
            loop {
                player3.my_turn();
                player3.play_random(ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player3.give_dice(&mut player0);
            player0.my_turn();
            player0.play_random(ACTIONS.to_vec());
            if player0.is_finished() {
                println!("Player 3 wins");
                break;
            }
            player0.give_dice(&mut player3);
        }
        assert!(player0.is_finished() || player3.is_finished());
    }


    #[test]
    #[ignore = "very long test"]
    fn player_0_vs_3_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0, board.clone());
        let mut player3 = Player::new(3, board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            player3.take_dice(dice.clone());
            loop {
                player3.my_turn();
                player3.play_random(ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 0 wins");
                winrates[0] += 1.0;
                break;
            }
            player3.give_dice(&mut player0);
            player0.my_turn();
            player0.play_random(ACTIONS.to_vec());
            if player0.is_finished() {
                println!("Player 3 wins");
                winrates[1] += 1.0;
                break;
            }
            player0.give_dice(&mut player3);
        }
        assert!(player0.is_finished() || player3.is_finished());
        board.borrow_mut().reset();
    }
    println!("Player 0 winrate: {}", winrates[0] / (1.0 * max_iter as f32));
    println!("Player 3 winrate: {}", winrates[1] / (1.0 * max_iter as f32));
    }
    
    #[test]
    #[ignore = "long test"]
    fn player_1_vs_2_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player1 = Player::new(1, board.clone());
        let mut player2 = Player::new(2, board);
        
            player2.take_dice(dice);
            loop {
                player2.my_turn();
                player2.play_random(ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player2.give_dice(&mut player1);
            player1.my_turn();
            player1.play_random(ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player1.give_dice(&mut player2);
        }
        assert!(player1.is_finished() || player2.is_finished());
    }


    #[test]
    #[ignore = "very long test"]
    fn player_1_vs_2_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player1 = Player::new(1, board.clone());
        let mut player2 = Player::new(2, board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            player2.take_dice(dice.clone());
            loop {
                player2.my_turn();
                player2.play_random(ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 1 wins");
                winrates[0] += 1.0;
                break;
            }
            player2.give_dice(&mut player1);
            player1.my_turn();
            player1.play_random(ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 2 wins");
                winrates[1] += 1.0;
                break;
            }
            player1.give_dice(&mut player2);
        }
        assert!(player1.is_finished() || player2.is_finished());
        board.borrow_mut().reset();
    }
    println!("Player 1 winrate: {}", winrates[0] / (1.0 * max_iter as f32));
    println!("Player 2 winrate: {}", winrates[1] / (1.0 * max_iter as f32));
    }


    #[test]
    #[ignore = "long test"]
    fn player_1_vs_3_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player1 = Player::new(1, board.clone());
        let mut player3 = Player::new(3, board);
        
            player3.take_dice(dice);
            loop {
                player3.my_turn();
                player3.play_random(ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player3.give_dice(&mut player1);
            player1.my_turn();
            player1.play_random(ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 3 wins");
                break;
            }
            player1.give_dice(&mut player3);
        }
        assert!(player1.is_finished() || player3.is_finished());
    }


    #[test]
    #[ignore = "very long test"]
    fn player_1_vs_3_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player1 = Player::new(1, board.clone());
        let mut player3 = Player::new(3, board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            player3.take_dice(dice.clone());
            loop {
                player3.my_turn();
                player3.play_random(ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 1 wins");
                winrates[0] += 1.0;
                break;
            }
            player3.give_dice(&mut player1);
            player1.my_turn();
            player1.play_random(ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 2 wins");
                winrates[1] += 1.0;
                break;
            }
            player1.give_dice(&mut player3);
        }
        assert!(player1.is_finished() || player3.is_finished());
        board.borrow_mut().reset();
    }
    println!("Player 1 winrate: {}", winrates[0] / (1.0 * max_iter as f32));
    println!("Player 3 winrate: {}", winrates[1] / (1.0 * max_iter as f32));
    }

    #[test]
    #[ignore = "long test"]
    fn player_2_vs_3_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player2 = Player::new(2, board.clone());
        let mut player3 = Player::new(3, board);
        
            player3.take_dice(dice);
            loop {
                player3.my_turn();
                player3.play_random(ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player3.give_dice(&mut player2);
            player2.my_turn();
            player2.play_random(ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 3 wins");
                break;
            }
            player2.give_dice(&mut player3);
        }
        assert!(player2.is_finished() || player3.is_finished());
    }


    #[test]
    #[ignore = "very long test"]
    fn player_2_vs_3_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player2 = Player::new(2, board.clone());
        let mut player3 = Player::new(3, board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            player3.take_dice(dice.clone());
            loop {
                player3.my_turn();
                player3.play_random(ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 1 wins");
                winrates[0] += 1.0;
                break;
            }
            player3.give_dice(&mut player2);
            player2.my_turn();
            player2.play_random(ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 2 wins");
                winrates[1] += 1.0;
                break;
            }
            player2.give_dice(&mut player3);
        }
        assert!(player2.is_finished() || player3.is_finished());
        board.borrow_mut().reset();
    }
    println!("Player 2 winrate: {}", winrates[0] / (1.0 * max_iter as f32));
    println!("Player 3 winrate: {}", winrates[1] / (1.0 * max_iter as f32));
    }


    
}

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

//     #[test]
//     #[ignore = "long test"]
//     fn second_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(1, board);

//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];

//         while !player.is_finished() {
//             player.my_turn();
//             player.random_play(actions.clone());
//             player.print_status();
//         }
//         assert!(player.is_finished());
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn third_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(2, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];

//         while !player.is_finished() {
//             player.my_turn();
//             player.random_play(actions.clone());
//             player.print_status();
//         }
//         assert!(player.is_finished());
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn fourth_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(3, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];
//         while !player.is_finished() {
//             player.my_turn();
//             player.random_play(actions.clone());
//             player.print_status();
//         }
//         assert!(player.is_finished());
//     }
//     #[test]
//     fn all_single_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];

//         for i in 0..4 {
//             let mut player = Player::new(i, board.clone());

//             while !player.is_finished() {
//                 player.my_turn();
//                 player.random_play(actions.clone());
//                 player.print_status();
//             }
//             assert!(player.is_finished());
//         }
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn replay_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];

//         for i in 0..4 {
//             let mut player = Player::new(i, board.clone());
//             for _ in 0..10 {
//                 while !player.is_finished() {
//                     player.my_turn();
//                     player.random_play(actions.clone());
//                 }
//                 assert!(player.is_finished());
//                 player.reset();
//             }
//         }
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn two_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(0, board.clone());
//         let mut player1 = Player::new(1, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];
//         loop {
//             player0.my_turn();
//             player0.random_play(actions.clone());
//             if player0.is_finished() {
//                 println!("Player 0 wins");
//                 break;
//             }
//             player1.my_turn();
//             player1.random_play(actions.clone());
//             if player1.is_finished() {
//                 println!("Player 1 wins");
//                 break;
//             }
//         }
//         assert!(player0.is_finished() || player1.is_finished());
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn two_players_repeated_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(0, board.clone());
//         let mut player1 = Player::new(1, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];
//         for _ in 0..20 {
//             loop {
//                 player0.my_turn();
//                 player0.random_play(actions.clone());
//                 if player0.is_finished() {
//                     println!("Player 0 wins");
//                     break;
//                 }
//                 player1.my_turn();
//                 player1.random_play(actions.clone());
//                 if player1.is_finished() {
//                     println!("Player 1 wins");
//                     break;
//                 }
//             }
//             assert!(player0.is_finished() || player1.is_finished());
//             player0.reset();
//             player1.reset();
//         }
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn other_two_players_repeated_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(2, board.clone());
//         let mut player1 = Player::new(3, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];
//         for _ in 0..20 {
//             loop {
//                 player0.my_turn();
//                 player0.random_play(actions.clone());
//                 player0.print_status();
//                 if player0.is_finished() {
//                     println!("Player 0 wins");
//                     break;
//                 }
//                 player1.my_turn();
//                 player1.random_play(actions.clone());
//                 player1.print_status();
//                 if player1.is_finished() {
//                     println!("Player 1 wins");
//                     break;
//                 }
//             }
//             assert!(player0.is_finished() || player1.is_finished());
//             player0.reset();
//             player1.reset();
//         }
//     }

//     #[test]
//     #[ignore = "super long test"]
//     fn all_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(0, board.clone());
//         let mut player1 = Player::new(1, board.clone());
//         let mut player2 = Player::new(2, board.clone());
//         let mut player3 = Player::new(3, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];
//         let mut winrate: Vec<f32> = vec![0.0; 4];
//         for _ in 0..30 {
//             loop {
//                 player0.my_turn();
//                 player0.random_play(actions.clone());
//                 // player0.print_status();
//                 if player0.is_finished() {
//                     println!("Player 0 wins");
//                     winrate[0] += 1.0;
//                     break;
//                 }
//                 player1.my_turn();
//                 player1.random_play(actions.clone());
//                 // player1.print_status();
//                 if player1.is_finished() {
//                     println!("Player 1 wins");
//                     winrate[1] += 1.0;
//                     break;
//                 }

//                 player2.my_turn();
//                 player2.random_play(actions.clone());
//                 // player2.print_status();
//                 if player2.is_finished() {
//                     println!("Player 2 wins");
//                     winrate[2] += 1.0;
//                     break;
//                 }

//                 player3.my_turn();
//                 player3.random_play(actions.clone());
//                 // player3.print_status();
//                 if player3.is_finished() {
//                     println!("Player 3 wins");
//                     winrate[3] += 1.0;
//                     break;
//                 }
//             }
//             assert!(
//                 player0.is_finished()
//                     || player1.is_finished()
//                     || player2.is_finished()
//                     || player3.is_finished()
//             );
//             player0.reset();
//             player1.reset();
//             player2.reset();
//             player3.reset();
//         }
//         println!("Player 0 winrate: {}", winrate[0] / 30.0);
//         println!("Player 1 winrate: {}", winrate[1] / 30.0);
//         println!("Player 2 winrate: {}", winrate[2] / 30.0);
//         println!("Player 3 winrate: {}", winrate[3] / 30.0);
//     }
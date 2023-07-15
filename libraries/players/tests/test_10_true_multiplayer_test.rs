use board::Board;
use dice::Dice;
use players::{Act, Player, Select};
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
            play_random(&mut player1, ACTIONS.to_vec());
            if player1.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player1.give_dice(&mut player_0);
            player_0.my_turn();
            play_random(&mut player_0, ACTIONS.to_vec());
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
                play_random(&mut player1, ACTIONS.to_vec());
                if player1.is_finished() {
                    println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player1.give_dice(&mut player_0);
                player_0.my_turn();
                play_random(&mut player_0, ACTIONS.to_vec());
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
        println!(
            "Player 0 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 1 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
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
            play_random(&mut player2, ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player2.give_dice(&mut player0);
            player0.my_turn();
            play_random(&mut player0, ACTIONS.to_vec());
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
                play_random(&mut player2, ACTIONS.to_vec());
                if player2.is_finished() {
                    println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player2.give_dice(&mut player0);
                player0.my_turn();
                play_random(&mut player0, ACTIONS.to_vec());
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
        println!(
            "Player 0 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 2 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
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
            play_random(&mut player3, ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player3.give_dice(&mut player0);
            player0.my_turn();
            play_random(&mut player0, ACTIONS.to_vec());
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
                play_random(&mut player3, ACTIONS.to_vec());
                if player3.is_finished() {
                    println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player3.give_dice(&mut player0);
                player0.my_turn();
                play_random(&mut player0, ACTIONS.to_vec());
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
        println!(
            "Player 0 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 3 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
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
            play_random(&mut player2, ACTIONS.to_vec());
            if player2.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player2.give_dice(&mut player1);
            player1.my_turn();
            play_random(&mut player1, ACTIONS.to_vec());
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
                play_random(&mut player2, ACTIONS.to_vec());
                if player2.is_finished() {
                    println!("Player 1 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player2.give_dice(&mut player1);
                player1.my_turn();
                play_random(&mut player1, ACTIONS.to_vec());
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
        println!(
            "Player 1 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 2 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
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
            play_random(&mut player3, ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player3.give_dice(&mut player1);
            player1.my_turn();
            play_random(&mut player1, ACTIONS.to_vec());
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
                play_random(&mut player3, ACTIONS.to_vec());
                if player3.is_finished() {
                    println!("Player 1 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player3.give_dice(&mut player1);
                player1.my_turn();
                play_random(&mut player1, ACTIONS.to_vec());
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
        println!(
            "Player 1 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 3 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
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
            play_random(&mut player3, ACTIONS.to_vec());
            if player3.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player3.give_dice(&mut player2);
            player2.my_turn();
            play_random(&mut player2, ACTIONS.to_vec());
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
                play_random(&mut player3, ACTIONS.to_vec());
                if player3.is_finished() {
                    println!("Player 2 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player3.give_dice(&mut player2);
                player2.my_turn();
                play_random(&mut player2, ACTIONS.to_vec());
                if player2.is_finished() {
                    println!("Player 3 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player2.give_dice(&mut player3);
            }
            assert!(player2.is_finished() || player3.is_finished());
            board.borrow_mut().reset();
        }
        println!(
            "Player 2 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 3 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
    }

    #[test]
    #[ignore = "super long test"]
    fn all_players_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0, board.clone());
        let mut player1 = Player::new(1, board.clone());
        let mut player2 = Player::new(2, board.clone());
        let mut player3 = Player::new(3, board.clone());

        let mut winrates = [0.0; 4];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            player0.take_dice(dice.clone());
            loop {
                player0.my_turn();
                play_random(&mut player0, ACTIONS.to_vec());
                if player0.is_finished() {
                    println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player0.give_dice(&mut player1);
                player1.my_turn();
                play_random(&mut player1, ACTIONS.to_vec());
                if player1.is_finished() {
                    println!("Player 1 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player1.give_dice(&mut player2);
                player2.my_turn();
                play_random(&mut player2, ACTIONS.to_vec());
                if player2.is_finished() {
                    println!("Player 2 wins");
                    winrates[2] += 1.0;
                    break;
                }
                player2.give_dice(&mut player3);
                player3.my_turn();
                play_random(&mut player3, ACTIONS.to_vec());
                if player3.is_finished() {
                    println!("Player 3 wins");
                    winrates[3] += 1.0;
                    break;
                }
                player3.give_dice(&mut player0);
            }
            assert!(
                player0.is_finished()
                    || player1.is_finished()
                    || player2.is_finished()
                    || player3.is_finished()
            );
            board.borrow_mut().reset();
        }
        println!(
            "Player 0 winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 1 winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 2 winrate: {}",
            winrates[2] / (1.0 * max_iter as f32)
        );
        println!(
            "Player 3 winrate: {}",
            winrates[3] / (1.0 * max_iter as f32)
        );
    }

    fn play_random(player: &mut Player, actions: Vec<Act>) {
        player.roll_dice();
        let dice_number = player.get_dice_number();
        let movesets = player.generate_vector_of_random_actions(actions, dice_number);
        player.action = player.select_random_piece(movesets);
        player.make_move(player.action.1, dice_number, player.action.0);
    }
}

#[cfg(test)]
mod playstyle_tests {
    use super::*;

    fn play_random(player: &mut Player, actions: Vec<Act>) {
        player.roll_dice();
        let dice_number = player.get_dice_number();
        let movesets = player.generate_vector_of_random_actions(actions, dice_number);
        player.action = player.select_random_piece(movesets);
        player.make_move(player.action.1, dice_number, player.action.0);
    }

    fn play_ordered(player: &mut Player, actions: Vec<Act>, select_which_piece: Select) {
        player.roll_dice();
        let dice_number = player.get_dice_number();
        let movesets =
            player.generate_vector_of_ordered_actions(actions, dice_number, select_which_piece);
        player.action = movesets
            .first()
            .copied()
            .unwrap_or((Act::Nothing, player.id(), 57));
        player.make_move(player.action.1, dice_number, player.action.0);
    }

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

    static AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Goal,
        Act::Starjump,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    #[test]
    fn aggressive_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut aggressive_player = Player::new(0, board.clone());
        let mut random_player = Player::new(2, board.clone());
        let take_nearest_piece = Select::Nearest;
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            random_player.take_dice(dice.clone());
            loop {
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS.to_vec());
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrates[0] += 1.0;
                    break;
                }
                random_player.give_dice(&mut aggressive_player);
                aggressive_player.my_turn();
                play_ordered(
                    &mut aggressive_player,
                    AGGRO_ACTIONS.to_vec(),
                    take_nearest_piece,
                );
                if aggressive_player.is_finished() {
                    println!("aggressive_player wins");
                    winrates[1] += 1.0;
                    break;
                }
                aggressive_player.give_dice(&mut random_player);
            }
            assert!(aggressive_player.is_finished() || random_player.is_finished());
            board.borrow_mut().reset();
        }
        println!(
            "Random player winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Aggressive player winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
    }

    static FAST_AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Goal,
        Act::Starjump,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    #[test]
    fn fast_aggressive_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut fast_aggressive_player = Player::new(1, board.clone());
        let mut random_player = Player::new(3, board.clone());
        let take_closest = Select::Nearest;
        let mut winrate = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            random_player.take_dice(dice.clone());
            loop {
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS.to_vec());
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrate[0] += 1.0;
                    break;
                }
                random_player.give_dice(&mut fast_aggressive_player);
                fast_aggressive_player.my_turn();
                play_ordered(
                    &mut fast_aggressive_player,
                    FAST_AGGRO_ACTIONS.to_vec(),
                    take_closest,
                );
                if fast_aggressive_player.is_finished() {
                    println!("fast aggressive_player wins");
                    winrate[1] += 1.0;
                    break;
                }
                fast_aggressive_player.give_dice(&mut random_player);
            }
            assert!(fast_aggressive_player.is_finished() || random_player.is_finished());
            board.borrow_mut().reset();
        }
        println!(
            "Random player winrate: {}",
            winrate[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Fast aggressive player winrate: {}",
            winrate[1] / (1.0 * max_iter as f32)
        );
    }

    static SAFE_ACTIONS: [Act; 10] = [
        Act::Join,
        Act::Safe,
        Act::Goal,
        Act::Move,
        Act::Kill,
        Act::Starjump,
        Act::Free,
        Act::Leave,
        Act::Nothing,
        Act::Die,
    ];

    #[test]
    fn safest_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut safe_player = Player::new(1, board.clone());
        let mut random_player = Player::new(3, board.clone());
        let take_closest = Select::Nearest;
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            random_player.take_dice(dice.clone());
            loop {
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS.to_vec());
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrates[0] += 1.0;
                    break;
                }
                random_player.give_dice(&mut safe_player);
                safe_player.my_turn();
                play_ordered(&mut safe_player, SAFE_ACTIONS.to_vec(), take_closest);
                if safe_player.is_finished() {
                    println!("safe player wins");
                    winrates[1] += 1.0;
                    break;
                }
                safe_player.give_dice(&mut random_player);
            }
            assert!(safe_player.is_finished() || random_player.is_finished());
            board.borrow_mut().reset();
        }
        println!(
            "Random player winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Safe player winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
    }

    static FAST_ACTIONS: [Act; 10] = [
        Act::Goal,
        Act::Starjump,
        Act::Move,
        Act::Leave,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    #[test]
    fn fastest_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut fastest_player = Player::new(1, board.clone());
        let mut random_player = Player::new(2, board.clone());
        let take_closest = Select::Nearest;
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            random_player.take_dice(dice.clone());
            loop {
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS.to_vec());
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrates[0] += 1.0;
                    break;
                }
                random_player.give_dice(&mut fastest_player);
                fastest_player.my_turn();
                play_ordered(&mut fastest_player, FAST_ACTIONS.to_vec(), take_closest);
                if fastest_player.is_finished() {
                    println!("fastest player wins");
                    winrates[1] += 1.0;
                    break;
                }
                fastest_player.give_dice(&mut random_player);
            }
            assert!(fastest_player.is_finished() || random_player.is_finished());
            board.borrow_mut().reset();
        }
        println!(
            "Random player winrate: {}",
            winrates[0] / (1.0 * max_iter as f32)
        );
        println!(
            "Fastest player winrate: {}",
            winrates[1] / (1.0 * max_iter as f32)
        );
    }

    #[test]
    #[ignore]
    fn final_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();
        let mut fastest_player = Player::new(0, board.clone());
        let mut random_player = Player::new(1, board.clone());
        let mut fast_aggressive_player = Player::new(2, board.clone());
        let mut aggressive_player = Player::new(3, board.clone());
        let take_closest = Select::Nearest;
        let max_iter: usize = 1000;
        let mut winrate: Vec<f32> = vec![0.0; 4];

        for _ in 0..max_iter {
            fastest_player.take_dice(dice.clone());
            loop {
                fastest_player.my_turn();
                play_ordered(&mut fastest_player, FAST_ACTIONS.to_vec(), take_closest);
                if fastest_player.is_finished() {
                    println!("fastest_player wins");
                    winrate[0] += 1.0;
                    break;
                }
                fastest_player.give_dice(&mut random_player);
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS.to_vec());
                if random_player.is_finished() {
                    println!("Random player wins");
                    winrate[1] += 1.0;
                    break;
                }
                random_player.give_dice(&mut fast_aggressive_player);
                fast_aggressive_player.my_turn();
                play_ordered(
                    &mut fast_aggressive_player,
                    FAST_AGGRO_ACTIONS.to_vec(),
                    take_closest,
                );
                if fast_aggressive_player.is_finished() {
                    println!("fast aggressive_player wins");
                    winrate[2] += 1.0;
                    break;
                }
                fast_aggressive_player.give_dice(&mut aggressive_player);
                aggressive_player.my_turn();
                play_ordered(&mut aggressive_player, AGGRO_ACTIONS.to_vec(), take_closest);
                if aggressive_player.is_finished() {
                    println!("aggressive_player wins");
                    winrate[3] += 1.0;
                    break;
                }
                aggressive_player.give_dice(&mut fastest_player);
            }
            assert!(
                fastest_player.is_finished()
                    || random_player.is_finished()
                    || fast_aggressive_player.is_finished()
                    || aggressive_player.is_finished()
            );
            board.borrow_mut().reset();
        }
        println!("fastest_player winrate: {}", winrate[0] / max_iter as f32);
        println!("Random player winrate: {}", winrate[1] / max_iter as f32);
        println!(
            "fast aggressive_player winrate: {}",
            winrate[2] / max_iter as f32
        );
        println!(
            "aggressive_player winrate: {}",
            winrate[3] / max_iter as f32
        );
    }
}

use board::Board;
use dice::Dice;
use players::{Act, Player, Select};
use std::time::Instant;
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

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player1 = Player::new(1);
        player1.setup(board);

        loop {
            player1.get_dice(dice.clone());
            player1.my_turn();
            play_random(&mut player1, ACTIONS);
            if player1.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player1.drop_dice();
            player1.get_dice(dice.clone());
            player0.my_turn();
            play_random(&mut player0, ACTIONS);
            if player0.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player0.drop_dice();
        }
        assert!(player0.is_finished() || player1.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_0_vs_1_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player1 = Player::new(1);
        player1.setup(board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            loop {
                player1.get_dice(dice.clone());
                player1.my_turn();
                play_random(&mut player1, ACTIONS);
                if player1.is_finished() {
                    // println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player1.drop_dice();
                player0.get_dice(dice.clone());
                player0.my_turn();
                play_random(&mut player0, ACTIONS);
                if player0.is_finished() {
                    // println!("Player 1 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player0.drop_dice();
            }
            assert!(player0.is_finished() || player1.is_finished());
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

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player2 = Player::new(2);
        player2.setup(board);

        loop {
            player2.get_dice(dice.clone());
            player2.my_turn();
            play_random(&mut player2, ACTIONS);
            if player2.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player2.drop_dice();
            player0.get_dice(dice.clone());
            player0.my_turn();
            play_random(&mut player0, ACTIONS);
            if player0.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player0.drop_dice();
        }
        assert!(player0.is_finished() || player2.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_0_vs_2_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player2 = Player::new(2);
        player2.setup(board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            loop {
                player2.get_dice(dice.clone());
                player2.my_turn();
                play_random(&mut player2, ACTIONS);
                if player2.is_finished() {
                    println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player2.drop_dice();
                player2.get_dice(dice.clone());
                player0.my_turn();
                play_random(&mut player0, ACTIONS);
                if player0.is_finished() {
                    println!("Player 2 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player0.drop_dice();
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

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board);

        loop {
            player3.get_dice(dice.clone());
            player3.my_turn();
            play_random(&mut player3, ACTIONS);
            if player3.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player3.drop_dice();
            player0.get_dice(dice.clone());
            player0.my_turn();
            play_random(&mut player0, ACTIONS);
            if player0.is_finished() {
                println!("Player 3 wins");
                break;
            }
            player0.drop_dice();
        }
        assert!(player0.is_finished() || player3.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_0_vs_3_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            loop {
                player3.get_dice(dice.clone());
                player3.my_turn();
                play_random(&mut player3, ACTIONS);
                if player3.is_finished() {
                    println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player3.drop_dice();
                player0.get_dice(dice.clone());
                player0.my_turn();
                play_random(&mut player0, ACTIONS);
                if player0.is_finished() {
                    println!("Player 3 wins");
                    winrates[1] += 1.0;
                    break;
                }
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

        let mut player1 = Player::new(1);
        player1.setup(board.clone());
        let mut player2 = Player::new(2);
        player2.setup(board);

        loop {
            player2.get_dice(dice.clone());
            player2.my_turn();
            play_random(&mut player2, ACTIONS);
            if player2.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player2.drop_dice();
            player1.get_dice(dice.clone());
            player1.my_turn();
            play_random(&mut player1, ACTIONS);
            if player1.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player1.drop_dice();
        }
        assert!(player1.is_finished() || player2.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_1_vs_2_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player1 = Player::new(1);
        player1.setup(board.clone());
        let mut player2 = Player::new(2);
        player2.setup(board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            loop {
                player2.get_dice(dice.clone());
                player2.my_turn();
                play_random(&mut player2, ACTIONS);
                if player2.is_finished() {
                    println!("Player 1 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player2.drop_dice();
                player1.get_dice(dice.clone());
                player1.my_turn();
                play_random(&mut player1, ACTIONS);
                if player1.is_finished() {
                    println!("Player 2 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player1.drop_dice();
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

        let mut player1 = Player::new(1);
        player1.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board);

        loop {
            player3.get_dice(dice.clone());
            player3.my_turn();
            play_random(&mut player3, ACTIONS);
            if player3.is_finished() {
                println!("Player 1 wins");
                break;
            }
            player3.drop_dice();
            player1.get_dice(dice.clone());
            player1.my_turn();
            play_random(&mut player1, ACTIONS);
            if player1.is_finished() {
                println!("Player 3 wins");
                break;
            }
            player1.drop_dice();
        }
        assert!(player1.is_finished() || player3.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_1_vs_3_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player1 = Player::new(1);
        player1.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            loop {
                player3.get_dice(dice.clone());
                player3.my_turn();
                play_random(&mut player3, ACTIONS);
                if player3.is_finished() {
                    println!("Player 1 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player3.drop_dice();
                player1.get_dice(dice.clone());
                player1.my_turn();
                play_random(&mut player1, ACTIONS);
                if player1.is_finished() {
                    println!("Player 2 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player1.drop_dice();
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

        let mut player2 = Player::new(2);
        player2.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board);

        loop {
            player3.get_dice(dice.clone());
            player3.my_turn();
            play_random(&mut player3, ACTIONS);
            if player3.is_finished() {
                println!("Player 2 wins");
                break;
            }
            player3.drop_dice();
            player2.get_dice(dice.clone());
            player2.my_turn();
            play_random(&mut player2, ACTIONS);
            if player2.is_finished() {
                println!("Player 3 wins");
                break;
            }
            player2.drop_dice();
        }
        assert!(player2.is_finished() || player3.is_finished());
    }

    #[test]
    #[ignore = "very long test"]
    fn player_2_vs_3_1000_iteration_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();

        let mut player2 = Player::new(2);
        player2.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board.clone());
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;
        for _ in 0..max_iter {
            loop {
                player3.get_dice(dice.clone());
                player3.my_turn();
                play_random(&mut player3, ACTIONS);
                if player3.is_finished() {
                    println!("Player 2 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player3.drop_dice();
                player2.get_dice(dice.clone());
                player2.my_turn();
                play_random(&mut player2, ACTIONS);
                if player2.is_finished() {
                    println!("Player 3 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player2.drop_dice();
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

        let mut player0 = Player::new(0);
        player0.setup(board.clone());
        let mut player1 = Player::new(1);
        player1.setup(board.clone());
        let mut player2 = Player::new(2);
        player2.setup(board.clone());
        let mut player3 = Player::new(3);
        player3.setup(board.clone());

        let mut winrates = [0.0; 4];
        let max_iter: usize = 1000;
        let mut timer: f64 = 0.0;
        for _ in 0..max_iter {
            loop {
                player0.get_dice(dice.clone());
                player0.my_turn();
                play_random(&mut player0, ACTIONS);
                if player0.is_finished() {
                    // println!("Player 0 wins");
                    winrates[0] += 1.0;
                    break;
                }
                player0.drop_dice();
                player1.get_dice(dice.clone());
                player1.my_turn();
                play_random(&mut player1, ACTIONS);
                if player1.is_finished() {
                    // println!("Player 1 wins");
                    winrates[1] += 1.0;
                    break;
                }
                player1.drop_dice();
                player2.get_dice(dice.clone());
                player2.my_turn();
                play_random(&mut player2, ACTIONS);
                if player2.is_finished() {
                    // println!("Player 2 wins");
                    winrates[2] += 1.0;
                    break;
                }
                player2.drop_dice();
                player3.get_dice(dice.clone());
                player3.my_turn();
                play_random(&mut player3, ACTIONS);
                if player3.is_finished() {
                    // println!("Player 3 wins");
                    winrates[3] += 1.0;
                    break;
                }
                player3.drop_dice();
            }
            assert!(
                player0.is_finished()
                    || player1.is_finished()
                    || player2.is_finished()
                    || player3.is_finished()
            );
            let start = Instant::now();
            board.borrow_mut().reset();
            let duration = start.elapsed();
            timer += duration.as_secs_f64();
            // println!("Time elapsed in expensive_function() is: {:?}", duration);
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
        println!(
            "Time elapsed in expensive_function() is: {:?}",
            timer / max_iter as f64
        );
    }

    fn play_random(player: &mut Player, actions: [Act; 10]) {
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

    fn play_random(player: &mut Player, actions: [Act; 10]) {
        player.roll_dice();
        let dice_number = player.get_dice_number();
        let movesets = player.generate_vector_of_random_actions(actions, dice_number);
        player.action = player.select_random_piece(movesets);
        player.make_move(player.action.1, dice_number, player.action.0);
    }

    fn play_ordered(player: &mut Player, actions: [Act; 10], select_which_piece: Select) {
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

        let mut aggressive_player = Player::new(0);
        aggressive_player.setup(board.clone());
        let mut random_player = Player::new(2);
        random_player.setup(board.clone());
        let take_nearest_piece = Select::Nearest;
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            loop {
                random_player.get_dice(dice.clone());
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS);
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrates[0] += 1.0;
                    break;
                }
                random_player.drop_dice();
                aggressive_player.get_dice(dice.clone());
                aggressive_player.my_turn();
                play_ordered(
                    &mut aggressive_player,
                    AGGRO_ACTIONS,
                    take_nearest_piece,
                );
                if aggressive_player.is_finished() {
                    println!("aggressive_player wins");
                    winrates[1] += 1.0;
                    break;
                }
                aggressive_player.drop_dice();
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

        let mut fast_aggressive_player = Player::new(1);
        fast_aggressive_player.setup(board.clone());
        let mut random_player = Player::new(3);
        random_player.setup(board.clone());
        let take_closest = Select::Nearest;
        let mut winrate = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            loop {
                random_player.get_dice(dice.clone());
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS);
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrate[0] += 1.0;
                    break;
                }
                random_player.drop_dice();
                fast_aggressive_player.get_dice(dice.clone());
                fast_aggressive_player.my_turn();
                play_ordered(
                    &mut fast_aggressive_player,
                    FAST_AGGRO_ACTIONS,
                    take_closest,
                );
                if fast_aggressive_player.is_finished() {
                    println!("fast aggressive_player wins");
                    winrate[1] += 1.0;
                    break;
                }
                fast_aggressive_player.drop_dice();
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

        let mut safe_player = Player::new(1);
        safe_player.setup(board.clone());
        let mut random_player = Player::new(3);
        random_player.setup(board.clone());
        let take_closest = Select::Nearest;
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            loop {
                random_player.get_dice(dice.clone());
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS);
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrates[0] += 1.0;
                    break;
                }
                random_player.drop_dice();
                safe_player.get_dice(dice.clone());
                safe_player.my_turn();
                play_ordered(&mut safe_player, SAFE_ACTIONS, take_closest);
                if safe_player.is_finished() {
                    println!("safe player wins");
                    winrates[1] += 1.0;
                    break;
                }
                safe_player.drop_dice();
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

        let mut fastest_player = Player::new(1);
        fastest_player.setup(board.clone());
        let mut random_player = Player::new(2);
        random_player.setup(board.clone());

        let take_closest = Select::Nearest;
        let mut winrates = [0.0; 2];
        let max_iter: usize = 1000;

        for _ in 0..max_iter {
            loop {
                random_player.get_dice(dice.clone());
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS);
                if random_player.is_finished() {
                    println!("random_player wins");
                    winrates[0] += 1.0;
                    break;
                }
                random_player.drop_dice();
                fastest_player.get_dice(dice.clone());
                fastest_player.my_turn();
                play_ordered(&mut fastest_player, FAST_ACTIONS, take_closest);
                if fastest_player.is_finished() {
                    println!("fastest player wins");
                    winrates[1] += 1.0;
                    break;
                }
                fastest_player.drop_dice();
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
        let mut fastest_player = Player::new(0);
        fastest_player.setup(board.clone());
        let mut random_player = Player::new(1);
        random_player.setup(board.clone());
        let mut fast_aggressive_player = Player::new(2);
        fast_aggressive_player.setup(board.clone());
        let mut aggressive_player = Player::new(3);
        aggressive_player.setup(board.clone());
        let take_closest = Select::Nearest;
        let max_iter: usize = 1000;
        let mut winrate: Vec<f32> = vec![0.0; 4];

        for _ in 0..max_iter {
            loop {
                fastest_player.get_dice(dice.clone());
                fastest_player.my_turn();
                play_ordered(&mut fastest_player, FAST_ACTIONS, take_closest);
                if fastest_player.is_finished() {
                    println!("fastest_player wins");
                    winrate[0] += 1.0;
                    break;
                }
                fastest_player.drop_dice();
                random_player.get_dice(dice.clone());
                random_player.my_turn();
                play_random(&mut random_player, ACTIONS);
                if random_player.is_finished() {
                    println!("Random player wins");
                    winrate[1] += 1.0;
                    break;
                }
                random_player.drop_dice();
                fast_aggressive_player.get_dice(dice.clone());
                fast_aggressive_player.my_turn();
                play_ordered(
                    &mut fast_aggressive_player,
                    FAST_AGGRO_ACTIONS,
                    take_closest,
                );
                if fast_aggressive_player.is_finished() {
                    println!("fast aggressive_player wins");
                    winrate[2] += 1.0;
                    break;
                }
                fast_aggressive_player.drop_dice();
                aggressive_player.get_dice(dice.clone());
                aggressive_player.my_turn();
                play_ordered(&mut aggressive_player, AGGRO_ACTIONS, take_closest);
                if aggressive_player.is_finished() {
                    println!("aggressive_player wins");
                    winrate[3] += 1.0;
                    break;
                }
                aggressive_player.drop_dice();
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

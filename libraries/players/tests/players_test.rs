use players::Player;

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn add_player_test() {
        let player = Player::new(0);
        assert_eq!(player.id(), 0);
    }

    #[test]
    fn get_pieces_test() {
        let mut player = Player::new(0);

        let piece = player.piece(0);
        assert_eq!(piece.id(), 0);

        let piece = player.piece(1);
        assert_eq!(piece.id(), 1);

        let piece = player.piece(2);
        assert_eq!(piece.id(), 2);

        let piece = player.piece(3);
        assert_eq!(piece.id(), 3);
    }

    #[test]
    fn get_piece_test() {
        let mut player = Player::new(0);
        let pieces = player.pieces();
        assert!(pieces[0].is_home());
        assert!(pieces[1].is_home());
        assert!(pieces[2].is_home());
        assert!(pieces[3].is_home());
        assert!(player.piece(0).is_safe());
    }

    #[test]
    fn free_piece_test() {
        let mut player = Player::new(0);
        player.free_piece(0);
        assert!(!player.piece(0).is_home());
        assert!(player.piece(0).is_safe());
    }

    #[test]
    fn play_random_piece() {
        let mut player = Player::new(0);
        player.my_turn();
        player.play_random();
    }
}

mod move_piece_test {

    use super::*;

    #[test]
    fn move_piece_test() {
        let mut player = Player::new(0);
        player.free_piece(0);
        assert!(!player.piece(0).is_home());
        assert_eq!(player.piece(0).position(), 0);

        player.make_move(0, 6);
        assert_eq!(player.piece(0).position(), 6);

        player.free_piece(1);
        player.make_move(1, 6);
        player.make_move(1, 2);
        assert_eq!(player.piece(1).position(), 8);
    }

    #[test]
    fn update_piece_state_test() {
        let mut player = Player::new(0);
        player.update_piece_state(0);
        assert!(player.piece(0).is_home());

        player.free_piece(0);
        assert!(player.piece(0).is_safe());
        assert!(player.piece(0).is_dangerous());

        player.make_move(0, 6);
        assert!(!player.piece(0).is_safe());

        player.make_move(0, 2);
        assert!(player.piece(0).is_safe());

        player.make_move(0, 5);
        assert!(!player.piece(0).is_safe());

        player.piece(0).set_position(50);
        assert!(!player.piece(0).is_goal());

        player.make_move(0, 1);
        assert_eq!(player.piece(0).position(), 52);
        assert!(player.piece(0).is_safe());

        player.piece(0).set_position(45);
        player.make_move(0, 6);
        assert_eq!(player.piece(0).position(), 52);
        assert!(player.piece(0).is_safe());

        // assert!(player.piece(0).is_safe());
    }

    #[test]
    fn move_piece_inside_test() {
        let mut player = Player::new(0);

        for j in 0..6 {
            let mut add: i8 = 0;
            if j > 0 {
                add = 1;
            }
            for i in (1 + j)..(6 + add) {
                player.piece(0).set_position(50 - j);
                player.update_piece_state(0);
                player.make_move(0, i);
                assert_eq!(player.piece(0).position(), 51 + i - j);
                assert!(!player.piece(0).is_goal())
            }
        }
    }

    #[test]
    fn move_piece_to_goal() {
        let mut player = Player::new(0);

        for i in (1..7).rev() {
            let mut sub = 0;
            if i == 1 {
                sub = 1;
            }
            player.piece(0).set_position(50 + i - sub);
            player.update_piece_state(0);
            player.make_move(0, 7 - i);
            assert_eq!(player.piece(0).position(), 99);
            assert!(player.piece(0).is_goal());
        }
    }

    #[test]
    fn move_back_test() {
        let mut player = Player::new(0);
        for j in 0..5 {
            for i in ((2 + j)..7).rev() {
                player.piece(0).set_position(56 - j);
                player.update_piece_state(0);
                player.make_move(0, i);

                let res = player.piece(0).position();
                let ans = 56 - i + 2 + j;
                assert_eq!(res, ans);
                
                let res = player.piece(0).is_goal();
                assert!(!res);
            }
        }
    }
}

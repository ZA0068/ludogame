use board::{Board, PlayerID};

mod player_id_tests {
    use super::*;

    #[test]
    fn player_id_test() {
        let player_id = PlayerID::Player0;
        assert_eq!(player_id, PlayerID::Player0);
    }

    #[test]
    fn player_id_test_2() {
        let player_id = [
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        let board = Board::default();

        for id in 0..4 {
            assert_eq!(
                board.clone().get_player_id(id),
                Some(player_id[id as usize].clone())
            );
        }

        assert_eq!(board.get_player_id(4), None);
    }
}

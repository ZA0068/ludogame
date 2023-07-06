use pieces::{Color, Piece};

#[cfg(test)]
mod pieces_test {
    use super::*;

    #[test]
    fn creating_all_piece_single_color_test() {
        for i in 0..4 {
            let piece = Piece::new(i, Color::Green);
            assert_eq!(piece.id(), i);
            assert!(piece.is_home());
            assert!(!piece.is_goal());
            assert!(piece.is_safe());
            assert!(!piece.is_dangerous());
            assert_eq!(piece.color(), Color::Green);
            assert_eq!(piece.position(), -1);
        }
    }

    #[test]
    fn creating_all_piece_all_color_test() {
        let colors = vec![Color::Red, Color::Blue, Color::Yellow, Color::Green];
        for color in colors.iter() {
            for i in 0..4 {
                let piece = Piece::new(i, color.clone());
                assert_eq!(piece.id(), i);
                assert!(piece.is_home());
                assert!(!piece.is_goal());
                assert!(piece.is_safe());
                assert!(!piece.is_dangerous());
                assert_eq!(piece.color(), color.clone());
                assert_eq!(piece.position(), -1);
            }
        }
    }

    #[test]
    fn move_piece_test() {
        let mut piece = Piece::new(1, Color::Green);
        piece.set_position(3);
        assert_eq!(piece.id(), 1);
        assert_eq!(piece.position(), 3);
    }

    #[test]
    fn free_piece_test() {
        let mut piece = Piece::new(1, Color::Green);
        assert_eq!(piece.id(), 1);
        piece.free();
        piece.set_position(0);
        assert_eq!(piece.position(), 0);
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(piece.is_dangerous());
    }

    #[test]
    fn dangerous_piece_test() {
        let mut piece = Piece::new(1, Color::Green);
        assert_eq!(piece.id(), 1);
        piece.dangerous();
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(piece.is_dangerous());
    }

    #[test]
    fn unsafe_piece_test() {
        let mut piece = Piece::new(1, Color::Green);
        assert_eq!(piece.id(), 1);
        piece.not_safe();
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(!piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn safe_piece_test() {
        let mut piece = Piece::new(1, Color::Green);
        assert_eq!(piece.id(), 1);
        piece.safe();
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn death_pieces_test() {
        let mut piece = Piece::new(1, Color::Green);
        assert_eq!(piece.id(), 1);
        piece.dead();
        assert_eq!(piece.position(), -1);
        assert!(piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());

        let mut piece = Piece::new(2, Color::Green);
        assert_eq!(piece.id(), 2);
        piece.home();
        assert_eq!(piece.position(), -1);
        assert!(piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn goal_piece_test() {
        let mut piece = Piece::new(1, Color::Green);
        assert_eq!(piece.id(), 1);
        piece.goal();
        assert_eq!(piece.position(), 99);
        assert!(!piece.is_home());
        assert!(piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }
}

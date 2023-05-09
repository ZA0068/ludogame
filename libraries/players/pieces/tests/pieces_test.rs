use pieces::Piece;

#[cfg(test)]
mod pieces_test {
    use super::*;

    #[test]
    fn creating_a_piece_test() {
        let piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        assert!(piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
        assert_eq!(piece.position(), -1);
    }

    #[test]
    fn move_piece_test() {
        let mut piece = Piece::new(1);
        piece.set_position(3);
        assert_eq!(piece.id(), 1);
        assert_eq!(piece.position(), 3);
    }

    #[test]
    fn free_piece_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.free();
        assert_eq!(piece.position(), 0);
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn dangerous_piece_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.dangerous();
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(piece.is_dangerous());
    }

    #[test]
    fn unsafe_piece_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.not_safe();
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(!piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn safe_piece_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.safe();
        assert!(!piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn death_pieces_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.dead();
        assert_eq!(piece.position(), -1);
        assert!(piece.is_home());
        assert!(!piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }

    #[test]
    fn goal_piece_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.goal();
        assert_eq!(piece.position(), 99);
        assert!(!piece.is_home());
        assert!(piece.is_goal());
        assert!(piece.is_safe());
        assert!(!piece.is_dangerous());
    }
}

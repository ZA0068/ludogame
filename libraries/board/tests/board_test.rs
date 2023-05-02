use board::Board;
use std::any::TypeId;

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[test]
    fn create_a_board_test() {
        let board = Board::new();
        assert_eq!(TypeId::of::<Board>(), board.type_id());
    }
    
}

use board::Board;
use std::any::{TypeId, Any};

#[cfg(test)]
mod dice_tests {
    use super::*;

    #[test]
    fn create_a_dice_test() {
        let dice = Dice::new();
        assert_eq!(dice.type_id(), any::TypeID(Dice));
    }
}
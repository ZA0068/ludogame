use dice::Dice;
use std::any::{Any, TypeId};

#[cfg(test)]
mod dice_test {
    use super::*;

    #[test]
    fn create_the_dice() {
        let dice = Dice::new(6);
        assert_eq!(dice.type_id(), TypeId::of::<Dice>());
    }

    #[test]
    fn rolling_the_dice() {
        let mut dice = Dice::new(6);
        dice.roll();
        let value = dice.get_value();
        assert!((1..=6).contains(&value));
    }

    #[test]
    fn default_dice_test() {
        let dice = Dice::default();
        assert_eq!(dice.type_id(), TypeId::of::<Dice>());
    }
}

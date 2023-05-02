use rand;

mod dice
{
    pub struct Dice
    {
        value: u8
    }

    impl Dice
    {
        pub fn new() -> Dice
        {
            Dice { value: 0 }
        }

        pub fn roll(&mut self) -> u8
        {
            self.value = rand::random::<u8>() % 6 + 1;
            self.value
        }

        pub fn get_value(&self) -> u8
        {
            self.value
        }
    }
}

pub use dice::Dice;
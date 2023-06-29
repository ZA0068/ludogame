mod dice {
    use rand::distributions::Uniform;
    use rand::Rng;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Dice {
        value: i8,
        dist: Uniform<i8>,
    }

    impl Dice {
        pub fn new(high_value: i8) -> Dice {
            let dist = Uniform::new_inclusive(1, high_value);
            Dice { value: 0, dist }
        }

        pub fn roll(&mut self) {
            let mut rng = rand::thread_rng();
            self.value = rng.sample(self.dist);
        }

        pub fn get_value(&self) -> i8 {
            self.value
        }
    }

    impl Default for Dice {
        fn default() -> Self {
            Self::new(6)
        }
    }
}

pub use dice::Dice;

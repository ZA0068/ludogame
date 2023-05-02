pub struct Board {
    pub home: [i8; 16],
    pub goal: [i8; 4],
    pub outside: [i8; 52],
    pub inside: [i8; 20],
    pub globe: [i8; 8],
    pub star: [i8; 8],
}

impl Board {
    pub fn new() -> Board {
        let outside: [i8; 52] = (0..52).map(|i| i as i8).collect::<Vec<i8>>().try_into().unwrap();
        let inside: [i8; 20] = (52..72).map(|i| i as i8).collect::<Vec<i8>>().try_into().unwrap();

        Board {
            home: [-1; 16],
            goal: [99; 4],
            outside,
            inside,
            globe: [0, 8, 13, 21, 26, 34, 39, 47],
            star: [5, 12, 18, 25, 31, 38, 44, 51],
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
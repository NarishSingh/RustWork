pub mod GameMoves {
    use std::fmt::{Display, Formatter};
    use rand::Rng;

    #[derive(Debug)]
    pub enum Moves {
        Rock,
        Paper,
        Scissors,
    }

    //Needed to print enum
    impl Display for Moves {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    /// Get comp move
    ///
    /// Return a random variant of the moves enum
    pub fn get_comp_move() -> Moves {
        match rand::thread_rng().gen_range(1..4) {
            1 => Moves::Rock,
            2 => Moves::Paper,
            3 => Moves::Scissors,
            _ => { panic!("RNG threading Error") }
        }
    }
}

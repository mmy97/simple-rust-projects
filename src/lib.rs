pub mod game {

    pub struct Game {
        pub grid: [[Option<Marker>; 3]; 3],
        pub winner: Option<Winner>,
        pub current_turn: Marker,
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                grid: Default::default(),
                winner: None,
                current_turn: Marker::X,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Marker {
        X,
        O,
    }

    #[derive(Debug, PartialEq)]
    pub enum Winner {
        O,
        X,
        Draw,
    }
}

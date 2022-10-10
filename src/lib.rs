pub mod game {
    use crate::game::CannotMark::{LocationNotEmpty, LocationOutOfBounds};
    use crate::game::Marker::X;
    use crate::game::Outcome::{Draw, Winner};
    use Marker::O;

    const GRID_SIZE: usize = 3;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Marker {
        X,
        O,
    }

    #[derive(Debug, PartialEq)]
    pub enum Outcome {
        Winner(Marker),
        Draw,
    }

    pub struct Game {
        pub grid: [[Option<Marker>; GRID_SIZE]; GRID_SIZE],
        pub outcome: Option<Outcome>,
        pub current_turn: Marker,
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                grid: Default::default(),
                outcome: None,
                current_turn: X,
            }
        }

        pub fn mark(&mut self, x: usize, y: usize) -> Result<(), CannotMark> {
            if x >= GRID_SIZE || y >= GRID_SIZE {
                return Err(LocationOutOfBounds);
            }

            if self.grid[x][y].is_some() {
                return Err(LocationNotEmpty);
            }

            self.mark_grid(x, y);

            self.update_outcome();

            self.swap_turns();
            Ok(())
        }

        fn swap_turns(&mut self) {
            self.current_turn = match self.current_turn {
                X => O,
                O => X,
            }
        }

        fn mark_grid(&mut self, x: usize, y: usize) {
            self.grid[x][y] = Some(self.current_turn);
        }

        fn update_outcome(&mut self) {
            self.check_row(0);
            self.check_row(1);
            self.check_row(2);

            self.check_col(0);
            self.check_col(1);
            self.check_col(2);

            let diagonal1 = [self.grid[0][0], self.grid[1][1], self.grid[2][2]];
            self.update_outcome_on_three_matches(diagonal1);

            let diagonal2 = [self.grid[2][0], self.grid[1][1], self.grid[0][2]];
            self.update_outcome_on_three_matches(diagonal2);

            self.check_draw();
        }

        fn check_row(&mut self, i: usize) {
            self.update_outcome_on_three_matches(self.grid[i])
        }

        fn check_col(&mut self, i: usize) {
            self.update_outcome_on_three_matches([
                self.grid[0][i],
                self.grid[1][i],
                self.grid[2][i],
            ]);
        }

        fn update_outcome_on_three_matches(&mut self, tiles: [Option<Marker>; 3]) {
            let first = tiles[0];

            if tiles.iter().all(|tile| tile.is_some() && *tile == first) {
                self.outcome = Some(Winner(first.unwrap()))
            }
        }

        fn check_draw(&mut self) {
            let all_tiles_used = self.grid.iter().flatten().all(|tile| tile.is_some());
            if all_tiles_used && self.outcome.is_none() {
                self.outcome = Some(Draw);
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum CannotMark {
        LocationOutOfBounds,
        LocationNotEmpty,
    }

}

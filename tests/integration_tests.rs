use tic_tac_toe::game::CannotMark::{LocationNotEmpty, LocationOutOfBounds};
use tic_tac_toe::game::Outcome::{Draw, Winner};
use tic_tac_toe::game::{Game, Marker, Outcome};
use Marker::{O, X};

#[test]
fn empty_grid_on_new_game() {
    let game: Game = Game::new();
    let is_grid_empty = game
        .grid
        .iter()
        .flatten()
        .all(|tile| tile.clone().is_none());

    assert!(is_grid_empty);
}

#[test]
fn cannot_mark_out_of_bounds_x() {
    let mut game: Game = Game::new();
    let error = game.mark(3, 0).err().unwrap();
    assert_eq!(error, LocationOutOfBounds)
}

#[test]
fn cannot_mark_out_of_bounds_y() {
    let mut game: Game = Game::new();
    let error = game.mark(0, 3).err().unwrap();
    assert_eq!(error, LocationOutOfBounds)
}

#[test]
fn cannot_choose_same_location_twice() {
    let mut game: Game = Game::new();
    game.mark(0, 0).unwrap();
    let error = game.mark(0, 0).err().unwrap();
    assert_eq!(error, LocationNotEmpty)
}

#[test]
fn x_wins_top_row() {
    let game = play_and_assert(vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (0, 2),
        (2, 0),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(X));
}

#[test]
fn o_wins_middle_row() {
    let game = play_and_assert(vec![
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (2, 1),
        (1, 2),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(O))
}

#[test]
fn x_wins_bottom_row() {
    let game = play_and_assert(vec![
        (2, 0),
        (0, 1),
        (2, 1),
        (0, 2),
        (2, 2),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(X));
}

#[test]
fn o_wins_left_col() {
    let game = play_and_assert(vec![
        (1, 1),
        (0, 0),
        (1, 2),
        (1, 0),
        (2, 2),
        (2, 0),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(O))
}

#[test]
fn x_wins_middle_col() {
    let game = play_and_assert(vec![
        (1, 1),
        (0, 0),
        (0, 1),
        (0, 2),
        (2, 1),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(X));
}

#[test]
fn o_wins_right_col() {
    let game = play_and_assert(vec![
        (1, 1),
        (0, 2),
        (0, 0),
        (1, 2),
        (2, 1),
        (2, 2),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(O))
}

#[test]
fn x_wins_first_diagonal() {
    let game = play_and_assert(vec![
        (0, 0),
        (0, 1),
        (1, 1),
        (0, 2),
        (2, 2),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(X));
}

#[test]
fn o_wins_second_diagonal() {
    let game = play_and_assert(vec![
        (0, 0),
        (2, 0),
        (0, 1),
        (1, 1),
        (2, 1),
        (0, 2),
    ]);
    assert_eq!(game.outcome.unwrap(), Winner(O))
}

#[test]
fn draw() {
    let game = play_and_assert(vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 1),
        (1, 0),
        (2, 0),
        (1, 2),
        (2, 2),
        (2, 1),
    ]);
    assert_eq!(game.outcome.unwrap(), Draw)
}

fn play_and_assert(positions: Vec<(usize, usize)>) -> Game {
    let mut game: Game = Game::new();
    let mut turn_num: u8 = 1;
    positions.iter().enumerate().for_each(|(i, (x, y))| {
        assert_eq!(game.outcome, None);
        assert_eq!(game.current_turn, expected_player(turn_num));
        game.mark(*x, *y).unwrap();
        assert_eq!(game.grid[*x][*y], Some(expected_player(turn_num)));
        turn_num += 1;
        assert_eq!(game.current_turn, expected_player(turn_num));
    });
    game
}

fn expected_player(turn_num: u8) -> Marker {
    if turn_num % 2 == 0 {
        O
    } else {
        X
    }
}

fn mark_on_new_game_then_assert_grid_updated(x: usize, y: usize) {
    let mut game: Game = Game::new();
    game.mark(x, y).unwrap();
    assert_eq!(game.grid[x][y].unwrap(), X);
}

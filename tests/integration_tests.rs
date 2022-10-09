use Marker::{O, X};
use tic_tac_toe::game::{Game, Marker};
use tic_tac_toe::game::InvalidChoice::LocationOutOfBounds;

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
fn crosses_start() {
    let game: Game = Game::new();
    assert_eq!(game.current_turn, X);
}

#[test]
fn no_winner_initially() {
    let game: Game = Game::new();
    assert_eq!(game.winner, None);
}

#[test]
fn turns_swap_on_each_action() {
    let mut game: Game = Game::new();
    game.mark(0,0).unwrap();
    assert_eq!(game.current_turn, O);
    game.mark(0,1).unwrap();
    assert_eq!(game.current_turn, X);
}

#[test]
fn mark_updates_grid_0_0() {
    mark_on_new_game_then_assert_grid_updated(0, 0);
}

#[test]
fn mark_updates_grid_0_1() {
    mark_on_new_game_then_assert_grid_updated(0, 1);
}

#[test]
fn mark_updates_grid_0_2() {
    mark_on_new_game_then_assert_grid_updated(0, 2);
}

#[test]
fn mark_updates_grid_1_0() {
    mark_on_new_game_then_assert_grid_updated(1, 0);
}

#[test]
fn mark_updates_grid_1_1() {
    mark_on_new_game_then_assert_grid_updated(1, 1);
}

#[test]
fn mark_updates_grid_1_2() {
    mark_on_new_game_then_assert_grid_updated(1, 2);
}

#[test]
fn mark_updates_grid_2_0() {
    mark_on_new_game_then_assert_grid_updated(2, 0);
}

#[test]
fn mark_updates_grid_2_1() {
    mark_on_new_game_then_assert_grid_updated(2, 1);
}

#[test]
fn mark_updates_grid_2_2() {
    mark_on_new_game_then_assert_grid_updated(2, 2);
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

fn mark_on_new_game_then_assert_grid_updated(x: usize, y: usize) {
    let mut game: Game = Game::new();
    game.mark(x, y).unwrap();
    assert_eq!(game.grid[x][y].unwrap(), X);
}
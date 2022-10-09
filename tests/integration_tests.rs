use tic_tac_toe::game::{Game, Marker};

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
    assert_eq!(game.current_turn, Marker::X);
}

#[test]
fn no_winner_initially() {
    let game: Game = Game::new();
    assert_eq!(game.winner, None);
}
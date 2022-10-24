use bevy::ecs::query::QueryIter;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState::Pressed;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::{random, thread_rng, Rng};
use KeyCode::{Down, Left, Right, Up};

const ARENA_WIDTH: i8 = 10;
const ARENA_HEIGHT: i8 = 10;
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .run();
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake)
            .add_system(take_keyboard_input)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.2))
                    .with_system(move_snake),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_food),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new().with_system(position_translation),
            );
    }
}

#[derive(Component, Debug)]
struct Snake(i8);

#[derive(Component, Copy, Clone, Debug)]
struct Segment(u8);

#[derive(Component, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component, Debug, Copy, Clone)]
struct Position {
    x: i8,
    y: i8,
}

#[derive(Component, Debug)]
struct Food;

fn spawn_snake(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Direction::Right)
        .insert(Segment(0))
        .insert(Position { x: 5, y: 3 });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Segment(1))
        .insert(Position { x: 4, y: 3 });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Segment(2))
        .insert(Position { x: 3, y: 3 });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Segment(3))
        .insert(Position { x: 2, y: 3 });
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (position, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(position.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(
                position.y as f32,
                window.height() as f32,
                ARENA_HEIGHT as f32,
            ),
            0.0,
        );
        transform.scale = Vec3::new(50.0, 50.0, 1.0);
    }
}

fn move_snake(mut query: Query<(&Snake, &Segment, &mut Position, Option<&Direction>)>) {
    let (snake_id, segment_number, mut position, direction): (
        &Snake,
        &Segment,
        Mut<Position>,
        Option<&Direction>,
    ) = query.iter_mut().find(|x| x.1 .0 == 0).unwrap();

    let mut prev_segment_x = position.x;
    let mut prev_segment_y = position.y;
    let mut prev_segment_order = segment_number.0;

    match direction {
        None => (),
        Some(Direction::Up) => position.y += 1,
        Some(Direction::Right) => position.x += 1,
        Some(Direction::Down) => position.y -= 1,
        Some(Direction::Left) => position.x -= 1,
    }

    while let Some((next_snake_id, next_segment_number, mut next_position, next_direction)) =
        query.iter_mut().find(|x| x.1 .0 == prev_segment_order + 1)
    {
        let next_segment_x_copy = next_position.x;
        let next_segment_y_copy = next_position.y;
        let next_segment_order_copy = next_segment_number.0;
        next_position.y = prev_segment_y;
        next_position.x = prev_segment_x;
        prev_segment_x = next_segment_x_copy;
        prev_segment_y = next_segment_y_copy;
        prev_segment_order = next_segment_order_copy;
    }
}

fn spawn_food(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: thread_rng().gen_range((0..ARENA_WIDTH)),
            y: thread_rng().gen_range((0..ARENA_HEIGHT)),
        });
}

fn take_keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut Direction, With<Snake>>,
) {
    use bevy::input::ButtonState;

    let mut direction = query.single_mut();

    let key = key_evr
        .iter()
        .find(|ev| ev.state == Pressed)
        .map(|ev| ev.key_code)
        .flatten();

    match key {
        Some(Up) => *direction = Direction::Up,
        Some(Right) => *direction = Direction::Right,
        Some(Down) => *direction = Direction::Down,
        Some(Left) => *direction = Direction::Left,
        _ => {}
    };
}

use ggez::{
    conf, event,
    graphics::{self, DrawParam, Image},
    Context, GameResult
};
use ggez::input::keyboard::KeyCode;
use glam::Vec2;
use hecs::{Entity, World};

use std::path;

const TILE_WIDTH: f32 = 32.0;

#[derive(Clone, Copy)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

pub struct Renderable {
    path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {}

pub struct BoxSpot {}

struct Game {
    world: World,
}

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

pub fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };
            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        {
            run_input_print(&self.world, context);
        }
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            run_rendering(&self.world, context);
        }
        Ok(())
    }
}

fn run_input_print(_world: &World, context: &mut Context) {
    if context.keyboard.is_key_pressed(KeyCode::Up) {
        println!("Up");
    }
    if context.keyboard.is_key_pressed(KeyCode::Down) {
        println!("Down");
    }
    if context.keyboard.is_key_pressed(KeyCode::Left) {
        println!("Left");
    }
    if context.keyboard.is_key_pressed(KeyCode::Right) {
        println!("Right");
    }
}

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/wall.png".to_string(),
        },
        Wall {},
    ))
}
pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

pub fn create_box(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/box.png".to_string(),
        },
        Box {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: "/images/box_spot.png".to_string(),
        },
        BoxSpot {},
    ))
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/player.png".to_string(),
        },
        Player {},
    ))
}

pub fn run_rendering(world: &World, context: &mut Context) {
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    for (_, (position, renderable)) in rendering_data.iter() {
        let image = Image::from_path(context, renderable.path.clone()).unwrap();
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(&image, draw_params);
    }
    canvas.finish(context).expect("expected to present");
}

pub fn main() -> GameResult {
    let mut world = World::new();
    initialize_level(&mut world);
    let context_builder = ggez::ContextBuilder::new("rsokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));
    let (context, event_loop) = context_builder.build()?;
    let game = Game { world };
    event::run(context, event_loop, game)
}

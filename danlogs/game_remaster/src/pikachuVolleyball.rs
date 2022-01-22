use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::Camera
};

pub const ARENA_HEIGHT: f32= 1280.0;
pub const ARENA_WIDTH: f32= 1280.0;

fn initialize_camera(world: &mut World){
    let mut transform= transform::default();
    transform.set_translation_xyz(
        ARENA_WIDTH * 0.5,
        ARENA_HEIGHT * 0.5,
        1.0
    );
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .build();
}

pub struct PikachuVolleyball;
impl SimpleState for PikachuVolleyball {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world= data.world;
        world.register::<Player>();
        initialize_camera(world);
    }    
}

pub const PLAYER_HEIGHT: f32= 64.0;
pub const PLAYER_WIDTH: f32= 64.0;

#[derive(PartialEq, Eq)]
pub enum Side{
    Left,
    Right
}

pub struct Player{
    pub side: Side,
    pub width: f32,
    pub height: f32
}

impl Player {
    fn new(side: Side) -> Player{
        Player{
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT
        }
    }
}

impl Component for Player{
    type Storage = DenseVecStorage<Self>;
}

fn initialize_players(world: &mut World){
    let mut left_transform= Transform::default();
    let mut right_transform= Transform::default();

    let y= PLAYER_HEIGHT * 0.5;
    left_transform.set_translation_xyz(PLAYER_WIDTH * 0.5, y, 0.0 );
    right_transform.set_translation_xyz(ARENA_WIDTH - PLAYER_WIDTH *0.5, y, 0.0);
    right_transform.set_rotation_y_axis(std::f32::consts::PI);

    world
        .create_entity()
        .with(Player::new(Side::Left))
        .with(left_transform)
        .build();
    
    world
        .create_entity()
        .with(Player::new(Side::Right))
        .with(right_transform)
        .build();

}
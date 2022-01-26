use amethyst::{
    core::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{ Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input:: { InputHandler, StringBindings },
    core::timing::Time,
};

use cate::pikachuvolleyball::{ Player, Side, ARENA_WIDTH, PLAYER_WIDTH};

const PLAYER_SPEED: f32= 128.0;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl <'s> System <'S> for PlayerSystem {
    type SystemData= (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData)){
        
    }
}
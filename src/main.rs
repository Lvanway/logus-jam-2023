use std::default;

use bevy::{prelude::*, render::{RenderPlugin, settings::{RenderCreation, Backends, WgpuSettings}}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings{backends: Some(Backends::VULKAN), ..default()})
        }))
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu), setup)
        .run();
}


#[derive(Component)]
struct Player {
    position: Vec2,
    velocity: Vec2,
    hit_points: i32
}

impl Player {
    fn new(position: Vec2, velocity: Vec2, hit_points: i32) -> Self {
        Self {
            position, 
            velocity, 
            hit_points
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
  }


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    InGame
}
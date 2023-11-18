use bevy::{prelude::*, render::{RenderPlugin, settings::{RenderCreation, Backends, WgpuSettings}}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings{backends: Some(Backends::VULKAN), ..default()})
        }))
        .run();
}

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use avian2d::prelude::*;

mod background;
mod player;
mod camera;
mod tiles;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(camera::CameraPlugin)
        .add_plugins(tiles::TilesPlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}


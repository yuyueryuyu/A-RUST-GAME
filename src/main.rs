use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use avian2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use big_brain::BigBrainPlugin;
use bevy_kira_audio::prelude::*;


mod background;
mod player;
mod camera;
mod tiles;
mod animator;
mod enemy;
mod game_layer;
mod damagable;
mod input;
mod controller;
mod physics;
mod healthbar;
mod items;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AudioPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(TnuaControllerPlugin::new(FixedUpdate))
        .add_plugins(TnuaAvian2dPlugin::new(FixedUpdate))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_plugins(camera::CameraPlugin)
        .add_plugins(tiles::TilesPlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(animator::AnimatorPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(damagable::DamagePlugin)
        .add_plugins(healthbar::HealthBarPlugin)
        .add_plugins(items::ItemsPlugin)
        .run();
}


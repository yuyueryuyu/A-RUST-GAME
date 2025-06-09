use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
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
mod menu;
mod save;
mod pause;
mod hint;
mod getitem;
mod bag_ui;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PausedState {
    #[default]
    Running,
    Paused,
    GetItem,
    BagUI,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AudioPlugin)
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(TnuaControllerPlugin::new(FixedUpdate))
        .add_plugins(TnuaAvian2dPlugin::new(FixedUpdate))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .insert_state(AppState::MainMenu)
        .init_state::<PausedState>()
        .add_plugins(camera::CameraPlugin {
            state: AppState::InGame,
        })
        .add_plugins(tiles::TilesPlugin {
            state: AppState::InGame,
        })
        .add_plugins(background::BackgroundPlugin {
            state: AppState::InGame,
        })
        .add_plugins(animator::AnimatorPlugin {
            state: AppState::InGame,
        })
        .add_plugins(player::PlayerPlugin {
            state: AppState::InGame,
        })
        .add_plugins(enemy::EnemyPlugin {
            state: AppState::InGame,
        })
        .add_plugins(damagable::DamagePlugin {
            state: AppState::InGame,
        })
        .add_plugins(healthbar::HealthBarPlugin {
            state: AppState::InGame,
        })
        .add_plugins(menu::MenuPlugin {
            state: AppState::MainMenu,
        })
        .add_plugins(pause::PausePlugin)
        .add_plugins(getitem::GetItemPlugin)
        .add_plugins(bag_ui::BagUIPlugin)
        .add_plugins(items::ItemsPlugin {
            state: AppState::InGame,
        })
        .add_plugins(hint::HintPlugin {
            state: AppState::InGame,
        })
        .add_plugins(save::SavingPlugin)
        .run();
}


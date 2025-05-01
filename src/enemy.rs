use bevy::prelude::*;

mod skeleton;
mod fire_demon;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(skeleton::SkeletonPlugin);
        app.add_plugins(fire_demon::FireDemonPlugin);
    }
}
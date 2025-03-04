use bevy::prelude::*;

mod skeleton;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(skeleton::SkeletonPlugin);
    }
}
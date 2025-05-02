use bevy::prelude::*;

mod skeleton;
mod fire_demon;

pub struct EnemyPlugin<S: States> {
    pub state: S,
}

impl<S:States> Plugin for EnemyPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(skeleton::SkeletonPlugin { state : self.state.clone() });
        app.add_plugins(fire_demon::FireDemonPlugin { state : self.state.clone() });
    }
}
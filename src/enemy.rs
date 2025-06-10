use bevy::prelude::*;

mod skeleton;
mod flying_eye;
pub(crate) mod fire_demon;
pub(crate) mod martial;
pub struct EnemyPlugin<S: States> {
    pub state: S,
}

impl<S:States> Plugin for EnemyPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(skeleton::SkeletonPlugin { state : self.state.clone() });
        app.add_plugins(flying_eye::FlyingEyesPlugin { state : self.state.clone() });
        app.add_plugins(fire_demon::FireDemonPlugin { state : self.state.clone() });
        app.add_plugins(martial::MartialPlugin { state : self.state.clone() });
    }
}
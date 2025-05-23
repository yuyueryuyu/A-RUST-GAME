use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    PlayerHitBox,
    EnemyHitBox,
    Player,  // Layer 1
    Enemy,   // Layer 2
    Sensor,
    Ground,  // Layer 3
}

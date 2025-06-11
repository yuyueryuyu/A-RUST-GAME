use bevy::prelude::*;
use avian2d::prelude::*;

use crate::game_layer::GameLayer;

/// 物理组件包
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub rigidbody: RigidBody,
    pub locked_axes: LockedAxes,
    pub ccd: SweptCcd,
    pub mass: Mass,
    pub gravity: GravityScale,
    pub collider: Collider,
    pub collision_margin: CollisionMargin,
    pub layer: CollisionLayers,
    pub friction: Friction,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            ccd: SweptCcd::default(),
            mass: Mass(1.0),
            gravity: GravityScale(30.0),
            collider: Collider::rectangle(0., 0.),
            collision_margin: CollisionMargin(0.1),
            layer: CollisionLayers::new(GameLayer::Default, [GameLayer::Default]),
            friction: Friction::default(),
        }
    }
}

use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::*;

#[derive(Bundle)]
pub struct ControllerBundle {
    controller: TnuaController,
    sensor_shape: TnuaAvian2dSensorShape,
}

impl ControllerBundle {
    pub fn new(sensor_len: f32) -> Self {
        let mut controller = TnuaController::default();
        controller.basis(TnuaBuiltinWalk::default());
        Self {
            controller: controller,
            sensor_shape: TnuaAvian2dSensorShape(
                Collider::rectangle(sensor_len, 0.)
            ),
        }
    }
}
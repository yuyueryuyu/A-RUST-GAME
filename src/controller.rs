//! 控制器组件包，调用bevy_tnua

use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::*;

/// 控制器组件包
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
use bevy::{
    prelude::*,
    render::{
        camera::{
            ScalingMode,
        },
        view::RenderLayers,
    },
    ecs::system::ParamSet
};

use crate::background;
use crate::player;

#[derive(Component, Reflect)]
struct InGameCamera;

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            ..default()
        },
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 800.0,
            },
            scale: 0.3,
            ..OrthographicProjection::default_2d()
        }),
        CameraFollow {
            dead_zone_width: 0.0,
            soft_zone_width: 0.8,
            damping: 1.0,
            look_ahead_time: 0.0,
            screen_offset: Vec2::new(0.0, 0.0),
            axis_constraints: [false, false, true],
            constraint_values: Vec3::ZERO,
            blend_factor: 0.1,
            previous_target_position: Vec3::ZERO,
        },
    ));
}

// 定义相机跟随组件
#[derive(Component, Reflect)]
pub struct CameraFollow {
    pub dead_zone_width: f32,
    pub soft_zone_width: f32,
    pub damping: f32,
    pub look_ahead_time: f32,
    pub screen_offset: Vec2,
    pub axis_constraints: [bool; 3],
    pub constraint_values: Vec3,
    pub blend_factor: f32,
    pub previous_target_position: Vec3,
}

// 世界坐标到屏幕坐标转换
fn world_to_screen(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    point: Vec3,
) -> Option<Vec2> {
    let viewport_size = camera.logical_viewport_size()?;
    let projection_matrix = camera.clip_from_view();
    let view_matrix = camera_transform.compute_matrix().inverse();
    
    let point_ndc = (projection_matrix * view_matrix).project_point3(point);
    if point_ndc.z < 0.0 || point_ndc.z > 1.0 {
        return None;
    }
    
    let screen_x = (point_ndc.x + 1.0) / 2.0 * viewport_size.x;
    let screen_y = (1.0 - point_ndc.y) / 2.0 * viewport_size.y;
    Some(Vec2::new(screen_x, screen_y))
}

// 屏幕坐标到世界坐标转换
fn screen_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    screen_pos: Vec2,
    depth: f32,
) -> Option<Vec3> {
    let viewport_size = camera.logical_viewport_size()?;
    let projection_matrix = camera.clip_from_view();
    let inv_view_proj = (projection_matrix * camera_transform.compute_matrix().inverse()).inverse();

    let ndc_x = (screen_pos.x / viewport_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (screen_pos.y / viewport_size.y) * 2.0;
    let world_pos = inv_view_proj.project_point3(Vec3::new(ndc_x, ndc_y, depth));
    Some(world_pos)
}

// 阻尼系统实现
fn compute_soft_damping(
    current_pos: Vec3,
    target_pos: Vec3,
    delta_time: f32,
    soft_zone_width: f32,
    damping: f32,
) -> Vec3 {
    let delta = target_pos - current_pos;
    let distance = delta.length();
    
    if distance < soft_zone_width {
        let damping_factor = (-damping * delta_time).exp();
        current_pos + delta * (1.0 - damping_factor)
    } else {
        target_pos
    }
}

// 预测系统实现
fn compute_predicted_position(current_pos: Vec3, velocity: Vec3, look_ahead_time: f32) -> Vec3 {
    current_pos + velocity * look_ahead_time
}

// 轴向约束应用
fn apply_axis_constraints(position: Vec3, constraints: [bool; 3], constraint_values: Vec3) -> Vec3 {
    let mut result = position;
    if constraints[0] {
        result.x = constraint_values.x;
    }
    if constraints[1] {
        result.y = constraint_values.y;
    }
    if constraints[2] {
        result.z = constraint_values.z;
    }
    result
}

// 主相机跟随系统
fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<player::Player>>,
    mut cameras: Query<(&mut Transform, &Camera, &GlobalTransform, &mut CameraFollow), Without<player::Player>>,
) {
    let delta_time = time.delta_secs();

    for (mut cam_transform, camera, cam_global_transform, mut follow) in cameras.iter_mut() {
        let Ok(target_transform) = player_query.get_single() else {
            continue;
        };

        // 计算目标速度
        let current_target_pos = target_transform.translation;
        let target_velocity = (current_target_pos - follow.previous_target_position) / delta_time;
        follow.previous_target_position = current_target_pos;

        // 预测目标位置
        let predicted_pos = compute_predicted_position(
            current_target_pos,
            target_velocity,
            follow.look_ahead_time,
        );

        // 死区检查
        if (predicted_pos - cam_transform.translation).length() < follow.dead_zone_width {
            continue;
        }

        // 应用软区阻尼
        let damped_pos = compute_soft_damping(
            cam_transform.translation,
            predicted_pos,
            delta_time,
            follow.soft_zone_width,
            follow.damping,
        );

        // 屏幕空间组合
        let screen_pos = match world_to_screen(camera, cam_global_transform, damped_pos) {
            Some(pos) => pos,
            None => continue,
        };

        let offset_screen_pos = screen_pos + follow.screen_offset * camera.logical_viewport_size().unwrap();
        let world_pos = match screen_to_world(
            camera,
            cam_global_transform,
            offset_screen_pos,
            damped_pos.z,
        ) {
            Some(pos) => pos,
            None => continue,
        };

        // 应用轴向约束
        let constrained_pos = apply_axis_constraints(
            world_pos,
            follow.axis_constraints,
            follow.constraint_values,
        );

        // 混合位置
        cam_transform.translation = cam_transform.translation.lerp(
            constrained_pos,
            follow.blend_factor.clamp(0.0, 1.0),
        );
    }
}

fn update_parallax_effect(
    mut param_set: ParamSet<(
        Query<&Transform, With<player::Player>>,
        Query<&Transform, With<InGameCamera>>,
        Query<(&mut Transform, &background::Background), Without<player::Player>>,
    )>,
) {
    // 先获取所有需要的数据并存储在本地变量中
    let player_pos = param_set.p0().single().translation;
    let camera_pos = param_set.p1().single().translation;

    // 然后处理背景
    for (mut bg_transform, background) in param_set.p2().iter_mut() {
        let cam_move_since_start = Vec2::new(camera_pos.x, camera_pos.y) - background.starting_position;
        let z_distance_from_target = bg_transform.translation.z - player_pos.z;
        let clipping_plane = camera_pos.z + if z_distance_from_target > 0.0 { 20.0 } else { -20.0 };
        let parallax_factor = f32::abs(z_distance_from_target) / clipping_plane;
        let new_position = background.starting_position + cam_move_since_start * parallax_factor;
        bg_transform.translation = Vec3::new(new_position.x, new_position.y, background.starting_z);
    }
}

pub struct CameraPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for CameraPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_camera),
        );
        app.add_systems(Update, update_parallax_effect.run_if(in_state(self.state.clone())));
        app.add_systems(FixedUpdate, camera_follow_system.run_if(in_state(self.state.clone())));
    }
}
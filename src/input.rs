use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::animator::Animator;
use crate::player::get_speed;
use crate::player::Player;

/** 键盘输入模块 */
#[derive(Bundle)]
pub struct PlayerInputBundle {
    input_manager: InputManagerBundle<Action>,
}

impl Default for PlayerInputBundle {
    fn default() -> Self {
        Self {
            input_manager: InputManagerBundle::with_map(PlayerInputBundle::default_input_map()),
        }
    }
}

/** 设置默认键位 */
impl PlayerInputBundle {
    pub fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        input_map.insert(Action::Up, KeyCode::KeyW);
        input_map.insert(Action::Down, KeyCode::KeyS);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Jump, KeyCode::Space);
        input_map.insert(Action::Run, KeyCode::ShiftLeft);
        input_map.insert(Action::Crouch, KeyCode::KeyC);
        input_map.insert(Action::Attack, MouseButton::Left);

        input_map
    }
}

/** 键位对应的动作 */
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Run,
    Jump,
    Crouch,
    Attack,
}

/** 每个动作对应的一些实用方法 */
impl Action {
    const DIRECTIONS: [Self; 4] = [Action::Up, Action::Down, Action::Left, Action::Right];

    fn direction(self) -> Option<Dir2> {
        match self {
            Action::Up => Some(Dir2::Y),
            Action::Down => Some(Dir2::NEG_Y),
            Action::Left => Some(Dir2::NEG_X),
            Action::Right => Some(Dir2::X),
            _ => None,
        }
    }
}

fn on_move(
    mut player: Single<
        (
            &ActionState<Action>,
            &mut LinearVelocity,
            &mut Transform,
            &mut Animator,
        ),
        With<Player>,
    >,
) {
    let (action_state, mut vel, mut transform, mut animator) = player.into_inner();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in Action::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += *direction;
            }
        }
    }

    // Then reconvert at the end, normalizing the magnitude
    let is_moving = direction_vector != Vec2::ZERO;
    if is_moving {
        direction_vector = direction_vector.normalize();
    }

    // Set Face Direction
    if direction_vector.x > 0.0 && !animator.get_bool("is_facing_right") {
        animator.set_bool("is_facing_right", true);
        transform.scale.x *= -1.0;
    }
    if direction_vector.x < 0.0 && animator.get_bool("is_facing_right") {
        animator.set_bool("is_facing_right", false);
        transform.scale.x *= -1.0;
    }

    // Set parameters
    animator.set_bool("is_moving", is_moving);
    if !animator.get_bool("is_on_wall") {
        vel.x = direction_vector.x * get_speed(&*animator);
        animator.set_float("velocity_y", vel.y);
    }
}

fn on_crouch(mut player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Crouch) {
        let crouching = animator.get_bool("is_crouching");
        animator.set_bool("is_crouching", !crouching);
    }
}

fn on_jump(
    mut player: Single<(&ActionState<Action>, &mut LinearVelocity, &mut Animator), With<Player>>,
) {
    let (action_state, mut vel, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Jump)
        && animator.get_bool("is_grounded")
        && animator.get_bool("can_move")
    {
        animator.set_trigger("jump");
        vel.y = 200.;
    }
}

fn on_run(mut player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    let is_running = action_state.pressed(&Action::Run);
    animator.set_bool("is_running", is_running);
}

fn on_attack(mut player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Attack) {
        animator.set_trigger("attack");
    }
}

fn on_slide(
    time: Res<Time>,
    mut player: Single<(&ActionState<Action>, &mut Animator), With<Player>>,
) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.pressed(&Action::Run) {
        let shift_press_time = animator.get_float("shift_press_time");
        animator.set_float("shift_press_time", shift_press_time + time.delta_secs());
    }
    if action_state.just_released(&Action::Run) {
        let shift_press_time = animator.get_float("shift_press_time");
        if shift_press_time <= 0.4 {
            animator.set_trigger("slide");
            let dir = if animator.get_bool("is_facing_right") {
                1.0
            } else {
                -1.0
            };
            animator.set_float("impulse_x", 600.);
        }
        animator.set_float("shift_press_time", 0.0);
    }
}

fn apply_impulse_x(mut query: Query<(&mut LinearVelocity, &mut Animator), With<Player>>) {
    for (mut v, mut animator) in &mut query {
        let mut vx = animator.get_float("impulse_x");
        if vx.abs() > 0. {
            v.x = vx;
            if vx > 0. {
                vx -= 60.;
            } else {
                vx += 60.;
            }
            animator.set_float("impulse_x", vx);
        }
    }
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
        app.add_systems(
            Update,
            (
                on_move,
                on_run,
                on_jump,
                on_crouch,
                on_attack,
                on_slide,
                apply_impulse_x,
            )
                .chain(),
        );
    }
}

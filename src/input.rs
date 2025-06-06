use bevy::asset;
use bevy_tnua::math::*;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua::builtins::*;
use bevy_kira_audio::prelude::*;

use crate::animator::Animator;
use crate::damagable::Damagable;
use crate::healthbar::Hint;
use crate::items::ItemList;
use crate::player::Player;
use crate::enemy::fire_demon::FireGlove;
use crate::PausedState;

const WALK_SPEED: f32 = 80.0;
const RUN_SPEED: f32 = 120.0;
const MOVE_ACC: f32 = 600.0;
const CROUCH_SPEED: f32 = 50.0;
const SLIDE_SPEED: f32 = 400.0;
const SLIDE_ACC: f32 = 400.0;
const JUMP_IMPULSE: f32 = 600.0;

pub fn get_speed(animator: &Animator) -> f32 {
    if animator.get_bool("can_move") {
        if animator.get_bool("is_moving") {
            if animator.get_bool("is_crouching") {
                return CROUCH_SPEED;
            }
            if animator.get_bool("is_running") {
                return RUN_SPEED;
            }
            return WALK_SPEED;
        }
    }
    0.0
}

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
    fn default_input_map() -> InputMap<Action> {
        let mut input_map = InputMap::default();

        input_map.insert(Action::Up, KeyCode::KeyW);
        input_map.insert(Action::Down, KeyCode::KeyS);
        input_map.insert(Action::Left, KeyCode::KeyA);
        input_map.insert(Action::Right, KeyCode::KeyD);
        input_map.insert(Action::Jump, KeyCode::Space);
        input_map.insert(Action::Run, KeyCode::ShiftLeft);
        input_map.insert(Action::Crouch, KeyCode::KeyC);
        input_map.insert(Action::Attack, MouseButton::Left);
        input_map.insert(Action::Defense, MouseButton::Right);
        input_map.insert(Action::UseItem, KeyCode::KeyR);
        input_map.insert(Action::PickItem, KeyCode::KeyE);
        input_map
    }
}

/** 键位对应的动作 */
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Run,
    Jump,
    Crouch,
    Attack,
    Defense,
    UseItem,
    PickItem,
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
    player: Single<
        (
            &ActionState<Action>,
            &LinearVelocity,
            &mut Transform,
            &mut Animator,
            &mut TnuaController,
        ),
        With<Player>,
    >,
) {
    let (action_state, vel, mut transform, mut animator, mut controller) = player.into_inner();

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
    animator.set_float("velocity_y", vel.y);
    
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: Vec3::new(direction_vector.x , 0., 0.)* get_speed(&*animator),
        float_height: 18.,
        air_acceleration: MOVE_ACC,
        acceleration: MOVE_ACC,
        max_slope: float_consts::FRAC_PI_4,
        ..Default::default()
    });
}

fn on_crouch(player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Crouch) {
        let crouching = animator.get_bool("is_crouching");
        animator.set_bool("is_crouching", !crouching);
    }
}

fn on_jump(
    player: Single<(&ActionState<Action>, &mut TnuaController, &mut Animator), With<Player>>,
) {
    let (action_state, mut controller, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Jump)
        && animator.get_bool("can_move")
    {
        animator.set_trigger("jump");
        controller.action(TnuaBuiltinJump {
            height: JUMP_IMPULSE,
            allow_in_air: animator.get_bool("is_on_wall") && animator.get_bool("can_wall_jump"),
            ..Default::default()
        });
    }
}

fn on_run(player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    let is_running = action_state.pressed(&Action::Run);
    animator.set_bool("is_running", is_running);
}

fn on_attack(player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Attack) {
        animator.set_trigger("attack");
    }
}

fn on_defense(player: Single<(&ActionState<Action>, &mut Animator, &mut Damagable), With<Player>>) {
    let (action_state, mut animator, mut damagable) = player.into_inner();
    if action_state.just_pressed(&Action::Defense) {
        animator.set_trigger("defense");
        damagable.set_defending(true);
    }
}

fn on_slide(
    time: Res<Time>,
    player: Single<(&ActionState<Action>, &mut Animator, &mut TnuaController, &mut Damagable), With<Player>>,
) {
    let (action_state, mut animator, mut controller, mut damagable) = player.into_inner();
    if action_state.pressed(&Action::Run) {
        let shift_press_time = animator.get_float("shift_press_time");
        animator.set_float("shift_press_time", shift_press_time + time.delta_secs());
    }
    if action_state.just_released(&Action::Run) {
        let shift_press_time = animator.get_float("shift_press_time");
        if shift_press_time <= 0.4 {
            animator.set_trigger("slide");
            damagable.set_invincible_with_time(0.5);
            let facing_direction = if animator.get_bool("is_facing_right") {
                1.
            } else {
                -1.
            };
            controller.action(TnuaBuiltinDash {
                displacement: Vec3::new(100., 0., 0.)* facing_direction ,
                speed: SLIDE_SPEED * 2.,
                acceleration: SLIDE_ACC * 2.,
                brake_acceleration: SLIDE_ACC * 2.,
                ..Default::default()
            });
        }
        animator.set_float("shift_press_time", 0.0);
    }
}

fn on_use(
    player: Single<(Entity, &ActionState<Action>, &mut Animator, &mut ItemList), With<Player>>,
) {
    let (entity, action_state, mut animator, mut item_list) = player.into_inner();
    if action_state.just_pressed(&Action::UseItem) {
        item_list.use_item(entity);
        animator.set_trigger("items");
    }
}

pub fn on_pick(
    mut trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut player_single: Single<(&ActionState<Action>, &mut Animator), With<Player>>,
    mut text: Single<&mut Text, With<Hint>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    let item = trigger.target();
    let player = trigger.collider;
    let (action_state, mut animator) = player_single.into_inner();
    if action_state.just_pressed(&Action::PickItem) {
        animator.set_bool("can_wall_jump", true);
        commands.entity(item).despawn();
        text.0 = "".to_string();
        next_state.set(PausedState::GetItem);
    }
}

pub struct PlayerInputPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerInputPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
        app.add_systems(
            Update,
            (
                on_move.run_if(in_state(self.state.clone())),
                on_run.run_if(in_state(self.state.clone())),
                on_jump.run_if(in_state(self.state.clone())),
                on_crouch.run_if(in_state(self.state.clone())),
                on_attack.run_if(in_state(self.state.clone())),
                on_slide.run_if(in_state(self.state.clone())),
                on_defense.run_if(in_state(self.state.clone())),
                on_use.run_if(in_state(self.state.clone())),
            )
        );
    }
}

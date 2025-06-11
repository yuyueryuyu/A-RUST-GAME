//! 玩家输入控制器
//! 调用leafwing_input_manager实现

use bevy_tnua::math::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_tnua::prelude::*;

use crate::animator::Animator;
use crate::damagable::Damagable;
use crate::items::ActiveItems;
use crate::items::NearingItem;
use crate::items::NotpickedItems;
use crate::items::PickItemTrigger;
use crate::player::Player;

/// 走路速度
const WALK_SPEED: f32 = 80.0;
/// 跑步速度
const RUN_SPEED: f32 = 120.0;
/// 移动加速度
const MOVE_ACC: f32 = 600.0;
/// 下蹲速度
const CROUCH_SPEED: f32 = 50.0;
/// 获取速度
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

/// 键盘输入模块
#[derive(Bundle)]
pub struct PlayerInputBundle {
    input_manager: InputMap<Action>,
}

impl Default for PlayerInputBundle {
    fn default() -> Self {
        Self {
            input_manager: PlayerInputBundle::default_input_map(),
        }
    }
}

/// 设置默认键位
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
        input_map.insert(Action::ReverseGravity, KeyCode::KeyG);
        input_map.insert(Action::ChangeItem, KeyCode::Tab);
        input_map
    }
}

/// 键位对应的动作
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
    ReverseGravity,
    ChangeItem
}

/// 每个动作对应的一些方法
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

/// 输入移动按键
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
                // 计算移动向量
                direction_vector += *direction;
            }
        }
    }

    // 设置是否正在活动
    let is_moving = direction_vector != Vec2::ZERO;
    if is_moving {
        direction_vector = direction_vector.normalize();
    }

    // 设置朝向
    if direction_vector.x > 0.0 && !animator.get_bool("is_facing_right") {
        animator.set_bool("is_facing_right", true);
        transform.scale.x *= -1.0;
    }
    if direction_vector.x < 0.0 && animator.get_bool("is_facing_right") {
        animator.set_bool("is_facing_right", false);
        transform.scale.x *= -1.0;
    }

    // 设置动画状态机参数
    animator.set_bool("is_moving", is_moving);
    animator.set_float("velocity_y", vel.y);
    
    // 设置控制器
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: Vec3::new(direction_vector.x , 0., 0.)* get_speed(&*animator),
        float_height: 18.,
        air_acceleration: MOVE_ACC,
        acceleration: MOVE_ACC,
        max_slope: float_consts::FRAC_PI_4,
        ..Default::default()
    });
}

/// 输入下蹲按键
fn on_crouch(player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Crouch) {
        let crouching = animator.get_bool("is_crouching");
        // 通知动画状态机下蹲
        animator.set_bool("is_crouching", !crouching);
    }
}

/// 输入跳跃按键
fn on_jump(
    player: Single<(&ActionState<Action>, &mut Animator), With<Player>>,
) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Jump) && animator.get_bool("can_move") {
        // 通知动画状态机跳跃
        animator.set_trigger("jump");
    }
}

/// 输入跑步按键
fn on_run(player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    let is_running = action_state.pressed(&Action::Run);
    // 通知动画状态机跑步
    animator.set_bool("is_running", is_running);
}

/// 输入攻击按键
fn on_attack(player: Single<(&ActionState<Action>, &mut Animator), With<Player>>) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::Attack) {
        animator.set_trigger("attack");
    }
}

/// 输入防守按键
fn on_defense(player: Single<(&ActionState<Action>, &mut Animator, &mut Damagable), With<Player>>) {
    let (action_state, mut animator, mut damagable) = player.into_inner();
    if action_state.just_pressed(&Action::Defense) {
        animator.set_trigger("defense");
        damagable.set_defending(true);
    }
}

/// 输入滑行按键
fn on_slide(
    time: Res<Time>,
    player: Single<(&ActionState<Action>, &mut Animator, &mut Damagable), With<Player>>,
) {
    let (action_state, mut animator, mut damagable) = player.into_inner();
    // 计算按跑步按键时间
    if action_state.pressed(&Action::Run) {
        let shift_press_time = animator.get_float("shift_press_time");
        animator.set_float("shift_press_time", shift_press_time + time.delta_secs());
    }
    if action_state.just_released(&Action::Run) {
        let shift_press_time = animator.get_float("shift_press_time");
        // 短按 -> 滑行
        if shift_press_time <= 0.4 {
            animator.set_trigger("slide");
            damagable.set_invincible_with_time(0.5);
        }
        animator.set_float("shift_press_time", 0.0);
    }
}

/// 输入使用物品按键
fn on_use(
    player: Single<(&ActionState<Action>, &mut Animator), With<Player>>,
) {
    let (action_state, mut animator) = player.into_inner();
    if action_state.just_pressed(&Action::UseItem) {
        animator.set_trigger("items");
    }
}

/// 输入拾取物品按键
fn on_pick(
    mut commands: Commands,
    player: Single<(Entity, &NearingItem, &ActionState<Action>), With<Player>>,
    items: Query<&NotpickedItems>
) {
    let (entity, nearing_items, action_state) = player.into_inner();
    if action_state.just_pressed(&Action::PickItem) {
        for item in (**nearing_items).clone() {
            let item_info = items.get(item).unwrap();
            // 触发拾取物品触发器
            commands.trigger(PickItemTrigger { 
                picker: entity, 
                item: item_info.id.clone(), 
                num: item_info.num 
            });
        }
    }
}

/// 输入反转重力按键
fn on_reverse(
    player: Single<(&Animator, &mut GravityScale, &ActionState<Action>), With<Player>>,
) {
    let (animator, mut gravity, action_state) = player.into_inner();
    // 若已经获得能力
    if action_state.just_pressed(&Action::ReverseGravity) && animator.get_bool("can_reverse_gravity") {
        gravity.0 = -gravity.0 - 100.;
    }
}

/// 输入更换物品按键
fn on_change_item(
    player: Single<(&mut ActiveItems, &ActionState<Action>), With<Player>>,
) {
    let (mut acts, action_state) = player.into_inner();
    if action_state.just_pressed(&Action::ChangeItem) {
        if acts.items.len() == 0 { return; }
        acts.current = (acts.current + 1) % acts.items.len();
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
                on_pick.run_if(in_state(self.state.clone())),
                on_reverse.run_if(in_state(self.state.clone())),
                on_change_item.run_if(in_state(self.state.clone())),
            )
        );
    }
}

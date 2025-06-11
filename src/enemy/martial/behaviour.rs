//! 武师boss行为树
use crate::animator::*;
use crate::damagable::Damagable;
use crate::enemy::martial::Martial;
use crate::player::Player;
use avian2d::prelude::GravityScale;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_tnua::builtins::*;
use bevy_tnua::math::*;
use bevy_tnua::prelude::*;
use big_brain::prelude::*;
use rand::Rng;

const WALK_SPEED: f32 = 25.0;
const NOTICED_SPEED: f32 = 50.0;
const FLOAT_HEIGHT: f32 = 26.;

fn get_speed(animator: &Animator, notice: &Notice) -> f32 {
    if animator.get_bool("can_move") {
        if notice.notice > 50. {
            return NOTICED_SPEED;
        }
        return WALK_SPEED;
    }
    0.0
}

#[derive(Component, Debug, Reflect)]
pub struct Notice {
    pub add_per_sec: f32,
    pub sub_per_sec: f32,
    pub notice: f32,
}

impl Notice {
    pub fn new(notice: f32, add_per_sec: f32, sub_per_sec: f32) -> Self {
        Self {
            notice,
            add_per_sec,
            sub_per_sec,
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct HealthState {
    pub current_health: f32,
    pub max_health: f32,
    pub is_phase_two: bool,
}

impl HealthState {
    pub fn new(max_health: f32) -> Self {
        Self {
            current_health: max_health,
            max_health,
            is_phase_two: false,
        }
    }

    pub fn is_below_half(&self) -> bool {
        self.current_health < self.max_health * 0.5
    }
}

#[derive(Component, Debug, Reflect)]
pub struct PhaseTwoTimer {
    pub teleport_timer: Timer,
    pub is_on_ceiling: bool,
    pub teleport_position: Option<Vec2>,
}

impl PhaseTwoTimer {
    pub fn new() -> Self {
        Self {
            teleport_timer: Timer::from_seconds(3.0, TimerMode::Once),
            is_on_ceiling: false,
            teleport_position: None,
        }
    }
}

pub fn notice_system(time: Res<Time>, mut query: Query<(&mut Notice, &Animator), With<Martial>>) {
    for (mut notice, animator) in &mut query {
        if animator.is_active("hit") {
            notice.notice = 100.;
        }
        if animator.get_bool("is_noticing") {
            notice.notice += notice.add_per_sec * time.delta_secs();
        } else {
            notice.notice -= notice.sub_per_sec * time.delta_secs();
        }

        notice.notice = notice.notice.clamp(0.0, 80.0);
    }
}

pub fn health_system(
    mut query: Query<(&mut HealthState, &mut Animator, &Damagable), With<Martial>>
) {
    for (mut health_state, mut animator, damagable) in &mut query {
        health_state.current_health = damagable.health;
        
        if health_state.is_below_half() && !health_state.is_phase_two {
            health_state.is_phase_two = true;
            animator.set_trigger("jump");
        }
    }
}

pub fn phase_two_timer_system(
    time: Res<Time>,
    mut query: Query<(&mut PhaseTwoTimer, &HealthState), With<Martial>>,
    player_pos: Single<&Transform, With<Player>>,
) {
    for (mut timer, health_state) in &mut query {
        if health_state.is_phase_two && timer.is_on_ceiling {
            timer.teleport_timer.tick(time.delta());
            
            if timer.teleport_timer.finished() {
                timer.teleport_position = Some(player_pos.translation.truncate() - Vec2::new(-20., 0.));
                //animator.set_trigger("teleport_to_player");
            }
        }
    }
}

#[derive(Clone, Component, Debug, Reflect, ActionBuilder)]
pub struct MoveToPlayer;

const CLOSE_MAX_DISTANCE: f32 = 80.;
const MID_MAX_DISTANCE: f32 = 120.;
const JUMP_ATTACK_DISTANCE: f32 = 200.;
const FAR_MAX_DISTANCE: f32 = 400.;

pub fn move_to_player_action_system(
    player_pos: Single<&Transform, With<Player>>,
    mut actor_query: Query<
        (
            &Transform,
            &mut TnuaController,
            &LinearVelocity,
            &mut Animator,
            &Notice,
            &HealthState,
        ),
        Without<Player>,
    >,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToPlayer, &ActionSpan)>,
) {
    for (Actor(actor), mut action_state, _move_to, span) in &mut action_query {
        let _guard = span.span().enter();
        let (actor_pos, mut controller, vel, mut animator, notice, health_state) =
            actor_query.get_mut(*actor).expect("actor has no position");
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let facing_direction = if delta.x.trunc() > 0. {
                    1.
                } else if delta.x.trunc() < 0. {
                    -1.
                } else {
                    0.
                };
                
                if facing_direction != 0. {
                    animator.set_float("facing_direction", facing_direction);
                }
                
                let distance = delta.length();
                if distance > FAR_MAX_DISTANCE {
                    let vx = get_speed(&animator, &notice) * facing_direction;
                    if controller.is_airborne().unwrap() && !health_state.is_phase_two {
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(vel.x, 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                    } else {
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(vx, 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                    }
                } else if distance > MID_MAX_DISTANCE {
                    let vx = 2.0 * get_speed(&animator, &notice) * facing_direction;
                    if controller.is_airborne().unwrap() && !health_state.is_phase_two {
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(vel.x, 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                    } else {
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(vx, 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                    }
                    *action_state = ActionState::Success;
                } else {
                    if controller.is_airborne().unwrap() && !health_state.is_phase_two {
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(vel.x, 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                    } else {
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(0., 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                        *action_state = ActionState::Success;
                    }
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct Attack1;

pub fn attack1_action_system(
    mut enemy_query: Query<(&Transform, &mut Animator, &HealthState), Without<Player>>,
    player_pos: Single<&Transform, With<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &Attack1, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _attack, span) in &mut query {
        let _guard = span.span().enter();

        let (actor_pos, mut animator, health_state) = enemy_query
            .get_mut(*actor)
            .expect("actor didn't have components");

        match *state {
            ActionState::Requested => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let distance = delta.length();
                
                if distance > FAR_MAX_DISTANCE {
                    *state = ActionState::Failure;
                } else if distance <= CLOSE_MAX_DISTANCE || health_state.is_phase_two {
                    animator.set_trigger("attack1");
                    *state = ActionState::Executing;
                } else {
                    *state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                if !animator.is_active("attack1") {
                    *state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct Attack2;

pub fn attack2_action_system(
    mut enemy_query: Query<(&Transform, &mut Animator, &HealthState), Without<Player>>,
    player_pos: Single<&Transform, With<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &Attack2, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _attack, span) in &mut query {
        let _guard = span.span().enter();

        let (actor_pos, mut animator, health_state) = enemy_query
            .get_mut(*actor)
            .expect("actor didn't have components");

        match *state {
            ActionState::Requested => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let distance = delta.length();
                
                // 只在第一阶段且玩家在近距离时执行
                if !health_state.is_phase_two && distance <= CLOSE_MAX_DISTANCE {
                    animator.set_trigger("attack2");
                    *state = ActionState::Executing;
                } else {
                    *state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                if !animator.is_active("attack2") {
                    *state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct JumpAttack;

pub fn jump_attack_action_system(
    mut enemy_query: Query<(&Transform, &mut Animator, &mut TnuaController, &HealthState), Without<Player>>,
    player_pos: Single<&Transform, With<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &JumpAttack, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _attack, span) in &mut query {
        let _guard = span.span().enter();

        let (actor_pos, mut animator, mut controller, health_state) = enemy_query
            .get_mut(*actor)
            .expect("actor didn't have components");

        match *state {
            ActionState::Requested => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let distance = delta.length();
                
                // 只在第一阶段且中距离时有概率执行跳跃攻击
                if !health_state.is_phase_two && distance > MID_MAX_DISTANCE && distance <= JUMP_ATTACK_DISTANCE {
                    let mut rng = rand::rng();
                    let random: f32 = rng.random();
                    if random > 0.7 { // 30% 概率
                        if !controller.is_airborne().unwrap() {
                            controller.action(TnuaBuiltinJump {
                                height: 400.,
                                ..Default::default()
                            });
                            animator.set_trigger("jump");
                            *state = ActionState::Executing;
                        } else {
                            *state = ActionState::Failure;
                        }
                    } else {
                        *state = ActionState::Failure;
                    }
                } else {
                    *state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let facing_direction = if delta.x.trunc() > 0. {
                    1.
                } else if delta.x.trunc() < 0. {
                    -1.
                } else {
                    0.
                };
                
                if controller.is_airborne().unwrap() {
                    controller.basis(TnuaBuiltinWalk {
                        desired_velocity: Vec3::new(80. * facing_direction, 0., 0.),
                        float_height: FLOAT_HEIGHT,
                        air_acceleration: 600.,
                        acceleration: 600.,
                        max_slope: float_consts::FRAC_PI_4,
                        ..Default::default()
                    });
                }
                
                if !animator.is_active("jump") && !controller.is_airborne().unwrap() {
                    *state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct PhaseTransition;

pub fn phase_transition_action_system(
    mut enemy_query: Query<(&Transform, &mut Animator, &mut TnuaController, &mut PhaseTwoTimer, &mut GravityScale), Without<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &PhaseTransition, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _transition, span) in &mut query {
        let _guard = span.span().enter();

        let (_actor_pos, mut animator, mut controller, mut timer, mut gravity) = enemy_query
            .get_mut(*actor)
            .expect("actor didn't have components");

        match *state {
            ActionState::Requested => {
                animator.set_trigger("jump");
                *state = ActionState::Executing;
                controller.action(TnuaBuiltinJump {
                    height: 100.,
                    ..Default::default()
                });
            }
            ActionState::Executing => {
                if !animator.in_state("Jump".to_string()) {
                    // 传送到天花板
                    timer.is_on_ceiling = true;
                    gravity.0 = -gravity.0 - 100.;
                    *state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct TeleportAttack;

pub fn teleport_attack_action_system(
    mut enemy_query: Query<(&mut Transform, &mut Animator, &mut PhaseTwoTimer, &mut GravityScale), Without<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &TeleportAttack, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _teleport, span) in &mut query {
        let _guard = span.span().enter();

        let (mut transform, mut animator, mut timer, mut gravity) = enemy_query
            .get_mut(*actor)
            .expect("actor didn't have components");

        match *state {
            ActionState::Requested => {
                println!("try hide!");
                if let Some(target_pos) = timer.teleport_position {
                    // 隐藏并传送到玩家位置
                    println!("hide!");
                    animator.set_trigger("hide");
                    transform.translation.x = target_pos.x;
                    transform.translation.y = target_pos.y;
                    gravity.0 += 110.;
                    timer.teleport_position = None;
                    *state = ActionState::Executing;
                } else {
                    *state = ActionState::Failure;
                    timer.into_inner().teleport_timer.reset();
                }
            }
            ActionState::Executing => {
                if animator.in_state("Hidden".to_string()) {
                    // 显现并攻击
                    animator.set_trigger("showup");
                } else if !animator.in_state("Attack1Prep".to_string()) && !animator.in_state("Attack1".to_string()){
                    // 攻击完成，回到天花板
                    gravity.0 -= 110.;
                    timer.into_inner().teleport_timer.reset();
                    println!("attack ok!");
                    *state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder, Reflect)]
pub struct NoticeScorer;

pub fn notice_scorer_system(
    notices: Query<&Notice>,
    mut query: Query<(&Actor, &mut Score), With<NoticeScorer>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(notice) = notices.get(*actor) {
            score.set(notice.notice / 100.);
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder, Reflect)]
pub struct HealthScorer;

pub fn health_scorer_system(
    health_states: Query<&HealthState>,
    mut query: Query<(&Actor, &mut Score), With<HealthScorer>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(health_state) = health_states.get(*actor) {
            if health_state.is_below_half() {
                score.set(0.99);
            } else {
                score.set(0.0);
            }
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder, Reflect)]
pub struct PhaseTwoScorer;

pub fn phase_two_scorer_system(
    health_states: Query<&HealthState>,
    timers: Query<&PhaseTwoTimer>,
    mut query: Query<(&Actor, &mut Score), With<PhaseTwoScorer>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let (Ok(health_state), Ok(timer)) = (health_states.get(*actor), timers.get(*actor)) {
            if health_state.is_phase_two && timer.teleport_timer.finished() {
                score.set(1.0);
            } else {
                score.set(0.0);
            }
        }
    }
}

pub struct MartialBehaviourPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for MartialBehaviourPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            notice_system.run_if(in_state(self.state.clone())),
            health_system.run_if(in_state(self.state.clone())),
            phase_two_timer_system.run_if(in_state(self.state.clone())),
        ));
        app.add_systems(
            PreUpdate,
            (
                attack1_action_system.run_if(in_state(self.state.clone())),
                attack2_action_system.run_if(in_state(self.state.clone())),
                jump_attack_action_system.run_if(in_state(self.state.clone())),
                move_to_player_action_system.run_if(in_state(self.state.clone())),
                phase_transition_action_system.run_if(in_state(self.state.clone())),
                teleport_attack_action_system.run_if(in_state(self.state.clone())),
            )
                .in_set(BigBrainSet::Actions),
        );
        app.add_systems(First, (
            notice_scorer_system.run_if(in_state(self.state.clone())),
            health_scorer_system.run_if(in_state(self.state.clone())),
            phase_two_scorer_system.run_if(in_state(self.state.clone())),
        ));
    }
}
use crate::animator::*;
use crate::enemy::fire_demon::FireDemon;
use crate::player::Player;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_tnua::builtins::*;
use bevy_tnua::math::*;
use bevy_tnua::prelude::*;
use big_brain::prelude::*;
use rand::Rng;

const WALK_SPEED: f32 = 20.0;
const NOTICED_SPEED: f32 = 40.0;
const FLOAT_HEIGHT: f32 = 78.;

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

pub fn notice_system(time: Res<Time>, mut query: Query<(&mut Notice, &Animator), With<FireDemon>>) {
    for (mut notice, animator) in &mut query {
        if animator.is_active("hit") {
            notice.notice = 100.;
        }
        if animator.get_bool("is_noticing") {
            notice.notice += notice.add_per_sec * time.delta_secs();
        } else {
            notice.notice -= notice.sub_per_sec * time.delta_secs();
        }

        if notice.notice >= 100. {
            notice.notice = 100.;
        }

        if notice.notice <= 0. {
            notice.notice = 0.
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder, Reflect)]
pub struct MoveToPlayer;

const CLOSE_MAX_DISTANCE: f32 = 80.;
const MID_MAX_DISTANCE: f32 = 100.;
const MID_FAR_MAX_DISTANCE: f32 = 200.;
const FAR_MAX_DISTANCE: f32 = 400.;

pub fn move_to_player_action_system(
    player_pos: Single<&Transform, With<Player>>,
    mut actor_query: Query<
        (
            &Transform,
            &mut TnuaController,
            &mut LinearVelocity,
            &mut Animator,
            &Notice,
        ),
        Without<Player>,
    >,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToPlayer, &ActionSpan)>,
) {
    for (Actor(actor), mut action_state, _move_to, span) in &mut action_query {
        let _guard = span.span().enter();
        let (actor_pos, mut controller, mut vel, mut animator, notice) =
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
                    if controller.is_airborne().unwrap() {
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
                } else if distance > MID_FAR_MAX_DISTANCE {
                    let vx = 4.0 * get_speed(&animator, &notice) * facing_direction;
                    if controller.is_airborne().unwrap() {
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
                    let vx = 3.0 * get_speed(&animator, &notice) * facing_direction;
                    if controller.is_airborne().unwrap() {
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
                    if controller.is_airborne().unwrap() {
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
pub struct Attack;

pub fn attack_action_system(
    mut enemy_query: Query<(&Transform, &mut Animator, &mut TnuaController), Without<Player>>,
    player_pos: Single<&Transform, With<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &Attack, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _attack, span) in &mut query {
        let _guard = span.span().enter();

        let (actor_pos, mut animator, mut controller) = enemy_query
            .get_mut(*actor)
            .expect("actor did't have notice");

        match *state {
            ActionState::Requested => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let distance = delta.length();
                if distance > FAR_MAX_DISTANCE {
                    *state = ActionState::Failure;
                    
                } else if distance > MID_MAX_DISTANCE {
                    if !controller.is_airborne().unwrap() {
                        controller.action(TnuaBuiltinJump {
                            height: 400.,
                            ..Default::default()
                        });
                        animator.set_trigger("attack");
                        *state = ActionState::Executing;
                    } else {
                        *state = ActionState::Failure;
                    }
                } else if distance > CLOSE_MAX_DISTANCE {
                    animator.set_trigger("attack");
                    *state = ActionState::Executing;
                } else {
                    let mut rng = rand::rng();
                    let random: f32 = rng.random();
                    if random > 0.5 {
                        animator.set_trigger("boom");
                        *state = ActionState::Executing;
                    } else {
                        let delta = (player_pos.translation - actor_pos.translation).truncate();
                        let facing_direction = if delta.x.trunc() > 0. {
                            1.
                        } else if delta.x.trunc() < 0. {
                            -1.
                        } else {
                            0.
                        };
                        controller.action(TnuaBuiltinJump {
                            height: 400.,
                            ..Default::default()
                        });
                        controller.basis(TnuaBuiltinWalk {
                            desired_velocity: Vec3::new(-120. * facing_direction, 0., 0.),
                            float_height: FLOAT_HEIGHT,
                            air_acceleration: 600.,
                            acceleration: 600.,
                            max_slope: float_consts::FRAC_PI_4,
                            ..Default::default()
                        });
                        *state = ActionState::Failure;
                    }
                    
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
                        desired_velocity: Vec3::new(120. * facing_direction, 0., 0.),
                        float_height: FLOAT_HEIGHT,
                        air_acceleration: 600.,
                        acceleration: 600.,
                        max_slope: float_consts::FRAC_PI_4,
                        ..Default::default()
                    });
                }
                if !animator.is_active("attack") &&
                   !animator.is_active("boom")
                 {
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
pub struct FireDemonBehaviourPlugin<S: States> {
    pub state: S
}

impl<S: States> Plugin for FireDemonBehaviourPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, notice_system.run_if(in_state(self.state.clone())));
        app.add_systems(
            PreUpdate,
            (
                attack_action_system.run_if(in_state(self.state.clone())),
                move_to_player_action_system.run_if(in_state(self.state.clone()))
            )
                .in_set(BigBrainSet::Actions),
        );
        app.add_systems(First, (notice_scorer_system.run_if(in_state(self.state.clone())), ));
    }
}

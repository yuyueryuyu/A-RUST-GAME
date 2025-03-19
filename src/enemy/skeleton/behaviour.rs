use avian2d::prelude::*;
use big_brain::prelude::*;
use bevy::prelude::*;
use crate::enemy::skeleton::Skeleton;
use crate::player::Player;
use crate::enemy::skeleton::Parameters;
use crate::animator::*;

const WALK_SPEED: f32 = 20.0;
const NOTICED_SPEED: f32 = 40.0;

fn get_speed(animator: &Animator, notice: &Notice) -> f32 {
    if animator.get_bool("can_move") {
        if notice.notice > 50. {
            return NOTICED_SPEED;
        }
        return WALK_SPEED;
    }
    0.0
}

#[derive(Component, Debug)]
pub struct Notice {
    pub add_per_sec: f32,
    pub sub_per_sec: f32,
    pub notice: f32,
}

impl Notice {
    pub fn new(notice: f32, add_per_sec: f32, sub_per_sec: f32) -> Self {
        Self { notice, add_per_sec, sub_per_sec }
    }
}

pub fn notice_system(
    time: Res<Time>,
    mut query: Query<(&mut Notice, &Parameters), With<Skeleton>>
) {
    for (mut notice, param) in &mut query {
        if param.get_bool("is_noticing") {
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

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToPlayer;

const MAX_DISTANCE: f32 = 30.;

pub fn move_to_player_action_system(
    player_pos: Single<&Transform, With<Player>>,
    mut actor_query: Query<(&Transform, &mut LinearVelocity, &mut Parameters, &Animator, &Notice), Without<Player>>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToPlayer, &ActionSpan)>,
) {

    for (Actor(actor), mut action_state, _move_to, span) in &mut action_query {
        let _guard = span.span().enter();
        let (actor_pos, mut actor_v, mut params, animator, notice) = actor_query.get_mut(*actor).expect("actor has no position");
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let facing_direction = if delta.x.trunc() > 0. {1.} else if delta.x.trunc() < 0. {-1.} else {0.};
                if facing_direction != 0. {
                    params.set_float("facing_direction", facing_direction);
                }
                let distance = delta.length();

                if distance > MAX_DISTANCE {
                    actor_v.x = get_speed(&animator, &notice) * facing_direction;
                } else {
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Attack;

pub fn attack_action_system(
    mut enemy_query: Query<(&Transform, &mut Animator), Without<Player>>,
    player_pos: Single<&Transform, With<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &Attack, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _attack, span) in &mut query {
        let _guard = span.span().enter();

        let (actor_pos, mut animator) = enemy_query.get_mut(*actor).expect("actor did't have notice");

        match *state {
            ActionState::Requested => {
                let delta = (player_pos.translation - actor_pos.translation).truncate();
                let distance = delta.length();

                if distance < MAX_DISTANCE {
                    animator.set_trigger("attack");
                    *state = ActionState::Executing;
                } else {
                    *state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                if !animator.is_active("attack") {
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
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct NoticeScorer;

pub fn thirsty_scorer_system(
    notices: Query<&Notice>,
    mut query: Query<(&Actor, &mut Score), With<NoticeScorer>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(notice) = notices.get(*actor) {
            score.set(notice.notice / 100.);
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Patrol;

pub fn patrol_action_system(
    mut enemy_query: Query<(&mut LinearVelocity, &mut Parameters, &Animator, &Notice), Without<Player>>,
    mut query: Query<(&Actor, &mut ActionState, &Patrol, &ActionSpan)>,
) {
    for (Actor(actor), mut state, _patrol, span) in &mut query {
        let _guard = span.span().enter();

        let (mut actor_v, mut params, animator, notice) = enemy_query.get_mut(*actor).expect("actor did't have notice");

        match *state {
            ActionState::Requested => {
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                let facing_direction = params.get_float("facing_direction");
                if params.get_bool("is_on_wall") && params.get_bool("is_grounded") {
                    params.set_float("facing_direction", -facing_direction);
                }
                actor_v.x = facing_direction * get_speed(&*animator, &notice);
                *state = ActionState::Success;
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct PatrolScorer;

pub fn patrol_scorer_system(
    notices: Query<&Notice>,
    mut query: Query<(&Actor, &mut Score), With<PatrolScorer>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(notice) = notices.get(*actor) {
            score.set(1. - notice.notice / 100.);
        }
    }
}
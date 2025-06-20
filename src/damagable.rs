//! 生命、受伤系统

use crate::{animator::*, save::load};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_tnua::{builtins::*, prelude::*};
use serde::{Serialize, Deserialize};

/// 生命、受伤组件
#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Damagable {
    /// 最大血量
    pub max_health: f32,
    /// 血量
    pub health: f32,
    /// 回复量
    pub healing_amount: f32,
    /// 最大架势条
    pub max_posture: f32,
    /// 架势
    pub posture: f32,
    /// 当前是否存活
    pub is_alive: bool,
    /// 是否无敌状态
    pub is_invincible: bool,
    /// 是否防守状态
    pub is_defending: bool,
    /// 防守时间
    pub defending_time: f32,
    /// 无敌时间
    pub invincibility_time: f32,
    pub time_since_hit: f32,
    pub time_since_defend: f32,
    pub time_since_death: f32,
}

impl Damagable {
    /// 新建组件
    pub fn new(max_health: f32) -> Self {
        Damagable {
            max_health: max_health.clone(),
            health: max_health.clone(),
            healing_amount: 40.,
            max_posture: 100.,
            posture: 0.,
            is_alive: true,
            is_invincible: false,
            is_defending: false,
            defending_time: 0.3,
            invincibility_time: 0.4,
            time_since_hit: 0.,
            time_since_defend: 0.,
            time_since_death: 0.,
        }
    }

    pub fn copy(&mut self, dam: Damagable) {
        self.max_health = dam.max_health;
        self.health = dam.health;
        self.healing_amount = dam.healing_amount;
        self.max_posture = dam.max_posture;
        self.posture = dam.posture;
        self.is_alive = dam.is_alive;
        self.is_invincible = dam.is_invincible;
        self.is_defending = dam.is_defending;
        self.defending_time = dam.defending_time;
        self.invincibility_time = dam.invincibility_time;
        self.time_since_hit = dam.time_since_hit;
        self.time_since_defend = dam.time_since_defend;
        self.time_since_death = dam.time_since_death;
    }

    /// 设置生命值
    pub fn set_health(&mut self, health: f32) {
        self.health = health;
        if self.health <= 0. {
            self.is_alive = false;
        }
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    /// 设置架势调
    pub fn set_posture(&mut self, posture: f32) {
        self.posture = posture;
    }

    /// 设置无敌状态
    pub fn set_invincible(&mut self, value: bool) {
        self.is_invincible = value;
        self.time_since_hit = 0.;
    }

    /// 设置无敌状态、时间
    pub fn set_invincible_with_time(&mut self, invincible_time: f32) {
        self.is_invincible = true;
        self.time_since_hit = self.invincibility_time-invincible_time;
    }

    /// 设置防守状态
    pub fn set_defending(&mut self, value: bool) {
        self.is_defending = value;
        self.time_since_defend = 0.;
    }

    /// 受到攻击
    pub fn take_hit(&mut self, damage: f32) {
        if self.is_alive && !self.is_invincible {
            self.set_invincible(true);
            if !self.is_defending {
                self.set_health(self.health - damage);
                self.set_posture(self.posture + damage / 3.);
                self.set_invincible(true);
            }
            if self.is_defending && self.time_since_defend > 0.15 {
                self.set_posture(self.posture + damage);
            }
        }

        println!("Health: {} Posture: {}", self.health, self.posture);
    }

    /// 治疗
    pub fn healing(&mut self) {
        self.set_health(self.health + self.healing_amount);
    }
}

/// 检查无敌状态
fn check_invincible(time: Res<Time>, mut query: Query<(&mut Animator, &mut Damagable)>) {
    for (mut animator, mut damagable) in &mut query {
        if !damagable.is_alive {
            animator.set_bool("is_alive", false);
        }
        if damagable.is_invincible {
            damagable.time_since_hit += time.delta_secs();
            if damagable.time_since_hit > damagable.invincibility_time {
                damagable.set_invincible(false);
            }
        }
    }
}

/// 检查防守状态
fn check_defending(time: Res<Time>, mut query: Query<&mut Damagable>) {
    for mut damagable in &mut query {
        if damagable.posture >= damagable.max_posture {
            // 破防
            damagable.set_posture(0.);
        }
        if damagable.is_defending {
            damagable.time_since_defend += time.delta_secs();
            if damagable.time_since_defend > damagable.defending_time {
                damagable.set_defending(false);
            }
        }
    }
}

/// 检查是否死亡
fn check_death(time: Res<Time>, mut query: Query<(&mut Animator, &mut Damagable, &mut Transform)>) {
    for (mut animator, mut damagable, mut transform) in &mut query {
        if !damagable.is_alive {
            damagable.time_since_death += time.delta_secs();
        }
        if damagable.time_since_death > 5. {
            let trans_data = load().unwrap();
            transform.translation.x = trans_data.translation[0];
            transform.translation.y = trans_data.translation[1];
            transform.scale.x = trans_data.scale[0];
            transform.scale.y = trans_data.scale[1];
            animator.parameters = trans_data.params.clone();
            animator.set_trigger("revival");
            damagable.copy(trans_data.damagable.clone());
        }
    }
}

/// hitbox关系
#[derive(Component)]
#[relationship(relationship_target = HasHitbox)]
pub struct HitboxOf(pub Entity);

/// hitbox关系
#[derive(Component, Deref)]
#[relationship_target(relationship = HitboxOf)]
pub struct HasHitbox(Vec<Entity>);

/// hitbox组件
#[derive(Component)]
pub struct HitBox {
    /// 伤害
    pub damage: f32,
}

/// 检查受攻击
pub fn check_hitbox(
    trigger: Trigger<OnCollisionStart>,
    hitbox_query: Query<(&GlobalTransform, &HitBox)>,
    mut damaged_query: Query<(
        &mut Damagable,
        &mut Animator,
        &mut TnuaController,
        &GlobalTransform,
    )>,
    asset_server: Res<AssetServer>, audio: Res<Audio>
) {
    let hitbox_entity = trigger.target();
    let damaged_entity = trigger.collider;
    let (hitbox_trans, hitbox) = hitbox_query.get(hitbox_entity).unwrap();
    let (mut damagable, mut animator, mut controller, damaged_trans) = damaged_query.get_mut(damaged_entity).unwrap();
    let delta_x = damaged_trans.translation().x - hitbox_trans.translation().x;
    let dir = if delta_x >= 0. { 1. } else { -1. };
    if !damagable.is_invincible && !damagable.is_defending && damagable.is_alive {
        animator.set_trigger("hit");
        controller.action(TnuaBuiltinKnockback {
            shove: Vec3::new(50., 0., 0.) * dir,
            ..Default::default()
        });
        audio.play(asset_server.load(
            "Audio/SFX/12_Player_Movement_SFX/61_Hit_03.wav"));
    } else if damagable.is_defending {
        controller.action(TnuaBuiltinKnockback {
            shove: Vec3::new(10., 0., 0.) * dir,
            ..Default::default()
        });
    }
    damagable.take_hit(hitbox.damage);
}

pub struct DamagePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for DamagePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_invincible.run_if(in_state(self.state.clone())), 
            check_defending.run_if(in_state(self.state.clone())),
            check_death.run_if(in_state(self.state.clone())),
        ));
    }
}

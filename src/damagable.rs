use crate::animator::*;
use avian2d::prelude::*;
use bevy::{prelude::*, state::commands};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_tnua::{builtins::*, prelude::*};

#[derive(Component, Debug)]
pub struct Damagable {
    pub max_health: f32,
    pub health: f32,
    pub healing_amount: f32,
    pub max_posture: f32,
    pub posture: f32,
    pub is_alive: bool,
    pub is_invincible: bool,
    pub is_defending: bool,
    pub defending_time: f32,
    pub invincibility_time: f32,
    pub time_since_hit: f32,
    pub time_since_defend: f32,
}

impl Damagable {
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
        }
    }

    pub fn set_health(&mut self, health: f32) {
        self.health = health;
        if self.health <= 0. {
            self.is_alive = false;
        }
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    pub fn set_posture(&mut self, posture: f32) {
        self.posture = posture;
    }

    pub fn set_invincible(&mut self, value: bool) {
        self.is_invincible = value;
        self.time_since_hit = 0.;
    }

    pub fn set_defending(&mut self, value: bool) {
        self.is_defending = value;
        self.time_since_defend = 0.;
    }

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

    pub fn healing(&mut self) {
        self.set_health(self.health + self.healing_amount);
    }
}

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

fn check_defending(time: Res<Time>, mut query: Query<&mut Damagable>) {
    for (mut damagable) in &mut query {
        if damagable.posture >= damagable.max_posture {
            // hitdown
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

fn check_hitbox(
    collisions: Res<Collisions>,
    hitbox_query: Query<(Entity, &Parent), With<Sensor>>,
    attacker_query: Query<&Transform>,
    mut actor_query: Query<(
        Entity,
        &mut Damagable,
        &mut Animator,
        &mut TnuaController,
        &Transform,
    )>,
    asset_server: Res<AssetServer>, audio: Res<Audio>
) {
    for (hitbox_entity, hitbox_parent) in &hitbox_query {
        for (damaged_entity, mut damagable, mut animator, mut controller, damaged_trans) in
        &mut actor_query
        {

            if let Ok(
                attacker_trans,
            ) = attacker_query.get(**hitbox_parent) { 
                if collisions.contains(damaged_entity, hitbox_entity)
                || collisions.contains(hitbox_entity, damaged_entity)
                {
                    
                    let dir = if (damaged_trans.translation - attacker_trans.translation).x >= 0. {
                        1.
                    } else {
                        -1.
                    };

                    if !damagable.is_invincible && !damagable.is_defending {
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

                    damagable.take_hit(10.);
                }

            }
            
        }
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_invincible, check_hitbox, check_defending));
    }
}

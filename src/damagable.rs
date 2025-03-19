use bevy::prelude::*;
use crate::animator::*;

#[derive(Component, Debug)]
pub struct Damagable {
    pub max_health: f32,
    pub health: f32,
    pub is_alive: bool,
    pub is_invincible: bool,
    pub invincibility_time: f32,
    pub time_since_hit: f32,
}

impl Damagable {
    pub fn new(max_health: f32) -> Self {
        Damagable {
            max_health: max_health.clone(),
            health: max_health.clone(),
            is_alive: true,
            is_invincible: false,
            invincibility_time: 0.25,
            time_since_hit: 0.,
        }
    }

    pub fn set_health(&mut self, health: f32) {
        self.health = health;
        if self.health < 0. {
            self.is_alive = false;
        } 
    } 

    pub fn set_invincible(&mut self, value: bool) {
        self.is_invincible = value;
    }

    pub fn is_invincible(&mut self) -> bool {
        self.is_invincible
    }

    pub fn take_hit(&mut self, damage: f32) {
        if self.is_alive && !self.is_invincible {
            self.set_health(self.health - damage);
            self.is_invincible = true;
            self.time_since_hit = 0.;
        }
    }
    
}

fn check_invincible(
    time: Res<Time>,
    mut query: Query<(&mut Animator, &mut Damagable)>,
) {
    for (mut animator, mut damagable) in &mut query { 
        if !damagable.is_alive {
            animator.set_bool("is_alive", false);
        }
        if damagable.is_invincible {
            damagable.time_since_hit += time.delta_secs();
            if damagable.time_since_hit > damagable.invincibility_time {
                damagable.time_since_hit = 0.;
                damagable.set_invincible(false);
            }
        }
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_invincible);
    }
}

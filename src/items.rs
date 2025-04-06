use bevy::prelude::*;

use crate::{damagable::{self, Damagable}, player::Player};

#[derive(Component)]
pub struct ItemList {
    pub items: Vec<Item>,
    pub item_now: usize,
}

impl ItemList {
    pub fn use_item(&mut self, user: Entity) {
        self.items[self.item_now].use_item(user);
    }
}

#[derive(Clone, Copy)]
pub enum ItemType {
    HealthPotion,
}

pub struct Item {
    pub item_type: ItemType,
    pub texture_path: String,
    pub nums: i32,
    pub user: Option<Entity>
}

impl Item {
    pub fn new(item_type: ItemType, path: String, nums: i32) -> Self {
        Item {
            item_type: item_type,
            texture_path: path,
            nums: nums,
            user: None
        }
    }
    pub fn use_item(&mut self, user: Entity) {
        if self.nums > 0 {
            self.user = Some(user);
        }
    }
    pub fn okay(&mut self) {
        self.nums -= 1;
        self.user = None;
    }
}

#[derive(Event)]
struct UseItemEvent {
    item_type: ItemType,
    user: Entity,     
}

fn use_item_system(
    mut ev_use_item: EventWriter<UseItemEvent>,
    item_list: Single<&mut ItemList, With<Player>>, // 监听玩家激活道具操作
) {
    let list = &mut item_list.into_inner();
    for item in &mut list.items {
        if let Some(user) = item.user {
            ev_use_item.send(UseItemEvent {
                item_type: item.item_type.clone(),
                user: user
            });
            item.okay();
        }
        
    }
}

// 事件处理系统
fn apply_item_effects(
    mut events: EventReader<UseItemEvent>,
    mut health_query: Query<&mut Damagable>,
) {
    for event in events.read() {
        match event.item_type {
            ItemType::HealthPotion => {
                if let Ok(mut damagable) = health_query.get_mut(event.user) {
                    damagable.healing();
                }
            }
        }
    }
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UseItemEvent>();
        app.add_systems(Update, (
            use_item_system, apply_item_effects
        ));
    }
}

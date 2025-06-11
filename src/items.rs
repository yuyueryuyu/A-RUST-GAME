use std::collections::HashMap;

use avian2d::prelude::{OnCollisionEnd, OnCollisionStart};
use bevy::{prelude::*, render::render_resource::encase::private::Length};
use serde::{Deserialize, Serialize};

use crate::{animator::Animator, blocks::InitialGate, damagable::{self, Damagable}, healthbar::Hint, player::Player, PausedState};
/// 完整的道具系统逻辑
/// 资源存储完整的道具信息，玩家的组件只存储道具名。

/// 使用道具时生成的道具子实体与角色的关系
#[derive(Component)]
#[relationship(relationship_target = HasItem)]
pub struct ItemOf(pub Entity);

/// 使用道具时生成的道具子实体与角色的关系
#[derive(Component, Deref)]
#[relationship_target(relationship = ItemOf)]
pub struct HasItem(Vec<Entity>);

/// 
#[derive(Component)]
#[relationship(relationship_target = NearingItem)]
pub struct ItemNear(pub Entity);

/// 
#[derive(Component, Deref)]
#[relationship_target(relationship = ItemNear)]
pub struct NearingItem(Vec<Entity>);

/// 角色道具背包
#[derive(Component, Debug)]
pub struct ItemBag {
    pub slots: HashMap<String, u32>,
}

impl ItemBag {
    pub fn get(&self, k: &String) -> u32 {
        *self.slots.get(k).unwrap_or(&0)
    }

    pub fn put(&mut self, k: String, v: u32) {
        self.slots.insert(k, v);
    }
}

/// 当前活跃的道具
#[derive(Component)]
pub struct ActiveItems {
    pub items: Vec<String>,
    pub current: usize,
}

impl ActiveItems {
    pub fn get_current_item(&self) -> String {
        self.items[self.current].clone()
    }
}

#[derive(Component)]
pub struct NotpickedItems {
    pub id: String,
    pub num: u32,
}

// 道具哈希表资源
#[derive(Resource)]
pub struct ItemList {
    pub infos: HashMap<String, ItemInfo>
}

/// 道具哈希表存储的道具详细信息，使用时，应当从道具表查找道具信息
#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub description: String,
    pub icon: Handle<Image>,
    pub max_stack: u32,
    pub item_type: ItemType,
}

/// 道具类别， 分为消耗性和能力型。
#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum ItemType {
    Consumable(ConsumableType),
    Ability(AbilityType),
}

/// 消耗性道具的类别，使用后生效
#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum ConsumableType {
    Healing,
    OpenTheDoor,
}

/// 能力型道具的类别，拾取后生效
#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum AbilityType {
    WallJump,
    ReverseGravity,
}

/// 道具使用触发器
#[derive(Event)]
pub struct UseItemTrigger {
    pub user: Entity,
    pub item: String,
}

fn use_potion_observer(
    trigger: Trigger<UseItemTrigger>,
    mut commands: Commands,
    item_list: Res<ItemList>,
    mut users: Query<(&mut Damagable, &mut ItemBag)>,
    door: Query<Entity, With<InitialGate>>,
) {
    let user = trigger.user;
    let item = &trigger.item;
    let map = &item_list.infos;
    let info = map.get(item).unwrap();
    let (mut damagable, mut bag) = users.get_mut(user).unwrap();
    let left = bag.get(item);
    if left == 0 { return; }
    bag.put(item.clone(), left-1);
    match info.item_type {
        ItemType::Consumable(it) => {
            match it {
                ConsumableType::Healing => {
                    damagable.healing();
                }
                ConsumableType::OpenTheDoor => {
                    commands.entity(door.single().unwrap()).despawn();
                }
            }
        }
        _ => {}
    }
}

pub fn item_canpick_observer(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands
) { 
    let item = trigger.target();
    let player = trigger.collider;
    commands.entity(item).insert(ItemNear(player));
}

pub fn item_cantpick_observer(
    trigger: Trigger<OnCollisionEnd>,
    mut commands: Commands,
) { 
    let item = trigger.target();
    let player = trigger.collider;
    commands.entity(item).remove_related::<ItemNear>(&[player]);
}

/// 道具拾取触发器
#[derive(Event)]
pub struct PickItemTrigger {
    pub picker: Entity,
    pub item: String,
    pub num: u32,
}

fn pick_item_observer(
    trigger: Trigger<PickItemTrigger>,
    mut commands: Commands,
    item_list: Res<ItemList>,
    mut users: Query<(&mut Animator, &mut ItemBag, &NearingItem)>,
    mut next_state: ResMut<NextState<PausedState>>,
    text: Single<&mut Text, With<Hint>>,
) {
    let user = trigger.picker;
    let item = &trigger.item;
    let map = &item_list.infos;
    let info = map.get(item).unwrap();
    if let Ok((mut animator, mut bag, nearing)) = users.get_mut(user) {
    let before_num = bag.get(item);
    if before_num + trigger.num > info.max_stack { return; }
    bag.put(item.clone(), before_num + trigger.num);
        match info.item_type {
            ItemType::Ability(it) => {
                match it {
                    AbilityType::WallJump => {
                        animator.set_bool("can_wall_jump", true);
                    }
                    AbilityType::ReverseGravity => {
                        animator.set_bool("can_reverse_gravity", true);
                    }
                }
                text.into_inner().0 = "".to_string();
                next_state.set(PausedState::GetItem);
            }
            _ => {
                for item in (**nearing).clone() {
                    commands.entity(item).despawn();
                }
            }
        }

    }
}

/// 初始化道具表
fn init_items(assets_server: Res<AssetServer>, mut item_list: ResMut<ItemList>) {
    let health_potion = ItemInfo {
        name: "Health Potion".to_string(),
        description: "The Secret Health Potion. You don't know why, but you feel better when you drink it.".to_string(),
        icon: assets_server.load("Art/Kyrise's 16x16 RPG Icon Pack - V1.3/icons/16x16/potion_02a.png"),
        max_stack: 10,
        item_type: ItemType::Consumable(ConsumableType::Healing),
    };
    item_list.infos.insert("HealthPotion".to_string(), health_potion);
    let key = ItemInfo {
        name: "Key".to_string(),
        description: "The key of your prison room. Use it to open the door!".to_string(),
        icon: assets_server.load("Art/Kyrise's 16x16 RPG Icon Pack - V1.3/icons/16x16/key_01a.png"),
        max_stack: 1,
        item_type: ItemType::Consumable(ConsumableType::OpenTheDoor),
    };
    item_list.infos.insert("Key".to_string(), key);
    let fire_glove = ItemInfo {
        name: "Fire Glove".to_string(),
        description: "Wall Jump: Press [Space] when at wall".to_string(),
        icon: assets_server.load("Art/Kyrise's 16x16 RPG Icon Pack - V1.3/icons/16x16/gloves_01e.png"),
        max_stack: 1,
        item_type: ItemType::Ability(AbilityType::WallJump),
    };
    item_list.infos.insert("FireGlove".to_string(), fire_glove);
    let martial_scroll = ItemInfo {
        name: "Martial Scroll".to_string(),
        description: "Reverse Gravity: Press [G]".to_string(),
        icon: assets_server.load("Art/Kyrise's 16x16 RPG Icon Pack - V1.3/icons/16x16/scroll_01a.png"),
        max_stack: 1,
        item_type: ItemType::Ability(AbilityType::ReverseGravity),
    };
    item_list.infos.insert("MartialScroll".to_string(), martial_scroll);
}

pub struct ItemsPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for ItemsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemList { infos: HashMap::new() });
        app.add_systems(Startup, init_items);
        app.add_observer(use_potion_observer);
        app.add_observer(pick_item_observer);
    }
}

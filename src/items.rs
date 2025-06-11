//! 完整的道具系统逻辑
//! 资源存储完整的道具信息，玩家的组件只存储道具名。
use std::collections::HashMap;
use bevy::prelude::*;
use avian2d::prelude::{OnCollisionEnd, OnCollisionStart};
use serde::{Deserialize, Serialize};

use crate::{animator::Animator, blocks::InitialGate, damagable::Damagable, healthbar::Hint, PausedState};

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
    /// 背包槽
    pub slots: HashMap<String, u32>,
}

impl ItemBag {
    /// 获取道具数量
    pub fn get(&self, k: &String) -> u32 {
        *self.slots.get(k).unwrap_or(&0)
    }
    /// 更新道具数量
    pub fn put(&mut self, k: String, v: u32) {
        self.slots.insert(k, v);
    }
}

/// 当前装备的道具
#[derive(Component)]
pub struct ActiveItems {
    /// 装备道具列表
    pub items: Vec<String>,
    /// 当前选中道具
    pub current: usize,
}

impl ActiveItems {
    /// 获取当前选择的道具
    pub fn get_current_item(&self) -> String {
        self.items[self.current].clone()
    }
}

/// 没有被拾取的道具的标识组件
#[derive(Component)]
pub struct NotpickedItems {
    /// 道具id
    pub id: String,
    /// 道具数量
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
    /// 道具名
    pub name: String,
    /// 道具描述
    pub description: String,
    /// 道具图标
    pub icon: Handle<Image>,
    /// 道具最多存放数
    pub max_stack: u32,
    /// 道具类别
    pub item_type: ItemType,
}

/// 道具类别， 分为消耗性和能力型。
#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum ItemType {
    /// 消耗性道具，需要使用触发能力
    Consumable(ConsumableType),
    /// 能力型道具，拾取即可获得能力
    Ability(AbilityType),
}

/// 消耗性道具的类别，使用后生效
#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum ConsumableType {
    /// 回血
    Healing,
    /// 开门
    OpenTheDoor,
}

/// 能力型道具的类别，拾取后生效
#[derive(Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum AbilityType {
    /// 蹬墙跳
    WallJump,
    /// 重力反转
    ReverseGravity,
}

/// 道具使用触发器
#[derive(Event)]
pub struct UseItemTrigger {
    /// 使用者
    pub user: Entity,
    /// 道具ID
    pub item: String,
}

/// 使用道具的观察者触发系统
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
        // 如果是消耗性道具
        ItemType::Consumable(it) => {
            match it {
                // 回血
                ConsumableType::Healing => {
                    damagable.healing();
                }
                // 开门
                ConsumableType::OpenTheDoor => {
                    commands.entity(door.single().unwrap()).despawn();
                }
            }
        }
        // 否则无效
        _ => {}
    }
}

/// 设置道具为可拾取的观察者系统
/// 当玩家进入道具Sensor的时候，设置道具和玩家具有ItemNear关系
pub fn item_canpick_observer(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands
) { 
    let item = trigger.target();
    let player = trigger.collider;
    commands.entity(item).insert(ItemNear(player));
}

/// 设置道具为不可拾取的观察者系统
/// 当玩家退出道具Sensor的时候，取消道具和玩家实体的关系
pub fn item_cantpick_observer(
    trigger: Trigger<OnCollisionEnd>,
    mut commands: Commands,
) { 
    let item = trigger.target();
    let player = trigger.collider;
    commands.entity(item).remove::<ItemNear>();
    commands.entity(player).remove::<NearingItem>();
}

/// 道具拾取触发器
#[derive(Event)]
pub struct PickItemTrigger {
    /// 拾取者
    pub picker: Entity,
    /// 拾取道具ID
    pub item: String,
    /// 拾取数量
    pub num: u32,
}

/// 拾取道具观察者系统
fn pick_item_observer(
    trigger: Trigger<PickItemTrigger>,
    mut commands: Commands,
    item_list: Res<ItemList>,
    mut users: Query<(&mut Animator, &mut ItemBag, &NearingItem, &mut ActiveItems)>,
    mut next_state: ResMut<NextState<PausedState>>,
    text: Single<&mut Text, With<Hint>>,
) {
    let user = trigger.picker;
    let item = &trigger.item;
    let map = &item_list.infos;
    let info = map.get(item).unwrap();
    if let Ok((mut animator, mut bag, nearing, mut acts)) = users.get_mut(user) {
        let before_num = bag.get(item);
        // 如果超过最大可存放数量，则不能拾取
        if before_num + trigger.num > info.max_stack { return; }
        // 把道具放进背包
        bag.put(item.clone(), before_num + trigger.num);
        match info.item_type {
            // 能力型道具
            ItemType::Ability(it) => {
                match it {
                    // 蹬墙跳
                    AbilityType::WallJump => {
                        animator.set_bool("can_wall_jump", true);
                    }
                    // 反转重力
                    AbilityType::ReverseGravity => {
                        animator.set_bool("can_reverse_gravity", true);
                    }
                }
                text.into_inner().0 = "".to_string();
                // 转换为获取能力型道具特殊UI状态
                next_state.set(PausedState::GetItem);
            }
            // 消耗性道具
            _ => {
                for item in (**nearing).clone() {
                    commands.entity(item).despawn();
                }
                if !acts.items.contains(item) {
                    acts.items.push(item.clone());
                }
                text.into_inner().0 = "".to_string();
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

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemList { infos: HashMap::new() });
        app.add_systems(Startup, init_items);
        app.add_observer(use_potion_observer);
        app.add_observer(pick_item_observer);
    }
}

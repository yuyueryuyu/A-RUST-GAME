//! 碰撞体图层逻辑
use avian2d::prelude::*;

/// 碰撞体图层，通过设置处理两个碰撞体的碰撞逻辑
#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    /// 默认碰撞体Layer
    #[default]
    Default, 
    /// 玩家hitbox
    PlayerHitBox,
    /// 敌人hitbox
    EnemyHitBox,
    /// 玩家
    Player,  
    /// 敌人
    Enemy,  
    /// 感知器
    Sensor,
    /// 地面
    Ground, 
}

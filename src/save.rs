//! 存档/读档功能

use bevy::{prelude::*, scene::ron};
use moonshine_save::prelude::*;

use std::{collections::{HashMap}, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use crate::{animator::{Animator, AnimatorParam}, damagable::Damagable, player::Player};

/// 存档事件
#[derive(Event)]
pub struct SaveRequest {
    /// 存档路径
    path: PathBuf,
}

impl GetFilePath for SaveRequest {
    /// 获取存档路径
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

/// 读档事件
#[derive(Event)]
pub struct LoadRequest {
    /// 读档路径 
    path: PathBuf,
}

impl GetFilePath for LoadRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

/// 存档数据
#[derive(Serialize, Deserialize, Resource)]
pub struct TransformData {
    pub translation: [f32; 3],
    pub rotation: [f32; 4], 
    pub scale: [f32; 3],
    pub params: HashMap<String, AnimatorParam>,
    pub damagable: Damagable,
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            translation: [120.0,
            44.25385,
            0.0],
            rotation: [0.0,
            0.0,
            -0.0,
            1.0,],
            scale:[1.0, 1.0, 1.0],
            params: HashMap::new(),
            damagable: Damagable::new(150.),
        }
    }
}

/// 触发存档事件
pub fn _trigger_save(mut events: EventWriter<SaveRequest>) {
    events.write(SaveRequest { path: "saved.ron".into() });
}

/// 触发读档事件
pub fn _trigger_load(mut events: EventWriter<LoadRequest>) {
    events.write(LoadRequest { path: "saved.ron".into() });
}

/// 存档
pub fn save(player: Single<(&Transform, &Animator, &Damagable), With<Player>>) {
    let (transform, animator, dam) = player.into_inner();
    let transform_data = TransformData {
        translation: [
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
        ],
        rotation: [
            transform.rotation.x,
            transform.rotation.y,
            transform.rotation.z,
            transform.rotation.w,
        ],
        scale: [transform.scale.x, transform.scale.y, transform.scale.z],
        params: animator.parameters.clone(),
        damagable: dam.clone(),
    };
    let config = ron::ser::PrettyConfig::default()
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        
    let ron_string = ron::ser::to_string_pretty(&transform_data, config).unwrap();
    
    let mut file = File::create("save.ron").unwrap();
    file.write_all(ron_string.as_bytes()).unwrap();
}

/// 读档
pub fn load() -> Option<TransformData> {
    let mut file = match File::open("save.ron") {
        Ok(file) => file,
        Err(_) => {
            println!("Could not open scene.ron");
            return None;
        }
    };
    
    // 读取文件内容
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        println!("Could not read scene.ron");
        return None;
    }
    
    // 反序列化数据
    let transform: TransformData = match ron::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to parse scene.ron: {}", e);
            return None;
        }
    };
    
    Some(transform)
}

/// 存档插件
pub struct SavingPlugin;

impl Plugin for SavingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugin)
        .add_plugins(LoadPlugin)
        .insert_resource(TransformData::default())
        .register_type::<Player>()
        .add_event::<SaveRequest>()
        .add_event::<LoadRequest>()
        .add_systems(PreUpdate, save.run_if(should_save));
    }
}

/// 仅当要求存档的时候进行存档
fn should_save(events: EventReader<SaveRequest>) -> bool {
    !events.is_empty()
}
use bevy::{asset::StrongHandle, prelude::*, scene::ron};
use moonshine_save::prelude::*;

use std::{collections::{HashMap, HashSet}, path::{Path, PathBuf}, sync::Arc};
use bevy::prelude::*;
use moonshine_save::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use crate::{animator::{Animator, AnimatorParam}, damagable::Damagable, player::Player};

// Save request with a dynamic path
#[derive(Event)]
pub struct SaveRequest {
    path: PathBuf,
}

impl GetFilePath for SaveRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

// Load request with a dynamic path
#[derive(Event)]
pub struct LoadRequest {
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Resource)]
pub struct TransformData {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],  // 四元数 (x, y, z, w)
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
            damagable: Damagable::new(100.),
        }
    }
}

impl GetFilePath for LoadRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

pub fn trigger_save(mut events: EventWriter<SaveRequest>) {
    events.send(SaveRequest { path: "saved.ron".into() });
}

pub fn trigger_load(mut events: EventWriter<LoadRequest>) {
    events.send(LoadRequest { path: "saved.ron".into() });
}

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

fn should_save(events: EventReader<SaveRequest>) -> bool {
    !events.is_empty()
}
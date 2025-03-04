use bevy::prelude::*;
use std::collections::HashMap;

const PARAMETERS_INITIALS: [(&str, ValueType); 5] = [
    ("is_moving", ValueType::Boolean(false)),
    ("is_grounded", ValueType::Boolean(true)),
    ("is_on_wall", ValueType::Boolean(false)),
    ("is_on_ceiling", ValueType::Boolean(false)),
    ("facing_direction", ValueType::Float(1.0)),
];

enum ValueType {
    Float(f32),
    Integer(i32),
    Boolean(bool)
}

#[derive(Component)]
pub struct Parameters {
    map: HashMap<&'static str, ValueType>,
}

impl Parameters {
    pub fn new() -> Self {
        return Self {
            map: PARAMETERS_INITIALS.into_iter().collect(),
        }
    }

    pub fn set_bool(&mut self, animation_string: &'static str, value: bool) {
        if let Some(val) = self.map.get(animation_string) {
            if let ValueType::Boolean(_bool_val) = val {
                self.map.insert(animation_string, ValueType::Boolean(value));
            } else {
                println!("warning: Value for '{}' is not a boolean.", animation_string);
            }
        } else {
            println!("warning: '{}' not found in the map.", animation_string);
        }
    }

    pub fn set_float(&mut self, animation_string: &'static str, value: f32) {
        if let Some(val) = self.map.get(animation_string) {
            if let ValueType::Float(_float_val) = val {
                self.map.insert(animation_string, ValueType::Float(value));
            } else {
                println!("warning: Value for '{}' is not a float.", animation_string);
            }
        } else {
            println!("warning: '{}' not found in the map.", animation_string);
        }
    }

    pub fn set_int(&mut self, animation_string: &'static str, value: i32) {
        if let Some(val) = self.map.get(animation_string) {
            if let ValueType::Integer(_int_val) = val {
                self.map.insert(animation_string, ValueType::Integer(value));
            } else {
                println!("warning: Value for '{}' is not an integer.", animation_string);
            }
        } else {
            println!("warning: '{}' not found in the map.", animation_string);
        }
    }

    pub fn get_bool(&self, animation_string: &'static str) -> bool {
        if let Some(val) = self.map.get(animation_string) {
            if let ValueType::Boolean(bool_val) = val {
                return *bool_val;
            } else {
                println!("warning: Value for '{}' is not a boolean.", animation_string);
            }
        } else {
            println!("warning: '{}' not found in the map.", animation_string);
        }
        false
    }

    pub fn get_float(&self, animation_string: &'static str) -> f32 {
        if let Some(val) = self.map.get(animation_string) {
            if let ValueType::Float(float_val) = val {
                return *float_val;
            } else {
                println!("warning: Value for '{}' is not a float.", animation_string);
            }
        } else {
            println!("warning: '{}' not found in the map.", animation_string);
        }
        0.0
    }

    pub fn get_int(&self, animation_string: &'static str) -> i32 {
        if let Some(val) = self.map.get(animation_string) {
            if let ValueType::Integer(int_val) = val {
                return *int_val;
            } else {
                println!("warning: Value for '{}' is not an integer.", animation_string);
            }
        } else {
            println!("warning: '{}' not found in the map.", animation_string);
        }
        0
    }
}

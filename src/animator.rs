use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance};
use serde::{Deserialize, Serialize};
use std::collections::{
    HashMap, HashSet
};
use std::time::Duration;

// 动画参数类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimatorParam {
    Bool(bool),
    Int(i32),
    Float(f32),
    Trigger(bool),
}

// 条件比较操作符
#[derive(Debug, Clone, Copy)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

// 转换条件
#[derive(Debug, Clone)]
pub struct Condition {
    pub param_name: String,
    pub operator: ConditionOperator,
    pub value: AnimatorParam,
}

impl Condition {
    pub fn new(param: String, op: ConditionOperator, value: AnimatorParam) -> Self {
        Self {
            param_name: param,
            operator: op,
            value: value,
        }
    }
}

// 状态转换
#[derive(Debug, Clone)]
pub struct Transition {
    pub conditions: Vec<Condition>,
    pub target_state: String,
    pub has_exit_time: bool,
    pub exit_time: f32, // 0.0 - 1.0 范围的归一化时间
}

// 动画状态
#[derive(Debug, Clone)]
pub struct AnimationState {
    pub name: String,
    pub first_index: usize,
    pub last_index: usize,
    pub transitions: Vec<Transition>,
    pub loop_animation: bool,
    pub on_enter: Option<fn(&mut Commands, Entity)>, // 进入状态时的回调
    pub on_exit: Option<fn(&mut Commands, Entity)>,  // 退出状态时的回调
    pub audio_path: Option<String>,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            first_index: 0,
            last_index: 0,
            transitions: vec![],
            loop_animation: false,
            on_enter: None,
            on_exit: None,
            audio_path: None,
        }
    }
}

// Animator组件
#[derive(Component, Debug)]
pub struct Animator {
    states: HashMap<String, AnimationState>,
    pub parameters: HashMap<String, AnimatorParam>,
    current_state: String,
    target_state: Option<String>,
    pub first_index: usize,
    last_index: usize,
    fps: u8,
    frame_timer: Timer,
    pub normalized_time: f32,
    active_triggers: HashSet<String>, // 当前激活的trigger集合
    consumed_triggers: HashSet<String>, // 已消费的trigger集合
}

impl Animator {
    pub fn new() -> Self {
        Animator {
            states: HashMap::new(),
            parameters: HashMap::new(),
            current_state: String::new(),
            target_state: None,
            first_index: 0,
            last_index: 0,
            fps: 0,
            frame_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            normalized_time: 0.0,
            active_triggers: HashSet::new(),
            consumed_triggers: HashSet::new(),
        }
    }

    pub fn with_params(mut self, params: HashMap<String, AnimatorParam>) -> Self {
        self.parameters = params;
        self
    }

    // 添加状态
    pub fn add_state(&mut self, state: AnimationState) {
        let name = state.name.clone();
        // 如果这是第一个状态，设为当前状态
        if self.states.is_empty() {
            self.current_state = name.clone();
        }
        self.states.insert(name, state);
    }

    pub fn get_state(&self, state_name: &str) -> &AnimationState {
        self.states.get(state_name).expect("don't have this state")
    }

    // 设置初始状态
    pub fn set_initial_state(&mut self, state_name: &str, first_index: usize, last_index: usize, fps: u8) {
        if self.states.contains_key(state_name) {
            self.current_state = state_name.to_string();
            self.first_index = first_index;
            self.last_index = last_index;
            self.fps = fps;
            self.frame_timer = self.timer_from_fps();
        }
    }

    // 添加参数
    pub fn add_parameter(&mut self, name: &str, param: AnimatorParam) {
        self.parameters.insert(name.to_string(), param);
    }

    // 设置Bool参数
    pub fn set_bool(&mut self, name: &str, value: bool) {
        if let Some(param) = self.parameters.get_mut(name) {
            if let AnimatorParam::Bool(_) = param {
                *param = AnimatorParam::Bool(value);
            }
        }
    }

    pub fn get_bool(&self, name: &str) -> bool {
        if let Some(param) = self.parameters.get(name) {
            if let AnimatorParam::Bool(bool_val) = param {
                return *bool_val;
            }
        }
        false
    }

    // 设置Int参数
    pub fn set_int(&mut self, name: &str, value: i32) {
        if let Some(param) = self.parameters.get_mut(name) {
            if let AnimatorParam::Int(_) = param {
                *param = AnimatorParam::Int(value);
            }
        }
    }

    pub fn get_int(&self, name: &str) -> i32{
        if let Some(param) = self.parameters.get(name) {
            if let AnimatorParam::Int(int_val) = param {
                return *int_val;
            }
        }
        0
    }

    // 设置Float参数
    pub fn set_float(&mut self, name: &str, value: f32) {
        if let Some(param) = self.parameters.get_mut(name) {
            if let AnimatorParam::Float(_) = param {
                *param = AnimatorParam::Float(value);
            }
        }
    }

    pub fn get_float(&self, name: &str) -> f32{
        if let Some(param) = self.parameters.get(name) {
            if let AnimatorParam::Float(float_val) = param {
                return *float_val;
            }
        }
        0.0
    }

    // 设置Trigger参数
    pub fn set_trigger(&mut self, name: &str) {
        if let Some(param) = self.parameters.get_mut(name) {
            if let AnimatorParam::Trigger(_) = param {
                *param = AnimatorParam::Trigger(true);
                self.active_triggers.insert(name.to_string());
            }
        }
    }

    pub fn is_active(&self, name: &str) -> bool {
        self.active_triggers.contains(name)
    }

    // 重置Trigger
    fn reset_trigger(&mut self, name: &str) {
        if let Some(param) = self.parameters.get_mut(name) {
            if let AnimatorParam::Trigger(_) = param {
                *param = AnimatorParam::Trigger(false);
                self.active_triggers.remove(name);
            }
        }
    }

    // 检查条件是否满足
    fn check_condition(&self, condition: &Condition) -> bool {
        if let Some(param) = self.parameters.get(&condition.param_name) {
            match (param, &condition.value) {
                (AnimatorParam::Bool(current), AnimatorParam::Bool(target)) => {
                    match condition.operator {
                        ConditionOperator::Equals => *current == *target,
                        ConditionOperator::NotEquals => *current != *target,
                        _ => false, // Bool只支持相等和不相等
                    }
                }
                (AnimatorParam::Int(current), AnimatorParam::Int(target)) => {
                    match condition.operator {
                        ConditionOperator::Equals => *current == *target,
                        ConditionOperator::NotEquals => *current != *target,
                        ConditionOperator::Greater => *current > *target,
                        ConditionOperator::Less => *current < *target,
                        ConditionOperator::GreaterOrEqual => *current >= *target,
                        ConditionOperator::LessOrEqual => *current <= *target,
                    }
                }
                (AnimatorParam::Float(current), AnimatorParam::Float(target)) => {
                    match condition.operator {
                        ConditionOperator::Equals => (*current - *target).abs() < f32::EPSILON,
                        ConditionOperator::NotEquals => (*current - *target).abs() >= f32::EPSILON,
                        ConditionOperator::Greater => *current > *target,
                        ConditionOperator::Less => *current < *target,
                        ConditionOperator::GreaterOrEqual => *current >= *target,
                        ConditionOperator::LessOrEqual => *current <= *target,
                    }
                }
                (AnimatorParam::Trigger(_), AnimatorParam::Trigger(target)) => {
                    if *target {
                        // 检查trigger是否被激活
                        self.active_triggers.contains(&condition.param_name)
                    } else {
                        // 检查trigger是否未被激活
                        !self.active_triggers.contains(&condition.param_name)
                    }
                }
                _ => false, // 类型不匹配
            }
        } else {
            false // 参数不存在
        }
    }

    // 更新动画状态
    pub fn update(
        &mut self, commands: &mut Commands, 
        entity: Entity, delta: Duration, atlas: &mut TextureAtlas,
    ) -> bool {

        let target_clone = self.target_state.clone();

        if let Some(target) = target_clone.as_ref() {
            self.frame_timer.tick(delta);

            if let Some(state) = self.states.get(&self.current_state) {
                if let Some(on_exit) = state.on_exit {
                    on_exit(commands, entity);
                }
            }

            // 切换到目标状态
            self.current_state = target.clone();
            self.target_state = None;
            self.normalized_time = 0.0;
            self.frame_timer = self.timer_from_fps();
            if let Some(state) = self.states.get(&self.current_state) {
                self.first_index = state.first_index;
                self.last_index = state.last_index;
            }
            atlas.index = self.first_index;

            if let Some(state) = self.states.get(&self.current_state) {
                if let Some(on_enter) = state.on_enter {
                    on_enter(commands, entity);
                }
            }
            
            // 重置所有已消费的triggers
            let triggers: Vec<_> = self.consumed_triggers.iter().cloned().collect();

            for trigger in &triggers {
                self.reset_trigger(trigger);
            }
            self.consumed_triggers.clear();
            
            
            return true;
        }

        let mut result = false;
        let states_clone = self.states.clone();
        
        // 更新当前状态的归一化时间
        if let Some(state) = states_clone.get(&self.current_state) {
            // 计算新的归一化时间
            self.frame_timer.tick(delta);
            
            // 如果是循环动画且完成一个周期，重置归一化时间
            if self.frame_timer.just_finished() {
                let fps = self.fps as f32;
                let total_frames = (self.last_index - self.first_index + 1) as f32;

                self.normalized_time += (self.frame_timer.elapsed_secs()) * fps / total_frames;
                self.frame_timer = self.timer_from_fps();

                if state.loop_animation {
                    self.normalized_time %= 1.0; 
                }

                if atlas.index == self.last_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    if self.normalized_time < 1.0 {
                        atlas.index = self.first_index;
                        result = true;
                    }
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
            }
            
            // 检查是否有满足条件的转换
            for transition in &state.transitions {

                // 检查退出时间
                if transition.has_exit_time && self.normalized_time < transition.exit_time {
                    continue; // 还没到退出时间
                }
                
                // 检查所有条件是否满足
                let all_conditions_met = transition.conditions.iter()
                    .all(|condition| self.check_condition(condition));
                
                if all_conditions_met {
                    // 执行转换
                    self.target_state = Some(transition.target_state.clone());
                    
                    // 记录已消费的triggers
                    for condition in &transition.conditions {
                        if let Some(AnimatorParam::Trigger(_)) = self.parameters.get(&condition.param_name) {
                            self.consumed_triggers.insert(condition.param_name.clone());
                        }
                    }
                    
                    break; // 只执行第一个满足条件的转换
                }
            }
        }
        return result;
    }

    pub fn get_cur_audio(&self) -> Option<String> {
        if let Some(state) = self.states.get(&self.current_state) {
            return state.audio_path.clone();
        }
        return None;
    }

    pub fn timer_from_fps(&mut self) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (self.fps as f32)), TimerMode::Once)
    }
    
}

fn update_animators(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>,
    mut query: Query<(Entity, &mut Animator, &mut Sprite)>,
) {
    for (entity, mut animator, mut sprite) in &mut query {
        // 更新动画状态机
        if let Some(atlas) = &mut sprite.texture_atlas {
            let result = animator.update(&mut commands, entity, time.delta(), atlas);
            if !result {
                continue;
            }
            if let Some(path) = animator.get_cur_audio() {
                audio.play(asset_server.load(path));
            }
        }

    }
}


// 将系统添加到Bevy的App
pub struct AnimatorPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for AnimatorPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, 
            update_animators.run_if(in_state(self.state.clone()))
        );
    }
}

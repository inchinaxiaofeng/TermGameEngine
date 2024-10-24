use crate::tge::scene::Scene;
use std::collections::HashMap;

pub struct Game {
    pub scenes: HashMap<String, Scene>, // 所有场景
    pub current_scene: Option<String>,  // 当前场景
}

impl Game {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
        }
    }

    pub fn start(&mut self) {
        if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get_mut(scene_name) {
                scene.start(); // 启动当前场景
            }
        }
    }

    pub fn switch_scene(&mut self, scene_name: String) {
        if let Some(current) = &self.current_scene {
            if let Some(scene) = self.scenes.get_mut(current) {
                scene.save(); // 保存当前场景资源
            }
        }
        self.current_scene = Some(scene_name);
        self.start(); // 启动新场景
    }
}

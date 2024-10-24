use crate::tge::{canvas::Canvas, renderer::Renderer, resource_manager::ResourceManager};

// 定义用户场景接口
pub trait UserScene {
    fn start(&mut self);
    fn update(&mut self);
}

pub struct Scene {
    pub name: String,
    pub resource_manager: ResourceManager, // 场景资源管理器
    pub canvas: Option<Canvas>,            // 当前画布
    pub renderer: Renderer,                // 统一渲染组件
    pub user_logic: Box<dyn UserScene>,    // 用户自定义逻辑
}

impl Scene {
    pub fn new(name: &str, user_logic: Box<dyn UserScene>) -> Self {
        Self {
            name: name.to_string(),
            resource_manager: ResourceManager::new(),
            canvas: Some(Canvas::new(80, 24, 1)), // 创建默认画布
            renderer: Renderer::new(),
            user_logic,
        }
    }

    pub fn start(&mut self) {
        // 初始化场景逻辑
        self.resource_manager.load_resources(); // 加载资源
        self.user_logic.start(); // 调用用户自定义的start方法
    }

    pub fn update(&mut self) {
        // 更新场景逻辑
        self.user_logic.update(); // 调用用户定义的update方法
        if let Some(canvas) = self.canvas.as_mut() {
            canvas.render(&mut self.renderer); // 渲染画布
        }
        self.renderer.render(); // 执行渲染
    }

    pub fn save(&mut self) {
        // 保存场景资源到全局
        self.resource_manager.save_resources();
    }
}

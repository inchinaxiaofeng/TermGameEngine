pub mod canvas;
pub mod entity;
pub mod game;
pub mod renderer;
pub mod resource_manager;
pub mod scene;
pub mod window; // 渲染组件

pub trait Renderable {
    fn render(&self, stdout: &mut std::io::Stdout);
}

pub trait Updateable {
    fn update(&mut self);
}

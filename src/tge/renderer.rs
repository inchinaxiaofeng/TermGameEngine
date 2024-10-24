use crate::tge::window::Window;

pub struct Renderer {
    render_queue: Vec<String>, // 渲染队列
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            render_queue: Vec::new(),
        }
    }

    pub fn queue_render(&mut self, window: &Window) {
        let render_output = window.render(); // 调用窗口的渲染逻辑
        self.render_queue.push(render_output); // 将渲染输出加入队列
    }

    pub fn render(&mut self) {
        // 执行渲染队列中的所有渲染任务
        for output in &self.render_queue {
            println!("{}", output); // 打印渲染结果
        }
        self.render_queue.clear(); // 清空渲染队列
    }
}

use crate::tge::game::Game;
use crate::tge::scene::Scene;
use crate::tge::scene::UserScene;
use crate::tge::window::Window;

struct DemoLogic {
    score: u32,
}

impl UserScene for DemoLogic {
    fn start(&mut self) {
        self.score = 0; // 初始化分数
        println!("Demo Scene Started!");
    }

    fn update(&mut self) {
        self.score += 1; // 模拟分数增加
        println!("Score: {}", self.score);
    }
}

pub fn load_scene(game: &mut Game) {
    let user_logic = Box::new(DemoLogic { score: 0 });
    let mut demo_scene = Scene::new("demo", user_logic);

    // 创建窗口并添加到画布
    let window1 = Window::new("Main Window", "Welcome to the Demo Game!", (1, 1), (40, 10));
    let window2 = Window::new("Score", "Score: 0", (1, 12), (20, 5));
    let window3 = Window::new("Help", "Press 'q' to exit", (1, 18), (30, 5));

    // 创建画布并添加窗口
    let mut canvas = demo_scene.canvas.take().unwrap();
    canvas.windows.push(window1);
    canvas.windows.push(window2);
    canvas.windows.push(window3);
    demo_scene.canvas = Some(canvas);

    game.scenes.insert("demo".to_string(), demo_scene);
    game.current_scene = Some("demo".to_string());
}

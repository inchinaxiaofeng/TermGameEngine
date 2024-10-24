/// Term Game Engine
mod tge; // 引入核心引擎模块
mod user; // 引入用户自定义模块

use crossterm::terminal;
use std::io::{self, Write};
use tge::game::Game;

fn main() {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();

    // 创建游戏
    let mut game = Game::new();

    // 加载用户自定义场景
    user::demo_scene::load_scene(&mut game);

    game.start(); // 启动游戏

    terminal::disable_raw_mode().unwrap();
}

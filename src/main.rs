// [TRANSLATION_NOTE]: main.cpp -> Rust 入口
// PvZ Portable 游戏主入口

mod const_enums;
mod game_constants;
mod sexy_app_framework;
mod sexy_tod_lib;
mod lawn;
mod lawn_app;

use lawn_app::{LawnApp, G_LAWN_APP};

fn main() {
    println!("RustPVZ - Plants vs. Zombies Portable (Rust Translation)");

    // 初始化全局应用实例
    let app = Box::into_raw(Box::new(LawnApp::new()));
    unsafe {
        G_LAWN_APP = app;
        // 初始化游戏子系统
        // mApp->Init();
        // mApp->mWidgetManager->Init();
        // mApp->mResourceManager->Init();
        // mApp->mMusic->Init();
        // mApp->mSoundManager->Init();
        println!("LawnApp initialized.");
        println!("Graphics, sound, and input systems require SDL2 bindings.");
        println!("Game logic modules are ready for integration.");
    }
}

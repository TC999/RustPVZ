// [TRANSLATION_NOTE]: main.cpp -> Rust 入口
// PvZ Portable 游戏主入口
// C++ 代码保真翻译 — 允许 C++ 风格的命名约定
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

mod const_enums;
mod game_constants;
mod sexy_app_framework;
mod sexy_tod_lib;
mod lawn;
mod lawn_app;

use lawn_app::{LawnApp, G_LAWN_APP};

fn main() {
    println!("RustPVZ - Plants vs. Zombies Portable (Rust Translation)");

    // 初始化全局应用实例 (对应 C++: gLawnApp = new LawnApp())
    let app = Box::into_raw(Box::new(LawnApp::new()));
    unsafe {
        G_LAWN_APP = app;

        // C++ main.cpp 初始化序列
        // 1. 设置参数字符串颜色 (C++: TodStringListSetColors)
        // 2. 注册全局函数指针
        //    gGetCurrentLevelName = LawnGetCurrentLevelName;
        //    gAppCloseRequest = LawnGetCloseRequest;
        //    gAppHasUsedCheatKeys = LawnHasUsedCheatKeys;

        // 3. 应用初始化 (C++: gLawnApp->SetArgs(argc, argv))
        // 4. 初始化 (C++: gLawnApp->Init())
        //    - 设置窗口
        //    - 初始化资源管理器
        //    - 加载资源
        //    - 初始化音乐/音效

        // 5. 启动 (C++: gLawnApp->Start())
        //    - 显示启动画面
        //    - 进入游戏选择界面

        // 6. 游戏循环由 SDL2 事件循环驱动
        //    [TODO]: 实现 SDL2 事件循环 + Update/Draw 调度

        // 7. 关闭 (C++: gLawnApp->Shutdown(); delete gLawnApp)
        println!("LawnApp initialized. Ready for SDL2 event loop.");
    }
}

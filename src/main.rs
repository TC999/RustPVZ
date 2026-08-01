// [TRANSLATION_NOTE]: main.cpp -> Rust 入口
// PvZ Portable 游戏主入口 — C++ 代码保真翻译

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_variables, unused_assignments, static_mut_refs)]

mod const_enums;
mod game_constants;
mod sexy_app_framework;
mod sexy_tod_lib;
mod lawn;
mod lawn_app;

use lawn_app::{LawnApp, G_LAWN_APP};
use crate::sexy_tod_lib::tod_string_file::{get_lawn_string_formats, tod_string_list_set_colors, get_tod_string_format_count};
use crate::sexy_app_framework::misc::resource_manager::ResourceManager;
use crate::sexy_app_framework::resources::extract_resources_by_name_callback;
use std::ffi::CString;

// =========================================================================
// 全局函数指针 (C++: gGetCurrentLevelName, gAppCloseRequest, gAppHasUsedCheatKeys, gExtractResourcesByName)
// =========================================================================
pub type GetCurrentLevelNameFn = unsafe fn() -> String;
pub type CloseRequestFn = unsafe fn() -> bool;
pub type HasUsedCheatKeysFn = unsafe fn() -> bool;
pub type ExtractResourcesByNameFn = unsafe fn(*mut ResourceManager, *const std::os::raw::c_char) -> bool;

pub static mut G_GET_CURRENT_LEVEL_NAME: Option<GetCurrentLevelNameFn> = None;
pub static mut G_APP_CLOSE_REQUEST: Option<CloseRequestFn> = None;
pub static mut G_APP_HAS_USED_CHEAT_KEYS: Option<HasUsedCheatKeysFn> = None;
pub static mut G_EXTRACT_RESOURCES_BY_NAME: Option<ExtractResourcesByNameFn> = None;

// =========================================================================
// C++ main() 的 Rust 翻译
// =========================================================================
fn main() {
    // C++: BuildUtf8ArgsFromWin32(argc, argv) — Windows 平台 UTF-8 参数转换
    // Rust 的 std::env::args_os() 已返回平台原生编码，不需要额外转换

    // C++: TodStringListSetColors(gLawnStringFormats, gLawnStringFormatCount)
    // 设置字符串格式颜色
    let formats = get_lawn_string_formats();
    let count = get_tod_string_format_count();
    // formats 是 &'static [TodStringListFormat; 12]，我们强制转为 *mut
    let formats_ptr = formats.as_ptr() as *mut _;
    tod_string_list_set_colors(formats_ptr, count);

    // C++: gGetCurrentLevelName = LawnGetCurrentLevelName;
    // C++: gAppCloseRequest = LawnGetCloseRequest;
    // C++: gAppHasUsedCheatKeys = LawnHasUsedCheatKeys;
    // C++: gExtractResourcesByName = Sexy::ExtractResourcesByName;
    unsafe {
        G_GET_CURRENT_LEVEL_NAME = Some(lawn_app::LawnGetCurrentLevelName);
        G_APP_CLOSE_REQUEST = Some(lawn_app::LawnGetCloseRequest);
        G_APP_HAS_USED_CHEAT_KEYS = Some(lawn_app::LawnHasUsedCheatKeys);
        G_EXTRACT_RESOURCES_BY_NAME = Some(extract_resources_by_name_callback);
    }

    // C++: gLawnApp = new LawnApp();
    let app = Box::into_raw(Box::new(LawnApp::new()));

    unsafe {
        G_LAWN_APP = app;

        // 收集命令行参数 (C++: gLawnApp->SetArgs(argc, argv))
        let args: Vec<String> = std::env::args().collect();
        let argc = args.len() as i32;
        // 构建 argv 数组: Vec<CString> -> Vec<*mut u8>
        let mut argv_cstrings: Vec<CString> = Vec::with_capacity(args.len());
        let mut argv_ptrs: Vec<*mut u8> = Vec::with_capacity(args.len());
        for arg in &args {
            let cstr = CString::new(arg.as_str()).unwrap();
            argv_ptrs.push(cstr.as_bytes_with_nul().as_ptr() as *mut u8);
            argv_cstrings.push(cstr);
        }
        // 注意: argv_ptrs 需要在生命周期内有效，但 SetArgs 只是存储指针
        // 这里我们让 argv_cstrings 和 argv_ptrs 在 unsafe 块内有效
        (*app).SetArgs(argc, argv_ptrs.as_mut_ptr());

        // C++: gLawnApp->Init();
        (*app).Init();

        // C++: gLawnApp->Start();
        (*app).Start();

        // C++: gLawnApp->Shutdown(); (非 Emscripten 环境)
        // C++: #ifndef __EMSCRIPTEN__
        // C++: gLawnApp->Shutdown();
        // C++: if (gLawnApp) delete gLawnApp;
        // C++: #endif
        // 非浏览器环境下执行关闭流程
        #[cfg(not(target_arch = "wasm32"))]
        {
            (*app).Shutdown();
            // Rust 中: 通过 Box::from_raw 释放内存 (对应 C++ delete)
            let _ = Box::from_raw(app);
        }
    }
}

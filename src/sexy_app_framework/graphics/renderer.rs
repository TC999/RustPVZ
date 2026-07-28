// [TRANSLATION_NOTE]: 渲染后端 — 使用 SDL2 实现实际的窗口/绘制操作
// 注意：SDL2 的 Canvas/TextureCreator 不是 Send/Sync，
// 因此使用 raw pointer + unsafe 存储在全局变量中，
// 所有操作必须在主线程上调用。

use std::ptr;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color as SdlColor;

use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::graphics::graphics::Image;

// 全局渲染器原始指针 (仅在主线程使用)
static mut G_CANVAS: *mut Canvas<Window> = ptr::null_mut();
static mut G_SDL: Option<sdl2::Sdl> = None;

/// 初始化 SDL2 窗口和 Canvas (主线程调用)
pub fn init_renderer(title: &str, width: u32, height: u32) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let window = video_subsys
        .window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    unsafe {
        G_CANVAS = Box::into_raw(Box::new(canvas));
        G_SDL = Some(sdl_context);
    }
    Ok(())
}

/// 获取 Canvas 的可变引用
fn canvas() -> &'static mut Canvas<Window> {
    unsafe {
        debug_assert!(!G_CANVAS.is_null(), "Renderer not initialized!");
        &mut *G_CANVAS
    }
}

/// 用 RGBA 颜色填充矩形
pub fn fill_rect(x: i32, y: i32, w: i32, h: i32, color: &Color) {
    let c = canvas();
    c.set_draw_color(SdlColor::RGBA(color.m_red as u8, color.m_green as u8, color.m_blue as u8, color.m_alpha as u8));
    let rect = sdl2::rect::Rect::new(x, y, w.max(0) as u32, h.max(0) as u32);
    let _ = c.fill_rect(rect);
}

/// 绘制矩形边框
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32, color: &Color) {
    let c = canvas();
    c.set_draw_color(SdlColor::RGBA(color.m_red as u8, color.m_green as u8, color.m_blue as u8, color.m_alpha as u8));
    let rect = sdl2::rect::Rect::new(x, y, w.max(0) as u32, h.max(0) as u32);
    let _ = c.draw_rect(rect);
}

/// 绘制线条
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: &Color) {
    let c = canvas();
    c.set_draw_color(SdlColor::RGBA(color.m_red as u8, color.m_green as u8, color.m_blue as u8, color.m_alpha as u8));
    let _ = c.draw_line((x1, y1), (x2, y2));
}

/// 绘制图像 (blit)
pub fn blt(image: &Image, x: i32, y: i32, src_rect: Option<sdl2::rect::Rect>) {
    if image.m_cached_texture_id.is_none() { return; }
    let tex_id = image.m_cached_texture_id.unwrap();
    let c = canvas();
    // 纹理通过 Canvas 的纹理缓存访问
    // 简化版：需要 texture_creator 来创建纹理
    // 此处为框架预留
}

/// 清空屏幕
pub fn clear(r: u8, g: u8, b: u8) {
    let c = canvas();
    c.set_draw_color(SdlColor::RGB(r, g, b));
    c.clear();
}

/// 交换缓冲区
pub fn present() {
    canvas().present();
}

/// 获取窗口尺寸
pub fn get_window_size() -> (u32, u32) {
    canvas().output_size().unwrap_or((800, 600))
}

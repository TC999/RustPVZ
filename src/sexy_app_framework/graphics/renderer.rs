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
pub fn blt(image: &Image, _x: i32, _y: i32, _src_rect: Option<sdl2::rect::Rect>) {
    if image.m_cached_texture_id.is_none() { return; }
    let _tex_id = image.m_cached_texture_id.unwrap();
    let _c = canvas();
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

/// 获取 SDL 事件泵（主线程调用）
pub fn pump_events() -> Option<sdl2::EventPump> {
    unsafe {
        if let Some(sdl) = G_SDL.as_mut() {
            sdl.event_pump().ok()
        } else {
            None
        }
    }
}

/// 显示/隐藏鼠标光标（对应 C++ SDL_ShowCursor）
pub fn set_show_cursor(show: bool) {
    unsafe {
        if let Some(sdl) = G_SDL.as_ref() {
            let mouse_util = sdl.mouse();
            let _ = mouse_util.show_cursor(show);
        }
    }
}

/// 用 SDL_ttf 渲染文字到屏幕。
/// [TRANSLATION_NOTE]: C++ 使用图像字体（ImageFont/FontData），Rust 移植渐进方案
/// 使用 TTF 系统字体（Windows arial.ttf）渲染，语义等价（显示文字）。
static mut G_TTF_CONTEXT: Option<sdl2::ttf::Sdl2TtfContext> = None;

pub fn draw_text(text: &str, x: i32, y: i32, r: u8, g: u8, b: u8, a: u8, size: u16) {
    unsafe {
        if G_TTF_CONTEXT.is_none() {
            G_TTF_CONTEXT = sdl2::ttf::init().ok();
        }
        let ttf = match G_TTF_CONTEXT.as_mut() {
            Some(t) => t,
            None => return,
        };
        let font = match ttf.load_font("C:\\Windows\\Fonts\\arial.ttf", size.max(8)) {
            Ok(f) => f,
            Err(_) => return,
        };
        let surface = match font.render(text).blended_wrapped(sdl2::pixels::Color::RGBA(r, g, b, a), 600) {
            Ok(s) => s,
            Err(_) => return,
        };
        let c = canvas();
        let tc = c.texture_creator();
        let texture = match tc.create_texture_from_surface(surface) {
            Ok(t) => t,
            Err(_) => return,
        };
        let (w, h) = (texture.query().width, texture.query().height);
        let _ = c.copy(&texture, None, sdl2::rect::Rect::new(x, y, w, h));
    }
}
/// 将内存图像（ARGB 像素，对应 C++ MemoryImage::GetBits）绘制到屏幕。
/// [TRANSLATION_NOTE]: 对应 C++ MemoryImage::Blt/NormalBlt/AdditiveBlt 的屏幕呈现路径。
/// 通过 SDL streaming 纹理上传像素，混合模式由 SDL 处理（BLEND = 标准 alpha 合成，
/// ADD = 加色混合），color 非白时以 color_mod 调制（对应 C++ colorize 路径）。
pub fn blt_image(
    src_bits: *const u32,
    src_width: i32,
    src_height: i32,
    src_rect: &crate::sexy_app_framework::misc::rect::Rect,
    dest_rect: &crate::sexy_app_framework::misc::rect::Rect,
    color: &crate::sexy_app_framework::graphics::color::Color,
    draw_mode: i32,
    mirror: bool,
) {
    if src_bits.is_null() || src_rect.m_width <= 0 || src_rect.m_height <= 0 {
        return;
    }

    let c = canvas();
    let tc = c.texture_creator();
    let mut tex = match tc.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::ARGB8888,
        src_rect.m_width.max(1) as u32,
        src_rect.m_height.max(1) as u32,
    ) {
        Ok(t) => t,
        Err(_) => return,
    };

    // 拷贝源区域像素到纹理（ARGB8888 内存字节序：B,G,R,A）
    let _ = tex.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let mut y = 0;
        while y < src_rect.m_height {
            let mut x = 0;
            while x < src_rect.m_width {
                let src_idx = ((src_rect.m_y + y) * src_width + (src_rect.m_x + x)) as usize;
                let src_pixel = unsafe { *src_bits.add(src_idx) };
                let dst_idx = y as usize * pitch + x as usize * 4;
                buffer[dst_idx] = (src_pixel & 0xFF) as u8;
                buffer[dst_idx + 1] = ((src_pixel >> 8) & 0xFF) as u8;
                buffer[dst_idx + 2] = ((src_pixel >> 16) & 0xFF) as u8;
                buffer[dst_idx + 3] = ((src_pixel >> 24) & 0xFF) as u8;
                x += 1;
            }
            y += 1;
        }
    });

    // C++: Graphics::DRAWMODE_NORMAL = 0, DRAWMODE_ADDITIVE = 1
    if draw_mode == 1 {
        tex.set_blend_mode(sdl2::render::BlendMode::Add);
    } else {
        tex.set_blend_mode(sdl2::render::BlendMode::Blend);
    }

    if color.m_red != 255 || color.m_green != 255 || color.m_blue != 255 {
        tex.set_color_mod(color.m_red as u8, color.m_green as u8, color.m_blue as u8);
    }

    let dest = sdl2::rect::Rect::new(dest_rect.m_x, dest_rect.m_y, dest_rect.m_width.max(0) as u32, dest_rect.m_height.max(0) as u32);
    if mirror {
        let _ = c.copy_ex(&tex, None, dest, 0.0, None, false, true);
    } else {
        let _ = c.copy(&tex, None, dest);
    }
}

/// 获取窗口尺寸
pub fn get_window_size() -> (u32, u32) {
    canvas().output_size().unwrap_or((800, 600))
}

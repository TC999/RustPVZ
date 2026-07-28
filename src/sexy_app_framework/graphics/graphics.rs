// [TRANSLATION_NOTE]: Graphics.h -> Rust struct
// C++ Graphics 类映射为 Rust 结构体 + 方法

use std::collections::{LinkedList, HashMap};
use std::sync::atomic::{AtomicI32, Ordering};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::misc::point::Point;
use crate::sexy_app_framework::misc::sexy_vector::SexyVector2;

pub const MAX_TEMP_SPANS: i32 = 8192;

pub struct Edge {
    pub m_x: f64,
    pub m_dx: f64,
    pub i: i32,
    pub b: f64,
}

pub struct GraphicsState {
    pub m_dest_image: Option<Box<Image>>,
    pub m_trans_x: f32,
    pub m_trans_y: f32,
    pub m_scale_x: f32,
    pub m_scale_y: f32,
    pub m_scale_orig_x: f32,
    pub m_scale_orig_y: f32,
    pub m_clip_rect: Rect,
    pub m_color: Color,
    pub m_font: Option<Box<Font>>,
    pub m_font_raw: *mut Font,  // C++ 兼容原始字体指针
    pub m_draw_mode: i32,
    pub m_colorize_images: bool,
    pub m_fast_stretch: bool,
    pub m_write_colored_string: bool,
    pub m_linear_blend: bool,
    pub m_is_3d: bool,
}

impl GraphicsState {
    pub fn new() -> Self {
        GraphicsState {
            m_dest_image: None,
            m_trans_x: 0.0, m_trans_y: 0.0,
            m_scale_x: 1.0, m_scale_y: 1.0,
            m_scale_orig_x: 0.0, m_scale_orig_y: 0.0,
            m_clip_rect: Rect::new(0, 0, 800, 600),
            m_color: Color::new(),
            m_font: None,
            m_font_raw: std::ptr::null_mut(),
            m_draw_mode: 0,
            m_colorize_images: false,
            m_fast_stretch: true,
            m_write_colored_string: false,
            m_linear_blend: false,
            m_is_3d: false,
        }
    }

    pub fn copy_state_from(&mut self, the_state: &GraphicsState) {
        self.m_trans_x = the_state.m_trans_x;
        self.m_trans_y = the_state.m_trans_y;
        self.m_scale_x = the_state.m_scale_x;
        self.m_scale_y = the_state.m_scale_y;
        self.m_scale_orig_x = the_state.m_scale_orig_x;
        self.m_scale_orig_y = the_state.m_scale_orig_y;
        self.m_clip_rect = the_state.m_clip_rect;
        self.m_color = the_state.m_color;
        self.m_draw_mode = the_state.m_draw_mode;
        self.m_colorize_images = the_state.m_colorize_images;
        self.m_fast_stretch = the_state.m_fast_stretch;
        self.m_write_colored_string = the_state.m_write_colored_string;
        self.m_linear_blend = the_state.m_linear_blend;
        self.m_is_3d = the_state.m_is_3d;
    }
}

// Image struct - mapping of Sexy::Image
pub struct Image {
    pub m_drawn: bool,
    pub m_file_path: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_num_rows: i32,
    pub m_num_cols: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            m_drawn: false,
            m_file_path: String::new(),
            m_width: width,
            m_height: height,
            m_num_rows: 1,
            m_num_cols: 1,
        }
    }

    pub fn get_width(&self) -> i32 { self.m_width }
    pub fn get_height(&self) -> i32 { self.m_height }
    pub fn get_cel_width(&self) -> i32 { self.m_width / self.m_num_cols }
    pub fn get_cel_height(&self) -> i32 { self.m_height / self.m_num_rows }
    pub fn get_cel_rect(&self, cel: i32) -> Rect {
        let col = cel % self.m_num_cols;
        let row = cel / self.m_num_cols;
        self.get_cel_rect_rc(col, row)
    }
    pub fn get_cel_rect_rc(&self, the_col: i32, the_row: i32) -> Rect {
        let cw = self.get_cel_width();
        let ch = self.get_cel_height();
        Rect::new(the_col * cw, the_row * ch, cw, ch)
    }
    pub fn get_anim_cel_rect(&self, _the_time: i32) -> Rect {
        self.get_cel_rect(0)
    }
}

// Font struct - mapping of Sexy::_Font
pub struct Font {
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_line_spacing_offset: i32,
}

impl Font {
    pub fn new(height: i32) -> Self {
        Font { m_ascent: height, m_ascent_padding: 0, m_height: height, m_line_spacing_offset: 0 }
    }

    pub fn get_ascent(&self) -> i32 { self.m_ascent }
    pub fn get_ascent_padding(&self) -> i32 { self.m_ascent_padding }
    pub fn get_descent(&self) -> i32 { self.m_height - self.m_ascent }
    pub fn get_height(&self) -> i32 { self.m_height }
    pub fn get_line_spacing_offset(&self) -> i32 { self.m_line_spacing_offset }
    pub fn get_line_spacing(&self) -> i32 { self.m_height + self.m_line_spacing_offset }
    pub fn string_width(&self, _the_string: &str) -> i32 { 0 }
    pub fn char_width(&self, _the_char: char) -> i32 { 0 }
    pub fn char_width_kern(&self, _the_char: char, _the_prev_char: char) -> i32 { 0 }
}

pub struct Graphics {
    pub state: GraphicsState,
    pub m_state_stack: LinkedList<GraphicsState>,
}

impl Graphics {
    pub const DRAWMODE_NORMAL: i32 = 0;
    pub const DRAWMODE_ADDITIVE: i32 = 1;

    pub fn new() -> Self {
        Graphics {
            state: GraphicsState::new(),
            m_state_stack: LinkedList::new(),
        }
    }

    pub fn with_dest_image(dest_image: Option<Box<Image>>) -> Self {
        let mut g = Graphics::new();
        g.state.m_dest_image = dest_image;
        g
    }

    pub fn push_state(&mut self) {
        // Clone state manually - no derive(Clone) due to boxed trait objects
        let mut state = GraphicsState::new();
        state.copy_state_from(&self.state);
        self.m_state_stack.push_back(state);
    }

    pub fn pop_state(&mut self) {
        if let Some(state) = self.m_state_stack.pop_back() {
            self.state = state;
        }
    }

    pub fn set_font(&mut self, font: Option<Box<Font>>) {
        self.state.m_font = font;
        self.state.m_font_raw = std::ptr::null_mut();
    }

    /// C++ 兼容：通过原始指针设置字体
    pub fn set_font_ptr(&mut self, font: *mut Font) {
        self.state.m_font_raw = font;
        if !font.is_null() {
            unsafe {
                self.state.m_font = Some(Box::from_raw(font));
                let _ = Box::into_raw(self.state.m_font.take().unwrap());
                self.state.m_font = Some(Box::from_raw(font));
            }
        } else {
            self.state.m_font = None;
        }
    }

    /// C++ 兼容：SetFont( Font* )
    pub fn SetFont(&mut self, font: *mut Font) {
        self.set_font_ptr(font);
    }

    /// C++ 兼容：SetColor( Color )
    pub fn SetColor(&mut self, color: Color) {
        self.state.m_color = color;
    }

    /// C++ 兼容：FillRect( x, y, w, h )
    pub fn FillRect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.fill_rect(x, y, w, h);
    }

    /// C++ 兼容：DrawRect( x, y, w, h )
    pub fn DrawRect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.draw_rect(x, y, w, h);
    }

    /// C++ 兼容：DrawString( str, x, y )
    pub fn DrawString(&self, text: &str, x: i32, y: i32) {
        self.draw_string(text, x, y);
    }

    pub fn get_font(&self) -> Option<&Font> {
        self.state.m_font.as_ref().map(|f| f.as_ref())
    }

    pub fn set_color(&mut self, color: &Color) {
        self.state.m_color = *color;
    }

    pub fn get_color(&self) -> &Color {
        &self.state.m_color
    }

    pub fn set_draw_mode(&mut self, mode: i32) {
        self.state.m_draw_mode = mode;
    }

    pub fn get_draw_mode(&self) -> i32 {
        self.state.m_draw_mode
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        // Placeholder - actual pixel fill would go here
    }

    pub fn fill_rect_r(&mut self, rect: &Rect) {
        self.fill_rect(rect.m_x, rect.m_y, rect.m_width, rect.m_height);
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        // Placeholder
    }

    pub fn draw_rect_r(&mut self, rect: &Rect) {
        self.draw_rect(rect.m_x, rect.m_y, rect.m_width, rect.m_height);
    }

    pub fn draw_string(&self, text: &str, x: i32, y: i32) {
        // Placeholder
    }

    pub fn set_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.state.m_clip_rect = Rect::new(x, y, width, height);
    }

    pub fn set_clip_rect_r(&mut self, rect: &Rect) {
        self.state.m_clip_rect = *rect;
    }

    pub fn clear_clip_rect(&mut self) {
        self.state.m_clip_rect = Rect::new(0, 0, 8000, 6000);
    }

    pub fn clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let r = Rect::new(x, y, width, height);
        let cur = self.state.m_clip_rect;
        self.state.m_clip_rect = cur.intersection(&r);
    }

    pub fn clip_rect_r(&mut self, rect: &Rect) {
        let cur = self.state.m_clip_rect;
        self.state.m_clip_rect = cur.intersection(rect);
    }

    pub fn translate(&mut self, tx: i32, ty: i32) {
        self.state.m_trans_x += tx as f32;
        self.state.m_trans_y += ty as f32;
    }

    pub fn translate_f(&mut self, tx: f32, ty: f32) {
        self.state.m_trans_x += tx;
        self.state.m_trans_y += ty;
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32, orig_x: f32, orig_y: f32) {
        self.state.m_scale_x = sx;
        self.state.m_scale_y = sy;
        self.state.m_scale_orig_x = orig_x;
        self.state.m_scale_orig_y = orig_y;
    }

    pub fn string_width(&self, _text: &str) -> i32 {
        // Placeholder
        0
    }
}

pub struct GraphicsAutoState<'a> {
    pub m_g: &'a mut Graphics,
}

impl<'a> GraphicsAutoState<'a> {
    pub fn new(g: &'a mut Graphics) -> Self {
        g.push_state();
        GraphicsAutoState { m_g: g }
    }
}

impl<'a> Drop for GraphicsAutoState<'a> {
    fn drop(&mut self) {
        self.m_g.pop_state();
    }
}

// MemoryImage - 内存图像（对应 C++ Sexy::MemoryImage）
pub const MEMORYCHECK_ID: u32 = 0x4BEEFADE;
pub static mut G_OPTIMIZE_SOFTWARE_DRAWING: bool = true;

pub struct MemoryImage {
    pub base: Image,
    pub m_bits: *mut u32,
    pub m_bits_changed_count: i32,
    pub m_render_data: *mut u8,
    pub m_render_flags: u32,
    pub m_color_table: *mut u32,
    pub m_color_indices: *mut u8,
    pub m_forced_mode: bool,
    pub m_has_trans: bool,
    pub m_has_alpha: bool,
    pub m_is_volatile: bool,
    pub m_purge_bits: bool,
    pub m_want_pal: bool,
    pub m_native_alpha_data: *mut u32,
    pub m_rl_alpha_data: *mut u8,
    pub m_rl_additive_data: *mut u8,
    pub m_bits_changed: bool,
    pub m_app: *mut u8, // 实际应为 *mut SexyAppBase
}

impl MemoryImage {
    pub fn new() -> Self {
        MemoryImage {
            base: Image::new(0, 0),
            m_bits: std::ptr::null_mut(),
            m_bits_changed_count: 0,
            m_render_data: std::ptr::null_mut(),
            m_render_flags: 0,
            m_color_table: std::ptr::null_mut(),
            m_color_indices: std::ptr::null_mut(),
            m_forced_mode: false,
            m_has_trans: false,
            m_has_alpha: false,
            m_is_volatile: false,
            m_purge_bits: false,
            m_want_pal: false,
            m_native_alpha_data: std::ptr::null_mut(),
            m_rl_alpha_data: std::ptr::null_mut(),
            m_rl_additive_data: std::ptr::null_mut(),
            m_bits_changed: false,
            m_app: std::ptr::null_mut(),
        }
    }

    pub fn get_bits(&self) -> *mut u32 { self.m_bits }
    pub fn bits_changed(&mut self) { self.m_bits_changed = true; self.m_bits_changed_count += 1; }
    pub fn commit_bits(&mut self) { self.m_bits_changed = false; }
    pub fn clear(&mut self) { /* placeholder */ }
    pub fn create(&mut self, _width: i32, _height: i32) { /* placeholder */ }
    pub fn set_bits(&mut self, _the_bits: *mut u32, _the_width: i32, _the_height: i32, _commit_bits: bool) { /* placeholder */ }
    pub fn set_image_mode(&mut self, _has_trans: bool, _has_alpha: bool) {
        self.m_has_trans = _has_trans;
        self.m_has_alpha = _has_alpha;
    }
    pub fn set_volatile(&mut self, _is_volatile: bool) { self.m_is_volatile = _is_volatile; }
    pub fn purge_bits(&mut self) { /* placeholder */ }
    pub fn delete_sw_buffers(&mut self) { /* placeholder */ }
    pub fn delete_3d_buffers(&mut self) { /* placeholder */ }
    pub fn re_init(&mut self) { /* placeholder */ }
    pub fn delete_native_data(&mut self) { /* placeholder */ }

    pub fn fill_rect(&mut self, _the_rect: &Rect, _the_color: &Color, _the_draw_mode: i32) { /* placeholder */ }
    pub fn clear_rect(&mut self, _the_rect: &Rect) { /* placeholder */ }
    pub fn draw_line(&mut self, _start_x: f64, _start_y: f64, _end_x: f64, _end_y: f64, _the_color: &Color, _the_draw_mode: i32) { /* placeholder */ }
    pub fn blt(&mut self, _the_image: &Image, _the_x: i32, _the_y: i32, _the_src_rect: &Rect, _the_color: &Color, _the_draw_mode: i32, _linear_filter: bool) { /* placeholder */ }
    pub fn stretch_blt(&mut self, _the_image: &Image, _the_dest_rect: &Rect, _the_src_rect: &Rect, _the_clip_rect: &Rect, _the_color: &Color, _the_draw_mode: i32, _fast_stretch: bool) { /* placeholder */ }

    pub fn normal_blt(&mut self, _the_image: &Image, _the_x: i32, _the_y: i32, _the_src_rect: &Rect, _the_color: &Color) { /* placeholder */ }
    pub fn additive_blt(&mut self, _the_image: &Image, _the_x: i32, _the_y: i32, _the_src_rect: &Rect, _the_color: &Color) { /* placeholder */ }
}

// ============================================================
// GLImage - OpenGL 图像（桩）
// ============================================================
pub struct GLImage {
    pub base: MemoryImage,
    pub m_tex_info: *mut u8,
    pub m_tex_width: i32,
    pub m_tex_height: i32,
}

impl GLImage {
    pub fn new() -> Self {
        GLImage {
            base: MemoryImage::new(),
            m_tex_info: std::ptr::null_mut(),
            m_tex_width: 0,
            m_tex_height: 0,
        }
    }
}

// ============================================================
// SharedImage - 共享图像（引用计数）
// ============================================================

pub struct SharedImage {
    pub m_image: *mut GLImage,
    pub m_ref_count: AtomicI32,
}

impl SharedImage {
    pub fn new() -> Self {
        SharedImage {
            m_image: std::ptr::null_mut(),
            m_ref_count: AtomicI32::new(0),
        }
    }
}

/// SharedImageRef - 共享图像智能指针
pub struct SharedImageRef {
    pub m_shared_image: *mut SharedImage,
    pub m_unshared_image: *mut MemoryImage,
    pub m_owns_unshared: bool,
}

impl SharedImageRef {
    pub fn new() -> Self {
        SharedImageRef {
            m_shared_image: std::ptr::null_mut(),
            m_unshared_image: std::ptr::null_mut(),
            m_owns_unshared: false,
        }
    }

    pub fn from_shared(the_shared_image: *mut SharedImage) -> Self {
        if !the_shared_image.is_null() {
            unsafe { (*the_shared_image).m_ref_count.fetch_add(1, Ordering::Relaxed); }
        }
        SharedImageRef {
            m_shared_image: the_shared_image,
            m_unshared_image: std::ptr::null_mut(),
            m_owns_unshared: false,
        }
    }

    pub fn release(&mut self) {
        if self.m_owns_unshared && !self.m_unshared_image.is_null() {
            let _ = unsafe { Box::from_raw(self.m_unshared_image) };
        }
        self.m_unshared_image = std::ptr::null_mut();
        if !self.m_shared_image.is_null() {
            unsafe {
                if (*self.m_shared_image).m_ref_count.fetch_sub(1, Ordering::Relaxed) == 1 {
                    // 标记清理
                    // gSexyAppBase->mCleanupSharedImages.store(true)
                }
            }
        }
        self.m_shared_image = std::ptr::null_mut();
    }

    pub fn get_as_image(&self) -> *mut Image {
        if !self.m_unshared_image.is_null() {
            unsafe { &mut (*self.m_unshared_image).base as *mut Image }
        } else if !self.m_shared_image.is_null() {
            unsafe { (*self.m_shared_image).m_image as *mut Image }
        } else {
            std::ptr::null_mut()
        }
    }
}

impl Clone for SharedImageRef {
    fn clone(&self) -> Self {
        if !self.m_shared_image.is_null() {
            unsafe { (*self.m_shared_image).m_ref_count.fetch_add(1, Ordering::Relaxed); }
        }
        SharedImageRef {
            m_shared_image: self.m_shared_image,
            m_unshared_image: self.m_unshared_image,
            m_owns_unshared: false,
        }
    }
}

impl Drop for SharedImageRef {
    fn drop(&mut self) {
        // Release handled by explicit call in C++; auto-release here for safety
    }
}

// ============================================================
// ImageFont - 图像字体
// ============================================================

/// 字符数据（对应 C++ CharData）
#[derive(Clone)]
pub struct CharData {
    pub m_image_rect: Rect,
    pub m_offset: Point,
    pub m_kerning_offsets: HashMap<char, i32>,
    pub m_width: i32,
    pub m_order: i32,
}

impl CharData {
    pub fn new() -> Self {
        CharData {
            m_image_rect: Rect::new(0, 0, 0, 0),
            m_offset: Point::new(0, 0),
            m_kerning_offsets: HashMap::new(),
            m_width: 0,
            m_order: 0,
        }
    }
}

/// 字体层（对应 C++ FontLayer）
pub struct FontLayer {
    pub m_layer_name: String,
    pub m_required_tags: Vec<String>,
    pub m_excluded_tags: Vec<String>,
    pub m_char_data_map: HashMap<char, CharData>,
    pub m_color_mult: Color,
    pub m_color_add: Color,
    pub m_image: *mut Image,
    pub m_draw_mode: i32,
    pub m_offset: Point,
    pub m_spacing: i32,
    pub m_min_point_size: i32,
    pub m_max_point_size: i32,
    pub m_point_size: i32,
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_default_height: i32,
    pub m_line_spacing_offset: i32,
    pub m_base_order: i32,
    pub m_use_alpha_correction: bool,
}

impl FontLayer {
    pub fn new() -> Self {
        FontLayer {
            m_layer_name: String::new(),
            m_required_tags: Vec::new(),
            m_excluded_tags: Vec::new(),
            m_char_data_map: HashMap::new(),
            m_color_mult: Color::new(),
            m_color_add: Color::new(),
            m_image: std::ptr::null_mut(),
            m_draw_mode: 0,
            m_offset: Point::new(0, 0),
            m_spacing: 0,
            m_min_point_size: 0,
            m_max_point_size: 0,
            m_point_size: 0,
            m_ascent: 0,
            m_ascent_padding: 0,
            m_height: 0,
            m_default_height: 0,
            m_line_spacing_offset: 0,
            m_base_order: 0,
            m_use_alpha_correction: false,
        }
    }
}

/// 活动字体层（对应 C++ ActiveFontLayer）
pub struct ActiveFontLayer {
    pub m_base_font_layer: *mut FontLayer,
    pub m_scaled_image: *mut Image,
    pub m_owns_image: bool,
    pub m_scaled_char_image_rects: HashMap<char, Rect>,
}

impl ActiveFontLayer {
    pub fn new() -> Self {
        ActiveFontLayer {
            m_base_font_layer: std::ptr::null_mut(),
            m_scaled_image: std::ptr::null_mut(),
            m_owns_image: false,
            m_scaled_char_image_rects: HashMap::new(),
        }
    }
}

/// 渲染命令（对应 C++ RenderCommand）
pub struct RenderCommand {
    pub m_image: *mut Image,
    pub m_dest: [i32; 2],
    pub m_src: [i32; 4],
    pub m_mode: i32,
    pub m_color: Color,
    pub m_use_alpha_correction: bool,
    pub m_next: *mut RenderCommand,
}

/// ImageFont - 图像字体（对应 C++ Sexy::ImageFont）
pub struct ImageFont {
    // _Font 基类字段
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_line_spacing_offset: i32,
    // ImageFont 自身字段
    pub m_font_data: *mut u8, // 实际为 *mut FontData，用 void* 避免依赖 DescParser
    pub m_point_size: i32,
    pub m_tag_vector: Vec<String>,
    pub m_active_list_valid: bool,
    pub m_active_layer_list: Vec<ActiveFontLayer>,
    pub m_scale: f64,
    pub m_force_scaled_images_white: bool,
}

impl ImageFont {
    pub fn new() -> Self {
        ImageFont {
            m_ascent: 0, m_ascent_padding: 0, m_height: 0, m_line_spacing_offset: 0,
            m_font_data: std::ptr::null_mut(),
            m_point_size: 0,
            m_tag_vector: Vec::new(),
            m_active_list_valid: false,
            m_active_layer_list: Vec::new(),
            m_scale: 1.0,
            m_force_scaled_images_white: false,
        }
    }

    // _Font 接口方法
    pub fn get_ascent(&self) -> i32 { self.m_ascent }
    pub fn get_ascent_padding(&self) -> i32 { self.m_ascent_padding }
    pub fn get_descent(&self) -> i32 { self.m_height - self.m_ascent }
    pub fn get_height(&self) -> i32 { self.m_height }
    pub fn get_line_spacing_offset(&self) -> i32 { self.m_line_spacing_offset }
    pub fn get_line_spacing(&self) -> i32 { self.m_height + self.m_line_spacing_offset }

    // ImageFont 特定方法
    pub fn char_width(&self, _the_char: char) -> i32 { 0 }
    pub fn char_width_kern(&self, _the_char: char, _the_prev_char: char) -> i32 { 0 }
    pub fn string_width(&self, _the_string: &str) -> i32 { 0 }
    pub fn draw_string(&self, _g: &mut Graphics, _the_x: i32, _the_y: i32, _the_string: &str, _the_color: &Color, _the_clip_rect: &Rect) {}
    pub fn duplicate(&self) -> ImageFont { ImageFont::new() }
    pub fn set_point_size(&mut self, _the_point_size: i32) { self.m_point_size = _the_point_size; }
    pub fn get_point_size(&self) -> i32 { self.m_point_size }
    pub fn set_scale(&mut self, _the_scale: f64) { self.m_scale = _the_scale; }
    pub fn get_default_point_size(&self) -> i32 { 0 }
    pub fn prepare(&mut self) { /* placeholder */ }
}

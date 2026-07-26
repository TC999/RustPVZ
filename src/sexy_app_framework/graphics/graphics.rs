// [TRANSLATION_NOTE]: Graphics.h -> Rust struct
// C++ Graphics 类映射为 Rust 结构体 + 方法

use std::collections::LinkedList;
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::graphics::color::Color;

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

// Placeholder types
pub struct Image {
    pub m_width: i32,
    pub m_height: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        Image { m_width: width, m_height: height }
    }
}

pub struct Font {
    pub m_height: i32,
}

impl Font {
    pub fn new(height: i32) -> Self {
        Font { m_height: height }
    }
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

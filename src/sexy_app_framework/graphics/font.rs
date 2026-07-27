// [TRANSLATION_NOTE]: Font.h + Font.cpp -> Rust trait
// _Font 抽象基类映射为 Rust trait + 默认实现

use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::misc::rect::Rect;

/// _Font 抽象基类的 Rust 映射
pub trait FontTrait {
    fn get_ascent(&self) -> i32;
    fn get_ascent_padding(&self) -> i32;
    fn get_descent(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_line_spacing_offset(&self) -> i32;
    fn get_line_spacing(&self) -> i32;
    fn string_width(&self, the_string: &str) -> i32;
    fn char_width(&self, the_char: char) -> i32;
    fn char_width_kern(&self, the_char: char, the_prev_char: char) -> i32;
    fn draw_string(&self, g: &mut super::graphics::Graphics, the_x: i32, the_y: i32, the_string: &str, the_color: &Color, the_clip_rect: &Rect);
    fn duplicate(&self) -> Box<dyn FontTrait>;
}

/// 默认 _Font 实现（对应 C++ _Font 基类的非虚方法）
pub struct Font {
    pub m_ascent: i32,
    pub m_ascent_padding: i32,
    pub m_height: i32,
    pub m_line_spacing_offset: i32,
}

impl Font {
    pub fn new() -> Self {
        Font {
            m_ascent: 0,
            m_ascent_padding: 0,
            m_height: 0,
            m_line_spacing_offset: 0,
        }
    }

    pub fn from_font(the_font: &Font) -> Self {
        Font {
            m_ascent: the_font.m_ascent,
            m_ascent_padding: the_font.m_ascent_padding,
            m_height: the_font.m_height,
            m_line_spacing_offset: the_font.m_line_spacing_offset,
        }
    }
}

impl FontTrait for Font {
    fn get_ascent(&self) -> i32 { self.m_ascent }
    fn get_ascent_padding(&self) -> i32 { self.m_ascent_padding }
    fn get_descent(&self) -> i32 { self.m_height - self.m_ascent }
    fn get_height(&self) -> i32 { self.m_height }
    fn get_line_spacing_offset(&self) -> i32 { self.m_line_spacing_offset }
    fn get_line_spacing(&self) -> i32 { self.m_height + self.m_line_spacing_offset }

    fn string_width(&self, _the_string: &str) -> i32 { 0 }
    fn char_width(&self, _the_char: char) -> i32 { 0 }
    fn char_width_kern(&self, _the_char: char, _the_prev_char: char) -> i32 { 0 }
    fn draw_string(&self, _g: &mut super::graphics::Graphics, _the_x: i32, _the_y: i32, _the_string: &str, _the_color: &Color, _the_clip_rect: &Rect) {}
    fn duplicate(&self) -> Box<dyn FontTrait> { Box::new(Font::from_font(self)) }
}

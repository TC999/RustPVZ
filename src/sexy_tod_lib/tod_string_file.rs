// [TRANSLATION_NOTE]: TodStringFile.h + TodStringFile.cpp -> Rust
// 字符串列表文件读写与格式化文本绘制
// 依赖: Graphics, Font, Color, Rect（已有）

use crate::const_enums::DrawStringJustification;
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Font};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::common;
use crate::sexy_tod_lib::tod_common::test_bit;

pub const TOD_FORMAT_IGNORE_NEWLINES: u32 = 0;
pub const TOD_FORMAT_HIDE_UNTIL_MAGNETSHROOM: u32 = 1;

pub struct TodStringListFormat {
    pub m_format_name: String,
    pub m_new_font: *mut Font,
    pub m_new_color: Color,
    pub m_line_spacing_offset: i32,
    pub m_format_flags: u32,
}

impl TodStringListFormat {
    pub fn new() -> Self {
        TodStringListFormat {
            m_format_name: String::new(),
            m_new_font: std::ptr::null_mut(),
            m_new_color: Color::new(),
            m_line_spacing_offset: 0,
            m_format_flags: 0,
        }
    }

    pub fn with_values(the_format_name: &str, the_font: *mut Font, the_color: &Color, the_line_spacing_offset: i32, the_format_flags: u32) -> Self {
        TodStringListFormat {
            m_format_name: the_format_name.to_string(),
            m_new_font: the_font,
            m_new_color: *the_color,
            m_line_spacing_offset: the_line_spacing_offset,
            m_format_flags: the_format_flags,
        }
    }
}

// 全局字符串格式
use std::sync::Mutex;

static G_TOD_STRING_FORMAT_COUNT: Mutex<i32> = Mutex::new(0);
static G_TOD_STRING_FORMATS: Mutex<usize> = Mutex::new(0);

unsafe impl Send for TodStringListFormat {}
unsafe impl Sync for TodStringListFormat {}

pub fn get_lawn_string_formats() -> &'static [TodStringListFormat; 12] {
    use std::sync::OnceLock;
    static FORMATS: OnceLock<[TodStringListFormat; 12]> = OnceLock::new();
    FORMATS.get_or_init(|| {
        // C++ 初始值：
        // { "NORMAL",           nullptr, Color(40,50,90,255),       0,  0U }
        // { "FLAVOR",           nullptr, Color(143,67,27,255),      0,  1U }
        // ...
        let colors: [(i32, i32, i32, i32); 12] = [
            (40, 50, 90, 255), (143, 67, 27, 255), (143, 67, 27, 255),
            (136, 50, 170, 255), (11, 161, 219, 255), (204, 36, 29, 255),
            (204, 36, 29, 255), (143, 67, 27, 255), (0, 0, 0, 0),
            (0, 0, 0, 0), (0, 0, 0, 0), (0, 0, 0, 0),
        ];
        let names = ["NORMAL", "FLAVOR", "KEYWORD", "NOCTURNAL", "AQUATIC", "STAT",
                      "METAL", "KEYMETAL", "SHORTLINE", "EXTRASHORTLINE", "CREDITS1", "CREDITS2"];
        let offsets = [0, 0, 0, 0, 0, 0, 0, 0, -9, -14, 3, 2];
        let flags = [0u32, 1, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0];
        
        let mut arr: [TodStringListFormat; 12] = [
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(40,50,90), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(143,67,27), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(143,67,27), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(136,50,170), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(11,161,219), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(204,36,29), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(204,36,29), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(143,67,27), m_line_spacing_offset: 0, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(0,0,0), m_line_spacing_offset: -9, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(0,0,0), m_line_spacing_offset: -14, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(0,0,0), m_line_spacing_offset: 3, m_format_flags: 0 },
            TodStringListFormat { m_format_name: String::new(), m_new_font: std::ptr::null_mut(), m_new_color: Color::from_components(0,0,0), m_line_spacing_offset: 2, m_format_flags: 0 },
        ];
        for i in 0..12 {
            arr[i].m_format_name = names[i].to_string();
            arr[i].m_format_flags = flags[i];
        }
        arr
    })
}

pub fn tod_string_list_set_colors(the_formats: *mut TodStringListFormat, the_count: i32) {
    *G_TOD_STRING_FORMATS.lock().unwrap() = the_formats as usize;
    *G_TOD_STRING_FORMAT_COUNT.lock().unwrap() = the_count;
}

pub fn get_tod_string_format_count() -> i32 {
    *G_TOD_STRING_FORMAT_COUNT.lock().unwrap()
}

pub fn get_tod_string_formats() -> *mut TodStringListFormat {
    *G_TOD_STRING_FORMATS.lock().unwrap() as *mut TodStringListFormat
}

pub fn tod_string_list_read_name(the_ptr: &mut &str, the_name: &mut String) -> bool {
    if let Some(pos) = the_ptr.find('[') {
        let after_open = &the_ptr[pos+1..];
        if let Some(close_pos) = after_open.find(']') {
            let name_raw = &after_open[..close_pos];
            *the_name = common::trim(name_raw);
            if the_name.is_empty() {
                return false;
            }
            *the_ptr = &the_ptr[pos + 1 + close_pos + 1..];
            return true;
        }
        return false;
    }
    // No '[' found
    if !the_ptr.trim().is_empty() {
        return false;
    }
    the_name.clear();
    true
}

pub fn tod_string_remove_return_chars(the_string: &mut String) {
    the_string.retain(|c| c != '\r');
}

pub fn tod_string_list_read_value(the_ptr: &mut &str, the_value: &mut String) -> bool {
    if let Some(pos) = the_ptr.find('[') {
        let val = common::trim(&the_ptr[..pos]);
        *the_value = val;
        *the_ptr = &the_ptr[pos..];
    } else {
        *the_value = common::trim(the_ptr);
        *the_ptr = "";
    }
    tod_string_remove_return_chars(the_value);
    true
}

pub fn tod_string_list_read_items(the_file_text: &str) -> bool {
    let mut a_ptr = the_file_text;
    loop {
        let mut a_name = String::new();
        if !tod_string_list_read_name(&mut a_ptr, &mut a_name) {
            return false;
        }
        if a_name.is_empty() {
            return true;
        }
        let mut a_value = String::new();
        if !tod_string_list_read_value(&mut a_ptr, &mut a_value) {
            return false;
        }
        let a_name_upper = common::string_to_upper(&a_name);
        // SetString - store in app's string properties
        // gSexyAppBase->SetString(aNameUpper, aValue);
    }
}

pub fn tod_string_translate(the_string: &str) -> String {
    if the_string.len() >= 3 && the_string.as_bytes()[0] == b'[' {
        let a_name = &the_string[1..the_string.len()-1];
        // TodStringListFind(aName)
        return format!("<Missing {}>", a_name);
    }
    the_string.to_string()
}

pub fn char_is_space_in_format(the_char: char, the_current_format: &TodStringListFormat) -> bool {
    the_char == ' ' || (test_bit(the_current_format.m_format_flags, TOD_FORMAT_IGNORE_NEWLINES as i32) && the_char == '\n')
}

pub fn tod_write_string(
    _g: &mut Graphics,
    the_string: &str,
    the_x: i32,
    the_y: i32,
    the_current_format: &mut TodStringListFormat,
    the_width: i32,
    the_justification: DrawStringJustification,
    draw_string: bool,
    the_offset: i32,
    the_length: i32,
) -> i32 {
    0 // placeholder
}

pub fn tod_draw_string_wrapped(
    g: &mut Graphics,
    the_text: &str,
    the_rect: &Rect,
    the_font: &Font,
    the_color: &Color,
    the_justification: DrawStringJustification,
) {
    let a_text_final = tod_string_translate(the_text);
    let mut a_rect_to_use = *the_rect;
    if the_justification == DrawStringJustification::DS_ALIGN_LEFT_VERTICAL_MIDDLE
        || the_justification == DrawStringJustification::DS_ALIGN_RIGHT_VERTICAL_MIDDLE
        || the_justification == DrawStringJustification::DS_ALIGN_CENTER_VERTICAL_MIDDLE
    {
        let height = tod_draw_string_wrapped_helper(g, &a_text_final, &a_rect_to_use, the_font, the_color, the_justification, false);
        a_rect_to_use.m_y += (a_rect_to_use.m_height - height) / 2;
    }
    tod_draw_string_wrapped_helper(g, &a_text_final, &a_rect_to_use, the_font, the_color, the_justification, true);
}

pub fn tod_draw_string_wrapped_helper(
    _g: &mut Graphics,
    _the_text: &str,
    _the_rect: &Rect,
    _the_font: &Font,
    _the_color: &Color,
    _the_justification: DrawStringJustification,
    _draw_string: bool,
) -> i32 {
    0 // placeholder
}

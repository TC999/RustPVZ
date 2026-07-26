// [TRANSLATION_NOTE]: Color.h -> Rust struct
// C++ Color 类的 constexpr 构造函数和操作符映射为 Rust 的 const fn 和 trait impl

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub m_red: i32,
    pub m_green: i32,
    pub m_blue: i32,
    pub m_alpha: i32,
}

impl Color {
    pub const fn new() -> Self {
        Color {
            m_red: 0,
            m_green: 0,
            m_blue: 0,
            m_alpha: 255,
        }
    }

    pub const fn from_rgb(the_color: i32) -> Self {
        let r = (the_color >> 16) & 0xFF;
        let g = (the_color >> 8) & 0xFF;
        let b = the_color & 0xFF;
        let mut a = (the_color >> 24) & 0xFF;
        if a == 0 {
            a = 0xFF;
        }
        Color {
            m_red: r,
            m_green: g,
            m_blue: b,
            m_alpha: a as i32,
        }
    }

    pub const fn from_rgb_alpha(the_color: i32, the_alpha: i32) -> Self {
        Color {
            m_red: (the_color >> 16) & 0xFF,
            m_green: (the_color >> 8) & 0xFF,
            m_blue: the_color & 0xFF,
            m_alpha: the_alpha,
        }
    }

    pub const fn from_components(red: i32, green: i32, blue: i32) -> Self {
        Color {
            m_red: red,
            m_green: green,
            m_blue: blue,
            m_alpha: 0xFF,
        }
    }

    pub const fn from_components_alpha(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        Color {
            m_red: red,
            m_green: green,
            m_blue: blue,
            m_alpha: alpha,
        }
    }

    pub const fn get_red(&self) -> i32 {
        self.m_red
    }
    pub const fn get_green(&self) -> i32 {
        self.m_green
    }
    pub const fn get_blue(&self) -> i32 {
        self.m_blue
    }
    pub const fn get_alpha(&self) -> i32 {
        self.m_alpha
    }

    pub const fn to_int(&self) -> u32 {
        ((self.m_alpha as u32) << 24)
            | ((self.m_red as u32) << 16)
            | ((self.m_green as u32) << 8)
            | (self.m_blue as u32)
    }

    pub const fn to_gl_color(&self) -> u32 {
        let a_gl_color = ((self.m_alpha as u32) << 24)
            | ((self.m_blue as u32) << 16)
            | ((self.m_green as u32) << 8)
            | (self.m_red as u32);
        // ToLE32
        Self::to_le32(a_gl_color)
    }

    const fn to_le32(v: u32) -> u32 {
        // On little-endian systems this is a no-op, but keep for 1:1 fidelity
        #[cfg(target_endian = "little")]
        {
            v
        }
        #[cfg(target_endian = "big")]
        {
            ((v & 0x000000FF) << 24) | ((v & 0x0000FF00) << 8) | ((v & 0x00FF0000) >> 8) | ((v & 0xFF000000) >> 24)
        }
    }

    pub fn index(&self, idx: i32) -> i32 {
        match idx {
            0 => self.m_red,
            1 => self.m_green,
            2 => self.m_blue,
            3 => self.m_alpha,
            _ => 0,
        }
    }

    pub fn index_mut(&mut self, idx: i32) -> &mut i32 {
        match idx {
            0 => &mut self.m_red,
            1 => &mut self.m_green,
            2 => &mut self.m_blue,
            3 => &mut self.m_alpha,
            _ => panic!("Color index out of range"),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new()
    }
}

// Static colors - will be initialized as lazily statics
use std::sync::LazyLock;

pub static BLACK: LazyLock<Color> = LazyLock::new(|| Color::new());
pub static WHITE: LazyLock<Color> = LazyLock::new(|| Color::from_components(255, 255, 255));

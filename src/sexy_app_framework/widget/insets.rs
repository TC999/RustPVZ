// [TRANSLATION_NOTE]: Insets.h -> Rust struct

#[derive(Clone, Copy, Debug, Default)]
pub struct Insets {
    pub m_left: i32,
    pub m_top: i32,
    pub m_right: i32,
    pub m_bottom: i32,
}

impl Insets {
    pub fn new() -> Self {
        Insets {
            m_left: 0,
            m_top: 0,
            m_right: 0,
            m_bottom: 0,
        }
    }

    pub fn new_insets(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Insets {
            m_left: left,
            m_top: top,
            m_right: right,
            m_bottom: bottom,
        }
    }
}

// [TRANSLATION_NOTE]: GameObject.h -> Rust struct

#[derive(Clone)]
pub struct GameObject {
    pub m_app: *mut std::ffi::c_void,
    pub m_board: *mut std::ffi::c_void,
    pub m_x: i32,
    pub m_y: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_visible: bool,
    pub m_row: i32,
    pub m_render_order: i32,
}

impl GameObject {
    pub fn new() -> Self {
        GameObject {
            m_app: std::ptr::null_mut(),
            m_board: std::ptr::null_mut(),
            m_x: 0,
            m_y: 0,
            m_width: 0,
            m_height: 0,
            m_visible: true,
            m_row: 0,
            m_render_order: 0,
        }
    }
}

impl Default for GameObject {
    fn default() -> Self {
        Self::new()
    }
}

// [TRANSLATION_NOTE]: GameObject.h + GameObject.cpp -> Rust struct
// C++ 构造函数中使用全局 gLawnApp 指针，Rust 中映射为 G_LAWN_APP

use crate::lawn_app::G_LAWN_APP;
use crate::const_enums::RenderLayer;
use crate::sexy_app_framework::graphics::graphics::Graphics;

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
        let app = unsafe { G_LAWN_APP };
        let board = if !app.is_null() {
            unsafe { (*app).m_board.as_mut().map(|b| b as *mut _).unwrap_or(std::ptr::null_mut()) as *mut std::ffi::c_void }
        } else {
            std::ptr::null_mut()
        };
        GameObject {
            m_app: app as *mut std::ffi::c_void,
            m_board: board,
            m_x: 0,
            m_y: 0,
            m_width: 0,
            m_height: 0,
            m_visible: true,
            m_row: -1,
            m_render_order: RenderLayer::RENDER_LAYER_TOP as i32,
        }
    }

    pub fn begin_draw(&self, g: &mut Graphics) -> bool {
        if !self.m_visible {
            return false;
        }
        g.translate(self.m_x, self.m_y);
        true
    }

    pub fn end_draw(&self, g: &mut Graphics) {
        g.translate(-self.m_x, -self.m_y);
    }

    pub fn make_parent_graphics_frame(&self, g: &mut Graphics) {
        g.translate(-self.m_x, -self.m_y);
    }
}

impl Default for GameObject {
    fn default() -> Self {
        Self::new()
    }
}

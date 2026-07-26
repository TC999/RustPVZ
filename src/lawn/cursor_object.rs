// stub
use crate::const_enums::*;
use super::game_object::GameObject;

pub struct CursorObject {
    pub m_type: CursorType,
    pub m_seed_type: SeedType,
    pub m_x: i32, pub m_y: i32,
}

pub struct CursorPreview {
    pub m_x: i32, pub m_y: i32,
}

pub struct MessageWidget;

pub struct SeedBank;

pub struct GameButton {
    pub m_id: i32,
    pub m_x: i32, pub m_y: i32,
    pub m_width: i32, pub m_height: i32,
}

pub struct ToolTipWidget;

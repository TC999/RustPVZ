// [TRANSLATION_NOTE]: Zombatar.h -> Rust 翻译
// 仅头文件的 Zombatar 记录操作函数和颜色表

#![allow(non_snake_case, dead_code)]

use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::common::{from_le32, to_le32};
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombatarRecordSlot {
    ZOMBATAR_SLOT_SKIN_PART = 0,
    ZOMBATAR_SLOT_SKIN_COLOR = 1,
    ZOMBATAR_SLOT_CLOTHES = 2,
    ZOMBATAR_SLOT_CLOTHES_COLOR = 3,
    ZOMBATAR_SLOT_TIDBITS = 4,
    ZOMBATAR_SLOT_TIDBITS_COLOR = 5,
    ZOMBATAR_SLOT_ACCESSORY = 6,
    ZOMBATAR_SLOT_ACCESSORY_COLOR = 7,
    ZOMBATAR_SLOT_FACIAL_HAIR = 8,
    ZOMBATAR_SLOT_FACIAL_HAIR_COLOR = 9,
    ZOMBATAR_SLOT_HAIR = 10,
    ZOMBATAR_SLOT_HAIR_COLOR = 11,
    ZOMBATAR_SLOT_EYEWEAR = 12,
    ZOMBATAR_SLOT_EYEWEAR_COLOR = 13,
    ZOMBATAR_SLOT_HATS = 14,
    ZOMBATAR_SLOT_HATS_COLOR = 15,
    ZOMBATAR_SLOT_BACKGROUND = 16,
    ZOMBATAR_SLOT_BACKGROUND_COLOR = 17,
}

pub fn ZombatarReadRecordSlot(theRecord: &[u8], theSlot: i32) -> u32 {
    let offset = (theSlot * 4) as usize;
    let mut aValue = [0u8; 4];
    aValue.copy_from_slice(&theRecord[offset..offset + 4]);
    from_le32(u32::from_le_bytes(aValue))
}

pub fn ZombatarReadSignedRecordSlot(theRecord: &[u8], theSlot: i32) -> i32 {
    let aValue = ZombatarReadRecordSlot(theRecord, theSlot);
    if aValue > i32::MAX as u32 {
        -1
    } else {
        aValue as i32
    }
}

pub fn ZombatarWriteRecordSlot(theRecord: &mut [u8], theSlot: i32, theValue: i32) {
    let offset = (theSlot * 4) as usize;
    let aValue = to_le32(theValue as u32);
    let bytes = aValue.to_le_bytes();
    theRecord[offset..offset + 4].copy_from_slice(&bytes);
}

pub static G_ZOMBATAR_COLORS: [Color; 48] = [
    Color { m_red: 134, m_green: 147, m_blue: 122, m_alpha: 255 }, Color { m_red: 79, m_green: 135, m_blue: 94, m_alpha: 255 }, Color { m_red: 127, m_green: 135, m_blue: 94, m_alpha: 255 }, Color { m_red: 120, m_green: 130, m_blue: 50, m_alpha: 255 },
    Color { m_red: 156, m_green: 163, m_blue: 105, m_alpha: 255 }, Color { m_red: 96, m_green: 151, m_blue: 11, m_alpha: 255 }, Color { m_red: 147, m_green: 184, m_blue: 77, m_alpha: 255 }, Color { m_red: 82, m_green: 143, m_blue: 54, m_alpha: 255 },
    Color { m_red: 121, m_green: 168, m_blue: 99, m_alpha: 255 }, Color { m_red: 65, m_green: 156, m_blue: 74, m_alpha: 255 }, Color { m_red: 107, m_green: 178, m_blue: 114, m_alpha: 255 }, Color { m_red: 104, m_green: 121, m_blue: 90, m_alpha: 255 },
    Color { m_red: 151, m_green: 33, m_blue: 33, m_alpha: 255 }, Color { m_red: 199, m_green: 53, m_blue: 53, m_alpha: 255 }, Color { m_red: 220, m_green: 112, m_blue: 47, m_alpha: 255 }, Color { m_red: 251, m_green: 251, m_blue: 172, m_alpha: 255 },
    Color { m_red: 240, m_green: 210, m_blue: 87, m_alpha: 255 }, Color { m_red: 165, m_green: 126, m_blue: 65, m_alpha: 255 }, Color { m_red: 106, m_green: 72, m_blue: 32, m_alpha: 255 }, Color { m_red: 72, m_green: 35, m_blue: 5, m_alpha: 255 },
    Color { m_red: 50, m_green: 56, m_blue: 61, m_alpha: 255 }, Color { m_red: 0, m_green: 0, m_blue: 10, m_alpha: 255 }, Color { m_red: 197, m_green: 239, m_blue: 239, m_alpha: 255 }, Color { m_red: 63, m_green: 109, m_blue: 242, m_alpha: 255 },
    Color { m_red: 13, m_green: 202, m_blue: 151, m_alpha: 255 }, Color { m_red: 158, m_green: 183, m_blue: 19, m_alpha: 255 }, Color { m_red: 30, m_green: 210, m_blue: 64, m_alpha: 255 }, Color { m_red: 225, m_green: 65, m_blue: 230, m_alpha: 255 },
    Color { m_red: 128, m_green: 47, m_blue: 204, m_alpha: 255 }, Color { m_red: 255, m_green: 255, m_blue: 255, m_alpha: 255 }, Color { m_red: 238, m_green: 19, m_blue: 24, m_alpha: 255 }, Color { m_red: 247, m_green: 89, m_blue: 215, m_alpha: 255 },
    Color { m_red: 239, m_green: 198, m_blue: 253, m_alpha: 255 }, Color { m_red: 160, m_green: 56, m_blue: 241, m_alpha: 255 }, Color { m_red: 86, m_green: 74, m_blue: 241, m_alpha: 255 }, Color { m_red: 74, m_green: 160, m_blue: 241, m_alpha: 255 },
    Color { m_red: 199, m_green: 244, m_blue: 251, m_alpha: 255 }, Color { m_red: 49, m_green: 238, m_blue: 237, m_alpha: 255 }, Color { m_red: 16, m_green: 194, m_blue: 66, m_alpha: 255 }, Color { m_red: 112, m_green: 192, m_blue: 33, m_alpha: 255 },
    Color { m_red: 16, m_green: 145, m_blue: 52, m_alpha: 255 }, Color { m_red: 248, m_green: 247, m_blue: 41, m_alpha: 255 }, Color { m_red: 227, m_green: 180, m_blue: 20, m_alpha: 255 }, Color { m_red: 241, m_green: 115, m_blue: 25, m_alpha: 255 },
    Color { m_red: 248, m_green: 247, m_blue: 175, m_alpha: 255 }, Color { m_red: 103, m_green: 85, m_blue: 54, m_alpha: 255 }, Color { m_red: 159, m_green: 17, m_blue: 20, m_alpha: 255 }, Color { m_red: 255, m_green: 255, m_blue: 255, m_alpha: 255 },
];

pub fn ZombatarGetColor(theIndex: i32) -> Color {
    if theIndex < 0 {
        return Color { m_red: 255, m_green: 255, m_blue: 255, m_alpha: 255 };
    }
    let max_index = (G_ZOMBATAR_COLORS.len() - 1) as i32;
    G_ZOMBATAR_COLORS[cmp::min(theIndex, max_index) as usize]
}

pub fn ZombatarRemapAccessoryForRuntime(theIndex: i32) -> i32 {
    match theIndex {
        5 => 14,
        6 => 5,
        7 => 6,
        8 => 12,
        9 => 7,
        10 => 9,
        11 => 10,
        12 => 11,
        14 => 8,
        _ => theIndex,
    }
}

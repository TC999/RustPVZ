// [TRANSLATION_NOTE]: MessageWidget.h + MessageWidget.cpp -> Rust 翻译
// 游戏消息/提示文字控件。依赖 Reanimation 系统，部分方法暂为 stub

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Font};
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::resources::{FONT_HOUSEOFTERROR28, FONT_HOUSEOFTERROR16, FONT_CONTINUUMBOLD14, FONT_CONTINUUMBOLD14OUTLINE};
use crate::sexy_tod_lib::tod_string_file::tod_string_translate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageStyle {
    MESSAGE_STYLE_OFF = 0,
    MESSAGE_STYLE_TUTORIAL_LEVEL1 = 1,
    MESSAGE_STYLE_TUTORIAL_LEVEL1_STAY = 2,
    MESSAGE_STYLE_TUTORIAL_LEVEL2 = 3,
    MESSAGE_STYLE_TUTORIAL_LATER = 4,
    MESSAGE_STYLE_TUTORIAL_LATER_STAY = 5,
    MESSAGE_STYLE_HINT_LONG = 6,
    MESSAGE_STYLE_HINT_FAST = 7,
    MESSAGE_STYLE_HINT_STAY = 8,
    MESSAGE_STYLE_HINT_TALL_FAST = 9,
    MESSAGE_STYLE_HINT_TALL_UNLOCKMESSAGE = 10,
    MESSAGE_STYLE_HINT_TALL_LONG = 11,
    MESSAGE_STYLE_BIG_MIDDLE = 12,
    MESSAGE_STYLE_BIG_MIDDLE_FAST = 13,
    MESSAGE_STYLE_HOUSE_NAME = 14,
    MESSAGE_STYLE_HUGE_WAVE = 15,
    MESSAGE_STYLE_SLOT_MACHINE = 16,
    MESSAGE_STYLE_ZEN_GARDEN_LONG = 17,
    MESSAGE_STYLE_ACHIEVEMENT = 18,
}

pub const MAX_MESSAGE_LENGTH: usize = 128;
pub const MAX_REANIM_LINES: i32 = 5;

pub struct MessageWidget {
    pub mApp: *mut crate::lawn_app::LawnApp,
    pub mLabel: [u8; MAX_MESSAGE_LENGTH],
    pub mDisplayTime: i32,
    pub mDuration: i32,
    pub mMessageStyle: MessageStyle,
    pub mTextReanimID: [ReanimationID; MAX_MESSAGE_LENGTH],
    pub mTextReanimByteOffset: [i32; MAX_MESSAGE_LENGTH],
    pub mTextReanimCount: i32,
    pub mReanimType: ReanimationType,
    pub mSlideOffTime: i32,
    pub mLabelNext: [u8; MAX_MESSAGE_LENGTH],
    pub mMessageStyleNext: MessageStyle,
}

impl MessageWidget {
    pub fn new(theApp: *mut crate::lawn_app::LawnApp) -> Self {
        MessageWidget {
            mApp: theApp,
            mLabel: [0u8; MAX_MESSAGE_LENGTH],
            mDisplayTime: 0,
            mDuration: 0,
            mMessageStyle: MessageStyle::MESSAGE_STYLE_OFF,
            mTextReanimID: [ReanimationID::REANIMATIONID_NULL; MAX_MESSAGE_LENGTH],
            mTextReanimByteOffset: [0i32; MAX_MESSAGE_LENGTH],
            mTextReanimCount: 0,
            mReanimType: ReanimationType::REANIM_NONE,
            mSlideOffTime: 100,
            mLabelNext: [0u8; MAX_MESSAGE_LENGTH],
            mMessageStyleNext: MessageStyle::MESSAGE_STYLE_OFF,
        }
    }

    pub fn ClearReanim(&mut self) {
        // Reanimation 系统未完全实现，暂为 stub
        for i in 0..MAX_MESSAGE_LENGTH {
            self.mTextReanimID[i] = ReanimationID::REANIMATIONID_NULL;
        }
    }

    pub fn ClearLabel(&mut self) {
        if self.mReanimType != ReanimationType::REANIM_NONE {
            self.mDuration = std::cmp::min(self.mDuration, 100 + self.mSlideOffTime + 1);
        } else {
            self.mDuration = 0;
        }
    }

    pub fn SetLabel(&mut self, theNewLabel: &str, theMessageStyle: MessageStyle) {
        let aLabel = tod_string_translate(theNewLabel);
        if aLabel.len() >= MAX_MESSAGE_LENGTH - 1 {
            return;
        }

        if self.mReanimType != ReanimationType::REANIM_NONE && self.mDuration > 0 {
            self.mMessageStyleNext = theMessageStyle;
            let bytes = aLabel.as_bytes();
            let len = std::cmp::min(bytes.len(), MAX_MESSAGE_LENGTH - 1);
            self.mLabelNext[..len].copy_from_slice(&bytes[..len]);
            self.mLabelNext[len] = 0;
            self.ClearLabel();
        } else {
            self.ClearReanim();
            let bytes = aLabel.as_bytes();
            let len = std::cmp::min(bytes.len(), MAX_MESSAGE_LENGTH - 1);
            self.mLabel[..len].copy_from_slice(&bytes[..len]);
            self.mLabel[len] = 0;
            self.mMessageStyle = theMessageStyle;
            self.mReanimType = ReanimationType::REANIM_NONE;

            self.mDuration = match theMessageStyle {
                MessageStyle::MESSAGE_STYLE_HINT_LONG
                | MessageStyle::MESSAGE_STYLE_BIG_MIDDLE
                | MessageStyle::MESSAGE_STYLE_ZEN_GARDEN_LONG
                | MessageStyle::MESSAGE_STYLE_HINT_TALL_LONG => 1500,

                MessageStyle::MESSAGE_STYLE_HINT_TALL_UNLOCKMESSAGE => 500,

                MessageStyle::MESSAGE_STYLE_HINT_FAST
                | MessageStyle::MESSAGE_STYLE_HINT_TALL_FAST
                | MessageStyle::MESSAGE_STYLE_BIG_MIDDLE_FAST
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL1
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL2
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LATER => 500,

                MessageStyle::MESSAGE_STYLE_HINT_STAY
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL1_STAY
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LATER_STAY => 10000,

                MessageStyle::MESSAGE_STYLE_HOUSE_NAME => 250,

                MessageStyle::MESSAGE_STYLE_HUGE_WAVE => {
                    self.mReanimType = ReanimationType::REANIM_TEXT_FADE_ON;
                    750
                }

                MessageStyle::MESSAGE_STYLE_SLOT_MACHINE => 750,

                MessageStyle::MESSAGE_STYLE_ACHIEVEMENT => 250,

                _ => 0,
            };

            // LayoutReanimText 暂为 stub
            self.mDisplayTime = self.mDuration;
        }
    }

    pub fn GetFont(&self) -> *mut Font {
        match self.mMessageStyle {
            MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL1
            | MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL1_STAY
            | MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL2
            | MessageStyle::MESSAGE_STYLE_TUTORIAL_LATER
            | MessageStyle::MESSAGE_STYLE_TUTORIAL_LATER_STAY
            | MessageStyle::MESSAGE_STYLE_HINT_LONG
            | MessageStyle::MESSAGE_STYLE_HINT_FAST
            | MessageStyle::MESSAGE_STYLE_HINT_STAY
            | MessageStyle::MESSAGE_STYLE_HINT_TALL_FAST
            | MessageStyle::MESSAGE_STYLE_HINT_TALL_UNLOCKMESSAGE
            | MessageStyle::MESSAGE_STYLE_HINT_TALL_LONG
            | MessageStyle::MESSAGE_STYLE_BIG_MIDDLE
            | MessageStyle::MESSAGE_STYLE_BIG_MIDDLE_FAST
            | MessageStyle::MESSAGE_STYLE_HOUSE_NAME
            | MessageStyle::MESSAGE_STYLE_HUGE_WAVE
            | MessageStyle::MESSAGE_STYLE_ZEN_GARDEN_LONG
            | MessageStyle::MESSAGE_STYLE_ACHIEVEMENT => unsafe { FONT_HOUSEOFTERROR28 },

            MessageStyle::MESSAGE_STYLE_SLOT_MACHINE => unsafe { FONT_HOUSEOFTERROR16 },
            _ => std::ptr::null_mut(),
        }
    }

    pub fn Update(&mut self) {
        unsafe {
            if self.mApp.is_null() {
                return;
            }
            let app = &*self.mApp;
            if app.m_board.is_none() {
                return;
            }
            let board = app.m_board.as_ref().unwrap();
            if board.mPaused {
                return;
            }
        }

        if self.mDuration < 10000 && self.mDuration > 0 {
            self.mDuration -= 1;
            if self.mDuration == 0 {
                self.mMessageStyle = MessageStyle::MESSAGE_STYLE_OFF;
                if self.mMessageStyleNext != MessageStyle::MESSAGE_STYLE_OFF {
                    // 通过字节数组重建字符串
                    let label_str = self.cstr_next_to_string();
                    self.SetLabel(&label_str, self.mMessageStyleNext);
                    self.mMessageStyleNext = MessageStyle::MESSAGE_STYLE_OFF;
                }
            }
        }

        // Reanim 文本动画更新 — 暂为 stub
    }

    fn cstr_label_to_string(&self) -> String {
        let mut s = String::new();
        for &b in self.mLabel.iter() {
            if b == 0 { break; }
            s.push(b as char);
        }
        s
    }

    fn cstr_next_to_string(&self) -> String {
        let mut s = String::new();
        for &b in self.mLabelNext.iter() {
            if b == 0 { break; }
            s.push(b as char);
        }
        s
    }

    pub fn Draw(&self, g: &mut Graphics) {
        if self.mDuration <= 0 {
            return;
        }

        let aFont = self.GetFont();
        if aFont.is_null() {
            return;
        }

        let label_str = self.cstr_label_to_string();

        unsafe {
            let mut aColor = match self.mMessageStyle {
                MessageStyle::MESSAGE_STYLE_HUGE_WAVE => Color { m_red: 255, m_green: 0, m_blue: 0, m_alpha: 255 },
                MessageStyle::MESSAGE_STYLE_HOUSE_NAME => Color { m_red: 255, m_green: 255, m_blue: 255, m_alpha: 255 },
                _ => Color { m_red: 253, m_green: 245, m_blue: 173, m_alpha: 255 },
            };

            let aPosY: i32 = match self.mMessageStyle {
                MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL1
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL1_STAY => 400,
                MessageStyle::MESSAGE_STYLE_TUTORIAL_LEVEL2
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LATER
                | MessageStyle::MESSAGE_STYLE_TUTORIAL_LATER_STAY
                | MessageStyle::MESSAGE_STYLE_HINT_TALL_FAST
                | MessageStyle::MESSAGE_STYLE_HINT_TALL_UNLOCKMESSAGE
                | MessageStyle::MESSAGE_STYLE_HINT_TALL_LONG
                | MessageStyle::MESSAGE_STYLE_ACHIEVEMENT => 476,
                MessageStyle::MESSAGE_STYLE_HINT_LONG
                | MessageStyle::MESSAGE_STYLE_HINT_FAST
                | MessageStyle::MESSAGE_STYLE_HINT_STAY => 527,
                MessageStyle::MESSAGE_STYLE_BIG_MIDDLE
                | MessageStyle::MESSAGE_STYLE_BIG_MIDDLE_FAST => 300,
                MessageStyle::MESSAGE_STYLE_HOUSE_NAME => 550,
                MessageStyle::MESSAGE_STYLE_HUGE_WAVE => 330,
                MessageStyle::MESSAGE_STYLE_SLOT_MACHINE => 93,
                MessageStyle::MESSAGE_STYLE_ZEN_GARDEN_LONG => 514,
                _ => 596,
            };

            // Reanim 文本 — 暂为 stub
            // 使用普通文本绘制
            let aFontPtr = aFont;
            g.SetFont(aFontPtr);
            let _width = (*aFontPtr).string_width(&label_str);
            let _ascent = (*aFontPtr).get_ascent();
            g.SetColor(aColor);
            g.DrawString(&label_str, 400 - _width / 2, aPosY + _ascent);
        }
    }
}

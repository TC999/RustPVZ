// [TRANSLATION_NOTE]: ToolTipWidget.h + ToolTipWidget.cpp -> Rust 翻译
// 工具提示控件。Graphics 的绘图方法尚未完全实现，使用 stub

#![allow(non_snake_case, dead_code)]

use crate::game_constants::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::sexy_app_framework::graphics::graphics::{Graphics, Font};
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::resources::{FONT_PICO129, FONT_TINYBOLD};
use crate::sexy_app_framework::common::{is_opening_punctuation, is_closing_punctuation};
use crate::sexy_tod_lib::tod_string_file::tod_string_translate;
use std::cmp;

pub struct ToolTipWidget {
    pub mTitle: String,
    pub mLabel: String,
    pub mWarningText: String,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mVisible: bool,
    pub mCenter: bool,
    pub mMinLeft: i32,
    pub mMaxBottom: i32,
    pub mGetsLinesWidth: i32,
    pub mWarningFlashCounter: i32,
    pub mMaxLinesWidth: i32,
}

impl ToolTipWidget {
    pub fn new() -> Self {
        ToolTipWidget {
            mTitle: String::new(),
            mLabel: String::new(),
            mWarningText: String::new(),
            mX: 0, mY: 0, mWidth: 0, mHeight: 0,
            mVisible: true,
            mCenter: false,
            mMinLeft: 0,
            mMaxBottom: BOARD_HEIGHT,
            mGetsLinesWidth: 0,
            mWarningFlashCounter: 0,
            mMaxLinesWidth: 0,
        }
    }

    pub fn GetLines(&self, theLines: &mut Vec<String>) {
        let label = &self.mLabel;
        let mut aLineWidth: i32 = 0;
        let mut aLineStart: usize = 0;
        let mut aPrevChar: u32 = 0;

        let mut aBreakDrawLen: i32 = -1;
        let mut aBreakResumePos: usize = 0;

        let chars: Vec<(usize, char)> = label.char_indices().collect();
        let mut i = 0;

        while i < chars.len() {
            let (aCharStart, aChar) = chars[i];
            let aCharU32 = aChar as u32;

            if aChar == '\r' {
                i += 1;
                continue;
            }

            let aCharEnd = if i + 1 < chars.len() { chars[i + 1].0 } else { label.len() };

            if aChar == '\n' {
                theLines.push(label[aLineStart..aCharStart].to_string());
                aLineWidth = 0;
                aLineStart = aCharEnd;
                aBreakDrawLen = -1;
                aPrevChar = 0;
                i += 1;
                continue;
            }

            unsafe {
                aLineWidth += (*FONT_PICO129).char_width(aChar);
            }

            if aChar == ' ' {
                aBreakDrawLen = (aCharStart - aLineStart) as i32;
                aBreakResumePos = aCharEnd;
                if aLineWidth >= self.mGetsLinesWidth {
                    let end = aLineStart + aBreakDrawLen as usize;
                    theLines.push(label[aLineStart..end].to_string());
                    let mut cur = aBreakResumePos;
                    while cur < label.len() && label.as_bytes()[cur] == b' ' {
                        cur += 1;
                        i += 1;
                    }
                    aLineStart = cur;
                    aLineWidth = 0;
                    aBreakDrawLen = -1;
                    aPrevChar = 0;
                    continue;
                }
            } else if is_auto_break_char(aCharU32)
                && !is_closing_punctuation(aChar)
                && aCharStart > aLineStart
                && !is_opening_punctuation(aPrevChar as u8 as char)
            {
                aBreakDrawLen = (aCharStart - aLineStart) as i32;
                aBreakResumePos = aCharStart;
                if aLineWidth >= self.mGetsLinesWidth {
                    let end = aLineStart + aBreakDrawLen as usize;
                    theLines.push(label[aLineStart..end].to_string());
                    aLineStart = aBreakResumePos;
                    aLineWidth = 0;
                    aBreakDrawLen = -1;
                    aPrevChar = 0;
                    continue;
                }
            }
            aPrevChar = aCharU32;
            i += 1;
        }

        if aLineStart < label.len() {
            theLines.push(label[aLineStart..].to_string());
        }
    }

    pub fn CalculateSize(&mut self) {
        let mut aLines: Vec<String> = Vec::new();
        unsafe {
            let aTitleWidth = (*FONT_TINYBOLD).string_width(&self.mTitle);
            let aWarningWidth = (*FONT_PICO129).string_width(&self.mWarningText);
            let mut aMaxWidth = cmp::max(aTitleWidth, aWarningWidth);

            self.mGetsLinesWidth = cmp::max(aMaxWidth - 30, 100);
            if self.mMaxLinesWidth > 0 {
                self.mGetsLinesWidth = cmp::min(self.mGetsLinesWidth, self.mMaxLinesWidth);
            }
            self.GetLines(&mut aLines);

            for line in &aLines {
                let aLineWidth = (*FONT_PICO129).string_width(line);
                aMaxWidth = cmp::max(aMaxWidth, aLineWidth);
            }

            let mut aHeight = 6;
            if !self.mTitle.is_empty() {
                aHeight = (*FONT_TINYBOLD).get_ascent() + 8;
            }
            if !self.mWarningText.is_empty() {
                aHeight += (*FONT_TINYBOLD).get_ascent() + 2;
            }
            aHeight += (aLines.len() as i32) * (*FONT_PICO129).get_ascent();

            self.mWidth = aMaxWidth + 10;
            self.mHeight = aHeight + (aLines.len() as i32) * 2 - 2;
        }
    }

    pub fn SetLabel(&mut self, theLabel: &str) {
        self.mLabel = tod_string_translate(theLabel);
        self.CalculateSize();
    }

    pub fn SetTitle(&mut self, theTitle: &str) {
        self.mTitle = tod_string_translate(theTitle);
        self.CalculateSize();
    }

    pub fn SetWarningText(&mut self, theWarningText: &str) {
        self.mWarningText = tod_string_translate(theWarningText);
        self.CalculateSize();
    }

    pub fn Draw(&self, g: &mut Graphics) {
        if !self.mVisible {
            return;
        }

        let mut aPosX = self.mX;
        if self.mCenter {
            aPosX -= self.mWidth / 2;
        }
        if self.mMinLeft - g.state.m_trans_x as i32 > aPosX {
            aPosX = self.mMinLeft - g.state.m_trans_x as i32;
        } else if aPosX + self.mWidth + g.state.m_trans_x as i32 > BOARD_WIDTH {
            aPosX = BOARD_WIDTH - g.state.m_trans_x as i32 - self.mWidth;
        }

        let mut aPosY = self.mY;
        if -(g.state.m_trans_y as i32) > aPosY {
            aPosY = -(g.state.m_trans_y as i32);
        } else if self.mMaxBottom < self.mY + self.mHeight + g.state.m_trans_y as i32 {
            aPosY = self.mMaxBottom - g.state.m_trans_y as i32 - self.mHeight;
        }

        g.SetColor(Color { m_red: 255, m_green: 255, m_blue: 200, m_alpha: 255 });
        g.FillRect(aPosX, aPosY, self.mWidth, self.mHeight);
        g.SetColor(Color { m_red: 0, m_green: 0, m_blue: 0, m_alpha: 255 });
        g.DrawRect(aPosX, aPosY, self.mWidth - 1, self.mHeight - 1);
        let mut aPosY = aPosY + 1;

        if !self.mTitle.is_empty() {
            unsafe { g.SetFont(FONT_TINYBOLD); }
            let title_width = unsafe { (*FONT_TINYBOLD).string_width(&self.mTitle) };
            let ascent = unsafe { (*FONT_TINYBOLD).get_ascent() };
            g.DrawString(&self.mTitle, aPosX + (self.mWidth - title_width) / 2, aPosY + ascent);
            aPosY += ascent + 2;
        }

        if !self.mWarningText.is_empty() {
            unsafe { g.SetFont(FONT_PICO129); }
            let warn_width = unsafe { (*FONT_PICO129).string_width(&self.mWarningText) };
            let ascent = unsafe { (*FONT_PICO129).get_ascent() };
            let x = aPosX + (self.mWidth - warn_width) / 2;
            let y = aPosY + ascent;

            let mut aWarningColor = Color { m_red: 255, m_green: 0, m_blue: 0, m_alpha: 255 };
            if self.mWarningFlashCounter > 0 && self.mWarningFlashCounter % 20 < 10 {
                aWarningColor = Color { m_red: 0, m_green: 0, m_blue: 0, m_alpha: 255 };
            }

            g.SetColor(aWarningColor);
            g.DrawString(&self.mWarningText, x, y);
            g.SetColor(Color { m_red: 0, m_green: 0, m_blue: 0, m_alpha: 255 });

            aPosY += ascent + 2;
        }

        let mut aLines: Vec<String> = Vec::new();
        self.GetLines(&mut aLines);

        unsafe { g.SetFont(FONT_PICO129); }
        for line in &aLines {
            let ascent = unsafe { (*FONT_PICO129).get_ascent() };
            let line_width = unsafe { (*FONT_PICO129).string_width(line) };
            g.DrawString(line, aPosX + (self.mWidth - line_width) / 2, aPosY + ascent);
            aPosY += ascent + 2;
        }
    }

    pub fn FlashWarning(&mut self) {
        self.mWarningFlashCounter = 70;
    }

    pub fn Update(&mut self) {
        if self.mWarningFlashCounter > 0 {
            self.mWarningFlashCounter -= 1;
        }
    }

    pub fn SetPosition(&mut self, theX: i32, theY: i32) {
        self.mX = theX;
        self.mY = theY;
    }
}

/// 近似 C++ 的 Sexy::IsAutoBreakChar 逻辑
fn is_auto_break_char(c: u32) -> bool {
    matches!(c as u8 as char, ',' | '.' | ';' | ':' | '!' | '?' | ')' | ']' | '}' | '”' | '』' | '】')
}

// [TRANSLATION_NOTE]: LawnCommon.h + LawnCommon.cpp -> Rust 翻译
// LawnEditWidget 继承 EditWidget，在 Rust 中使用组合 + 裸指针模拟
// Dialog, EditWidget, Checkbox 等类型尚未实现，使用 *mut c_void 作为 opaque 指针

#![allow(non_snake_case, dead_code)]

use std::cmp;
use std::ptr;

use crate::const_enums::GameMode;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Image, Font};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::common::get_app_data_path;
use crate::sexy_app_framework::resources::{IMAGE_EDITBOX, FONT_BRIANNETOD16};

// C++ 中的 gLawnEditWidgetColors 二维数组
pub static G_LAWN_EDIT_WIDGET_COLORS: [[i32; 4]; 5] = [
    [0,   0,   0,   0],
    [0,   0,   0,   0],
    [240, 240, 255, 255],
    [255, 255, 255, 255],
    [0,   0,   0,   255],
];

// ====================================================================================================
// ★ 常用逻辑判断
// ====================================================================================================

/// 判断在 [theNumber - theRange, theNumber + theRange] 区间内是否存在 theMod 的整数倍数
pub fn ModInRange(theNumber: i32, theMod: i32, theRange: i32) -> bool {
    let theRange = theRange.abs();
    let mut i = theNumber - theRange;
    while i <= theNumber + theRange {
        if i % theMod == 0 {
            return true;
        }
        i += 1;
    }
    false
}

/// 判断点 (x1, y1) 是否位于点 (x2, y2) 周围的 (theRangeX, theRangeY) 范围内
pub fn GridInRange(x1: i32, y1: i32, x2: i32, y2: i32, theRangeX: i32, theRangeY: i32) -> bool {
    x1 >= x2 - theRangeX && x1 <= x2 + theRangeX && y1 >= y2 - theRangeY && y1 <= y2 + theRangeY
}

// ====================================================================================================
// ★ 动画、特效与绘制相关
// ====================================================================================================

pub fn TileImageHorizontally(g: &mut Graphics, theImage: *mut Image, theX: i32, theY: i32, theWidth: i32) {
    let mut theWidth = theWidth;
    let mut theX = theX;
    while theWidth > 0 {
        let aImageWidth;
        unsafe {
            aImageWidth = cmp::min(theWidth, (*theImage).get_width());
        }
        // g->DrawImage(theImage, theX, theY, Rect(0, 0, aImageWidth, theImage->GetHeight()));
        // DrawImage 尚未完全实现，使用 stub
        let _ = g;
        let _ = theImage;
        theX += aImageWidth;
        theWidth -= aImageWidth;
    }
}

pub fn TileImageVertically(g: &mut Graphics, theImage: *mut Image, theX: i32, theY: i32, theHeight: i32) {
    let mut theHeight = theHeight;
    let mut theY = theY;
    while theHeight > 0 {
        let aImageHeight;
        unsafe {
            aImageHeight = cmp::min(theHeight, (*theImage).get_height());
        }
        // g->DrawImage(theImage, theX, theY, Rect(0, 0, theImage->GetWidth(), aImageHeight));
        let _ = g;
        let _ = theImage;
        theY += aImageHeight;
        theHeight -= aImageHeight;
    }
}

// ====================================================================================================
// ★ LawnEditWidget
// ====================================================================================================

/// LawnEditWidget 类 — 对应 C++ 中继承自 EditWidget 的编辑框控件
/// Dialog, EditWidget 等基类尚未实现，使用 *mut c_void 作为 opaque 指针
pub struct LawnEditWidget {
    // EditWidget 基类部分（用 opaque 指针模拟）
    pub mBase: *mut std::ffi::c_void,
    pub mDialog: *mut std::ffi::c_void,  // Dialog*
    pub mAutoCapFirstLetter: bool,
    pub mId: i32,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mFont: *mut Font,
}

impl LawnEditWidget {
    pub fn new(theId: i32, _theListener: *mut std::ffi::c_void, theDialog: *mut std::ffi::c_void) -> Self {
        LawnEditWidget {
            mBase: ptr::null_mut(),
            mDialog: theDialog,
            mAutoCapFirstLetter: true,
            mId: theId,
            mX: 0,
            mY: 0,
            mWidth: 0,
            mHeight: 0,
            mFont: ptr::null_mut(),
        }
        // 注意：C++ 中调用 EditWidget(theId, theListener) 的基类构造函数
    }

    pub fn KeyDown(&mut self, theKey: i32) {
        // EditWidget::KeyDown(theKey);  // 基类方法暂未实现
        if theKey == 27 /* KEYCODE_ESCAPE */ {
            // 转发到 Dialog（暂未实现）
        }
    }

    pub fn KeyChar(&mut self, _theChar: u8) {
        let mut ch = _theChar;
        if self.mAutoCapFirstLetter && auto_cap_char(&mut ch) {
            self.mAutoCapFirstLetter = false;
        }
        // EditWidget::KeyChar(theChar);
    }

    pub fn KeyText(&mut self, theText: &str) {
        if !self.mAutoCapFirstLetter {
            // EditWidget::KeyText(theText);
            return;
        }

        let mut aTextBytes = theText.as_bytes().to_vec();
        for aCh in aTextBytes.iter_mut() {
            if auto_cap_char(aCh) {
                self.mAutoCapFirstLetter = false;
                break;
            }
        }

        // EditWidget::KeyText(aText);  // 基类方法暂未实现
    }

    pub fn SetFont(&mut self, theFont: *mut Font) {
        self.mFont = theFont;
    }

    pub fn SetColors(&mut self, _colors: &[[i32; 4]; 5], _numColors: i32) {
        // stub
    }
}

// Uppercase ASCII letters in place; locale-independent, safe on UTF-8 bytes.
fn auto_cap_char(theChar: &mut u8) -> bool {
    if *theChar >= b'a' && *theChar <= b'z' {
        *theChar = *theChar - b'a' + b'A';
        return true;
    }
    *theChar >= b'A' && *theChar <= b'Z'
}

// ====================================================================================================
// ★ 控件工厂函数
// ====================================================================================================

pub fn CreateEditWidget(theId: i32, theListener: *mut std::ffi::c_void, theDialog: *mut std::ffi::c_void) -> LawnEditWidget {
    let mut aEditWidget = LawnEditWidget::new(theId, theListener, theDialog);
    unsafe {
        aEditWidget.SetFont(FONT_BRIANNETOD16);
    }
    aEditWidget.SetColors(&G_LAWN_EDIT_WIDGET_COLORS, 5 /* NUM_COLORS */);
    aEditWidget.mWidth = 200;
    aEditWidget.mHeight = 24;
    aEditWidget
}

pub fn DrawEditBox(g: &mut Graphics, theWidget: &LawnEditWidget) {
    let aDest = Rect::new(
        theWidget.mX - 8,
        theWidget.mY - 4,
        theWidget.mWidth + 16,
        theWidget.mHeight + 8,
    );
    // g->DrawImageBox(aDest, IMAGE_EDITBOX);
    // DrawImageBox 尚未实现，使用 stub
    let _ = g;
    let _ = aDest;
}

// ====================================================================================================
// ★ Checkbox 工厂
// ====================================================================================================

/// Checkbox 的 Rust 映射（partial stub）
pub struct Checkbox {
    pub mChecked: bool,
    pub mHasAlpha: bool,
    pub mHasTransparencies: bool,
    pub mId: i32,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
}

impl Checkbox {
    pub fn new(theId: i32, _theListener: *mut std::ffi::c_void) -> Self {
        Checkbox {
            mChecked: false,
            mHasAlpha: true,
            mHasTransparencies: true,
            mId: theId,
            mX: 0,
            mY: 0,
            mWidth: 20,
            mHeight: 20,
        }
    }
}

pub fn MakeNewCheckbox(theId: i32, theListener: *mut std::ffi::c_void, theDefault: bool) -> Checkbox {
    let mut aCheckbox = Checkbox::new(theId, theListener);
    aCheckbox.mChecked = theDefault;
    aCheckbox.mHasAlpha = true;
    aCheckbox.mHasTransparencies = true;
    aCheckbox
}

// ====================================================================================================
// ★ 路径与存档
// ====================================================================================================

pub fn GetSavedGameName(theGameMode: GameMode, theProfileId: i32) -> String {
    get_app_data_path(&format!("userdata/game{}_{}.v4", theProfileId, theGameMode as i32))
}

pub fn GetLegacySavedGameName(theGameMode: GameMode, theProfileId: i32) -> String {
    get_app_data_path(&format!("userdata/game{}_{}.dat", theProfileId, theGameMode as i32))
}

pub fn GetCurrentDaysSince2000(theTime: i64) -> i32 {
    // C++: tm aNowTM = gLawnApp->GetLocalTime(theTime);
    let secs_per_day: i64 = 86400;
    let days_since_epoch = theTime / secs_per_day;
    // 从 2000-01-01 到 1970-01-01 是 10957 天
    let days_since_2000 = days_since_epoch - 10957;
    days_since_2000 as i32
}

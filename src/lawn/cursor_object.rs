// stub - CursorObject, CursorPreview, MessageWidget, GameButton, ToolTipWidget

use crate::const_enums::*;

pub struct CursorObject {
    pub mType: CursorType,
    pub mSeedType: SeedType,
    pub mX: i32, pub mY: i32,
}

impl CursorObject {
    pub fn new() -> Self {
        CursorObject {
            mType: CursorType::CURSOR_TYPE_NORMAL,
            mSeedType: SeedType::SEED_NONE,
            mX: 0, mY: 0,
        }
    }
}

pub struct CursorPreview;

pub struct MessageWidget;

pub struct GameButton {
    pub mId: i32,
    pub mX: i32, pub mY: i32,
    pub mWidth: i32, pub mHeight: i32,
    pub mDrawStoneButton: bool,
    pub mBtnNoDraw: bool,
    pub mDisabled: bool,
    pub mParentWidget: *mut std::ffi::c_void,
    pub mButtonImage: *mut std::ffi::c_void,
    pub mOverImage: *mut std::ffi::c_void,
    pub mDownImage: *mut std::ffi::c_void,
}

impl GameButton {
    pub fn new(id: i32) -> Self {
        GameButton {
            mId: id,
            mX: 0, mY: 0, mWidth: 0, mHeight: 0,
            mDrawStoneButton: false,
            mBtnNoDraw: false,
            mDisabled: false,
            mParentWidget: std::ptr::null_mut(),
            mButtonImage: std::ptr::null_mut(),
            mOverImage: std::ptr::null_mut(),
            mDownImage: std::ptr::null_mut(),
        }
    }

    pub fn SetLabel(&mut self, _label: &str) {}
    pub fn Resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.mX = x; self.mY = y; self.mWidth = w; self.mHeight = h;
    }
}

pub struct ToolTipWidget;

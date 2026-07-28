// [TRANSLATION_NOTE]: CursorObject.h/CursorObject.cpp -> Rust 翻译

use crate::const_enums::*;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct CursorObject {
    pub mType: CursorType,
    pub mSeedType: SeedType,
    pub mX: i32, pub mY: i32,
    pub mVisible: bool,
    pub mReanimCursorID: ReanimationID,
    pub mCursorCount: i32,
}

impl CursorObject {
    pub fn new() -> Self {
        CursorObject {
            mType: CursorType::CURSOR_TYPE_NORMAL,
            mSeedType: SeedType::SEED_NONE,
            mX: 0, mY: 0,
            mVisible: true,
            mReanimCursorID: ReanimationID::REANIMATIONID_NULL,
            mCursorCount: 0,
        }
    }

    /// C++ CursorObject::Update() — 更新光标动画
    pub unsafe fn Update(&mut self) {
        // Update cursor animation based on type
    }

    /// C++ CursorObject::Draw() — 绘制光标
    pub unsafe fn Draw(&self, _g: &mut Graphics) {
        if !self.mVisible { return; }
        // TODO: Draw cursor sprite based on mType
    }

    pub unsafe fn Die(&mut self) {
        self.mVisible = false;
    }
}

pub struct CursorPreview {
    pub mVisible: bool,
    pub mX: i32,
    pub mY: i32,
}

impl CursorPreview {
    pub fn new() -> Self {
        CursorPreview { mVisible: true, mX: 0, mY: 0 }
    }
}

pub struct GameButton {
    pub mId: i32,
    pub mX: i32, pub mY: i32,
    pub mWidth: i32, pub mHeight: i32,
    pub mDrawStoneButton: bool,
    pub mBtnNoDraw: bool,
    pub mDisabled: bool,
    pub mIsOver: bool,
    pub mIsDown: bool,
    pub mLabel: String,
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
            mIsOver: false,
            mIsDown: false,
            mLabel: String::new(),
            mParentWidget: std::ptr::null_mut(),
            mButtonImage: std::ptr::null_mut(),
            mOverImage: std::ptr::null_mut(),
            mDownImage: std::ptr::null_mut(),
        }
    }

    pub fn SetLabel(&mut self, label: &str) { self.mLabel = label.to_string(); }
    pub fn Resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.mX = x; self.mY = y; self.mWidth = w; self.mHeight = h;
    }
    pub fn Update(&mut self) {}
    pub fn IsMouseOver(&self) -> bool { self.mIsOver }
    pub fn Draw(&self, _g: &mut Graphics) {
        if self.mBtnNoDraw { return; }
        // TODO: Draw button
    }
}

pub struct ToolTipWidget {
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mVisible: bool,
    pub mLabel: String,
    pub mTitle: String,
    pub mWarningText: String,
}

impl ToolTipWidget {
    pub fn new() -> Self {
        ToolTipWidget {
            mX: 0, mY: 0, mWidth: 0, mHeight: 0,
            mVisible: false,
            mLabel: String::new(),
            mTitle: String::new(),
            mWarningText: String::new(),
        }
    }

    pub fn SetLabel(&mut self, label: &str) { self.mLabel = label.to_string(); }
    pub fn SetTitle(&mut self, title: &str) { self.mTitle = title.to_string(); }
    pub fn SetWarningText(&mut self, text: &str) { self.mWarningText = text.to_string(); }
    pub fn Draw(&self, _g: &mut Graphics) {
        if !self.mVisible { return; }
        // TODO: Draw tooltip with label/title/warning
    }
    pub fn CalculateSize(&mut self) {
        self.mWidth = 100;
        self.mHeight = 50;
    }
}

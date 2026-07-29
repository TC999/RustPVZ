// [TRANSLATION_NOTE]: GameButton.h + GameButton.cpp -> Rust stub

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct GameButton {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mDisabled: bool,
    pub mBtnNoDraw: bool,
    pub mLabel: String,
    pub mOver: bool,
    pub mDown: bool,
}

impl GameButton {
    pub fn new(theApp: *mut LawnApp) -> Self {
        GameButton {
            base: LawnDialog::new(theApp, 0, true, "", "", "", 0),
            mApp: theApp,
            mX: 0, mY: 0, mWidth: 100, mHeight: 30,
            mDisabled: false,
            mBtnNoDraw: false,
            mLabel: String::new(),
            mOver: false, mDown: false,
        }
    }

    pub fn Draw(&self, g: &mut Graphics) {
        if self.mBtnNoDraw { return; }
        // [TODO]: Draw button background, label, overlay based on state
        self.base.Draw(g);
    }

    pub unsafe fn Update(&mut self) {
        // [TODO]: Handle mouse-over/down state changes
    }

    pub fn SetLabel(&mut self, _theLabel: &str) {
        self.mLabel = _theLabel.to_string();
    }

    pub fn Resize(&mut self, theX: i32, theY: i32, theWidth: i32, theHeight: i32) {
        self.mX = theX; self.mY = theY;
        self.mWidth = theWidth; self.mHeight = theHeight;
    }
}

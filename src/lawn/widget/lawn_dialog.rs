// [TRANSLATION_NOTE]: LawnDialog.h + LawnDialog.cpp -> Rust 翻译
// LawnDialog — 游戏内通用对话框基类

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::sexy_app_framework::widget::widget_mod::{Widget, WidgetContainer};
use crate::sexy_app_framework::widget::widget_manager::WidgetManager;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Font, Image};
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::misc::key_codes;
use crate::sexy_app_framework::widget::insets::Insets;

pub const DIALOG_HEADER_OFFSET: i32 = 45;

/// ReanimationWidget — 对话框中的动画控件
pub struct ReanimationWidget {
    pub base: Widget,
    pub mApp: *mut LawnApp,
    pub mPosX: f32,
    pub mPosY: f32,
}

impl ReanimationWidget {
    pub fn new() -> Self {
        ReanimationWidget {
            base: Widget::new(),
            mApp: std::ptr::null_mut(),
            mPosX: 0.0,
            mPosY: 0.0,
        }
    }
    pub fn Dispose(&mut self) {}
    pub fn Draw(&self, _g: &mut Graphics) {}
    pub fn Update(&mut self) {}
}

/// LawnDialog — PvZ 游戏对话框基类
pub struct LawnDialog {
    pub base: Widget,
    pub mApp: *mut LawnApp,
    pub mButtonDelay: i32,
    pub mReanimation: *mut ReanimationWidget,
    pub mDrawStandardBack: bool,
    pub mTallBottom: bool,
    pub mVerticalCenterText: bool,
    pub mDialogHeader: String,
    pub mDialogLines: String,
    pub mDialogFooter: String,
    pub mId: i32,
    pub mContentInsets: Insets,
    pub mBackgroundInsets: Insets,
    pub mIsModal: bool,
    pub mResult: i32,
    pub mWidth: i32,
    pub mHeight: i32,
}

impl LawnDialog {
    pub fn new(theApp: *mut LawnApp, theId: i32, isModal: bool, theDialogHeader: &str, theDialogLines: &str, _theDialogFooter: &str, _theButtonMode: i32) -> Self {
        LawnDialog {
            base: Widget::new(),
            mApp: theApp,
            mButtonDelay: 0,
            mReanimation: std::ptr::null_mut(),
            mDrawStandardBack: true,
            mTallBottom: false,
            mVerticalCenterText: true,
            mDialogHeader: theDialogHeader.to_string(),
            mDialogLines: theDialogLines.to_string(),
            mDialogFooter: String::new(),
            mId: theId,
            mContentInsets: Insets::new_insets(20, 20, 20, 20),
            mBackgroundInsets: Insets::new_insets(0, 0, 0, 0),
            mIsModal: isModal,
            mResult: 0,
            mWidth: 400,
            mHeight: 200,
        }
    }

    pub fn GetLeft(&self) -> i32 { self.base.mX }
    pub fn GetWidth(&self) -> i32 { self.mWidth }
    pub fn GetTop(&self) -> i32 { self.base.mY }

    pub fn SetButtonDelay(&mut self, theDelay: i32) { self.mButtonDelay = theDelay; }

    pub fn Update(&mut self) {}
    pub fn ButtonPress(&mut self, _theId: i32) {}
    pub fn ButtonDepress(&mut self, _theId: i32) {}
    pub fn CheckboxChecked(&mut self) {}

    pub fn KeyDown(&mut self, _theKey: u32) {}

    pub fn AddedToManager(&mut self, _theWidgetManager: *mut WidgetManager) {}
    pub fn RemovedFromManager(&mut self, _theWidgetManager: *mut WidgetManager) {}

    pub fn Resize(&mut self, theX: i32, theY: i32, theWidth: i32, theHeight: i32) {
        self.base.mX = theX;
        self.base.mY = theY;
        self.mWidth = theWidth;
        self.mHeight = theHeight;
    }

    pub fn Draw(&self, _g: &mut Graphics) {}

    pub fn CalcSize(&mut self, _theExtraX: i32, _theExtraY: i32) {
        self.mWidth = 400;
        self.mHeight = 200;
    }

    pub fn GetPreferredHeight(&self, _theWidth: i32) -> i32 { self.mHeight }
}

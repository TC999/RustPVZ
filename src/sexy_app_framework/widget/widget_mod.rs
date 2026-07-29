// [TRANSLATION_NOTE]: WidgetContainer.h + WidgetContainer.cpp -> Rust 翻译
// 控件容器基类

#![allow(non_snake_case, dead_code)]

use std::collections::LinkedList;
use crate::sexy_app_framework::graphics::graphics::Graphics;
use crate::sexy_app_framework::misc::point::Point;
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::misc::flags::FlagsMod;
use crate::sexy_app_framework::widget::widget_manager::WidgetManager;

pub type WidgetList = LinkedList<*mut Widget>;

/// WidgetContainer — 控件容器基类
/// C++ 中 Widget 和 Dialog 均继承此类
pub struct WidgetContainer {
    pub mWidgets: WidgetList,
    pub mWidgetManager: *mut WidgetManager,
    pub mParent: *mut WidgetContainer,
    pub mUpdateIteratorModified: bool,
    pub mLastWMUpdateCount: u32,
    pub mUpdateCnt: u32,
    pub mDirty: bool,
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mHasAlpha: bool,
    pub mClip: bool,
    pub mWidgetFlagsMod: FlagsMod,
    pub mPriority: i32,
    pub mZOrder: i32,
    // Widget 特有字段（C++ 中 Widget 继承 WidgetContainer，此处合并）
    pub mVisible: bool,
    pub mMouseVisible: bool,
    pub mDisabled: bool,
    pub mHasFocus: bool,
    pub mIsDown: bool,
    pub mIsOver: bool,
    pub mHasTransparencies: bool,
    pub mColors: Vec<crate::sexy_app_framework::graphics::color::Color>,
    pub mMouseInsets: crate::sexy_app_framework::widget::insets::Insets,
    pub mDoFinger: bool,
    pub mWantsFocus: bool,
    pub mTabPrev: *mut Widget,
    pub mTabNext: *mut Widget,
}

impl WidgetContainer {
    pub fn new() -> Self {
        WidgetContainer {
            mWidgets: LinkedList::new(),
            mWidgetManager: std::ptr::null_mut(),
            mParent: std::ptr::null_mut(),
            mUpdateIteratorModified: false,
            mLastWMUpdateCount: 0,
            mUpdateCnt: 0,
            mDirty: true,
            mX: 0, mY: 0, mWidth: 0, mHeight: 0,
            mHasAlpha: false,
            mClip: true,
            mWidgetFlagsMod: FlagsMod::new(),
            mPriority: 0,
            mZOrder: 0,
            mVisible: true,
            mMouseVisible: true,
            mDisabled: false,
            mHasFocus: false,
            mIsDown: false,
            mIsOver: false,
            mHasTransparencies: false,
            mColors: Vec::new(),
            mMouseInsets: crate::sexy_app_framework::widget::insets::Insets::new(),
            mDoFinger: false,
            mWantsFocus: false,
            mTabPrev: std::ptr::null_mut(),
            mTabNext: std::ptr::null_mut(),
        }
    }

    pub fn GetRect(&self) -> Rect {
        Rect::new(self.mX, self.mY, self.mWidth, self.mHeight)
    }

    pub fn Intersects(&self, theWidget: &WidgetContainer) -> bool {
        self.GetRect().intersects(&theWidget.GetRect())
    }

    pub fn AddWidget(&mut self, theWidget: *mut Widget) {
        unsafe {
            if !theWidget.is_null() {
                (*theWidget).mParent = self as *mut _ as *mut WidgetContainer;
                (*theWidget).mWidgetManager = self.mWidgetManager;
                self.mWidgets.push_back(theWidget);
            }
        }
    }

    pub fn RemoveWidget(&mut self, theWidget: *mut Widget) {
        unsafe {
            if !theWidget.is_null() {
                (*theWidget).mParent = std::ptr::null_mut();
            }
        }
        self.mWidgets = self.mWidgets.iter().filter(|w| **w != theWidget).cloned().collect();
    }

    pub fn RemoveAllWidgets(&mut self) {
        self.mWidgets.clear();
    }

    pub fn MarkDirty(&mut self) { self.mDirty = true; }
    pub fn MarkDirtyFull(&mut self) { self.mDirty = true; }

    pub fn Update(&mut self) { self.mUpdateCnt += 1; }

    pub fn Draw(&self, _g: &mut Graphics) {}

    pub fn SetVisible(&mut self, isVisible: bool) { self.mVisible = isVisible; }
    pub fn SetDisabled(&mut self, isDisabled: bool) { self.mDisabled = isDisabled; }

    pub fn Resize(&mut self, theX: i32, theY: i32, theWidth: i32, theHeight: i32) {
        self.mX = theX; self.mY = theY; self.mWidth = theWidth; self.mHeight = theHeight;
    }

    pub fn GetAbsPos(&self) -> Point {
        let mut aPos = Point::new(self.mX, self.mY);
        let mut aParent = self.mParent;
        unsafe {
            while !aParent.is_null() {
                aPos.m_x += (*aParent).mX;
                aPos.m_y += (*aParent).mY;
                aParent = (*aParent).mParent;
            }
        }
        aPos
    }
}

/// Widget 类型别名（WidgetContainer 已包含 Widget 所有字段）
pub type Widget = WidgetContainer;

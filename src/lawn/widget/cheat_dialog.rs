// [TRANSLATION_NOTE]: CheatDialog.h + CheatDialog.cpp -> Rust 翻译

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::widget::widget_manager::WidgetManager;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct CheatDialog {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
}

impl CheatDialog {
    pub fn new(theApp: *mut LawnApp) -> Self {
        CheatDialog {
            base: LawnDialog::new(theApp, 0, true, "CHEAT", "Enter New Level:", "", 1),
            mApp: theApp,
        }
    }
    pub fn GetPreferredHeight(&self, theWidth: i32) -> i32 { self.base.GetPreferredHeight(theWidth) }
    pub fn Resize(&mut self, theX: i32, theY: i32, theWidth: i32, theHeight: i32) { self.base.Resize(theX, theY, theWidth, theHeight); }
    pub fn AddedToManager(&mut self, wm: *mut WidgetManager) { self.base.AddedToManager(wm); }
    pub fn RemovedFromManager(&mut self, wm: *mut WidgetManager) { self.base.RemovedFromManager(wm); }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
    pub fn EditWidgetText(&mut self, _id: i32, _s: &str) {}
    pub fn AllowChar(&self, _id: i32, c: u8) -> bool { c.is_ascii_digit() || c == b'-' || c == b'c' || c == b'C' || c == b'f' || c == b'F' }
    pub fn ApplyCheat(&self) -> bool { false }
}

// [TRANSLATION_NOTE]: ContinueDialog.h + ContinueDialog.cpp -> Rust 翻译

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct ContinueDialog {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
}

impl ContinueDialog {
    pub fn new(theApp: *mut LawnApp) -> Self {
        ContinueDialog {
            base: LawnDialog::new(theApp, 0, true, "Continue", "Continue where you left off?", "", 0),
            mApp: theApp,
        }
    }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
    pub fn Update(&mut self) {}
}

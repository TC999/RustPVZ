// [TRANSLATION_NOTE]: UserDialog.h + UserDialog.cpp -> Rust 翻译

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct UserDialog {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
}

impl UserDialog {
    pub fn new(theApp: *mut LawnApp) -> Self {
        UserDialog {
            base: LawnDialog::new(theApp, 0, true, "User", "Select User:", "", 0),
            mApp: theApp,
        }
    }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
}

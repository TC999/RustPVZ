// [TRANSLATION_NOTE]: ImitaterDialog.h + ImitaterDialog.cpp -> Rust 翻译

#![allow(non_snake_case, dead_code)]

use crate::const_enums::SeedType;
use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct ImitaterDialog {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
    pub mSeedType: SeedType,
}

impl ImitaterDialog {
    pub fn new(theApp: *mut LawnApp) -> Self {
        ImitaterDialog {
            base: LawnDialog::new(theApp, 0, true, "Imitater", "Choose a plant to imitate:", "", 0),
            mApp: theApp,
            mSeedType: SeedType::SEED_NONE,
        }
    }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
}

// [TRANSLATION_NOTE]: AchievementsScreen.h + AchievementsScreen.cpp -> Rust stub

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct AchievementsScreen {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
}

impl AchievementsScreen {
    pub fn new(theApp: *mut LawnApp) -> Self {
        AchievementsScreen {
            base: LawnDialog::new(theApp, 0, true, "", "", "", 0),
            mApp: theApp,
        }
    }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
    pub fn Update(&mut self) {}
}

// [TRANSLATION_NOTE]: ZombatarTOS.h + ZombatarTOS.cpp -> Rust 翻译

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct ZombatarTOS {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
}

impl ZombatarTOS {
    pub fn new(theApp: *mut LawnApp) -> Self {
        ZombatarTOS {
            base: LawnDialog::new(theApp, 0, true, "Zombatar TOS", "Terms of Service", "", 1),
            mApp: theApp,
        }
    }
    pub fn Draw(&self, g: &mut Graphics) { self.base.Draw(g); }
}

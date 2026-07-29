// [TRANSLATION_NOTE]: NewUserDialog.h + NewUserDialog.cpp -> Rust 翻译
// 新建/重命名用户对话框

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::lawn_common::{LawnEditWidget, CreateEditWidget};
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::widget::widget_manager::WidgetManager;
use crate::sexy_app_framework::graphics::graphics::Graphics;

pub struct NewUserDialog {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
    pub mNameEditWidget: LawnEditWidget,
}

impl NewUserDialog {
    pub fn new(theApp: *mut LawnApp, isRename: bool) -> Self {
        let header = if isRename { "RENAME USER" } else { "NEW USER" };
        let msg = "Please enter your name:";
        let mut dlg = NewUserDialog {
            base: LawnDialog::new(theApp, 0, true, header, msg, "", 1),
            mApp: theApp,
            mNameEditWidget: LawnEditWidget::new(0, std::ptr::null_mut(), std::ptr::null_mut()),
        };
        dlg.base.mVerticalCenterText = false;
        dlg.mNameEditWidget = CreateEditWidget(0, std::ptr::null_mut(), std::ptr::null_mut());
        dlg.base.CalcSize(110, 40);
        dlg
    }

    pub fn AddedToManager(&mut self, theWidgetManager: *mut WidgetManager) {
        self.base.AddedToManager(theWidgetManager);
    }

    pub fn RemovedFromManager(&mut self, theWidgetManager: *mut WidgetManager) {
        self.base.RemovedFromManager(theWidgetManager);
    }

    pub fn GetPreferredHeight(&self, theWidth: i32) -> i32 {
        self.base.GetPreferredHeight(theWidth) + 40
    }

    pub fn Resize(&mut self, theX: i32, theY: i32, theWidth: i32, theHeight: i32) {
        self.base.Resize(theX, theY, theWidth, theHeight);
    }

    pub fn Draw(&self, g: &mut Graphics) {
        self.base.Draw(g);
    }

    pub fn EditWidgetText(&mut self, _theId: i32, _theString: &str) {
        // stub
    }

    pub fn AllowChar(&self, _theId: i32, theChar: u8) -> bool {
        theChar.is_ascii_alphanumeric() || theChar == b' '
    }

    pub fn GetName(&self) -> String {
        String::new()  // stub
    }

    pub fn SetName(&mut self, _theName: &str) {}
}

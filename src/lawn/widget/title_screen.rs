// [TRANSLATION_NOTE]: TitleScreen.cpp -> Rust 翻译
// 标题画面 — 启动画面、Logo、加载界面、主菜单

#![allow(non_snake_case, dead_code)]

use crate::lawn_app::LawnApp;
use crate::lawn::widget::lawn_dialog::LawnDialog;
use crate::sexy_app_framework::graphics::graphics::Graphics;

// C++ TitleState 枚举 (TitleScreen.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TitleState {
    TITLESTATE_WAITING_FOR_FIRST_DRAW = 0,
    TITLESTATE_POPCAP_LOGO = 1,
    TITLESTATE_PARTNER_LOGO = 2,
    TITLESTATE_SCREEN = 3,
    TITLESTATE_CREDITS = 4,
}

pub struct TitleScreen {
    pub base: LawnDialog,
    pub mApp: *mut LawnApp,
    // C++ TitleScreen 字段
    pub mTitleState: TitleState,
    pub mTitleStateCounter: i32,
    pub mTitleStateDuration: i32,
    pub mTitleAge: i32,
    pub mDrawnYet: bool,
    pub mDisplayPartnerLogo: bool,
    pub mLoaderScreenIsLoaded: bool,
    pub mNeedToInit: bool,
    pub mQuickLoadKey: i32,
    pub mStartButton: *mut std::ffi::c_void,
    pub mWidth: i32,
    pub mHeight: i32,
}

impl TitleScreen {
    pub fn new(theApp: *mut LawnApp) -> Self {
        TitleScreen {
            base: LawnDialog::new(theApp, 0, true, "", "", "", 0),
            mApp: theApp,
            mTitleState: TitleState::TITLESTATE_WAITING_FOR_FIRST_DRAW,
            mTitleStateCounter: 0,
            mTitleStateDuration: 0,
            mTitleAge: 0,
            mDrawnYet: false,
            mDisplayPartnerLogo: false,
            mLoaderScreenIsLoaded: false,
            mNeedToInit: true,
            mQuickLoadKey: 0, // KEYCODE_UNKNOWN
            mStartButton: std::ptr::null_mut(),
            mWidth: 800,
            mHeight: 600,
        }
    }

    /// C++ TitleScreen::Draw (TitleScreen.cpp:90)
    pub unsafe fn Draw(&self, g: &mut Graphics) {
        // C++: g->SetLinearBlend(true);
        g.SetLinearBlend(true);

        // C++: TITLESTATE_WAITING_FOR_FIRST_DRAW
        if self.mTitleState as i32 == TitleState::TITLESTATE_WAITING_FOR_FIRST_DRAW as i32 {
            g.SetColor(crate::sexy_app_framework::graphics::color::Color::from_components(0, 0, 0));
            g.FillRect(0, 0, self.mWidth, self.mHeight);
            if !self.mDrawnYet {
                // C++: TodTraceAndLogLn
                // mDrawnYet = true 在外部设置
            }
            return;
        }

        // C++: TITLESTATE_POPCAP_LOGO
        if self.mTitleState as i32 == TitleState::TITLESTATE_POPCAP_LOGO as i32 {
            g.SetColor(crate::sexy_app_framework::graphics::color::Color::from_components(0, 0, 0));
            g.FillRect(0, 0, self.mWidth, self.mHeight);

            let an_alpha = if self.mTitleStateCounter < self.mTitleStateDuration - 50 {
                if !self.mDisplayPartnerLogo {
                    // TodAnimateCurve(50, 0, mTitleStateCounter, 255, 0, CURVE_LINEAR)
                    255
                } else { 255 }
            } else {
                // TodAnimateCurve(mTitleStateDuration, mTitleStateDuration-50, mTitleStateCounter, 0, 255, CURVE_LINEAR)
                255
            };
            g.SetColorizeImages(true);
            g.SetColor(crate::sexy_app_framework::graphics::color::Color::from_components_alpha(255, 255, 255, an_alpha));
            // g->DrawImage(IMAGE_POPCAP_LOGO, ...)
            g.SetColorizeImages(false);
            return;
        }

        // C++: TITLESTATE_PARTNER_LOGO
        if self.mTitleState as i32 == TitleState::TITLESTATE_PARTNER_LOGO as i32 {
            g.SetColor(crate::sexy_app_framework::graphics::color::Color::from_components(0, 0, 0));
            g.FillRect(0, 0, self.mWidth, self.mHeight);
            g.SetColorizeImages(true);
            g.SetColor(crate::sexy_app_framework::graphics::color::Color::from_components_alpha(255, 255, 255, 255));
            // g->DrawImage(IMAGE_PARTNER_LOGO, ...)
            g.SetColorizeImages(false);
            return;
        }

        // C++: 加载未完成 — 黑屏
        if !self.mLoaderScreenIsLoaded {
            g.SetColor(crate::sexy_app_framework::graphics::color::Color::from_components(0, 0, 0));
            g.FillRect(0, 0, self.mWidth, self.mHeight);
            return;
        }

        // C++: 标题画面绘制
        // g->DrawImage(IMAGE_TITLESCREEN, 0, 0);
        // [TODO]: 完整标题画面绘制（背景、按钮、加载条）
    }

    /// C++ TitleScreen::Update (TitleScreen.cpp:220)
    pub unsafe fn Update(&mut self) {
        let app = &mut *self.mApp;

        // C++: Widget::Update(); if (mApp->mShutdown) return;
        if (*app).m_close_request { return; }

        // C++: MarkDirty();
        if !self.mDrawnYet { return; }

        // C++: TITLESTATE_WAITING_FOR_FIRST_DRAW → 切换到 Logo
        if self.mTitleState as i32 == TitleState::TITLESTATE_WAITING_FOR_FIRST_DRAW as i32 {
            // C++: mApp->mMusic->MusicTitleScreenInit();
            // C++: mApp->StartLoadingThread();
            self.mTitleState = TitleState::TITLESTATE_POPCAP_LOGO;
            self.mTitleStateDuration = if self.mDisplayPartnerLogo { 150 } else { 200 };
            self.mTitleStateCounter = self.mTitleStateDuration;
        }

        // C++: 快速加载键
        // [TODO]: KeyCode check

        self.mTitleAge += 1;
        if self.mTitleStateCounter > 0 {
            self.mTitleStateCounter -= 1;
        }

        // C++: Logo 状态切换
        if self.mTitleState as i32 == TitleState::TITLESTATE_POPCAP_LOGO as i32 {
            if self.mTitleStateCounter == 0 {
                self.mTitleState = TitleState::TITLESTATE_SCREEN;
                self.mTitleStateDuration = 100;
                self.mTitleStateCounter = 100;
            }
            return;
        }

        if !self.mLoaderScreenIsLoaded { return; }

        // C++: 加载进度处理
        // [TODO]: mStartButton 更新、加载进度条
        // [TODO]: 加载完成后切换到主菜单
    }

    /// C++ TitleScreen::Resize (TitleScreen.cpp:503)
    pub unsafe fn Resize(&mut self, the_x: i32, the_y: i32, the_width: i32, the_height: i32) {
        self.mWidth = the_width;
        self.mHeight = the_height;
    }

    /// C++ TitleScreen::MouseDown (TitleScreen.cpp:540)
    pub unsafe fn MouseDown(&mut self, _x: i32, _y: i32, _click_count: i32) {
        // C++: 如果是等待状态或加载未完成，不处理
        if self.mTitleState as i32 == TitleState::TITLESTATE_WAITING_FOR_FIRST_DRAW as i32 { return; }
        if !self.mLoaderScreenIsLoaded { return; }

        // C++: 点击任意处跳过 Logo
        if self.mTitleState as i32 == TitleState::TITLESTATE_POPCAP_LOGO as i32
            || self.mTitleState as i32 == TitleState::TITLESTATE_PARTNER_LOGO as i32
        {
            self.mTitleState = TitleState::TITLESTATE_SCREEN;
            self.mTitleStateDuration = 0;
            self.mTitleStateCounter = 0;
            return;
        }

        // [TODO]: 点击主菜单按钮
    }

    /// C++ TitleScreen::KeyDown (TitleScreen.cpp:550)
    pub unsafe fn KeyDown(&mut self, _the_key: i32) {
        // C++: 按键跳过所有等待
        if self.mTitleState as i32 != TitleState::TITLESTATE_SCREEN as i32 {
            self.mTitleState = TitleState::TITLESTATE_SCREEN;
            self.mTitleStateDuration = 0;
            self.mTitleStateCounter = 100;
        }
    }

    /// C++ TitleScreen::ButtonDepress (TitleScreen.cpp:526)
    pub unsafe fn ButtonDepress(&mut self, _the_id: i32) {
        // [TODO]: 处理按钮点击（开始游戏、选项等）
    }
}

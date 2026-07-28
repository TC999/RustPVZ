// [TRANSLATION_NOTE]: Music.h + Music.cpp -> Rust 翻译
// 音乐播放模块。底层依赖 SDL_mixer，此处为基础接口定义

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MusicTune {
    MUSIC_TUNE_NONE = -1,
    MUSIC_TUNE_DAY_GRASSWALK = 1,
    MUSIC_TUNE_NIGHT_MOONGRAINS = 2,
    MUSIC_TUNE_POOL_WATERYGRAVES = 3,
    MUSIC_TUNE_FOG_RIGORMORMIST = 4,
    MUSIC_TUNE_ROOF_GRAZETHEROOF = 5,
    MUSIC_TUNE_CHOOSE_YOUR_SEEDS = 6,
    MUSIC_TUNE_TITLE_CRAZY_DAVE_MAIN_THEME = 7,
    MUSIC_TUNE_ZEN_GARDEN = 8,
    MUSIC_TUNE_PUZZLE_CEREBRAWL = 9,
    MUSIC_TUNE_MINIGAME_LOONBOON = 10,
    MUSIC_TUNE_CONVEYER = 11,
    MUSIC_TUNE_FINAL_BOSS_BRAINIAC_MANIAC = 12,
    MUSIC_TUNE_CREDITS_ZOMBIES_ON_YOUR_LAWN = 13,
    NUM_MUSIC_TUNES = 14,
}

/// Music 类 — 游戏音乐播放管理
pub struct Music {
    pub mApp: *mut crate::lawn_app::LawnApp,
    pub mCurrentMusicTune: MusicTune,
    pub mMusicVolume: f64,
}

impl Music {
    pub fn new(theApp: *mut crate::lawn_app::LawnApp) -> Self {
        Music {
            mApp: theApp,
            mCurrentMusicTune: MusicTune::MUSIC_TUNE_NONE,
            mMusicVolume: 0.0,
        }
    }

    pub fn StartGameMusic(&mut self) {
        // 暂为 stub — 需 SDL_mixer 支持
    }

    pub fn StartLevelMusic(&mut self) {
        // 暂为 stub
    }

    pub fn StopAllMusic(&mut self) {
        // 暂为 stub
    }

    pub fn SetVolume(&mut self, theVolume: f64) {
        self.mMusicVolume = theVolume;
    }

    pub fn MusicTitleScreen(&mut self) {
        self.mCurrentMusicTune = MusicTune::MUSIC_TUNE_TITLE_CRAZY_DAVE_MAIN_THEME;
    }

    pub fn MusicCreditsScreen(&mut self) {
        self.mCurrentMusicTune = MusicTune::MUSIC_TUNE_CREDITS_ZOMBIES_ON_YOUR_LAWN;
    }

    pub fn IsMusicPlaying(&self) -> bool {
        self.mCurrentMusicTune != MusicTune::MUSIC_TUNE_NONE
    }

    pub fn Update(&mut self) {
        // 暂为 stub
    }
}

// [TRANSLATION_NOTE]: SoundManager.h -> Rust trait
// 声音管理器接口（纯虚类映射为 trait）

#![allow(non_snake_case, dead_code)]

pub const MAX_SOURCE_SOUNDS: i32 = 256;
pub const MAX_CHANNELS: i32 = 32;

/// DummySoundManager — 什么都不做的默认实现（对应 C++ SDLSoundManager 的空壳替代）
/// [TRANSLATION_NOTE]: 完整 SDLSoundManager 翻译在后续音频子系统阶段进行，
/// 此处提供满足接口的最小实现以保证应用初始化流程可运行。
pub struct DummySoundManager;

impl DummySoundManager {
    pub fn new() -> Self { DummySoundManager }
}

impl SoundManager for DummySoundManager {
    fn initialized(&self) -> bool { true }
    fn load_sound(&mut self, _the_sfx_id: isize, _the_filename: &str) -> bool { true }
    fn load_sound_from_path(&mut self, _the_filename: &str) -> isize { -1 }
    fn release_sound(&mut self, _the_sfx_id: isize) {}
    fn set_volume(&mut self, _the_volume: f64) {}
    fn set_base_volume(&mut self, _the_sfx_id: isize, _the_base_volume: f64) -> bool { true }
    fn set_base_pan(&mut self, _the_sfx_id: isize, _the_base_pan: i32) -> bool { true }
    fn get_sound_instance(&mut self, _the_sfx_id: isize) -> *mut std::ffi::c_void { std::ptr::null_mut() }
    fn release_sounds(&mut self) {}
    fn release_channels(&mut self) {}
    fn get_master_volume(&self) -> f64 { 0.0 }
    fn set_master_volume(&mut self, _the_volume: f64) {}
    fn flush(&mut self) {}
    fn stop_all_sounds(&mut self) {}
    fn get_free_sound_id(&mut self) -> isize { -1 }
    fn get_num_sounds(&self) -> i32 { 0 }
}

/// SoundManager 抽象接口
pub trait SoundManager {
    fn initialized(&self) -> bool;
    fn load_sound(&mut self, the_sfx_id: isize, the_filename: &str) -> bool;
    fn load_sound_from_path(&mut self, the_filename: &str) -> isize;
    fn release_sound(&mut self, the_sfx_id: isize);
    fn set_volume(&mut self, the_volume: f64);
    fn set_base_volume(&mut self, the_sfx_id: isize, the_base_volume: f64) -> bool;
    fn set_base_pan(&mut self, the_sfx_id: isize, the_base_pan: i32) -> bool;
    fn get_sound_instance(&mut self, the_sfx_id: isize) -> *mut std::ffi::c_void;
    fn release_sounds(&mut self);
    fn release_channels(&mut self);
    fn get_master_volume(&self) -> f64;
    fn set_master_volume(&mut self, the_volume: f64);
    fn flush(&mut self);
    fn stop_all_sounds(&mut self);
    fn get_free_sound_id(&mut self) -> isize;
    fn get_num_sounds(&self) -> i32;
}

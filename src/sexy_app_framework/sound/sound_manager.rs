// [TRANSLATION_NOTE]: SoundManager.h -> Rust trait
// 声音管理器接口（纯虚类映射为 trait）

#![allow(non_snake_case, dead_code)]

pub const MAX_SOURCE_SOUNDS: i32 = 256;
pub const MAX_CHANNELS: i32 = 32;

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

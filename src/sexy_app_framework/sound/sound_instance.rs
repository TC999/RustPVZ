// [TRANSLATION_NOTE]: SoundInstance.h -> Rust trait
// 声音实例接口（纯虚类映射为 trait）

#![allow(non_snake_case, dead_code)]

/// SoundInstance 抽象接口
pub trait SoundInstance {
    fn play(&mut self, the_looped: bool, the_auto_release: bool) -> bool;
    fn stop(&mut self);
    fn is_playing(&self) -> bool;
    fn is_above_priority_cutoff(&self) -> bool;
    fn set_volume(&mut self, the_volume: f64) -> bool;
    fn set_pan(&mut self, the_pan: i32) -> bool;
    fn set_base_volume(&mut self, the_base_volume: f64) -> bool;
    fn set_base_pan(&mut self, the_base_pan: i32) -> bool;
    fn adjust_volume(&mut self, the_volume: f64) -> bool;
    fn adjust_pan(&mut self, the_pan: i32) -> bool;
    fn get_volume(&self) -> f64;
    fn get_pan(&self) -> i32;
    fn duplicate(&self) -> *mut std::ffi::c_void;
    fn get_num_instances(&self) -> i32;
    fn get_sound_id(&self) -> isize;
}

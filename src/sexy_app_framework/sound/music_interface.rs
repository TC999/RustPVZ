// [TRANSLATION_NOTE]: MusicInterface.h -> Rust trait
// 音乐播放接口

#![allow(non_snake_case, dead_code)]

/// MusicInterface 抽象接口
pub trait MusicInterface {
    fn load_music(&mut self, the_music_id: isize, the_filename: &str) -> bool;
    fn play_music(&mut self, the_music_id: isize, the_offset: f64, the_no_loop: bool) -> bool;
    fn stop_music(&mut self);
    fn pause_music(&mut self);
    fn resume_music(&mut self);
    fn set_volume(&mut self, the_volume: f64);
    fn set_loop_points(&mut self, the_start: f64, the_end: f64) -> bool;
    fn get_music_length(&self, the_music_id: isize) -> f64;
    fn get_music_position(&self) -> f64;
    fn set_music_position(&mut self, the_position: f64);
    fn music_is_playing(&self) -> bool;
    fn music_is_paused(&self) -> bool;
    fn release_music(&mut self, the_music_id: isize);
    fn stop_all_music(&mut self);
    fn unload_music(&mut self, the_music_id: isize);
}

/// DummyMusicInterface — 什么都不做的默认实现
pub struct DummyMusicInterface;

impl DummyMusicInterface {
    pub fn new() -> Self { DummyMusicInterface }
}

impl MusicInterface for DummyMusicInterface {
    fn load_music(&mut self, _the_music_id: isize, _the_filename: &str) -> bool { true }
    fn play_music(&mut self, _the_music_id: isize, _the_offset: f64, _the_no_loop: bool) -> bool { true }
    fn stop_music(&mut self) {}
    fn pause_music(&mut self) {}
    fn resume_music(&mut self) {}
    fn set_volume(&mut self, _the_volume: f64) {}
    fn set_loop_points(&mut self, _the_start: f64, _the_end: f64) -> bool { true }
    fn get_music_length(&self, _the_music_id: isize) -> f64 { 0.0 }
    fn get_music_position(&self) -> f64 { 0.0 }
    fn set_music_position(&mut self, _the_position: f64) {}
    fn music_is_playing(&self) -> bool { false }
    fn music_is_paused(&self) -> bool { false }
    fn release_music(&mut self, _the_music_id: isize) {}
    fn stop_all_music(&mut self) {}
    fn unload_music(&mut self, _the_music_id: isize) {}
}

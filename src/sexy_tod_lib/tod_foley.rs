// [TRANSLATION_NOTE]: TodFoley.h + TodFoley.cpp -> Rust
// 拟音音效系统：管理游戏中的短音效播放/停止/暂停
// SoundInstance/SoundManager 用桩类型替代（声音模块暂未完整实现）

use crate::sexy_tod_lib::tod_common::tod_pick_from_array;
use crate::sexy_tod_lib::tod_debug::_tod_assert;

pub const MAX_FOLEY_TYPES: i32 = 110;
pub const MAX_FOLEY_INSTANCES: i32 = 8;

// ============================================================
// 桩类型：SoundInstance（对应 C++ Sexy::SoundInstance）
// ============================================================
pub struct SoundInstance {
    pub m_ref_count: i32,
}

impl SoundInstance {
    pub fn new() -> Self { SoundInstance { m_ref_count: 1 } }
    pub fn play(&mut self, _looping: bool, _auto_release: bool) {}
    pub fn stop(&mut self) {}
    pub fn release(&mut self) { self.m_ref_count -= 1; }
    pub fn set_volume(&mut self, _vol: f64) {}
    pub fn adjust_pitch(&mut self, _pitch: f32) {}
    pub fn is_playing(&self) -> bool { false }
    pub fn get_sound_position(&self) -> u32 { 0 }
    pub fn set_sound_position(&mut self, _pos: u32) {}
}

// ============================================================
// 枚举
// ============================================================
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum FoleyFlags {
    FOLEYFLAGS_LOOP,
    FOLEYFLAGS_ONE_AT_A_TIME,
    FOLEYFLAGS_MUTE_ON_PAUSE,
    FOLEYFLAGS_USES_MUSIC_VOLUME,
    FOLEYFLAGS_DONT_REPEAT,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum FoleyType {
    FOLEY_SUN, FOLEY_SPLAT, FOLEY_LAWNMOWER, FOLEY_THROW, FOLEY_SPAWN_SUN,
    FOLEY_CHOMP, FOLEY_CHOMP_SOFT, FOLEY_PLANT, FOLEY_USE_SHOVEL, FOLEY_DROP,
    FOLEY_BLEEP, FOLEY_GROAN, FOLEY_BRAINS, FOLEY_SUKHBIR, FOLEY_JACKINTHEBOX,
    FOLEY_ART_CHALLENGE, FOLEY_ZAMBONI, FOLEY_THUNDER, FOLEY_FROZEN, FOLEY_ZOMBIESPLASH,
    FOLEY_BOWLINGIMPACT, FOLEY_SQUISH, FOLEY_TIRE_POP, FOLEY_EXPLOSION, FOLEY_SLURP,
    FOLEY_LIMBS_POP, FOLEY_POGO_ZOMBIE, FOLEY_SNOW_PEA_SPARKLES, FOLEY_ZOMBIE_FALLING,
    FOLEY_PUFF, FOLEY_FUME, FOLEY_COIN, FOLEY_KERNEL_SPLAT, FOLEY_DIGGER,
    FOLEY_JACK_SURPRISE, FOLEY_VASE_BREAKING, FOLEY_POOL_CLEANER, FOLEY_BASKETBALL,
    FOLEY_IGNITE, FOLEY_FIREPEA, FOLEY_THUMP, FOLEY_SQUASH_HMM, FOLEY_MAGNETSHROOM,
    FOLEY_BUTTER, FOLEY_BUNGEE_SCREAM, FOLEY_BOSS_EXPLOSION_SMALL, FOLEY_SHIELD_HIT,
    FOLEY_SWING, FOLEY_BONK, FOLEY_RAIN, FOLEY_DOLPHIN_BEFORE_JUMPING, FOLEY_DOLPHIN_APPEARS,
    FOLEY_PLANT_WATER, FOLEY_ZOMBIE_ENTERING_WATER, FOLEY_GRAVEBUSTERCHOMP, FOLEY_CHERRYBOMB,
    FOLEY_JALAPENO_IGNITE, FOLEY_REVERSE_EXPLOSION, FOLEY_PLASTIC_HIT, FOLEY_WINMUSIC,
    FOLEY_BALLOONINFLATE, FOLEY_BIGCHOMP, FOLEY_MELONIMPACT, FOLEY_PLANTGROW,
    FOLEY_SHOOP, FOLEY_JUICY, FOLEY_NEWSPAPER_RARRGH, FOLEY_NEWSPAPER_RIP,
    FOLEY_FLOOP, FOLEY_COFFEE, FOLEY_LOW_GROAN, FOLEY_PRIZE, FOLEY_YUCK,
    FOLEY_UMBRELLA, FOLEY_GRASSSTEP, FOLEY_SHOVEL, FOLEY_COB_LAUNCH, FOLEY_WATERING,
    FOLEY_POLEVAULT, FOLEY_GRAVESTONE_RUMBLE, FOLEY_DIRT_RISE, FOLEY_FERTILIZER,
    FOLEY_PORTAL, FOLEY_WAKEUP, FOLEY_BUGSPRAY, FOLEY_SCREAM, FOLEY_PAPER,
    FOLEY_MONEYFALLS, FOLEY_IMP, FOLEY_HYDRAULIC_SHORT, FOLEY_HYDRAULIC,
    FOLEY_GARGANTUDEATH, FOLEY_CERAMIC, FOLEY_BOSS_BOULDER_ATTACK, FOLEY_CHIME,
    FOLEY_CRAZY_DAVE_SHORT, FOLEY_CRAZY_DAVE_LONG, FOLEY_CRAZY_DAVE_EXTRA_LONG,
    FOLEY_CRAZY_DAVE_CRAZY, FOLEY_PHONOGRAPH, FOLEY_DANCER, FOLEY_FINAL_FANFARE,
    FOLEY_CRAZY_DAVE_SCREAM, FOLEY_CRAZY_DAVE_SCREAM_2,
    NUM_FOLEY,
}

// ============================================================
// FoleyParams - 音效参数定义
// ============================================================
pub struct FoleyParams {
    pub m_foley_type: FoleyType,
    pub m_pitch_range: f32,
    pub m_sfx_id: [Option<&'static str>; 10], // 用字符串 ID 替代 intptr_t*
    pub m_foley_flags: u32,
}

// ============================================================
// FoleyInstance / FoleyTypeData
// ============================================================
pub struct FoleyInstance {
    pub m_instance: Option<Box<SoundInstance>>,
    pub m_ref_count: i32,
    pub m_paused: bool,
    pub m_start_time: u32,
    pub m_pause_offset: i32,
}

impl FoleyInstance {
    pub fn new() -> Self {
        FoleyInstance {
            m_instance: None,
            m_ref_count: 0,
            m_paused: false,
            m_start_time: 0,
            m_pause_offset: 0,
        }
    }
}

pub struct FoleyTypeData {
    pub m_foley_instances: [FoleyInstance; MAX_FOLEY_INSTANCES as usize],
    pub m_last_variation_played: i32,
}

impl FoleyTypeData {
    pub fn new() -> Self {
        FoleyTypeData {
            m_foley_instances: [
                FoleyInstance::new(), FoleyInstance::new(), FoleyInstance::new(), FoleyInstance::new(),
                FoleyInstance::new(), FoleyInstance::new(), FoleyInstance::new(), FoleyInstance::new(),
            ],
            m_last_variation_played: -1,
        }
    }
}

// ============================================================
// TodFoley - 主音效类
// ============================================================
pub struct TodFoley {
    pub m_foley_type_data: [FoleyTypeData; MAX_FOLEY_TYPES as usize],
}

impl TodFoley {
    pub fn new() -> Self {
        TodFoley {
            m_foley_type_data: [
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(), FoleyTypeData::new(),
                FoleyTypeData::new(), FoleyTypeData::new(),
            ],
        }
    }

    fn release_finished_instances(&mut self, _g_foley_param_array_size: i32) {
        for a_foley_type in 0.._g_foley_param_array_size {
            for i in 0..MAX_FOLEY_INSTANCES {
                let a_instance = &mut self.m_foley_type_data[a_foley_type as usize].m_foley_instances[i as usize];
                if a_instance.m_ref_count == 0 {
                    // assert a_instance.m_instance is None
                } else if !a_instance.m_paused {
                    if let Some(ref mut inst) = a_instance.m_instance {
                        if !inst.is_playing() {
                            inst.release();
                            a_instance.m_instance = None;
                            a_instance.m_ref_count = 0;
                        }
                    }
                }
            }
        }
    }

    fn has_played_too_recently(&mut self, the_foley_type: FoleyType, g_update_count: u32, _g_foley_param_array_size: i32) -> bool {
        let a_data = &self.m_foley_type_data[the_foley_type as usize];
        for i in 0..MAX_FOLEY_INSTANCES {
            let a_instance = &a_data.m_foley_instances[i as usize];
            if a_instance.m_ref_count != 0 && g_update_count - a_instance.m_start_time < 10 {
                return true;
            }
        }
        false
    }

    fn find_instance(&mut self, the_foley_type: FoleyType, _g_foley_param_array_size: i32) -> Option<&mut FoleyInstance> {
        let a_data = &mut self.m_foley_type_data[the_foley_type as usize];
        for i in 0..MAX_FOLEY_INSTANCES {
            if a_data.m_foley_instances[i as usize].m_ref_count > 0 {
                return Some(&mut a_data.m_foley_instances[i as usize]);
            }
        }
        None
    }

    fn get_free_instance_index(&mut self, the_foley_type: FoleyType, _g_foley_param_array_size: i32) -> Option<usize> {
        let a_data = &self.m_foley_type_data[the_foley_type as usize];
        for i in 0..MAX_FOLEY_INSTANCES {
            if a_data.m_foley_instances[i as usize].m_ref_count == 0 {
                return Some(i as usize);
            }
        }
        None
    }

    pub fn play_foley_pitch(&mut self, the_foley_type: FoleyType, the_pitch: f32, g_foley_param_array: &[FoleyParams], g_update_count: u32, g_sfx_volume: f64, g_music_volume: f64) {
        let g_foley_param_array_size = g_foley_param_array.len() as i32;
        self.release_finished_instances(g_foley_param_array_size);

        let a_params = &g_foley_param_array[the_foley_type as usize];
        if self.has_played_too_recently(the_foley_type, g_update_count, g_foley_param_array_size)
            && (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_LOOP as i32))) == 0 {
            return;
        }

        if (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_ONE_AT_A_TIME as i32))) != 0 {
            if let Some(a_instance) = self.find_instance(the_foley_type, g_foley_param_array_size) {
                a_instance.m_ref_count += 1;
                a_instance.m_start_time = g_update_count;
                return;
            }
        }

        // 先读取 last_variation_played，避免借用冲突
        let last_variation = self.m_foley_type_data[the_foley_type as usize].m_last_variation_played;
        // 获取空闲实例的索引
        let free_idx = self.get_free_instance_index(the_foley_type, g_foley_param_array_size);

        if let Some(free_idx) = free_idx {
            let mut a_variations = Vec::new();
            for i in 0..10 {
                let not_repeat = (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_DONT_REPEAT as i32))) == 0
                    || last_variation != i as i32;
                if not_repeat && a_params.m_sfx_id[i].is_some() {
                    a_variations.push(i);
                }
            }
            _tod_assert(a_variations.len() > 0, file!(), line!(), "No variations available");
            let a_variation = tod_pick_from_array(&a_variations);
            self.m_foley_type_data[the_foley_type as usize].m_last_variation_played = a_variation as i32;

            let a_instance = &mut self.m_foley_type_data[the_foley_type as usize].m_foley_instances[free_idx];

            let mut sound = Box::new(SoundInstance::new());
            if the_pitch != 0.0 {
                sound.adjust_pitch(the_pitch);
            }
            if (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_USES_MUSIC_VOLUME as i32))) != 0 {
                if g_sfx_volume < 1e-6 {
                    sound.set_volume(0.0);
                } else {
                    sound.set_volume(g_music_volume / g_sfx_volume);
                }
            }
            let is_looping = (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_LOOP as i32))) != 0;
            sound.play(is_looping, false);
            a_instance.m_instance = Some(sound);
            a_instance.m_ref_count = 1;
            a_instance.m_start_time = g_update_count;
        }
    }

    pub fn play_foley(&mut self, the_foley_type: FoleyType, g_foley_param_array: &[FoleyParams], g_update_count: u32, g_sfx_volume: f64, g_music_volume: f64) {
        let a_params = &g_foley_param_array[the_foley_type as usize];
        let a_pitch = if a_params.m_pitch_range != 0.0 {
            // Rand(aParams->mPitchRange)
            0.0
        } else { 0.0 };
        self.play_foley_pitch(the_foley_type, a_pitch, g_foley_param_array, g_update_count, g_sfx_volume, g_music_volume);
    }

    pub fn stop_foley(&mut self, the_foley_type: FoleyType, g_foley_param_array: &[FoleyParams]) {
        let g_foley_param_array_size = g_foley_param_array.len() as i32;
        self.release_finished_instances(g_foley_param_array_size);
        if let Some(a_instance) = self.find_instance(the_foley_type, g_foley_param_array_size) {
            _tod_assert(a_instance.m_ref_count > 0, file!(), line!(), "");
            a_instance.m_ref_count -= 1;
            if a_instance.m_ref_count == 0 {
                if let Some(mut inst) = a_instance.m_instance.take() {
                    inst.release();
                }
            }
        }
    }

    pub fn game_pause(&mut self, the_entering_pause: bool, g_foley_param_array: &[FoleyParams]) {
        let g_foley_param_array_size = g_foley_param_array.len() as i32;
        self.release_finished_instances(g_foley_param_array_size);
        for a_foley_type in 0..g_foley_param_array_size {
            let a_params = &g_foley_param_array[a_foley_type as usize];
            if (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_MUTE_ON_PAUSE as i32))) != 0 {
                let a_data = &mut self.m_foley_type_data[a_foley_type as usize];
                for i in 0..MAX_FOLEY_INSTANCES {
                    let a_instance = &mut a_data.m_foley_instances[i as usize];
                    if a_instance.m_ref_count != 0 {
                        if the_entering_pause {
                            a_instance.m_paused = true;
                            a_instance.m_pause_offset = 0;
                            if let Some(ref mut inst) = a_instance.m_instance {
                                inst.stop();
                            }
                        } else if a_instance.m_paused {
                            a_instance.m_paused = false;
                            let is_looping = (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_LOOP as i32))) != 0;
                            if let Some(ref mut inst) = a_instance.m_instance {
                                inst.play(is_looping, false);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn cancel_paused_foley(&mut self, g_foley_param_array: &[FoleyParams]) {
        let g_foley_param_array_size = g_foley_param_array.len() as i32;
        self.release_finished_instances(g_foley_param_array_size);
        for a_foley_type in 0..g_foley_param_array_size {
            let a_data = &mut self.m_foley_type_data[a_foley_type as usize];
            for i in 0..MAX_FOLEY_INSTANCES {
                let a_instance = &mut a_data.m_foley_instances[i as usize];
                if a_instance.m_ref_count != 0 && a_instance.m_paused {
                    a_instance.m_ref_count = 0;
                    if let Some(mut inst) = a_instance.m_instance.take() {
                        inst.release();
                    }
                }
            }
        }
    }

    pub fn is_foley_playing(&mut self, the_foley_type: FoleyType, g_foley_param_array: &[FoleyParams]) -> bool {
        let g_foley_param_array_size = g_foley_param_array.len() as i32;
        self.release_finished_instances(g_foley_param_array_size);
        self.find_instance(the_foley_type, g_foley_param_array_size).is_some()
    }

    pub fn rehookup_sound_with_music_volume(&mut self, g_foley_param_array: &[FoleyParams], g_sfx_volume: f64, g_music_volume: f64) {
        let g_foley_param_array_size = g_foley_param_array.len() as i32;
        self.release_finished_instances(g_foley_param_array_size);
        for a_foley_type in 0..g_foley_param_array_size {
            let a_params = &g_foley_param_array[a_foley_type as usize];
            if (a_params.m_foley_flags & (1 << (FoleyFlags::FOLEYFLAGS_USES_MUSIC_VOLUME as i32))) != 0 {
                let a_data = &mut self.m_foley_type_data[a_foley_type as usize];
                for i in 0..MAX_FOLEY_INSTANCES {
                    let a_instance = &mut a_data.m_foley_instances[i as usize];
                    if a_instance.m_ref_count != 0 {
                        if g_sfx_volume < 1e-6 {
                            if let Some(ref mut inst) = a_instance.m_instance {
                                inst.set_volume(0.0);
                            }
                        } else {
                            if let Some(ref mut inst) = a_instance.m_instance {
                                inst.set_volume(g_music_volume / g_sfx_volume);
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================
// 全局函数
// ============================================================
pub fn tod_foley_initialize(g_foley_param_array: &mut [FoleyParams], _g_foley_param_array_size: i32) {
    // gFoleyParamArray = theFoleyParamArray;
    // gFoleyParamArraySize = theFoleyParamArraySize;
}

pub fn tod_foley_dispose() {
}

pub fn lookup_foley(the_foley_type: FoleyType, g_foley_param_array: &[FoleyParams]) -> &FoleyParams {
    _tod_assert(the_foley_type as i32 >= 0 && (the_foley_type as usize) < g_foley_param_array.len(), file!(), line!(), "");
    &g_foley_param_array[the_foley_type as usize]
}

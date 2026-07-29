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
pub fn tod_foley_initialize(_g_foley_param_array: &mut [FoleyParams], _g_foley_param_array_size: i32) {
}

pub fn tod_foley_dispose() {
}

/// C++ gLawnFoleyParamArray 的 Rust 等价实现
/// 使用 match 语句代替静态大数组，每个 FoleyType 返回对应的参数
pub fn get_foley_params(the_foley_type: FoleyType) -> FoleyParams {
    let empty: [Option<&'static str>; 10] = [None, None, None, None, None, None, None, None, None, None];
    let sfx = |s: &'static str| -> [Option<&'static str>; 10] {
        let mut a = empty;
        a[0] = Some(s);
        a
    };
    let sfx2 = |s0: &'static str, s1: &'static str| -> [Option<&'static str>; 10] {
        let mut a = empty;
        a[0] = Some(s0); a[1] = Some(s1); a
    };
    let sfx3 = |s0: &'static str, s1: &'static str, s2: &'static str| -> [Option<&'static str>; 10] {
        let mut a = empty; a[0] = Some(s0); a[1] = Some(s1); a[2] = Some(s2); a
    };
    let sfx4 = |s0: &'static str, s1: &'static str, s2: &'static str, s3: &'static str| -> [Option<&'static str>; 10] {
        let mut a = empty; a[0]=Some(s0); a[1]=Some(s1); a[2]=Some(s2); a[3]=Some(s3); a
    };
    let sfx6 = |s0: &'static str, s1: &'static str, s2: &'static str, s3: &'static str, s4: &'static str, s5: &'static str| -> [Option<&'static str>; 10] {
        let mut a = empty; a[0]=Some(s0); a[1]=Some(s1); a[2]=Some(s2); a[3]=Some(s3); a[4]=Some(s4); a[5]=Some(s5); a
    };
    let sfx9 = |s0:&'static str,s1:&'static str,s2:&'static str,s3:&'static str,s4:&'static str,s5:&'static str,s6:&'static str,s7:&'static str,s8:&'static str|->[Option<&'static str>;10]{
        let mut a=empty; a[0]=Some(s0);a[1]=Some(s1);a[2]=Some(s2);a[3]=Some(s3);a[4]=Some(s4);a[5]=Some(s5);a[6]=Some(s6);a[7]=Some(s7);a[8]=Some(s8);a
    };

    match the_foley_type {
        FoleyType::FOLEY_SUN => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_POINTS"), m_foley_flags: 0 },
        FoleyType::FOLEY_SPLAT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx3("SOUND_SPLAT","SOUND_SPLAT2","SOUND_SPLAT3"), m_foley_flags: 0 },
        FoleyType::FOLEY_LAWNMOWER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_LAWNMOWER"), m_foley_flags: 0 },
        FoleyType::FOLEY_THROW => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx4("SOUND_THROW","SOUND_THROW","SOUND_THROW","SOUND_THROW2"), m_foley_flags: 0 },
        FoleyType::FOLEY_SPAWN_SUN => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_THROW"), m_foley_flags: 0 },
        FoleyType::FOLEY_CHOMP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx2("SOUND_CHOMP","SOUND_CHOMP2"), m_foley_flags: 0 },
        FoleyType::FOLEY_CHOMP_SOFT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 4.0, m_sfx_id: sfx("SOUND_CHOMPSOFT"), m_foley_flags: 0 },
        FoleyType::FOLEY_PLANT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx2("SOUND_PLANT","SOUND_PLANT2"), m_foley_flags: 0 },
        FoleyType::FOLEY_USE_SHOVEL => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_PLANT2"), m_foley_flags: 0 },
        FoleyType::FOLEY_DROP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_TAP2"), m_foley_flags: 0 },
        FoleyType::FOLEY_BLEEP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_BLEEP"), m_foley_flags: 0 },
        FoleyType::FOLEY_GROAN => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx6("SOUND_GROAN","SOUND_GROAN2","SOUND_GROAN3","SOUND_GROAN4","SOUND_GROAN5","SOUND_GROAN6"), m_foley_flags: 0 },
        FoleyType::FOLEY_BRAINS => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx9("SOUND_GROAN","SOUND_GROAN2","SOUND_GROAN3","SOUND_GROAN4","SOUND_GROAN5","SOUND_GROAN6","SOUND_SUKHBIR4","SOUND_SUKHBIR5","SOUND_SUKHBIR6"), m_foley_flags: 0 },
        FoleyType::FOLEY_SUKHBIR => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx9("SOUND_GROAN","SOUND_GROAN2","SOUND_GROAN3","SOUND_GROAN4","SOUND_GROAN5","SOUND_GROAN6","SOUND_SUKHBIR","SOUND_SUKHBIR2","SOUND_SUKHBIR3"), m_foley_flags: 0 },
        FoleyType::FOLEY_JACKINTHEBOX => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_JACKINTHEBOX"), m_foley_flags: 7 },
        FoleyType::FOLEY_ZOMBIE_FALLING => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx2("SOUND_ZOMBIE_FALLING_1","SOUND_ZOMBIE_FALLING_2"), m_foley_flags: 0 },
        FoleyType::FOLEY_PUFF => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_PUFF"), m_foley_flags: 0 },
        FoleyType::FOLEY_FUME => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_FUME"), m_foley_flags: 0 },
        FoleyType::FOLEY_COIN => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_COIN"), m_foley_flags: 0 },
        FoleyType::FOLEY_KERNEL_SPLAT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx2("SOUND_KERNELPULT","SOUND_KERNELPULT2"), m_foley_flags: 0 },
        FoleyType::FOLEY_DIGGER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_DIGGER_ZOMBIE"), m_foley_flags: 7 },
        FoleyType::FOLEY_JACK_SURPRISE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 1.0, m_sfx_id: sfx3("SOUND_JACK_SURPRISE","SOUND_JACK_SURPRISE","SOUND_JACK_SURPRISE2"), m_foley_flags: 0 },
        FoleyType::FOLEY_VASE_BREAKING => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -5.0, m_sfx_id: sfx("SOUND_VASE_BREAKING"), m_foley_flags: 0 },
        FoleyType::FOLEY_POOL_CLEANER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_POOL_CLEANER"), m_foley_flags: 0 },
        FoleyType::FOLEY_BASKETBALL => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_BASKETBALL"), m_foley_flags: 0 },
        FoleyType::FOLEY_IGNITE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 5.0, m_sfx_id: sfx4("SOUND_IGNITE","SOUND_IGNITE","SOUND_IGNITE","SOUND_IGNITE2"), m_foley_flags: 0 },
        FoleyType::FOLEY_FIREPEA => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_FIREPEA"), m_foley_flags: 0 },
        FoleyType::FOLEY_THUMP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_GARGANTUAR_THUMP"), m_foley_flags: 0 },
        FoleyType::FOLEY_SQUASH_HMM => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx3("SOUND_SQUASH_HMM","SOUND_SQUASH_HMM","SOUND_SQUASH_HMM2"), m_foley_flags: 0 },
        FoleyType::FOLEY_MAGNETSHROOM => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_MAGNETSHROOM"), m_foley_flags: 0 },
        FoleyType::FOLEY_BUTTER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_BUTTER"), m_foley_flags: 0 },
        FoleyType::FOLEY_BUNGEE_SCREAM => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx3("SOUND_BUNGEE_SCREAM","SOUND_BUNGEE_SCREAM2","SOUND_BUNGEE_SCREAM3"), m_foley_flags: 0 },
        FoleyType::FOLEY_BOSS_EXPLOSION_SMALL => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_EXPLOSION"), m_foley_flags: 0 },
        FoleyType::FOLEY_SHIELD_HIT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx2("SOUND_SHIELDHIT","SOUND_SHIELDHIT2"), m_foley_flags: 0 },
        FoleyType::FOLEY_SWING => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_SWING"), m_foley_flags: 0 },
        FoleyType::FOLEY_BONK => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_BONK"), m_foley_flags: 0 },
        FoleyType::FOLEY_RAIN => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_RAIN"), m_foley_flags: 5 },
        FoleyType::FOLEY_DOLPHIN_BEFORE_JUMPING => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_DOLPHIN_BEFORE_JUMPING"), m_foley_flags: 0 },
        FoleyType::FOLEY_DOLPHIN_APPEARS => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_DOLPHIN_APPEARS"), m_foley_flags: 0 },
        FoleyType::FOLEY_PLANT_WATER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_PLANT_WATER"), m_foley_flags: 0 },
        FoleyType::FOLEY_ZOMBIE_ENTERING_WATER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_ZOMBIE_ENTERING_WATER"), m_foley_flags: 0 },
        FoleyType::FOLEY_GRAVEBUSTERCHOMP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_GRAVEBUSTERCHOMP"), m_foley_flags: 4 },
        FoleyType::FOLEY_CHERRYBOMB => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_CHERRYBOMB"), m_foley_flags: 0 },
        FoleyType::FOLEY_JALAPENO_IGNITE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_JALAPENO"), m_foley_flags: 0 },
        FoleyType::FOLEY_REVERSE_EXPLOSION => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_REVERSE_EXPLOSION"), m_foley_flags: 0 },
        FoleyType::FOLEY_PLASTIC_HIT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 5.0, m_sfx_id: sfx2("SOUND_PLASTICHIT","SOUND_PLASTICHIT2"), m_foley_flags: 0 },
        FoleyType::FOLEY_WINMUSIC => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_WINMUSIC"), m_foley_flags: 8 },
        FoleyType::FOLEY_BALLOONINFLATE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_BALLOONINFLATE"), m_foley_flags: 0 },
        FoleyType::FOLEY_BIGCHOMP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -2.0, m_sfx_id: sfx("SOUND_BIGCHOMP"), m_foley_flags: 0 },
        FoleyType::FOLEY_MELONIMPACT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -5.0, m_sfx_id: sfx2("SOUND_MELONIMPACT","SOUND_MELONIMPACT2"), m_foley_flags: 0 },
        FoleyType::FOLEY_PLANTGROW => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -2.0, m_sfx_id: sfx("SOUND_PLANTGROW"), m_foley_flags: 0 },
        FoleyType::FOLEY_SHOOP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -5.0, m_sfx_id: sfx("SOUND_SHOOP"), m_foley_flags: 0 },
        FoleyType::FOLEY_JUICY => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_JUICY"), m_foley_flags: 0 },
        FoleyType::FOLEY_NEWSPAPER_RARRGH => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -2.0, m_sfx_id: sfx3("SOUND_NEWSPAPER_RARRGH","SOUND_NEWSPAPER_RARRGH2","SOUND_NEWSPAPER_RARRGH2"), m_foley_flags: 0 },
        FoleyType::FOLEY_NEWSPAPER_RIP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: -2.0, m_sfx_id: sfx("SOUND_NEWSPAPER_RIP"), m_foley_flags: 0 },
        FoleyType::FOLEY_FLOOP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_FLOOP"), m_foley_flags: 0 },
        FoleyType::FOLEY_COFFEE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_COFFEE"), m_foley_flags: 0 },
        FoleyType::FOLEY_LOW_GROAN => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx2("SOUND_LOWGROAN","SOUND_LOWGROAN2"), m_foley_flags: 0 },
        FoleyType::FOLEY_PRIZE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_PRIZE"), m_foley_flags: 0 },
        FoleyType::FOLEY_YUCK => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 1.0, m_sfx_id: sfx3("SOUND_YUCK","SOUND_YUCK","SOUND_YUCK2"), m_foley_flags: 0 },
        FoleyType::FOLEY_UMBRELLA => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_THROW2"), m_foley_flags: 0 },
        FoleyType::FOLEY_GRASSSTEP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 2.0, m_sfx_id: sfx("SOUND_GRASSSTEP"), m_foley_flags: 0 },
        FoleyType::FOLEY_SHOVEL => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 5.0, m_sfx_id: sfx("SOUND_SHOVEL"), m_foley_flags: 0 },
        FoleyType::FOLEY_COB_LAUNCH => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_COBLAUNCH"), m_foley_flags: 0 },
        FoleyType::FOLEY_WATERING => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_WATERING"), m_foley_flags: 0 },
        FoleyType::FOLEY_POLEVAULT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 5.0, m_sfx_id: sfx("SOUND_POLEVAULT"), m_foley_flags: 0 },
        FoleyType::FOLEY_GRAVESTONE_RUMBLE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 10.0, m_sfx_id: sfx("SOUND_GRAVESTONE_RUMBLE"), m_foley_flags: 0 },
        FoleyType::FOLEY_DIRT_RISE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 5.0, m_sfx_id: sfx("SOUND_DIRT_RISE"), m_foley_flags: 0 },
        FoleyType::FOLEY_FERTILIZER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_FERTILIZER"), m_foley_flags: 0 },
        FoleyType::FOLEY_PORTAL => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_PORTAL"), m_foley_flags: 0 },
        FoleyType::FOLEY_WAKEUP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_WAKEUP"), m_foley_flags: 0 },
        FoleyType::FOLEY_BUGSPRAY => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_BUGSPRAY"), m_foley_flags: 0 },
        FoleyType::FOLEY_SCREAM => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_SCREAM"), m_foley_flags: 0 },
        FoleyType::FOLEY_PAPER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_PAPER"), m_foley_flags: 0 },
        FoleyType::FOLEY_MONEYFALLS => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_MONEYFALLS"), m_foley_flags: 0 },
        FoleyType::FOLEY_IMP => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 5.0, m_sfx_id: sfx2("SOUND_IMP","SOUND_IMP2"), m_foley_flags: 0 },
        FoleyType::FOLEY_HYDRAULIC_SHORT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 3.0, m_sfx_id: sfx("SOUND_HYDRAULIC_SHORT"), m_foley_flags: 0 },
        FoleyType::FOLEY_HYDRAULIC => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_HYDRAULIC"), m_foley_flags: 0 },
        FoleyType::FOLEY_GARGANTUDEATH => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 3.0, m_sfx_id: sfx("SOUND_GARGANTUDEATH"), m_foley_flags: 0 },
        FoleyType::FOLEY_CERAMIC => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_CERAMIC"), m_foley_flags: 0 },
        FoleyType::FOLEY_BOSS_BOULDER_ATTACK => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_BOSSBOULDERATTACK"), m_foley_flags: 0 },
        FoleyType::FOLEY_CHIME => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_CHIME"), m_foley_flags: 0 },
        FoleyType::FOLEY_CRAZY_DAVE_SHORT => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx3("SOUND_CRAZYDAVESHORT1","SOUND_CRAZYDAVESHORT2","SOUND_CRAZYDAVESHORT3"), m_foley_flags: 16 },
        FoleyType::FOLEY_CRAZY_DAVE_LONG => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx3("SOUND_CRAZYDAVELONG1","SOUND_CRAZYDAVELONG2","SOUND_CRAZYDAVELONG3"), m_foley_flags: 16 },
        FoleyType::FOLEY_CRAZY_DAVE_EXTRA_LONG => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx3("SOUND_CRAZYDAVEEXTRALONG1","SOUND_CRAZYDAVEEXTRALONG2","SOUND_CRAZYDAVEEXTRALONG3"), m_foley_flags: 16 },
        FoleyType::FOLEY_CRAZY_DAVE_CRAZY => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_CRAZYDAVECRAZY"), m_foley_flags: 0 },
        FoleyType::FOLEY_PHONOGRAPH => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_PHONOGRAPH"), m_foley_flags: 0 },
        FoleyType::FOLEY_DANCER => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_DANCER"), m_foley_flags: 6 },
        FoleyType::FOLEY_FINAL_FANFARE => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_FINALFANFARE"), m_foley_flags: 0 },
        FoleyType::FOLEY_CRAZY_DAVE_SCREAM => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_CRAZYDAVESCREAM"), m_foley_flags: 0 },
        FoleyType::FOLEY_CRAZY_DAVE_SCREAM_2 => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: sfx("SOUND_CRAZYDAVESCREAM2"), m_foley_flags: 0 },
        _ => FoleyParams { m_foley_type: the_foley_type, m_pitch_range: 0.0, m_sfx_id: empty, m_foley_flags: 0 },
    }
}

pub fn lookup_foley(the_foley_type: FoleyType, g_foley_param_array: &[FoleyParams]) -> &FoleyParams {
    _tod_assert(the_foley_type as i32 >= 0 && (the_foley_type as usize) < g_foley_param_array.len(), file!(), line!(), "");
    &g_foley_param_array[the_foley_type as usize]
}

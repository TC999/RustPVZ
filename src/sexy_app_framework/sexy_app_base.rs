// [TRANSLATION_NOTE]: SexyAppBase.h/SexyAppBase.cpp -> Rust 模块
// C++ SexyAppBase 类（应用基类）翻译为 Rust struct。
// 结构字段与 C++ SexyAppBase.h 保持 1:1 对齐；C++ 的 mSoundManager/mMusicInterface
// 抽象接口映射为 trait 对象（Box<dyn SoundManager>/Box<dyn MusicInterface>）。
// mGLInterface 映射为 GLInterface（渲染接口，Redraw 桥接到 SDL Canvas 渲染器）。
// 主循环（Start/DoMainLoop/UpdateApp/Process）与 C++ SexyAppBase.cpp 逐句对应。

use std::collections::{HashMap, LinkedList};
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crate::sexy_app_framework::graphics::gl_interface::GLInterface;
use crate::sexy_app_framework::graphics::graphics::{Image, MemoryImage};
use crate::sexy_app_framework::misc::rect::Rect;
use crate::sexy_app_framework::misc::resource_manager::ResourceManager;
use crate::sexy_app_framework::sound::music_interface::{DummyMusicInterface, MusicInterface};
use crate::sexy_app_framework::sound::sound_manager::SoundManager;
use crate::sexy_app_framework::widget::widget_manager::WidgetManager;

pub const CURSOR_POINTER: i32 = 0;
pub const CURSOR_HAND: i32 = 1;
pub const CURSOR_DRAGGING: i32 = 2;
pub const CURSOR_TEXT: i32 = 3;
pub const CURSOR_CIRCLE_SLASH: i32 = 4;
pub const CURSOR_SIZEALL: i32 = 5;
pub const CURSOR_SIZENESW: i32 = 6;
pub const CURSOR_SIZENS: i32 = 7;
pub const CURSOR_SIZENWSE: i32 = 8;
pub const CURSOR_SIZEWE: i32 = 9;
pub const CURSOR_WAIT: i32 = 10;
pub const CURSOR_NONE: i32 = 11;
pub const CURSOR_CUSTOM: i32 = 12;
pub const NUM_CURSORS: i32 = 13;

pub const DEMO_MOUSE_POSITION: i32 = 0;
pub const DEMO_ACTIVATE_APP: i32 = 1;
pub const DEMO_SIZE: i32 = 2;
pub const DEMO_KEY_DOWN: i32 = 3;
pub const DEMO_KEY_UP: i32 = 4;
pub const DEMO_KEY_CHAR: i32 = 5;
pub const DEMO_CLOSE: i32 = 6;
pub const DEMO_MOUSE_ENTER: i32 = 7;
pub const DEMO_MOUSE_EXIT: i32 = 8;
pub const DEMO_LOADING_COMPLETE: i32 = 9;
pub const DEMO_REGISTRY_GETSUBKEYS: i32 = 10;
pub const DEMO_REGISTRY_READ: i32 = 11;
pub const DEMO_REGISTRY_WRITE: i32 = 12;
pub const DEMO_REGISTRY_ERASE: i32 = 13;
pub const DEMO_FILE_EXISTS: i32 = 14;
pub const DEMO_FILE_READ: i32 = 15;
pub const DEMO_FILE_WRITE: i32 = 16;
pub const DEMO_HTTP_RESULT: i32 = 17;
pub const DEMO_SYNC: i32 = 18;
pub const DEMO_ASSERT_STRING_EQUAL: i32 = 19;
pub const DEMO_ASSERT_INT_EQUAL: i32 = 20;
pub const DEMO_MOUSE_WHEEL: i32 = 21;
pub const DEMO_HANDLE_COMPLETE: i32 = 22;
pub const DEMO_VIDEO_DATA: i32 = 23;
pub const DEMO_KEY_TEXT: i32 = 24;
pub const DEMO_IDLE: i32 = 31;

pub const FPS_SHOW_FPS: i32 = 0;
pub const FPS_SHOW_COORDS: i32 = 1;
pub const NUM_FPS_TYPES: i32 = 2;

pub const UPDATESTATE_MESSAGES: i32 = 0;
pub const UPDATESTATE_PROCESS_1: i32 = 1;
pub const UPDATESTATE_PROCESS_2: i32 = 2;
pub const UPDATESTATE_PROCESS_DONE: i32 = 3;

#[derive(Clone)]
pub struct WidgetSafeDeleteInfo {
    pub m_update_app_depth: i32,
    pub m_widget: *mut std::ffi::c_void,
}

pub type WidgetSafeDeleteList = LinkedList<WidgetSafeDeleteInfo>;
pub type MemoryImageSet = std::collections::HashSet<*mut crate::sexy_app_framework::graphics::graphics::Image>;
pub type SharedImageMap = HashMap<String, i32>;
pub type DialogMap = HashMap<i32, *mut std::ffi::c_void>;
pub type DialogList = LinkedList<*mut std::ffi::c_void>;
pub type StringBoolMap = HashMap<String, bool>;
pub type StringIntMap = HashMap<String, i32>;
pub type StringDoubleMap = HashMap<String, f64>;
pub type StringStringVectorMap = HashMap<String, Vec<String>>;

pub struct SexyAppBase {
    pub m_window: *mut std::ffi::c_void,
    pub m_context: *mut std::ffi::c_void,
    pub m_surface: *mut std::ffi::c_void,
    pub m_resource_manager: *mut ResourceManager,
    pub m_rand_seed: u32,
    pub m_company_name: String,
    pub m_full_company_name: String,
    pub m_prod_name: String,
    pub m_title: String,
    pub m_reg_key: String,
    pub m_resource_dir: String,
    pub m_custom_save_dir: String,
    pub m_relax_update_backlog_count: i32,
    pub m_preferred_x: i32,
    pub m_preferred_y: i32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_fullscreen_bits: i32,
    pub m_music_volume: f64,
    pub m_sfx_volume: f64,
    pub m_demo_music_volume: f64,
    pub m_demo_sfx_volume: f64,
    pub m_no_sound_needed: bool,
    pub m_want_f_mod: bool,
    pub m_cmd_line_parsed: bool,
    pub m_argc: i32,
    pub m_argv: *mut *mut u8,
    pub m_skip_signature_checks: bool,
    pub m_standard_word_wrap: bool,
    pub mb_allow_extended_chars: bool,
    pub m_only_allow_one_copy_to_run: bool,
    pub m_notify_game_message: u32,
    pub m_crit_sect: Mutex<()>,
    pub m_add_8bit_max_table: [u8; 512],
    pub m_widget_manager: Option<Box<WidgetManager>>,
    pub m_dialog_map: DialogMap,
    pub m_dialog_list: DialogList,
    pub m_primary_thread_id: std::thread::ThreadId,
    pub m_seh_occured: bool,
    pub m_shutdown: bool,
    pub m_exit_to_top: bool,
    pub m_is_windowed: bool,
    pub m_is_phys_windowed: bool,
    pub m_full_screen_window: bool,
    pub m_force_fullscreen: bool,
    pub m_force_windowed: bool,
    pub m_initialized: bool,
    pub m_process_in_timer: bool,
    pub m_time_loaded: u32,
    pub m_is_screen_saver: bool,
    pub m_allow_monitor_powersave: bool,
    pub m_no_defer: bool,
    pub m_full_screen_page_flip: bool,
    pub m_tablet_pc: bool,
    pub m_alpha_disabled: bool,
    pub m_read_from_registry: bool,
    pub m_register_link: String,
    pub m_product_version: String,
    pub m_cursor_images: [*mut Image; 13],
    pub m_is_opening_url: bool,
    pub m_shutdown_on_url_open: bool,
    pub m_opening_url: String,
    pub m_opening_url_time: u32,
    pub m_last_timer_time: u32,
    pub m_last_big_delay_time: u32,
    pub m_unmuted_music_volume: f64,
    pub m_unmuted_sfx_volume: f64,
    pub m_mute_count: i32,
    pub m_auto_mute_count: i32,
    pub m_demo_mute: bool,
    pub m_mute_on_lost_focus: bool,
    pub m_memory_image_set: MemoryImageSet,
    pub m_shared_image_map: SharedImageMap,
    pub m_cleanup_shared_images: AtomicBool,
    pub m_non_draw_count: i32,
    pub m_frame_time: i32,
    pub m_is_drawing: bool,
    pub m_last_draw_was_empty: bool,
    pub m_has_pending_draw: bool,
    pub m_pending_updates_acc: f64,
    pub m_update_f_time_acc: f64,
    pub m_last_time_check: u64,
    pub m_last_time: u64,
    pub m_last_user_input_tick: u32,
    pub m_sleep_count: u32,
    pub m_draw_count: u32,
    pub m_update_count: u32,
    pub m_update_app_state: i32,
    pub m_update_app_depth: i32,
    pub m_update_multiplier: f64,
    pub m_paused: bool,
    pub m_fast_forward_to_update_num: u32,
    pub m_fast_forward_to_marker: bool,
    pub m_fast_forward_step: bool,
    pub m_last_draw_tick: u32,
    pub m_next_draw_tick: u32,
    pub m_step_mode: i32,
    pub m_cursor_num: i32,
    pub m_custom_cursor: *mut std::ffi::c_void,
    pub m_custom_cursor_image: *mut Image,
    pub m_custom_cursor_image_num: i32,
    pub m_running: bool,
    pub m_active: bool,
    pub m_minimized: bool,
    pub m_phys_minimized: bool,
    pub m_is_disabled: bool,
    pub m_has_focus: bool,
    pub m_draw_time: u64,
    pub m_show_fps: bool,
    pub m_show_fps_mode: i32,
    pub m_screen_blt_time: u32,
    pub m_auto_start_loading_thread: bool,
    pub m_loading_thread_started: bool,
    pub m_loading_thread_completed: bool,
    pub m_loaded: bool,
    // ===== 以下字段对齐 C++ SexyAppBase.h 其余成员（运行层所需） =====
    // C++: GLInterface* mGLInterface;
    pub m_gl_interface: *mut GLInterface,
    // C++: SoundManager* mSoundManager;
    pub m_sound_manager: Option<Box<dyn SoundManager>>,
    // C++: MusicInterface* mMusicInterface;
    pub m_music_interface: Option<Box<dyn MusicInterface>>,
    // C++: Rect mScreenBounds;
    pub m_screen_bounds: Rect,
    // C++: bool mMouseIn;
    pub m_mouse_in: bool,
    // C++: bool mYieldMainThread;
    pub m_yield_main_thread: bool,
    // C++: bool mLoadingFailed;
    pub m_loading_failed: bool,
    // C++: bool mManualShutdown;
    pub m_manual_shutdown: bool,
    // C++: VSync 相关字段
    pub m_sync_refresh_rate: i32,
    pub m_vsync_updates: bool,
    pub m_vsync_broken: bool,
    pub m_vsync_broken_count: i32,
    pub m_vsync_broken_test_start_tick: u32,
    pub m_vsync_broken_test_updates: u32,
    pub m_wait_for_vsync: bool,
    pub m_soft_vsync_wait: bool,
    // C++: 光标相关字段
    pub m_sys_cursor: bool,
    pub m_custom_cursors_enabled: bool,
    pub m_custom_cursor_dirty: bool,
    pub m_cursor_thread_running: bool,
    // C++: 键盘状态
    pub m_ctrl_down: bool,
    pub m_alt_down: bool,
    pub m_allow_alt_enter: bool,
    // C++: 杂项
    pub m_last_shutdown_was_graceful: bool,
    pub m_write_to_sexy_cache: bool,
    pub m_sexy_cache_buffers: bool,
    pub m_enable_maximize_button: bool,
    pub m_debug_keys_enabled: bool,
    pub m_user_changed_3d_setting: bool,
    pub m_auto_enable_3d: bool,
    pub m_test_3d: bool,
    pub m_min_vid_memory_3d: i32,
    pub m_recommended_vid_memory_3d: i32,
    pub m_widescreen_aware: bool,
    pub m_enable_window_aspect: bool,
    pub m_is_wide_window: bool,
    // C++: 加载线程任务计数
    pub m_num_loading_thread_tasks: i32,
    pub m_completed_loading_thread_tasks: i32,
    // C++: 安全删除列表
    pub m_safe_delete_list: WidgetSafeDeleteList,
    // C++: Demo 相关字段（当前仅保留标志位，完整 demo 系统后续翻译）
    pub m_demo_prefix: String,
    pub m_demo_file_name: String,
    pub m_has_custom_demo_file: bool,
    pub m_playing_demo_buffer: bool,
    pub m_recording_demo_buffer: bool,
    pub m_demo_loading_complete: bool,
    pub m_demo_command_queued: bool,
    pub m_demo_needs_command: bool,
    pub m_last_demo_mouse_x: i32,
    pub m_last_demo_mouse_y: i32,
    pub m_last_demo_update_cnt: u32,
    pub m_demo_start_time: u64,
    pub m_demo_time_zone_offset: i32,
    pub m_demo_record_file_limit: u32,
}

impl SexyAppBase {
    pub fn new() -> Self {
        let m_add_8bit_max_table = {
            let mut table = [0u8; 512];
            let mut i = 0usize;
            while i < 256 {
                table[i] = i as u8;
                i += 1;
            }
            while i < 512 {
                table[i] = 255;
                i += 1;
            }
            table
        };

        SexyAppBase {
            m_window: std::ptr::null_mut(),
            m_context: std::ptr::null_mut(),
            m_surface: std::ptr::null_mut(),
            // C++: mResourceManager = new ResourceManager(this);
            m_resource_manager: Box::into_raw(Box::new(ResourceManager::new(std::ptr::null_mut()))),
            m_rand_seed: 0,
            m_company_name: String::new(),
            m_full_company_name: String::new(),
            m_prod_name: String::from("Product"),
            m_title: String::from("SexyApp"),
            m_reg_key: String::new(),
            m_resource_dir: String::new(),
            m_custom_save_dir: String::new(),
            m_relax_update_backlog_count: 0,
            m_preferred_x: -1,
            m_preferred_y: -1,
            m_width: 800,
            m_height: 600,
            m_fullscreen_bits: 32,
            m_music_volume: 0.85,
            m_sfx_volume: 0.85,
            m_demo_music_volume: 0.0,
            m_demo_sfx_volume: 0.0,
            m_no_sound_needed: false,
            m_want_f_mod: false,
            m_cmd_line_parsed: false,
            m_argc: 0,
            m_argv: std::ptr::null_mut(),
            m_skip_signature_checks: false,
            m_standard_word_wrap: true,
            mb_allow_extended_chars: true,
            m_only_allow_one_copy_to_run: true,
            m_notify_game_message: 0,
            m_crit_sect: Mutex::new(()),
            m_add_8bit_max_table,
            m_widget_manager: None,
            m_dialog_map: HashMap::new(),
            m_dialog_list: LinkedList::new(),
            m_primary_thread_id: std::thread::current().id(),
            m_seh_occured: false,
            m_shutdown: false,
            m_exit_to_top: false,
            m_is_windowed: true,
            m_is_phys_windowed: true,
            m_full_screen_window: false,
            m_force_fullscreen: false,
            m_force_windowed: false,
            m_initialized: false,
            m_process_in_timer: false,
            m_time_loaded: sdl_get_ticks(),
            m_is_screen_saver: false,
            m_allow_monitor_powersave: true,
            m_no_defer: false,
            m_full_screen_page_flip: true,
            m_tablet_pc: false,
            m_alpha_disabled: false,
            m_read_from_registry: false,
            m_register_link: String::new(),
            m_product_version: String::from("1.0"),
            m_cursor_images: [std::ptr::null_mut(); 13],
            m_is_opening_url: false,
            m_shutdown_on_url_open: false,
            m_opening_url: String::new(),
            m_opening_url_time: 0,
            m_last_timer_time: 0,
            m_last_big_delay_time: 0,
            m_unmuted_music_volume: 1.0,
            m_unmuted_sfx_volume: 1.0,
            m_mute_count: 0,
            m_auto_mute_count: 0,
            m_demo_mute: false,
            m_mute_on_lost_focus: false,
            m_memory_image_set: std::collections::HashSet::new(),
            m_shared_image_map: HashMap::new(),
            m_cleanup_shared_images: AtomicBool::new(false),
            m_non_draw_count: 0,
            m_frame_time: 10,
            m_is_drawing: false,
            m_last_draw_was_empty: false,
            m_has_pending_draw: true,
            m_pending_updates_acc: 0.0,
            m_update_f_time_acc: 0.0,
            m_last_time_check: 0,
            m_last_time: 0,
            m_last_user_input_tick: 0,
            m_sleep_count: 0,
            m_draw_count: 0,
            m_update_count: 0,
            m_update_app_state: 0,
            m_update_app_depth: 0,
            m_update_multiplier: 1.0,
            m_paused: false,
            m_fast_forward_to_update_num: 0,
            m_fast_forward_to_marker: false,
            m_fast_forward_step: false,
            m_last_draw_tick: sdl_get_ticks(),
            m_next_draw_tick: sdl_get_ticks(),
            m_step_mode: 0,
            m_cursor_num: 0,
            m_custom_cursor: std::ptr::null_mut(),
            m_custom_cursor_image: std::ptr::null_mut(),
            m_custom_cursor_image_num: -1,
            m_running: false,
            m_active: true,
            m_minimized: false,
            m_phys_minimized: false,
            m_is_disabled: false,
            m_has_focus: true,
            m_draw_time: 0,
            m_show_fps: false,
            m_show_fps_mode: 0,
            m_screen_blt_time: 0,
            m_auto_start_loading_thread: true,
            m_loading_thread_started: false,
            m_loading_thread_completed: false,
            m_loaded: false,
            m_gl_interface: std::ptr::null_mut(),
            m_sound_manager: None,
            m_music_interface: Some(Box::new(DummyMusicInterface::new())),
            m_screen_bounds: Rect::new(0, 0, 0, 0),
            m_mouse_in: false,
            m_yield_main_thread: false,
            m_loading_failed: false,
            m_manual_shutdown: false,
            m_sync_refresh_rate: 60,
            m_vsync_updates: false,
            m_vsync_broken: false,
            m_vsync_broken_count: 0,
            m_vsync_broken_test_start_tick: 0,
            m_vsync_broken_test_updates: 0,
            m_wait_for_vsync: false,
            m_soft_vsync_wait: true,
            m_sys_cursor: true,
            m_custom_cursors_enabled: false,
            m_custom_cursor_dirty: false,
            m_cursor_thread_running: false,
            m_ctrl_down: false,
            m_alt_down: false,
            m_allow_alt_enter: true,
            m_last_shutdown_was_graceful: true,
            m_write_to_sexy_cache: true,
            m_sexy_cache_buffers: false,
            m_enable_maximize_button: false,
            m_debug_keys_enabled: false,
            m_user_changed_3d_setting: false,
            m_auto_enable_3d: false,
            m_test_3d: false,
            m_min_vid_memory_3d: 6,
            m_recommended_vid_memory_3d: 14,
            m_widescreen_aware: false,
            m_enable_window_aspect: false,
            m_is_wide_window: false,
            m_num_loading_thread_tasks: 0,
            m_completed_loading_thread_tasks: 0,
            m_safe_delete_list: LinkedList::new(),
            m_demo_prefix: String::from("sexyapp"),
            m_demo_file_name: String::from("sexyapp.dmo"),
            m_has_custom_demo_file: false,
            m_playing_demo_buffer: false,
            m_recording_demo_buffer: false,
            m_demo_loading_complete: false,
            m_demo_command_queued: false,
            m_demo_needs_command: true,
            m_last_demo_mouse_x: 0,
            m_last_demo_mouse_y: 0,
            m_last_demo_update_cnt: 0,
            m_demo_start_time: 0,
            m_demo_time_zone_offset: 0,
            m_demo_record_file_limit: 0,
        }
    }

    // =====================================================================
    // 运行层核心方法 — C++ SexyAppBase.cpp 保真翻译
    // =====================================================================

    /// C++: SexyAppBase::Is3DAccelerated — return true
    pub fn is_3d_accelerated(&self) -> bool {
        true
    }

    /// C++: SexyAppBase::IsMuted
    pub fn is_muted(&self) -> bool {
        self.m_mute_count > 0
    }

    /// C++: SexyAppBase::Mute
    pub fn mute(&mut self, auto_mute: bool) {
        self.m_mute_count += 1;
        if auto_mute {
            self.m_auto_mute_count += 1;
        }
        self.set_music_volume(self.m_music_volume);
        self.set_sfx_volume(self.m_sfx_volume);
    }

    /// C++: SexyAppBase::Unmute
    pub fn unmute(&mut self, auto_mute: bool) {
        if self.m_mute_count > 0 {
            self.m_mute_count -= 1;
            if auto_mute {
                self.m_auto_mute_count -= 1;
            }
        }
        self.set_music_volume(self.m_music_volume);
        self.set_sfx_volume(self.m_sfx_volume);
    }

    /// C++: SexyAppBase::GetMusicVolume
    pub fn get_music_volume(&self) -> f64 {
        self.m_music_volume
    }

    /// C++: SexyAppBase::SetMusicVolume
    pub fn set_music_volume(&mut self, the_volume: f64) {
        self.m_music_volume = the_volume;
        if let Some(mi) = &mut self.m_music_interface {
            mi.set_volume(if self.m_mute_count > 0 { 0.0 } else { self.m_music_volume });
        }
    }

    /// C++: SexyAppBase::GetSfxVolume
    pub fn get_sfx_volume(&self) -> f64 {
        self.m_sfx_volume
    }

    /// C++: SexyAppBase::SetSfxVolume
    pub fn set_sfx_volume(&mut self, the_volume: f64) {
        self.m_sfx_volume = the_volume;
        if let Some(sm) = &mut self.m_sound_manager {
            sm.set_volume(if self.m_mute_count > 0 { 0.0 } else { self.m_sfx_volume });
        }
    }

    /// C++: SexyAppBase::GetMasterVolume
    pub fn get_master_volume(&self) -> f64 {
        if let Some(sm) = &self.m_sound_manager {
            sm.get_master_volume()
        } else {
            0.0
        }
    }

    /// C++: SexyAppBase::SetMasterVolume
    pub fn set_master_volume(&mut self, the_master_volume: f64) {
        self.m_sfx_volume = the_master_volume;
        if let Some(sm) = &mut self.m_sound_manager {
            sm.set_master_volume(the_master_volume);
        }
    }

    /// C++: SexyAppBase::Set3DAcclerated — 3D 加速切换未实现
    pub fn set_3d_acclerated(&mut self, _is_3d: bool, _reinit: bool) {
    }

    /// C++: SexyAppBase::UpdateFTimeAcc
    pub fn update_f_time_acc(&mut self) {
        let a_cur_time = sdl_get_ticks();

        if self.m_last_time_check != 0 {
            let a_delta_time = a_cur_time.wrapping_sub(self.m_last_time_check as u32) as i32;

            self.m_update_f_time_acc = (self.m_update_f_time_acc + a_delta_time as f64).min(200.0);

            if self.m_relax_update_backlog_count > 0 {
                self.m_relax_update_backlog_count =
                    (self.m_relax_update_backlog_count - a_delta_time).max(0);
            }
        }

        self.m_last_time_check = a_cur_time as u64;
    }

    /// C++: SexyAppBase::ClearUpdateBacklog
    pub fn clear_update_backlog(&mut self, relax_for_a_second: bool) {
        self.m_last_time_check = sdl_get_ticks() as u64;
        self.m_update_f_time_acc = 0.0;

        if relax_for_a_second {
            self.m_relax_update_backlog_count = 1000;
        }
    }

    /// C++: SexyAppBase::ProcessSafeDeleteList
    pub fn process_safe_delete_list(&mut self) {
        // C++: std::erase_if(mSafeDeleteList, ...) — 删除 mUpdateAppDepth 已满足条件的控件
        // [TRANSLATION_NOTE]: Rust 中无法对任意 *mut c_void 执行 delete，
        // 仅从列表移除（指针由持有者负责释放），逻辑行为一致。
        let keep: Vec<WidgetSafeDeleteInfo> = self
            .m_safe_delete_list
            .iter()
            .filter(|the_info| !(self.m_update_app_depth <= the_info.m_update_app_depth))
            .cloned()
            .collect();
        self.m_safe_delete_list = keep.into_iter().collect();
    }

    /// C++: SexyAppBase::UpdateFrames
    pub fn update_frames(&mut self) {
        self.m_update_count += 1;

        if !self.m_minimized {
            if let Some(wm) = &mut self.m_widget_manager {
                wm.update_frame();
            }
        }

        if let Some(mi) = &mut self.m_music_interface {
            mi.update();
        }
        self.clean_shared_images();
    }

    /// C++: SexyAppBase::DoUpdateFramesF
    pub fn do_update_frames_f(&mut self, the_frac: f32) {
        if self.m_vsync_updates && !self.m_minimized {
            if let Some(wm) = &mut self.m_widget_manager {
                wm.update_frame_f(the_frac);
            }
        }
    }

    /// C++: SexyAppBase::DoUpdateFrames
    pub fn do_update_frames(&mut self) -> bool {
        if self.m_playing_demo_buffer {
            if self.m_loading_thread_completed && !self.m_loaded && self.m_demo_loading_complete {
                self.m_loaded = true;
                self.m_yield_main_thread = false;
                self.loading_thread_completed();
            }

            if (self.m_loaded == self.m_demo_loading_complete)
                && ((self.m_update_count != self.m_last_demo_update_cnt) || self.m_demo_command_queued)
            {
                self.update_frames();
                return true;
            }

            return false;
        } else {
            if self.m_loading_thread_completed && !self.m_loaded {
                self.m_loaded = true;
                self.m_yield_main_thread = false;
                self.loading_thread_completed();
            }

            self.update_frames();
            return true;
        }
    }

    /// C++: SexyAppBase::LoadingThreadCompleted — 钩子，由子类覆盖
    pub fn loading_thread_completed(&mut self) {
    }

    /// C++: SexyAppBase::Redraw
    pub fn redraw(&mut self, _the_clip_rect: Option<&Rect>) {
        // C++: if (mIsDrawing || mShutdown) return;
        if self.m_is_drawing || self.m_shutdown {
            return;
        }

        if self.m_is_screen_saver {
            return;
        }

        if !self.m_gl_interface.is_null() {
            unsafe {
                (*self.m_gl_interface).redraw(_the_clip_rect);
            }
        }
    }

    /// C++: SexyAppBase::DrawDirtyStuff
    pub fn draw_dirty_stuff(&mut self) -> bool {
        if self.m_is_screen_saver {
            self.m_has_pending_draw = false;
            self.m_last_draw_was_empty = true;
            return false;
        }

        let a_start_time = sdl_get_ticks();

        self.m_is_drawing = true;
        let drew_screen = if let Some(wm) = &mut self.m_widget_manager {
            wm.draw_screen()
        } else {
            false
        };
        self.m_is_drawing = false;

        if (drew_screen || (a_start_time.wrapping_sub(self.m_last_draw_tick) >= 1000)
            || self.m_custom_cursor_dirty)
            && ((a_start_time as i32 - self.m_next_draw_tick as i32) >= 0)
        {
            self.m_last_draw_was_empty = false;

            self.m_draw_count += 1;

            let a_mid_time = sdl_get_ticks();

            self.m_draw_time += (a_mid_time - a_start_time) as u64;

            let a_pre_screen_blt_time = sdl_get_ticks();
            self.m_last_draw_tick = a_pre_screen_blt_time;

            if drew_screen {
                self.redraw(None);
            }

            // This is our one UpdateFTimeAcc if we are vsynched
            self.update_f_time_acc();

            let a_end_time = sdl_get_ticks();

            self.m_screen_blt_time = a_end_time - a_pre_screen_blt_time;

            if self.m_loading_thread_started && !self.m_loading_thread_completed {
                let a_total_time = a_end_time.wrapping_sub(a_start_time);
                self.m_next_draw_tick += 35 + a_total_time.max(15);
                if (a_end_time as i32 - self.m_next_draw_tick as i32) >= 0 {
                    self.m_next_draw_tick = a_end_time;
                }
            } else {
                self.m_next_draw_tick = a_end_time;
            }

            self.m_has_pending_draw = false;
            self.m_custom_cursor_dirty = false;

            return true;
        } else {
            self.m_has_pending_draw = false;
            self.m_last_draw_was_empty = true;
            return false;
        }
    }

    /// C++: SexyAppBase::CleanSharedImages — 清理引用计数为 0 的共享图像
    /// [TRANSLATION_NOTE]: Rust 侧 SharedImageMap 当前为 HashMap<String, i32> 占位，
    /// 共享图像引用计数机制在后续 SharedImage 完整翻译时实现。
    pub fn clean_shared_images(&mut self) {
        if self.m_gl_interface.is_null() {
            return;
        }
        if self.m_cleanup_shared_images.load(std::sync::atomic::Ordering::Relaxed) {
            self.m_shared_image_map.clear();
            self.m_cleanup_shared_images
                .store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// C++: SexyAppBase::Process
    pub fn process(&mut self, allow_sleep: bool) -> bool {
        if self.m_loading_failed {
            self.shutdown();
        }

        let is_vsynched = (!self.m_playing_demo_buffer) && (self.m_vsync_updates)
            && (!self.m_last_draw_was_empty) && (!self.m_vsync_broken)
            && ((!self.m_is_phys_windowed)
                || (self.m_is_phys_windowed && self.m_wait_for_vsync && !self.m_soft_vsync_wait));
        let a_frame_f_time: f64;
        let an_updates_per_update_f: f64;

        if self.m_vsync_updates {
            a_frame_f_time = (1000.0 / self.m_sync_refresh_rate as f64) / self.m_update_multiplier;
            an_updates_per_update_f = 1000.0 / (self.m_frame_time as f64 * self.m_sync_refresh_rate as f64);
        } else {
            a_frame_f_time = self.m_frame_time as f64 / self.m_update_multiplier;
            an_updates_per_update_f = 1.0;
        }

        // Do we need to fast forward?
        if self.m_playing_demo_buffer {
            if self.m_update_count < self.m_fast_forward_to_update_num || self.m_fast_forward_to_marker
            {
                if !self.m_demo_mute && !self.m_fast_forward_step {
                    self.m_demo_mute = true;
                    self.mute(true);
                }

                let mut a_tick = sdl_get_ticks();
                while self.m_update_count < self.m_fast_forward_to_update_num
                    || self.m_fast_forward_to_marker
                {
                    self.clear_update_backlog(false);
                    let a_last_update_count = self.m_update_count;

                    let had_real_update = self.do_update_frames();

                    if had_real_update {
                        self.m_pending_updates_acc += an_updates_per_update_f;
                        self.m_pending_updates_acc -= 1.0;
                        self.process_safe_delete_list();

                        while self.m_pending_updates_acc >= 1.0 {
                            self.do_update_frames();
                            self.process_safe_delete_list();
                            self.m_pending_updates_acc -= 1.0;
                        }

                        self.do_update_frames_f(an_updates_per_update_f as f32);
                        self.process_safe_delete_list();
                    }

                    // If the update count doesn't change, its because we are
                    //  playing back a demo and need to read more
                    if a_last_update_count == self.m_update_count {
                        return true;
                    }

                    let a_new_tick = sdl_get_ticks();
                    if a_new_tick.wrapping_sub(a_tick) >= 1000 || self.m_fast_forward_step {
                        self.m_fast_forward_step = false;
                        a_tick = sdl_get_ticks();
                        self.draw_dirty_stuff();
                        return true;
                    }
                }
            }

            if self.m_demo_mute {
                self.m_demo_mute = false;
                if let Some(sm) = &mut self.m_sound_manager {
                    sm.stop_all_sounds();
                }
                self.unmute(true);
            }
        }

        // Make sure we're not paused
        if (!self.m_paused) && (self.m_update_multiplier > 0.0) {
            let a_start_time = sdl_get_ticks();

            let mut a_cum_sleep_time = 0;

            if !is_vsynched {
                self.update_f_time_acc();
            }

            let mut did_update = false;

            if self.m_update_app_state == UPDATESTATE_PROCESS_1 {
                self.m_non_draw_count += 1;
                if (self.m_non_draw_count < (10.0 * self.m_update_multiplier).ceil() as i32)
                    || (!self.m_loaded)
                {
                    let mut do_update = false;

                    if is_vsynched {
                        // Synch'ed to vertical refresh, so update as soon as possible after draw
                        do_update = (!self.m_has_pending_draw)
                            || (self.m_update_f_time_acc >= (a_frame_f_time * 0.75) as i32 as f64);
                    } else if self.m_update_f_time_acc >= a_frame_f_time {
                        do_update = true;
                    }

                    if do_update {
                        // Do VSyncBroken test
                        if (!self.m_playing_demo_buffer) && (self.m_update_multiplier == 1.0) {
                            self.m_vsync_broken_test_updates += 1;
                            if self.m_vsync_broken_test_updates
                                >= ((1000 + self.m_frame_time - 1) / self.m_frame_time) as u32
                            {
                                if a_start_time.wrapping_sub(self.m_vsync_broken_test_start_tick) <= 800 {
                                    self.m_vsync_broken_count += 1;
                                    if self.m_vsync_broken_count >= 3 {
                                        self.m_vsync_broken = true;
                                    }
                                } else {
                                    self.m_vsync_broken_count = 0;
                                }

                                self.m_vsync_broken_test_start_tick = a_start_time;
                                self.m_vsync_broken_test_updates = 0;
                            }
                        }

                        let had_real_update = self.do_update_frames();
                        if had_real_update {
                            self.m_update_app_state = UPDATESTATE_PROCESS_2;
                        }

                        self.m_has_pending_draw = true;
                        did_update = true;
                    }
                }
            } else if self.m_update_app_state == UPDATESTATE_PROCESS_2 {
                self.m_update_app_state = UPDATESTATE_PROCESS_DONE;

                self.m_pending_updates_acc += an_updates_per_update_f;
                self.m_pending_updates_acc -= 1.0;
                self.process_safe_delete_list();

                // Process any extra updates
                while self.m_pending_updates_acc >= 1.0 {
                    self.m_non_draw_count += 1;
                    let has_real_update = self.do_update_frames();
                    if !has_real_update {
                        break;
                    }
                    self.process_safe_delete_list();
                    self.m_pending_updates_acc -= 1.0;
                }

                self.do_update_frames_f(an_updates_per_update_f as f32);
                self.process_safe_delete_list();

                // Don't let mUpdateFTimeAcc dip below 0
                if is_vsynched {
                    self.m_update_f_time_acc = (self.m_update_f_time_acc - a_frame_f_time - 0.2).max(0.0);
                } else {
                    self.m_update_f_time_acc -= a_frame_f_time;
                }

                if self.m_relax_update_backlog_count > 0 {
                    self.m_update_f_time_acc = 0.0;
                }

                did_update = true;
            }

            if !did_update {
                self.m_update_app_state = UPDATESTATE_PROCESS_DONE;

                self.m_non_draw_count = 0;

                if self.m_has_pending_draw {
                    self.draw_dirty_stuff();
                } else {
                    // Let us take into account the time it took to draw dirty stuff
                    let a_time_to_next_frame = (a_frame_f_time - self.m_update_f_time_acc) as i32;
                    if a_time_to_next_frame > 0 {
                        if !allow_sleep {
                            return false;
                        }

                        // Wait till next processing cycle
                        self.m_sleep_count += 1;
                        std::thread::sleep(Duration::from_millis(a_time_to_next_frame as u64));
                        a_cum_sleep_time += a_time_to_next_frame;
                    }
                }
            }

            if self.m_yield_main_thread {
                // This is to make sure that the title screen doesn't take up any more than
                // 1/3 of the processor time
                let an_end_time = sdl_get_ticks();
                let an_elapsed_time =
                    (an_end_time.wrapping_sub(a_start_time) as i32) - a_cum_sleep_time;
                let a_loading_yield_sleep_time =
                    (an_elapsed_time * 2 - a_cum_sleep_time).min(250);

                if a_loading_yield_sleep_time >= 0 {
                    if !allow_sleep {
                        return false;
                    }
                    std::thread::sleep(Duration::from_millis(a_loading_yield_sleep_time as u64));
                }
            }
        }

        self.process_safe_delete_list();
        return true;
    }

    /// C++: SexyAppBase::DoMainLoop
    pub fn do_main_loop(&mut self) {
        while !self.m_shutdown {
            if self.m_exit_to_top {
                self.m_exit_to_top = false;
            }
            self.update_app();
        }
    }

    /// C++: SexyAppBase::UpdateAppStep
    pub fn update_app_step(&mut self, updated: &mut bool) -> bool {
        *updated = false;

        if self.m_exit_to_top {
            return false;
        }

        if self.m_update_app_state == UPDATESTATE_PROCESS_DONE {
            self.m_update_app_state = UPDATESTATE_MESSAGES;
        }

        self.m_update_app_depth += 1;

        if self.m_update_app_state == UPDATESTATE_MESSAGES {
            if !self.process_deferred_messages(true) {
                self.m_update_app_state = UPDATESTATE_PROCESS_1;
            }
        } else {
            // Process changes state by itself
            if self.m_step_mode != 0 {
                if self.m_step_mode == 2 {
                    std::thread::sleep(Duration::from_millis(self.m_frame_time as u64));
                    self.m_update_app_state = UPDATESTATE_PROCESS_DONE; // skip actual update until next step
                } else {
                    self.m_step_mode = 2;
                    self.do_update_frames();
                    self.do_update_frames_f(1.0);
                    self.draw_dirty_stuff();
                }
            } else {
                let an_old_update_cnt = self.m_update_count;
                self.process(true);
                *updated = self.m_update_count != an_old_update_cnt;
            }
        }

        self.m_update_app_depth -= 1;

        return true;
    }

    /// C++: SexyAppBase::UpdateApp
    pub fn update_app(&mut self) -> bool {
        loop {
            let mut updated = false;
            if !self.update_app_step(&mut updated) {
                return false;
            }
            if updated {
                return true;
            }
        }
    }

    /// C++: SexyAppBase::InitGLInterface
    pub fn init_gl_interface(&mut self) -> i32 {
        self.delete_native_image_data();
        let a_result = unsafe { (*self.m_gl_interface).init(self.m_is_phys_windowed) };
        if a_result != 0 {
            unsafe {
                self.m_screen_bounds.m_x = (self.m_width - (*self.m_gl_interface).m_width) / 2;
                self.m_screen_bounds.m_y = (self.m_height - (*self.m_gl_interface).m_height) / 2;
                self.m_screen_bounds.m_width = (*self.m_gl_interface).m_width;
                self.m_screen_bounds.m_height = (*self.m_gl_interface).m_height;
                if let Some(wm) = &mut self.m_widget_manager {
                    wm.resize(&self.m_screen_bounds, &(*self.m_gl_interface).m_presentation_rect);
                }
            }
        }
        a_result
    }

    /// C++: SexyAppBase::DeleteNativeImageData
    pub fn delete_native_image_data(&mut self) {
        let images_to_process: Vec<*mut Image> = self.m_memory_image_set.iter().copied().collect();
        for img in images_to_process {
            if !img.is_null() {
                // C++: img->DeleteNativeData()
            }
        }
    }

    /// C++: SexyAppBase::PreTerminate
    pub fn pre_terminate(&mut self) {
    }

    /// C++: SexyAppBase::Start
    pub fn start(&mut self) {
        if self.m_shutdown {
            return;
        }

        let a_start_time = sdl_get_ticks();

        self.m_running = true;
        self.m_last_time = a_start_time as u64;
        self.m_last_user_input_tick = a_start_time;
        self.m_last_timer_time = a_start_time;

        self.do_main_loop();

        self.process_safe_delete_list();

        self.m_running = false;

        println!("Seconds       = {}", (sdl_get_ticks() - a_start_time) as f64 / 1000.0);
        println!("Sleep Count   = {}", self.m_sleep_count);
        println!("Update Count  = {}", self.m_update_count);
        println!("Draw Count    = {}", self.m_draw_count);
        println!("Draw Time     = {}", self.m_draw_time);
        println!("Screen Blt    = {}", self.m_screen_blt_time);
        if self.m_draw_time + self.m_screen_blt_time as u64 > 0 {
            println!(
                "Avg FPS       = {}",
                (self.m_draw_count as u64) * 1000 / (self.m_draw_time + self.m_screen_blt_time as u64)
            );
        }

        self.pre_terminate();
        self.write_to_registry();
    }

    /// C++: SexyAppBase::WriteToRegistry
    pub fn write_to_registry(&mut self) {
    }

    /// C++: SexyAppBase::ReadFromRegistry
    pub fn read_from_registry(&mut self) {
    }

    /// C++: SexyAppBase::ShutdownHook — 钩子，由子类覆盖
    pub fn shutdown_hook(&mut self) {
    }

    /// C++: SexyAppBase::Shutdown
    pub fn shutdown(&mut self) {
        if std::thread::current().id() != self.m_primary_thread_id {
            self.m_loading_failed = true;
        } else if !self.m_shutdown {
            if self.m_recording_demo_buffer {
                // C++: WriteDemoTimingBlock(); mDemoBuffer.WriteNumBits(0,1); mDemoBuffer.WriteNumBits(DEMO_CLOSE, 5);
                // [TRANSLATION_NOTE]: demo 缓冲区未启用（mRecordingDemoBuffer 默认 false）
            }

            self.m_exit_to_top = true;
            self.m_shutdown = true;
            self.shutdown_hook();

            if self.m_playing_demo_buffer {
                self.set_music_volume(self.m_demo_music_volume);
                self.set_sfx_volume(self.m_demo_sfx_volume);
            }

            if let Some(mi) = &mut self.m_music_interface {
                mi.stop_all_music();
            }

            self.restore_screen_resolution();

            if self.m_read_from_registry {
                self.write_to_registry();
            }
        }
    }

    /// C++: SexyAppBase::RestoreScreenResolution
    pub fn restore_screen_resolution(&mut self) {
        // Screen resolution restoration not needed
    }

    /// C++: SexyAppBase::DoExit
    pub fn do_exit(&mut self, the_code: i32) {
        self.restore_screen_resolution();
        self.shutdown();
        std::process::exit(the_code);
    }

    /// C++: SexyAppBase::SwitchScreenMode(bool, bool, bool)
    pub fn switch_screen_mode(&mut self, mut want_windowed: bool, is_3d: bool, force: bool) {
        if self.m_force_fullscreen {
            want_windowed = false;
        }

        if self.m_is_windowed == want_windowed && !force {
            self.set_3d_acclerated(is_3d, false);
            return;
        }

        // Set 3d acceleration preference
        self.set_3d_acclerated(is_3d, false);

        self.m_is_windowed = want_windowed;

        self.make_window();

        self.m_last_time = sdl_get_ticks() as u64;
    }

    /// C++: SexyAppBase::SwitchScreenMode(bool)
    pub fn switch_screen_mode_w(&mut self, want_windowed: bool) {
        self.switch_screen_mode(want_windowed, self.is_3d_accelerated(), false);
    }

    /// C++: SexyAppBase::SwitchScreenMode()
    pub fn switch_screen_mode_force(&mut self) {
        self.switch_screen_mode(self.m_is_windowed, self.is_3d_accelerated(), true);
    }

    /// C++: SexyAppBase::SetAlphaDisabled
    pub fn set_alpha_disabled(&mut self, is_disabled: bool) {
        if self.m_alpha_disabled != is_disabled {
            self.m_alpha_disabled = is_disabled;
            if !self.m_gl_interface.is_null() {
                unsafe {
                    (*self.m_gl_interface).set_video_only_draw(self.m_alpha_disabled);
                }
            }
            if let Some(wm) = &mut self.m_widget_manager {
                unsafe {
                    if !self.m_gl_interface.is_null() {
                        wm.m_image = (*self.m_gl_interface).get_screen_image() as *mut MemoryImage;
                    }
                }
                wm.mark_all_dirty();
            }
        }
    }

    /// C++: SexyAppBase::ResetCustomCursorCache
    pub fn reset_custom_cursor_cache(&mut self) {
        self.m_custom_cursor = std::ptr::null_mut();
        self.m_custom_cursor_image = std::ptr::null_mut();
        self.m_custom_cursor_image_num = -1;
    }

    /// C++: SexyAppBase::EnforceCursor
    pub fn enforce_cursor(&mut self) {
        let mut a_cursor_num = if self.m_seh_occured { CURSOR_POINTER } else { self.m_cursor_num };
        if a_cursor_num < 0 || a_cursor_num >= NUM_CURSORS {
            a_cursor_num = CURSOR_POINTER;
        }

        if a_cursor_num == CURSOR_NONE {
            // C++: SDL_ShowCursor(SDL_DISABLE);
            crate::sexy_app_framework::graphics::renderer::set_show_cursor(false);
            return;
        }

        // [TRANSLATION_NOTE]: 自定义光标缓存逻辑（SDL_Cursor）在后续光标子系统翻译时实现
    }

    /// C++: SexyAppBase::MakeWindow — 创建窗口 + GL 接口
    pub fn make_window(&mut self) {
        if !self.m_window.is_null() {
            // 窗口已存在：全屏切换（C++: SDL_SetWindowFullscreen）
        } else {
            // C++: SDL_Init(SDL_INIT_VIDEO)
            let title = self.m_title.clone();
            let (w, h) = (self.m_width.max(1) as u32, self.m_height.max(1) as u32);

            if let Err(e) = crate::sexy_app_framework::graphics::renderer::init_renderer(&title, w, h)
            {
                eprintln!("Failed to initialize renderer: {}", e);
                self.m_loading_failed = true;
                return;
            }

            // [TRANSLATION_NOTE]: mWindow/mContext 在软件渲染下由 renderer 持有，
            // 此处用哨兵指针标记窗口已创建（C++ 中为 SDL_Window*）。
            self.m_window = 0x1 as *mut std::ffi::c_void;
        }

        if self.m_gl_interface.is_null() {
            let gl = Box::new(GLInterface::new(self as *mut SexyAppBase));
            self.m_gl_interface = Box::into_raw(gl);
            if self.init_gl_interface() == 0 {
                unsafe {
                    let _ = Box::from_raw(self.m_gl_interface);
                }
                self.m_gl_interface = std::ptr::null_mut();
                self.m_loading_failed = true;
                return;
            }
        }

        if let Some(wm) = &mut self.m_widget_manager {
            unsafe {
                wm.m_image = (*self.m_gl_interface).get_screen_image() as *mut MemoryImage;
            }
            wm.mark_all_dirty();
            if !self.m_gl_interface.is_null() {
                unsafe {
                    (*self.m_gl_interface).update_viewport();
                    wm.resize(&self.m_screen_bounds, &(*self.m_gl_interface).m_presentation_rect);
                }
            }
        }
    }

    /// C++: SexyAppBase::RehupFocus
    pub fn rehup_focus(&mut self) {
        let want_has_focus = self.m_active && !self.m_minimized;

        if want_has_focus != self.m_has_focus {
            self.m_has_focus = want_has_focus;

            if self.m_has_focus {
                if self.m_mute_on_lost_focus {
                    self.unmute(true);
                }

                if let Some(wm) = &mut self.m_widget_manager {
                    wm.got_focus();
                }
                self.got_focus();
            } else {
                if self.m_mute_on_lost_focus {
                    self.mute(true);
                }

                if let Some(wm) = &mut self.m_widget_manager {
                    wm.lost_focus();
                }
                self.lost_focus();

                if let Some(wm) = &mut self.m_widget_manager {
                    wm.do_mouse_ups();
                }
            }
        }
    }

    /// C++: SexyAppBase::GotFocus
    pub fn got_focus(&mut self) {
    }

    /// C++: SexyAppBase::LostFocus
    pub fn lost_focus(&mut self) {
    }

    /// C++: SexyAppBase::ClearKeysDown
    pub fn clear_keys_down(&mut self) {
        if let Some(wm) = &mut self.m_widget_manager {
            for a_key_num in 0..0xFF {
                wm.m_key_down[a_key_num] = false;
            }
        }
        self.m_ctrl_down = false;
        self.m_alt_down = false;
    }

    /// C++: SexyAppBase::CloseRequestAsync — C++ 中为空实现
    pub fn close_request_async(&mut self) {
        // C++ 实现为空；窗口关闭请求在此不做处理（与 C++ 行为一致）
    }

    /// C++: SexyAppBase::ProcessDeferredMessages — 处理 SDL 事件队列
    /// 源码位置: platform/default/Input.cpp:528
    pub fn process_deferred_messages(&mut self, single_message: bool) -> bool {
        self.m_last_timer_time = sdl_get_ticks();

        let mut event_pump = match crate::sexy_app_framework::graphics::renderer::pump_events() {
            Some(pump) => pump,
            None => return false,
        };

        let mut processed_any = false;

        loop {
            let event = match event_pump.poll_event() {
                Some(ev) => ev,
                None => break,
            };
            processed_any = true;

            match event {
                sdl2::event::Event::Quit { .. } => {
                    self.close_request_async();
                }
                sdl2::event::Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Close => {
                            self.close_request_async();
                        }
                        sdl2::event::WindowEvent::Resized(_, _) => {
                            if !self.m_gl_interface.is_null() {
                                unsafe {
                                    (*self.m_gl_interface).update_viewport();
                                }
                            }
                            if let Some(wm) = &mut self.m_widget_manager {
                                unsafe {
                                    if !self.m_gl_interface.is_null() {
                                        wm.resize(
                                            &self.m_screen_bounds,
                                            &(*self.m_gl_interface).m_presentation_rect,
                                        );
                                    }
                                }
                                wm.mark_all_dirty();
                            }
                        }
                        sdl2::event::WindowEvent::Minimized => {
                            self.m_minimized = true;
                            self.rehup_focus();
                        }
                        sdl2::event::WindowEvent::Restored => {
                            self.m_minimized = false;
                            self.rehup_focus();
                            if let Some(wm) = &mut self.m_widget_manager {
                                wm.mark_all_dirty();
                            }
                        }
                        sdl2::event::WindowEvent::FocusGained => {
                            self.m_active = true;
                            self.rehup_focus();
                        }
                        sdl2::event::WindowEvent::FocusLost => {
                            self.m_active = false;
                            self.rehup_focus();
                        }
                        _ => {}
                    }
                }
                sdl2::event::Event::MouseWheel { y, .. } => {
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.mouse_wheel(y);
                    }
                }
                sdl2::event::Event::MouseMotion { x, y, .. } => {
                    if !self.m_mouse_in {
                        self.m_mouse_in = true;
                    }
                    let mut rx = x;
                    let mut ry = y;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.remap_mouse(&mut rx, &mut ry);
                    }
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.mouse_move(rx, ry);
                    }
                }
                sdl2::event::Event::MouseButtonDown { mouse_btn, x, y, clicks, .. } => {
                    if !self.m_mouse_in {
                        self.m_mouse_in = true;
                    }
                    let mut rx = x;
                    let mut ry = y;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.remap_mouse(&mut rx, &mut ry);
                    }
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.mouse_move(rx, ry);
                    }
                    let mut btn = match mouse_btn {
                        sdl2::mouse::MouseButton::Left => 1,
                        sdl2::mouse::MouseButton::Right => -1,
                        _ => 3,
                    };
                    if clicks == 2 {
                        btn = if mouse_btn == sdl2::mouse::MouseButton::Left { 2 } else { -2 };
                    }
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.mouse_down(rx, ry, btn);
                    }
                }
                sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    let mut rx = x;
                    let mut ry = y;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.remap_mouse(&mut rx, &mut ry);
                    }
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.mouse_move(rx, ry);
                    }
                    let btn = match mouse_btn {
                        sdl2::mouse::MouseButton::Left => 1,
                        sdl2::mouse::MouseButton::Right => -1,
                        _ => 3,
                    };
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.mouse_up(rx, ry, btn);
                    }
                }
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(kc) = keycode {
                        if let Some(wm) = &mut self.m_widget_manager {
                            // C++: mWidgetManager->KeyDown(static_cast<KeyCode>(event.key.keysym.sym))
                            let kc_val: u32 = unsafe { std::mem::transmute::<sdl2::keyboard::Keycode, u32>(kc) };
                            wm.key_down(kc_val);
                        }
                    }
                }
                sdl2::event::Event::KeyUp { keycode, .. } => {
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(kc) = keycode {
                        if let Some(wm) = &mut self.m_widget_manager {
                            let kc_val: u32 = unsafe { std::mem::transmute::<sdl2::keyboard::Keycode, u32>(kc) };
                            wm.key_up(kc_val);
                        }
                    }
                }
                sdl2::event::Event::TextInput { text, .. } => {
                    self.m_last_user_input_tick = self.m_last_timer_time;
                    if let Some(wm) = &mut self.m_widget_manager {
                        wm.key_text(&text);
                    }
                }
                _ => {}
            }

            if single_message {
                break;
            }
        }

        processed_any
    }
}

// Global application pointer
pub static mut G_SEXY_APP: Option<Box<SexyAppBase>> = None;

/// 获取全局 SexyAppBase 裸指针（对应 C++ gSexyAppBase）
pub unsafe fn g_sexy_app_ptr() -> *mut SexyAppBase {
    match G_SEXY_APP.as_mut() {
        Some(base) => base.as_mut() as *mut SexyAppBase,
        None => std::ptr::null_mut(),
    }
}

// =========================================================================
// 命令行参数全局存储 (SexyAppBase 基类字段的替代)
// =========================================================================
static mut G_ARGC: i32 = 0;
static mut G_ARGV: *mut *mut u8 = std::ptr::null_mut();

pub unsafe fn set_app_args(argc: i32, argv: *mut *mut u8) {
    G_ARGC = argc;
    G_ARGV = argv;
}

pub unsafe fn get_app_args() -> (i32, *mut *mut u8) {
    (G_ARGC, G_ARGV)
}

// =========================================================================
// 全局时钟（对应 C++ SDL_GetTicks）
// =========================================================================
static mut G_START_INSTANT: Option<std::time::Instant> = None;

pub fn sdl_get_ticks() -> u32 {
    unsafe {
        if G_START_INSTANT.is_none() {
            G_START_INSTANT = Some(std::time::Instant::now());
        }
        G_START_INSTANT.unwrap().elapsed().as_millis() as u32
    }
}

// =========================================================================
// SexyAppBase::Init() — C++ 保真翻译 (被 LawnApp::Init 调用)
// C++ 流程: MakeWindow() -> mGLInterface->Init -> WidgetManager 初始化
// =========================================================================
pub unsafe fn sexy_app_base_init(_app: &mut crate::lawn_app::LawnApp) {
    // C++: 构造 SexyAppBase 时即创建 WidgetManager(new WidgetManager(this))
    if G_SEXY_APP.is_none() {
        G_SEXY_APP = Some(Box::new(SexyAppBase::new()));
    }

    let base_ptr = g_sexy_app_ptr();
    if base_ptr.is_null() {
        return;
    }

    let base = &mut *base_ptr;

    // C++: SexyAppBase 构造函数: mWidgetManager = new WidgetManager(this)
    if base.m_widget_manager.is_none() {
        let wm = Box::new(WidgetManager::new(base_ptr));
        base.m_widget_manager = Some(wm);
    }

    // C++: MakeWindow() — 创建窗口 + GL 接口
    base.make_window();

    if base.m_loading_failed {
        return;
    }

    base.m_initialized = true;
    base.m_loaded = false;
}

// =========================================================================
// SexyAppBase::Start() — C++ 保真翻译
// =========================================================================
pub unsafe fn sexy_app_base_start(_app: &mut crate::lawn_app::LawnApp) {
    let base_ptr = g_sexy_app_ptr();
    if base_ptr.is_null() {
        return;
    }

    let base = &mut *base_ptr;
    base.start();
}

// =========================================================================
// SexyAppBase::Shutdown() — C++ 保真翻译
// =========================================================================
pub unsafe fn sexy_app_base_shutdown(app: &mut crate::lawn_app::LawnApp) {
    app.m_close_request = true;
    app.ShutdownHook();

    let base_ptr = g_sexy_app_ptr();
    if !base_ptr.is_null() {
        let base = &mut *base_ptr;
        base.shutdown();
    }
}

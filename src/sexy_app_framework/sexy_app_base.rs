// [TRANSLATION_NOTE]: SexyAppBase.h -> Rust 模块
// C++ SexyAppBase 类（应用基类）翻译为 Rust struct

use std::collections::{HashMap, LinkedList};
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

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

pub const FPS_ShowFPS: i32 = 0;
pub const FPS_ShowCoords: i32 = 1;
pub const Num_FPS_Types: i32 = 2;

pub const UPDATESTATE_MESSAGES: i32 = 0;
pub const UPDATESTATE_PROCESS_1: i32 = 1;
pub const UPDATESTATE_PROCESS_2: i32 = 2;
pub const UPDATESTATE_PROCESS_DONE: i32 = 3;

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
    pub m_widget_manager: Option<Box<super::widget::widget_manager::WidgetManager>>,
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
    pub m_cursor_images: [*mut crate::sexy_app_framework::graphics::graphics::Image; 13],
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
    pub m_custom_cursor_image: *mut crate::sexy_app_framework::graphics::graphics::Image,
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
}

impl SexyAppBase {
    pub fn new() -> Self {
        SexyAppBase {
            m_window: std::ptr::null_mut(),
            m_context: std::ptr::null_mut(),
            m_surface: std::ptr::null_mut(),
            m_rand_seed: 0,
            m_company_name: String::new(),
            m_full_company_name: String::new(),
            m_prod_name: String::new(),
            m_title: String::new(),
            m_reg_key: String::new(),
            m_resource_dir: String::new(),
            m_custom_save_dir: String::new(),
            m_relax_update_backlog_count: 0,
            m_preferred_x: 0,
            m_preferred_y: 0,
            m_width: 800,
            m_height: 600,
            m_fullscreen_bits: 32,
            m_music_volume: 1.0,
            m_sfx_volume: 1.0,
            m_demo_music_volume: 1.0,
            m_demo_sfx_volume: 1.0,
            m_no_sound_needed: false,
            m_want_f_mod: false,
            m_cmd_line_parsed: false,
            m_argc: 0,
            m_argv: std::ptr::null_mut(),
            m_skip_signature_checks: false,
            m_standard_word_wrap: true,
            mb_allow_extended_chars: false,
            m_only_allow_one_copy_to_run: false,
            m_notify_game_message: 0,
            m_crit_sect: Mutex::new(()),
            m_add_8bit_max_table: [0u8; 512],
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
            m_time_loaded: 0,
            m_is_screen_saver: false,
            m_allow_monitor_powersave: false,
            m_no_defer: false,
            m_full_screen_page_flip: true,
            m_tablet_pc: false,
            m_alpha_disabled: false,
            m_read_from_registry: true,
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
            m_mute_on_lost_focus: true,
            m_memory_image_set: std::collections::HashSet::new(),
            m_shared_image_map: HashMap::new(),
            m_cleanup_shared_images: AtomicBool::new(false),
            m_non_draw_count: 0,
            m_frame_time: 0,
            m_is_drawing: false,
            m_last_draw_was_empty: false,
            m_has_pending_draw: false,
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
            m_last_draw_tick: 0,
            m_next_draw_tick: 0,
            m_step_mode: 0,
            m_cursor_num: 0,
            m_custom_cursor: std::ptr::null_mut(),
            m_custom_cursor_image: std::ptr::null_mut(),
            m_custom_cursor_image_num: 0,
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
            m_auto_start_loading_thread: false,
            m_loading_thread_started: false,
            m_loading_thread_completed: false,
            m_loaded: false,
        }
    }
}

// Global application pointer
pub static mut G_SEXY_APP: Option<Box<SexyAppBase>> = None;

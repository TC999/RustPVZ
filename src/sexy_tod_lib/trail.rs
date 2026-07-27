// [TRANSLATION_NOTE]: Trail.h + Trail.cpp -> Rust
// 轨迹效果系统
// 未实现的类型（FloatParameterTrack、TodParticle定义等）用桩类型替代

use crate::sexy_app_framework::misc::sexy_vector::SexyVector2;
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::graphics::graphics::{Graphics, Image};
use crate::sexy_tod_lib::data_array::DataArray;

pub const MAX_TRAIL_TRIANGLES: i32 = 38;

// ---- 枚举（C++ 中在 Trail.h 中定义）----
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TrailType {
    TRAIL_NONE = -1,
    TRAIL_ICE,
    NUM_TRAILS,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TrailTracks {
    TRACK_WIDTH_OVER_LENGTH,
    TRACK_WIDTH_OVER_TIME,
    TRACK_ALPHA_OVER_LENGTH,
    TRACK_ALPHA_OVER_TIME,
    NUM_TRAIL_TRACKS,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TrailFlags {
    TRAIL_FLAG_LOOPS = 0,
}

// ---- 桩类型：FloatParameterTrack ----
#[derive(Clone, Copy)]
pub struct FloatParameterTrack {
    pub m_count: i32,
    pub m_nodes: [f32; 16],
}

impl FloatParameterTrack {
    pub fn new() -> Self {
        FloatParameterTrack { m_count: 0, m_nodes: [0.0; 16] }
    }
}

pub fn float_track_is_set(_the_track: &FloatParameterTrack) -> bool {
    false
}

pub fn float_track_set_default(the_track: &mut FloatParameterTrack, the_value: f32) {
    the_track.m_count = 1;
    the_track.m_nodes[0] = the_value;
}

pub fn float_track_evaluate(_the_track: &FloatParameterTrack, _the_time_value: f32, _the_interp: f32) -> f32 {
    0.0
}

pub fn float_track_is_constant_zero(_the_track: &FloatParameterTrack) -> bool {
    false
}

// ---- TrailParams ----
pub struct TrailParams {
    pub m_trail_type: TrailType,
    pub m_trail_file_name: String,
}

// 全局轨迹参数
static mut G_TRAIL_PARAM_ARRAY_SIZE: i32 = 0;
static mut G_TRAIL_PARAM_ARRAY: *mut TrailParams = std::ptr::null_mut();

pub static mut G_LAWN_TRAIL_ARRAY: [TrailParams; 1] = [
    TrailParams { m_trail_type: TrailType::TRAIL_ICE, m_trail_file_name: String::new() },
];

// ---- TrailDefinition ----
pub struct TrailDefinition {
    pub m_image: *mut Image,
    pub m_max_points: i32,
    pub m_min_point_distance: f32,
    pub m_trail_flags: u32,
    pub m_trail_duration: FloatParameterTrack,
    pub m_width_over_length: FloatParameterTrack,
    pub m_width_over_time: FloatParameterTrack,
    pub m_alpha_over_length: FloatParameterTrack,
    pub m_alpha_over_time: FloatParameterTrack,
}

impl TrailDefinition {
    pub fn new() -> Self {
        TrailDefinition {
            m_image: std::ptr::null_mut(),
            m_max_points: 2,
            m_min_point_distance: 1.0,
            m_trail_flags: 0,
            m_trail_duration: FloatParameterTrack::new(),
            m_width_over_length: FloatParameterTrack::new(),
            m_width_over_time: FloatParameterTrack::new(),
            m_alpha_over_length: FloatParameterTrack::new(),
            m_alpha_over_time: FloatParameterTrack::new(),
        }
    }
}

// ---- TrailPoint ----
#[derive(Clone, Copy)]
pub struct TrailPoint {
    pub a_pos: SexyVector2,
}

impl TrailPoint {
    pub fn new() -> Self {
        TrailPoint { a_pos: SexyVector2::new_xy(0.0, 0.0) }
    }
}

// ---- Trail ----
pub struct Trail {
    pub m_trail_points: [TrailPoint; 20],
    pub m_num_trail_points: i32,
    pub m_dead: bool,
    pub m_render_order: i32,
    pub m_trail_age: i32,
    pub m_trail_duration: i32,
    pub m_definition: *mut TrailDefinition,
    pub m_trail_holder: *mut TrailHolder,
    pub m_trail_interp: [f32; 4],
    pub m_trail_center: SexyVector2,
    pub m_is_attachment: bool,
    pub m_color_override: Color,
}

impl Trail {
    pub fn new() -> Self {
        Trail {
            m_trail_points: [TrailPoint::new(); 20],
            m_num_trail_points: 0,
            m_dead: false,
            m_render_order: 0,
            m_trail_age: 0,
            m_trail_duration: 0,
            m_definition: std::ptr::null_mut(),
            m_trail_holder: std::ptr::null_mut(),
            m_trail_interp: [0.0; 4],
            m_trail_center: SexyVector2::new_xy(0.0, 0.0),
            m_is_attachment: false,
            m_color_override: Color::new(),
        }
    }
}

impl Default for Trail {
    fn default() -> Self {
        Trail::new()
    }
}

impl Trail {
    pub fn update(&mut self) {
        // 简化：trail 年龄增长，检查是否过期
        self.m_trail_age += 1;
        if self.m_trail_duration > 0 && self.m_trail_age >= self.m_trail_duration {
            self.m_dead = true;
        }
    }

    pub fn draw(&self, _g: &mut Graphics) {
        // placeholder
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        if self.m_num_trail_points < 20 {
            self.m_trail_points[self.m_num_trail_points as usize].a_pos = SexyVector2::new_xy(x, y);
            self.m_num_trail_points += 1;
        }
    }

    pub fn get_normal_at_point(&self, _n_index: i32, _the_normal: &mut SexyVector2) -> bool {
        false
    }
}

// ---- TrailHolder ----
pub struct TrailHolder {
    pub m_trails: DataArray<Trail>,
}

impl TrailHolder {
    pub fn new() -> Self {
        TrailHolder { m_trails: DataArray::new() }
    }

    pub fn initialize_holder(&mut self) {
        self.m_trails.data_array_initialize(64u32, "TrailHolder");
    }

    pub fn dispose_holder(&mut self) {
        unsafe { self.m_trails.data_array_dispose(); }
    }

    pub fn alloc_trail(&mut self, _the_render_order: i32, _the_trail_type: TrailType) -> *mut Trail {
        unsafe { self.m_trails.data_array_alloc() }
    }

    pub fn alloc_trail_from_def(&mut self, _the_render_order: i32, _the_definition: *mut TrailDefinition) -> *mut Trail {
        unsafe { self.m_trails.data_array_alloc() }
    }
}

// ---- 全局轨迹定义 ----
static mut G_TRAIL_DEF_COUNT: i32 = 0;
static mut G_TRAIL_DEF_ARRAY: *mut TrailDefinition = std::ptr::null_mut();

// ---- 全局函数 ----
pub fn trail_load_a_def(the_trail_def: &mut TrailDefinition, _the_trail_file_name: &str) -> bool {
    float_track_set_default(&mut the_trail_def.m_width_over_length, 1.0);
    float_track_set_default(&mut the_trail_def.m_width_over_time, 1.0);
    float_track_set_default(&mut the_trail_def.m_trail_duration, 100.0);
    float_track_set_default(&mut the_trail_def.m_alpha_over_length, 1.0);
    float_track_set_default(&mut the_trail_def.m_alpha_over_time, 1.0);
    true
}

pub fn trail_load_definitions(the_trail_param_array: *mut TrailParams, the_trail_param_array_size: i32) {
    unsafe {
        G_TRAIL_PARAM_ARRAY_SIZE = the_trail_param_array_size;
        G_TRAIL_PARAM_ARRAY = the_trail_param_array;
        G_TRAIL_DEF_COUNT = the_trail_param_array_size;

        // 分配 TrailDefinition 数组
        let layout = std::alloc::Layout::array::<TrailDefinition>(the_trail_param_array_size as usize).unwrap();
        G_TRAIL_DEF_ARRAY = std::alloc::alloc(layout) as *mut TrailDefinition;

        for i in 0..the_trail_param_array_size {
            let def = &mut *G_TRAIL_DEF_ARRAY.add(i as usize);
            *def = TrailDefinition::new();
            let params = &*the_trail_param_array.add(i as usize);
            trail_load_a_def(def, &params.m_trail_file_name);
        }
    }
}

pub fn trail_free_definitions() {
    unsafe {
        if !G_TRAIL_DEF_ARRAY.is_null() {
            let layout = std::alloc::Layout::array::<TrailDefinition>(G_TRAIL_DEF_COUNT as usize).unwrap();
            std::alloc::dealloc(G_TRAIL_DEF_ARRAY as *mut u8, layout);
        }
        G_TRAIL_DEF_ARRAY = std::ptr::null_mut();
        G_TRAIL_DEF_COUNT = 0;
        G_TRAIL_PARAM_ARRAY = std::ptr::null_mut();
        G_TRAIL_PARAM_ARRAY_SIZE = 0;
    }
}

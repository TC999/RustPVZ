// [TRANSLATION_NOTE]: TodCommon.h + TodCommon.cpp -> Rust 模块
// 游戏核心工具函数：曲线动画、随机选择、绘图、数学工具等

use crate::const_enums::TodCurves;
use crate::sexy_app_framework::common::{rand_range, rand_float};
use crate::sexy_app_framework::graphics::color::Color;
use crate::sexy_app_framework::misc::sexy_matrix::SexyMatrix3;

pub const RENDERIMAGEFLAG_SANDING: u32 = 0x1000;

pub fn deg_to_rad(deg: f32) -> f32 { deg * 0.017453292f32 }
pub fn rad_to_deg(rad: f32) -> f32 { rad * 57.29578f32 }

#[derive(Clone, Copy, Debug)]
pub struct TodWeightedArray {
    pub m_item: isize,
    pub m_weight: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct TodWeightedGridArray {
    pub m_x: i32,
    pub m_y: i32,
    pub m_weight: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct TodSmoothArray {
    pub m_item: i32,
    pub m_weight: f32,
    pub m_last_picked: f32,
    pub m_second_last_picked: f32,
}

pub fn tod_pick_from_array<T: Copy>(the_array: &[T]) -> T {
    debug_assert!(the_array.len() > 0);
    if the_array.len() > 0 {
        let idx = rand_range(the_array.len() as i32);
        the_array[idx as usize]
    } else {
        // Safe default for empty arrays
        the_array[0]
    }
}

pub fn tod_pick_from_weighted_array(the_array: &[TodWeightedArray]) -> isize {
    let mut total_weight = 0;
    for item in the_array {
        total_weight += item.m_weight;
    }
    if total_weight <= 0 {
        return 0;
    }
    let mut pick = rand_range(total_weight);
    for item in the_array {
        pick -= item.m_weight;
        if pick < 0 {
            return item.m_item;
        }
    }
    the_array[the_array.len() - 1].m_item
}

pub fn tod_pick_array_item_from_weighted_array(the_array: &mut [TodWeightedArray]) -> *mut TodWeightedArray {
    let total_weight: i32 = the_array.iter().map(|a| a.m_weight).sum();
    if total_weight <= 0 {
        return std::ptr::null_mut();
    }
    let mut pick = rand_range(total_weight);
    for item in the_array.iter_mut() {
        pick -= item.m_weight;
        if pick < 0 {
            return item as *mut TodWeightedArray;
        }
    }
    std::ptr::null_mut()
}

pub fn tod_pick_from_weighted_grid_array(the_array: &[TodWeightedGridArray]) -> Option<&TodWeightedGridArray> {
    let total_weight: i32 = the_array.iter().map(|a| a.m_weight).sum();
    if total_weight <= 0 {
        return None;
    }
    let mut pick = rand_range(total_weight);
    for item in the_array {
        pick -= item.m_weight;
        if pick < 0 {
            return Some(item);
        }
    }
    None
}

impl TodSmoothArray {
    pub fn new() -> Self {
        TodSmoothArray {
            m_item: 0,
            m_weight: 0.0,
            m_last_picked: 0.0,
            m_second_last_picked: 0.0,
        }
    }
}

pub fn tod_calc_smooth_weight(weight: f32, last_picked: f32, second_last_picked: f32) -> f32 {
    weight - last_picked * 0.5 - second_last_picked * 0.25
}

pub fn tod_update_smooth_array_pick(the_array: &mut [TodSmoothArray], count: i32, pick_index: i32) {
    for i in 0..count as usize {
        let entry = &mut the_array[i];
        entry.m_second_last_picked = entry.m_last_picked;
        entry.m_last_picked *= 0.95;
    }
    if pick_index >= 0 && (pick_index as usize) < count as usize {
        the_array[pick_index as usize].m_last_picked = 1.0;
    }
}

pub fn tod_pick_from_smooth_array(the_array: &mut [TodSmoothArray], count: i32) -> i32 {
    let mut total_weight = 0.0;
    for i in 0..count as usize {
        total_weight += tod_calc_smooth_weight(
            the_array[i].m_weight,
            the_array[i].m_last_picked,
            the_array[i].m_second_last_picked,
        );
    }
    if total_weight <= 0.0 {
        return -1;
    }
    let mut pick = rand_float(total_weight);
    for i in 0..count as usize {
        pick -= tod_calc_smooth_weight(
            the_array[i].m_weight,
            the_array[i].m_last_picked,
            the_array[i].m_second_last_picked,
        );
        if pick < 0.0 {
            tod_update_smooth_array_pick(the_array, count, i as i32);
            return i as i32;
        }
    }
    tod_update_smooth_array_pick(the_array, count, (count - 1) as i32);
    count - 1
}

// Curve functions
pub fn tod_curve_quad(the_time: f32) -> f32 {
    the_time * the_time
}

pub fn tod_curve_inv_quad(the_time: f32) -> f32 {
    1.0 - ((1.0 - the_time) * (1.0 - the_time))
}

pub fn tod_curve_s(the_time: f32) -> f32 {
    the_time * the_time * (3.0 - 2.0 * the_time)
}

pub fn tod_curve_inv_quad_s(the_time: f32) -> f32 {
    if the_time < 0.5 {
        0.5 * tod_curve_quad(the_time * 2.0)
    } else {
        0.5 * (1.0 + tod_curve_inv_quad((the_time - 0.5) * 2.0))
    }
}

pub fn tod_curve_bounce(the_time: f32) -> f32 {
    if the_time < 0.75 {
        1.0 - tod_curve_quad(1.0 - the_time / 0.75)
    } else {
        1.0 - tod_curve_quad(1.0 - (the_time - 0.75) / 0.25) * 0.3
    }
}

pub fn tod_curve_quad_s(the_time: f32) -> f32 {
    if the_time < 0.5 {
        0.5 * tod_curve_quad(the_time * 2.0)
    } else {
        0.5 + 0.5 * tod_curve_inv_quad((the_time - 0.5) * 2.0)
    }
}

pub fn tod_curve_cubic(the_time: f32) -> f32 {
    the_time * the_time * the_time
}

pub fn tod_curve_inv_cubic(the_time: f32) -> f32 {
    1.0 - ((1.0 - the_time) * (1.0 - the_time) * (1.0 - the_time))
}

pub fn tod_curve_cubic_s(the_time: f32) -> f32 {
    if the_time < 0.5 {
        0.5 * tod_curve_cubic(the_time * 2.0)
    } else {
        0.5 + 0.5 * tod_curve_inv_cubic((the_time - 0.5) * 2.0)
    }
}

pub fn tod_curve_poly(the_time: f32, the_poly: f32) -> f32 {
    the_time.powf(the_poly)
}

pub fn tod_curve_inv_poly(the_time: f32, the_poly: f32) -> f32 {
    1.0 - (1.0 - the_time).powf(the_poly)
}

pub fn tod_curve_poly_s(the_time: f32, the_poly: f32) -> f32 {
    if the_time < 0.5 {
        0.5 * tod_curve_poly(the_time * 2.0, the_poly)
    } else {
        0.5 + 0.5 * tod_curve_inv_poly((the_time - 0.5) * 2.0, the_poly)
    }
}

pub fn tod_curve_circle(the_time: f32) -> f32 {
    (1.0 - (1.0 - the_time) * (1.0 - the_time)).sqrt()
}

pub fn tod_curve_inv_circle(the_time: f32) -> f32 {
    1.0 - (1.0 - the_time * the_time).sqrt()
}

pub fn tod_curve_evaluate(the_time: f32, position_start: f32, position_end: f32, curve: TodCurves) -> f32 {
    let f = match curve {
        TodCurves::CURVE_CONSTANT => 0.0,
        TodCurves::CURVE_LINEAR => the_time,
        TodCurves::CURVE_EASE_IN => tod_curve_quad(the_time),
        TodCurves::CURVE_EASE_OUT => tod_curve_inv_quad(the_time),
        TodCurves::CURVE_EASE_IN_OUT => tod_curve_s(the_time),
        TodCurves::CURVE_EASE_IN_OUT_WEAK => tod_curve_s(the_time),
        TodCurves::CURVE_FAST_IN_OUT => tod_curve_inv_quad_s(the_time),
        TodCurves::CURVE_FAST_IN_OUT_WEAK => tod_curve_inv_quad_s(the_time),
        TodCurves::CURVE_WEAK_FAST_IN_OUT => tod_curve_inv_quad_s(the_time),
        TodCurves::CURVE_BOUNCE => tod_curve_bounce(the_time),
        TodCurves::CURVE_BOUNCE_FAST_MIDDLE => tod_curve_quad_s(the_time),
        TodCurves::CURVE_BOUNCE_SLOW_MIDDLE => tod_curve_s(the_time),
        TodCurves::CURVE_SIN_WAVE => (the_time * std::f32::consts::PI * 2.0).sin(),
        TodCurves::CURVE_EASE_SIN_WAVE => (the_time * std::f32::consts::PI * 2.0).sin() * the_time,
    };
    position_start + f * (position_end - position_start)
}

pub fn tod_curve_evaluate_clamped(the_time: f32, position_start: f32, position_end: f32, curve: TodCurves) -> f32 {
    let t = if the_time < 0.0 {
        0.0
    } else if the_time > 1.0 {
        1.0
    } else {
        the_time
    };
    tod_curve_evaluate(t, position_start, position_end, curve)
}

pub fn tod_animate_curve_float_time(
    time_start: f32, time_end: f32, time_age: f32,
    position_start: f32, position_end: f32, curve: TodCurves,
) -> f32 {
    let age = time_age - time_start;
    let duration = time_end - time_start;
    if duration <= 0.0 {
        return position_end;
    }
    tod_curve_evaluate_clamped(age / duration, position_start, position_end, curve)
}

pub fn tod_animate_curve_float(
    time_start: i32, time_end: i32, time_age: i32,
    position_start: f32, position_end: f32, curve: TodCurves,
) -> f32 {
    tod_animate_curve_float_time(
        time_start as f32, time_end as f32, time_age as f32,
        position_start, position_end, curve,
    )
}

pub fn tod_animate_curve(
    time_start: i32, time_end: i32, time_age: i32,
    position_start: i32, position_end: i32, curve: TodCurves,
) -> i32 {
    tod_animate_curve_float(time_start, time_end, time_age, position_start as f32, position_end as f32, curve).round() as i32
}

// Matrix helpers
pub fn tod_scale_transform_matrix(m: &mut SexyMatrix3, x: f32, y: f32, scale_x: f32, scale_y: f32) {
    m.m[0][0] = scale_x;
    m.m[1][1] = scale_y;
    m.m[0][2] = x - scale_x * x;
    m.m[1][2] = y - scale_y * y;
}

pub fn sexy_matrix3_translation(m: &mut SexyMatrix3, x: f32, y: f32) {
    m.m[0][2] = x;
    m.m[1][2] = y;
}

// Math helpers
pub fn clamp_byte(num: u8, min: u8, max: u8) -> u8 {
    if num <= min { min } else if num >= max { max } else { num }
}

pub fn clamp_int(num: i32, min: i32, max: i32) -> i32 {
    if num <= min { min } else if num >= max { max } else { num }
}

pub fn clamp_float(num: f32, min: f32, max: f32) -> f32 {
    if num <= min { min } else if num >= max { max } else { num }
}

pub fn distance_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

pub fn float_lerp(start: f32, end: f32, factor: f32) -> f32 {
    start + factor * (end - start)
}

pub fn float_round_to_int(value: f32) -> i32 {
    if value > 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}

pub fn float_approx_equal(v1: f32, v2: f32) -> bool {
    (v1 - v2).abs() < f32::EPSILON
}

pub fn rand_range_int(min: i32, max: i32) -> i32 {
    if min >= max {
        min
    } else {
        min + rand_range(max - min + 1)
    }
}

pub fn rand_range_float(min: f32, max: f32) -> f32 {
    min + rand_float(max - min)
}

pub fn get_flashing_color(counter: u32, flash_time: i32) -> Color {
    if (counter as i32 % (flash_time * 2)) < flash_time {
        Color::from_components(255, 255, 255)
    } else {
        Color::from_components(255, 0, 0)
    }
}

pub fn color_component_multiply(color1: i32, color2: i32) -> i32 {
    (color1 * color2) >> 8
}

pub fn colors_multiply(c1: &Color, c2: &Color) -> Color {
    Color::from_components_alpha(
        (c1.m_red * c2.m_red) >> 8,
        (c1.m_green * c2.m_green) >> 8,
        (c1.m_blue * c2.m_blue) >> 8,
        (c1.m_alpha * c2.m_alpha) >> 8,
    )
}

pub fn color_add(c1: &Color, c2: &Color) -> Color {
    Color::from_components_alpha(
        c1.m_red + c2.m_red,
        c1.m_green + c2.m_green,
        c1.m_blue + c2.m_blue,
        c1.m_alpha + c2.m_alpha,
    )
}

pub fn set_bit(num: &mut u32, idx: i32, value: bool) {
    if value {
        *num |= 1 << idx;
    } else {
        *num &= !(1 << idx);
    }
}

pub fn test_bit(num: u32, idx: i32) -> bool {
    (num & (1 << idx)) != 0
}

// String helpers
pub fn tod_replace_string(text: &str, find: &str, substitute: &str) -> String {
    text.replace(find, substitute)
}

pub fn tod_replace_number_string(text: &str, find: &str, number: i32) -> String {
    text.replace(find, &number.to_string())
}

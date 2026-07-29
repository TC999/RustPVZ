// [TRANSLATION_NOTE]: FilterEffect.h + FilterEffect.cpp -> Rust
// 滤镜效果：对 MemoryImage 进行色调/饱和度/亮度等图像处理
// 使用 unsafe 块直接操作像素缓冲区（m_bits）

use std::collections::HashMap;
use crate::sexy_app_framework::graphics::graphics::{Image, MemoryImage};
use crate::sexy_tod_lib::tod_common::clamp_int;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum FilterEffect {
    FILTER_EFFECT_NONE = -1,
    FILTER_EFFECT_WASHED_OUT,
    FILTER_EFFECT_LESS_WASHED_OUT,
    FILTER_EFFECT_WHITE,
    NUM_FILTER_EFFECTS,
}

pub type ImageFilterMap = HashMap<*mut Image, *mut Image>;

// 全局滤镜映射表（每个滤镜类型一个映射表）
static mut G_FILTER_MAP: Option<[ImageFilterMap; 4]> = None;

fn get_filter_map() -> &'static mut [ImageFilterMap; 4] {
    unsafe {
        if G_FILTER_MAP.is_none() {
            G_FILTER_MAP = Some([
                ImageFilterMap::new(),
                ImageFilterMap::new(),
                ImageFilterMap::new(),
                ImageFilterMap::new(),
            ]);
        }
        G_FILTER_MAP.as_mut().unwrap()
    }
}

fn rgb_to_hsl(r: f32, g: f32, b: f32, h: &mut f32, s: &mut f32, l: &mut f32) {
    let maxval = r.max(g).max(b);
    let minval = r.min(g).min(b);

    *l = (minval + maxval) / 2.0;
    if *l <= 0.0 {
        return;
    }

    let delta = maxval - minval;
    *s = delta;
    if *s <= 0.0 {
        return;
    }
    *s /= if *l <= 0.5 { minval + maxval } else { 2.0 - minval - maxval };

    let r2 = (maxval - r) / delta;
    let g2 = (maxval - g) / delta;
    let b2 = (maxval - b) / delta;

    if maxval == r {
        *h = if g == minval { 5.0 + b2 } else { 1.0 - g2 };
    } else if maxval == g {
        *h = if b == minval { 1.0 + r2 } else { 3.0 - b2 };
    } else {
        *h = if r == minval { 3.0 + g2 } else { 5.0 - r2 };
    }
    *h /= 6.0;
}

fn hsl_to_rgb(h: f32, sl: f32, l: f32, r: &mut f32, g: &mut f32, b: &mut f32) {
    let v = if l <= 0.5 { l * (1.0 + sl) } else { l + sl - l * sl };
    if v <= 0.0 {
        *r = 0.0; *g = 0.0; *b = 0.0;
        return;
    }

    let y = 2.0 * l - v;
    let sv = (v - y) / v;
    let h6 = h * 6.0;
    let sextant = clamp_int(h6 as i32, 0, 5);
    let vsf = v * sv * (h6 - sextant as f32);
    let x = y + vsf;
    let z = v - vsf;

    match sextant {
        0 => { *r = v; *g = x; *b = y; }
        1 => { *r = z; *g = v; *b = y; }
        2 => { *r = y; *g = v; *b = x; }
        3 => { *r = y; *g = z; *b = v; }
        4 => { *r = x; *g = y; *b = v; }
        5 => { *r = v; *g = y; *b = z; }
        _ => {}
    }
}

pub fn filter_effect_init_for_app() {
}

pub fn filter_effect_dispose_for_app() {
    let map = get_filter_map();
    for i in 0..4 {
        for (_, &mut img_ptr) in map[i].iter_mut() {
            if !img_ptr.is_null() {
                let _ = unsafe { Box::from_raw(img_ptr) };
            }
        }
        map[i].clear();
    }
}

pub fn filter_effect_do_lum_sat(the_image: &mut MemoryImage, the_lum: f32, the_sat: f32) {
    unsafe {
        let mut ptr = the_image.m_bits;
        if ptr.is_null() { return; }
        for _y in 0..the_image.base.m_height {
            for _x in 0..the_image.base.m_width {
                let pixel = *ptr;
                let mut b = (pixel & 0xFF) as f32 / 255.0;
                let mut g = ((pixel >> 8) & 0xFF) as f32 / 255.0;
                let mut r = ((pixel >> 16) & 0xFF) as f32 / 255.0;
                let a = (pixel >> 24) & 0xFF;

                let mut h = 0.0; let mut s = 0.0; let mut l = 0.0;
                rgb_to_hsl(r, g, b, &mut h, &mut s, &mut l);
                s *= the_sat;
                l *= the_lum;
                hsl_to_rgb(h, s, l, &mut r, &mut g, &mut b);

                *ptr = (a << 24)
                    | (clamp_int((r * 255.0) as i32, 0, 255) << 16) as u32
                    | (clamp_int((g * 255.0) as i32, 0, 255) << 8) as u32
                    | clamp_int((b * 255.0) as i32, 0, 255) as u32;
                ptr = ptr.add(1);
            }
        }
    }
}

pub fn filter_effect_do_washed_out(the_image: &mut MemoryImage) {
    filter_effect_do_lum_sat(the_image, 1.8, 0.2);
}

pub fn filter_effect_do_less_washed_out(the_image: &mut MemoryImage) {
    filter_effect_do_lum_sat(the_image, 1.2, 0.3);
}

pub fn filter_effect_do_white(the_image: &mut MemoryImage) {
    unsafe {
        let mut ptr = the_image.m_bits;
        if ptr.is_null() { return; }
        for _y in 0..the_image.base.m_height {
            for _x in 0..the_image.base.m_width {
                *ptr |= 0x00FFFFFF;
                ptr = ptr.add(1);
            }
        }
    }
}

pub fn filter_effect_create_image(the_image: &Image, the_filter_effect: FilterEffect) -> *mut MemoryImage {
    // 在堆上分配新的 MemoryImage
    let mut a_image = Box::new(MemoryImage::new());
    a_image.base.m_width = the_image.m_width;
    a_image.base.m_height = the_image.m_height;
    let a_num_bits = (the_image.m_width * the_image.m_height) as usize;
    
    // 分配像素缓冲区（+1 用于 MEMORYCHECK_ID）
    let layout = std::alloc::Layout::array::<u32>(a_num_bits + 1).unwrap();
    a_image.m_bits = unsafe { std::alloc::alloc(layout) as *mut u32 };
    if !a_image.m_bits.is_null() {
        unsafe { std::ptr::write_bytes(a_image.m_bits, 0, a_num_bits + 1); }
    }
    a_image.m_has_trans = true;
    a_image.m_has_alpha = true;
    if !a_image.m_bits.is_null() {
        unsafe {
            *a_image.m_bits.add(a_num_bits) = crate::sexy_app_framework::graphics::graphics::MEMORYCHECK_ID;
        }
    }

    // 这里原本会使用 Graphics 绘制原图到新图像并修复边缘
    // 由于 Graphics::DrawImage 尚未实现，暂时跳过
    // Graphics aMemoryGraphics(&a_image);
    // aMemoryGraphics.DrawImage(theImage, 0, 0);
    // FixPixelsOnAlphaEdgeForBlending(&a_image);

    match the_filter_effect {
        FilterEffect::FILTER_EFFECT_WASHED_OUT => filter_effect_do_washed_out(&mut a_image),
        FilterEffect::FILTER_EFFECT_LESS_WASHED_OUT => filter_effect_do_less_washed_out(&mut a_image),
        FilterEffect::FILTER_EFFECT_WHITE => filter_effect_do_white(&mut a_image),
        _ => {}
    }

    a_image.m_bits_changed = true;
    a_image.base.m_num_cols = the_image.m_num_cols;
    a_image.base.m_num_rows = the_image.m_num_rows;

    Box::into_raw(a_image)
}

pub fn filter_effect_get_image(the_image: &Image, the_filter_effect: FilterEffect) -> *mut Image {
    let _idx = the_filter_effect as i32;
    // 简化：不缓存，每次都创建新图像
    // C++ 中使用 gFilterMap 缓存已处理的图像
    let a_image = filter_effect_create_image(the_image, the_filter_effect);
    if a_image.is_null() {
        return std::ptr::null_mut();
    }
    a_image as *mut Image
}

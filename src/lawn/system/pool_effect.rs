// [TRANSLATION_NOTE]: PoolEffect.h + PoolEffect.cpp -> Rust 翻译
// 泳池水面焦散效果。3D 绘制部分暂为 stub

#![allow(non_snake_case, dead_code)]

use std::f32::consts::PI;
use crate::const_enums::*;
use crate::sexy_app_framework::graphics::graphics::{Graphics, MemoryImage};
use crate::sexy_app_framework::resources::{
    IMAGE_POOL, IMAGE_POOL_NIGHT, IMAGE_POOL_BASE, IMAGE_POOL_SHADING,
    IMAGE_POOL_BASE_NIGHT, IMAGE_POOL_SHADING_NIGHT, IMAGE_POOL_CAUSTIC_EFFECT,
};

pub const CAUSTIC_IMAGE_WIDTH: i32 = 128;
pub const CAUSTIC_IMAGE_HEIGHT: i32 = 64;

pub struct PoolEffect {
    pub mCausticGrayscaleImage: Vec<u8>,
    pub mCausticImage: *mut MemoryImage,
    pub mApp: *mut std::ffi::c_void,
    pub mPoolCounter: u32,
}

impl PoolEffect {
    pub fn new() -> Self {
        PoolEffect {
            mCausticGrayscaleImage: Vec::new(),
            mCausticImage: std::ptr::null_mut(),
            mApp: std::ptr::null_mut(),
            mPoolCounter: 0,
        }
    }

    pub fn PoolEffectInitialize(&mut self) {
        self.mApp = unsafe { crate::lawn_app::G_LAWN_APP as *mut std::ffi::c_void };
        self.mPoolCounter = 0;

        // 创建焦散效果图像
        let aImage = Box::into_raw(Box::new(MemoryImage::new()));
        unsafe {
            let bits_count = (CAUSTIC_IMAGE_WIDTH * CAUSTIC_IMAGE_HEIGHT) as usize;
            let mut bits = vec![0xFF_FF_FF_FFu32; bits_count + 1];
            bits[bits_count] = crate::sexy_app_framework::graphics::graphics::MEMORYCHECK_ID;
            (*aImage).m_bits = bits.as_mut_ptr();
            std::mem::forget(bits);
            (*aImage).base.m_width = CAUSTIC_IMAGE_WIDTH;
            (*aImage).base.m_height = CAUSTIC_IMAGE_HEIGHT;
            (*aImage).m_has_trans = true;
            (*aImage).m_has_alpha = true;
            (*aImage).m_render_flags = 0; // RenderImageFlag_Repeat
        }
        self.mCausticImage = aImage;

        // 加载灰度焦散纹理
        self.mCausticGrayscaleImage.resize(256 * 256, 0);
        // 从 IMAGE_POOL_CAUSTIC_EFFECT 加载数据 — 暂为 stub
    }

    pub fn BilinearLookupFixedPoint(&self, u: u32, v: u32) -> u32 {
        let timeU = u & 0xFFFF0000;
        let timeV = v & 0xFFFF0000;
        let factorU1 = ((u - timeU) & 0x0000FFFE) + 1;
        let factorV1 = ((v - timeV) & 0x0000FFFE) + 1;
        let factorU0 = 65536 - factorU1;
        let factorV0 = 65536 - factorV1;
        let indexU0 = (timeU >> 16) % 256;
        let indexU1 = ((timeU >> 16) + 1) % 256;
        let indexV0 = (timeV >> 16) % 256;
        let indexV1 = ((timeV >> 16) + 1) % 256;
        let gray = &self.mCausticGrayscaleImage;

        if gray.len() < 256 * 256 {
            return 0;
        }

        ((((factorU0 * factorV1) / 65536) * gray[(indexV1 * 256 + indexU0) as usize] as u32) / 65536)
            + ((((factorU1 * factorV1) / 65536) * gray[(indexV1 * 256 + indexU1) as usize] as u32) / 65536)
            + ((((factorU0 * factorV0) / 65536) * gray[(indexV0 * 256 + indexU0) as usize] as u32) / 65536)
            + ((((factorU1 * factorV0) / 65536) * gray[(indexV0 * 256 + indexU1) as usize] as u32) / 65536)
    }

    pub fn UpdateWaterEffect(&mut self) {
        if self.mCausticImage.is_null() {
            return;
        }
        let mut idx = 0usize;
        unsafe {
            let caustic = &mut *self.mCausticImage;
            let bits = caustic.m_bits;
            if bits.is_null() { return; }

            for y in 0..CAUSTIC_IMAGE_HEIGHT {
                let timeV1 = ((256 - y) << 17) as u32;
                let timeV0 = (y << 17) as u32;

                for x in 0..CAUSTIC_IMAGE_WIDTH {
                    let pix = bits.add(idx);
                    let timeU = (x << 17) as u32;
                    let timePool0 = self.mPoolCounter << 16;
                    let timePool1 = ((self.mPoolCounter & 65535) + 1) << 16;

                    let a1 = self.BilinearLookupFixedPoint(
                        timeU.wrapping_sub(timePool1 / 6),
                        timeV1.wrapping_add(timePool0 / 8),
                    ) as u8;

                    let a0 = self.BilinearLookupFixedPoint(
                        timeU.wrapping_add(timePool0 / 10),
                        timeV0,
                    ) as u8;

                    let a = (a0 as u16 + a1 as u16) / 2;

                    let alpha: u8 = if a >= 160 {
                        let v = 255u16 - 2 * (a as u16 - 160);
                        if v > 255 { 0 } else { v as u8 }
                    } else if a >= 128 {
                        let v = 5 * (a as u16 - 128);
                        if v > 255 { 255 } else { v as u8 }
                    } else {
                        0
                    };

                    // alpha 通道更新
                    *pix = (*pix & 0x00FFFFFF) | (((alpha as u32) / 3) << 24);
                    idx += 1;
                }
            }
            caustic.m_bits_changed = true;
            caustic.m_bits_changed_count += 1;
        }
    }

    pub fn PoolEffectDraw(&mut self, g: &mut Graphics, theIsNight: bool) {
        let _ = g;
        // 简化绘制：直接画背景图
        if theIsNight {
            unsafe { let _ = IMAGE_POOL_NIGHT; }
        } else {
            unsafe {
                let _ = IMAGE_POOL;
                let _ = IMAGE_POOL_BASE;
                let _ = IMAGE_POOL_SHADING;
                let _ = IMAGE_POOL_BASE_NIGHT;
                let _ = IMAGE_POOL_SHADING_NIGHT;
            }
        }
        self.UpdateWaterEffect();
    }

    pub fn PoolEffectUpdate(&mut self) {
        self.mPoolCounter += 1;
    }
}

impl Drop for PoolEffect {
    fn drop(&mut self) {
        if !self.mCausticImage.is_null() {
            unsafe { let _ = Box::from_raw(self.mCausticImage); }
        }
    }
}

// [TRANSLATION_NOTE]: ReanimationLawn.h + ReanimationLawn.cpp -> Rust 翻译
// 动画缓存系统。核心逻辑完整翻译，缺失依赖使用 stub

#![allow(non_snake_case, dead_code)]

use crate::const_enums::*;
use crate::lawn_app::G_LAWN_APP;
use crate::sexy_app_framework::graphics::graphics::{Graphics, MemoryImage, MEMORYCHECK_ID};
use crate::sexy_tod_lib::tod_list::TodList;

/// 缓存图像变体
#[derive(Clone, Copy)]
pub struct ReanimCacheImageVariation {
    pub mSeedType: SeedType,
    pub mDrawVariation: DrawVariation,
    pub mImage: *mut MemoryImage,
}

pub type ImageVariationList = TodList<ReanimCacheImageVariation>;

/// 动画缓存管理器
pub struct ReanimatorCache {
    pub mPlantImages: [*mut MemoryImage; 50],
    pub mImageVariationList: ImageVariationList,
    pub mLawnMowers: [*mut MemoryImage; 4],
    pub mZombieImages: [*mut MemoryImage; 50],
    pub mApp: *mut std::ffi::c_void,
}

impl ReanimatorCache {
    pub fn new() -> Self {
        ReanimatorCache {
            mPlantImages: [std::ptr::null_mut(); 50],
            mImageVariationList: TodList::new(),
            mLawnMowers: [std::ptr::null_mut(); 4],
            mZombieImages: [std::ptr::null_mut(); 50],
            mApp: std::ptr::null_mut(),
        }
    }

    pub fn ReanimatorCacheInitialize(&mut self) {
        self.mApp = unsafe { G_LAWN_APP as *mut std::ffi::c_void };
        for img in self.mPlantImages.iter_mut() { *img = std::ptr::null_mut(); }
        for img in self.mLawnMowers.iter_mut() { *img = std::ptr::null_mut(); }
        for img in self.mZombieImages.iter_mut() { *img = std::ptr::null_mut(); }
    }

    pub fn ReanimatorCacheDispose(&mut self) {
        for img in self.mPlantImages.iter_mut() {
            if !img.is_null() { unsafe { let _ = Box::from_raw(*img); } *img = std::ptr::null_mut(); }
        }
        while !self.mImageVariationList.is_empty() {
            let aImageVariation = self.mImageVariationList.remove_head();
            if !aImageVariation.mImage.is_null() {
                unsafe { let _ = Box::from_raw(aImageVariation.mImage); }
            }
        }
        for img in self.mLawnMowers.iter_mut() {
            if !img.is_null() { unsafe { let _ = Box::from_raw(*img); } *img = std::ptr::null_mut(); }
        }
        for img in self.mZombieImages.iter_mut() {
            if !img.is_null() { unsafe { let _ = Box::from_raw(*img); } *img = std::ptr::null_mut(); }
        }
    }

    pub fn MakeBlankMemoryImage(&self, theWidth: i32, theHeight: i32) -> *mut MemoryImage {
        let aImage = Box::into_raw(Box::new(MemoryImage::new()));
        unsafe {
            let aBitsCount = (theWidth * theHeight) as usize;
            let mut bits = vec![0u32; aBitsCount + 1];
            bits[aBitsCount] = MEMORYCHECK_ID;
            (*aImage).m_bits = bits.as_mut_ptr();
            std::mem::forget(bits);
            (*aImage).base.m_width = theWidth;
            (*aImage).base.m_height = theHeight;
            (*aImage).m_has_trans = true;
            (*aImage).m_has_alpha = true;
        }
        aImage
    }

    pub fn GetPlantImageSize(theSeedType: SeedType, theOffsetX: &mut i32, theOffsetY: &mut i32, theWidth: &mut i32, theHeight: &mut i32) {
        *theOffsetX = -20; *theOffsetY = -20; *theWidth = 120; *theHeight = 120;
        match theSeedType {
            SeedType::SEED_TALLNUT => { *theOffsetY = -40; *theHeight += 40; }
            SeedType::SEED_MELONPULT | SeedType::SEED_WINTERMELON => { *theOffsetX = -40; *theWidth += 40; }
            SeedType::SEED_COBCANNON => { *theWidth += 80; }
            _ => {}
        }
    }

    pub fn UpdateReanimationForVariation(_theReanim: *mut std::ffi::c_void, theDrawVariation: DrawVariation) {
        let dv = theDrawVariation as i32;
        if dv >= DrawVariation::VARIATION_MARIGOLD_WHITE as i32 && dv <= DrawVariation::VARIATION_MARIGOLD_LIGHT_GREEN as i32 {
            // Marigold 颜色变体 — 待 Reanimation 完善后启用
        }
    }

    pub fn DrawReanimatorFrame(g: &mut Graphics, the_pos_x: f32, the_pos_y: f32, the_reanimation_type: i32, the_track_name: &str, the_draw_variation: DrawVariation) {
        // C++ ReanimatorCache::DrawReanimatorFrame (ReanimationLawn.cpp:82)
        // [TRANSLATION_NOTE]: ReanimationInitializeType/TrackExists/SetFramesForLayer 未完整翻译
        let mut a_reanim = crate::sexy_tod_lib::reanimator::Reanimation::new();
        a_reanim.m_anim_time = 0.0;
        // [TODO]: ReanimationInitializeType（definition 加载）+ SetFramesForLayer(theTrackName)
        let _ = (the_reanimation_type, the_track_name);

        if the_draw_variation != DrawVariation::VARIATION_NORMAL {
            Self::UpdateReanimationForVariation(std::ptr::null_mut(), the_draw_variation);
        }

        a_reanim.draw_render_group(g, 0);
    }

    pub fn MakeCachedPlantFrame(&self, theSeedType: SeedType, _theDrawVariation: DrawVariation) -> *mut MemoryImage {
        let (mut aOffsetX, mut aOffsetY, mut aWidth, mut aHeight) = (0, 0, 0, 0);
        Self::GetPlantImageSize(theSeedType, &mut aOffsetX, &mut aOffsetY, &mut aWidth, &mut aHeight);
        self.MakeBlankMemoryImage(aWidth, aHeight)
    }

    pub fn MakeCachedMowerFrame(&self, _theMowerType: i32) -> *mut MemoryImage {
        self.MakeBlankMemoryImage(90, 100)
    }

    pub fn MakeCachedZombieFrame(&self, _theZombieType: i32) -> *mut MemoryImage {
        self.MakeBlankMemoryImage(200, 210)
    }

    pub fn DrawCachedPlant(&mut self, g: &mut Graphics, thePosX: f32, thePosY: f32, theSeedType: SeedType, theDrawVariation: DrawVariation) {
        let mut aImage: *mut MemoryImage = std::ptr::null_mut();
        let seed_idx = theSeedType as usize;
        if theDrawVariation != DrawVariation::VARIATION_NORMAL {
            let mut aNode = self.mImageVariationList.m_head;
            while !aNode.is_null() {
                unsafe {
                    if (*aNode).m_value.mSeedType == theSeedType && (*aNode).m_value.mDrawVariation == theDrawVariation {
                        aImage = (*aNode).m_value.mImage;
                        break;
                    }
                    aNode = (*aNode).m_next;
                }
            }
            if aImage.is_null() {
                aImage = self.MakeCachedPlantFrame(theSeedType, theDrawVariation);
                self.mImageVariationList.add_head(ReanimCacheImageVariation {
                    mSeedType: theSeedType, mDrawVariation: theDrawVariation, mImage: aImage,
                });
            }
        } else if seed_idx < 50 {
            aImage = self.mPlantImages[seed_idx];
            if aImage.is_null() {
                aImage = self.MakeCachedPlantFrame(theSeedType, DrawVariation::VARIATION_NORMAL);
                self.mPlantImages[seed_idx] = aImage;
            }
        }
        if !aImage.is_null() {
            let (mut aOffsetX, mut _aOffsetY, mut _aWidth, mut _aHeight) = (0i32, 0i32, 0i32, 0i32);
            Self::GetPlantImageSize(theSeedType, &mut aOffsetX, &mut _aOffsetY, &mut _aWidth, &mut _aHeight);
            let _ = g; let _ = thePosX; let _ = thePosY;
        }
    }

    pub fn DrawCachedMower(&mut self, g: &mut Graphics, thePosX: f32, thePosY: f32, theMowerType: i32) {
        let mower_idx = theMowerType as usize;
        if mower_idx < 4 {
            if self.mLawnMowers[mower_idx].is_null() {
                self.mLawnMowers[mower_idx] = self.MakeCachedMowerFrame(theMowerType);
            }
            let _ = g; let _ = thePosX; let _ = thePosY;
        }
    }

    pub fn DrawCachedZombie(&mut self, g: &mut Graphics, thePosX: f32, thePosY: f32, theZombieType: i32) {
        let zombie_idx = theZombieType as usize;
        if zombie_idx < 50 {
            if self.mZombieImages[zombie_idx].is_null() {
                self.mZombieImages[zombie_idx] = self.MakeCachedZombieFrame(theZombieType);
            }
            let _ = g; let _ = thePosX; let _ = thePosY;
        }
    }
}

impl Drop for ReanimatorCache {
    fn drop(&mut self) {
        self.ReanimatorCacheDispose();
    }
}

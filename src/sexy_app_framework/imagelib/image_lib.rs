// [TRANSLATION_NOTE]: ImageLib.h + ImageLib.cpp -> Rust 翻译
// 图像编解码库接口。实现暂为 stub，待集成 image crate 后完善

#![allow(non_snake_case, dead_code)]

use std::ptr;

/// ImageLib 命名空间下的 Image 类型
#[derive(Clone)]
pub struct Image {
    pub mWidth: i32,
    pub mHeight: i32,
    pub mBits: *mut u32,
}

impl Image {
    pub fn new() -> Self {
        Image {
            mWidth: 0,
            mHeight: 0,
            mBits: ptr::null_mut(),
        }
    }

    pub fn GetWidth(&self) -> i32 { self.mWidth }
    pub fn GetHeight(&self) -> i32 { self.mHeight }
    pub fn GetBits(&self) -> *mut u32 { self.mBits }
}

pub static mut G_ALPHA_COMPOSE_COLOR: i32 = 0;
pub static mut G_AUTO_LOAD_ALPHA: bool = false;
pub static mut G_IGNORE_JPEG2000_ALPHA: bool = true;

/// 写入 JPEG 图像文件
pub fn WriteJPEGImage(theFileName: &str, _theImage: *mut Image) -> bool {
    let _ = theFileName;
    false  // 暂未实现
}

/// 写入 PNG 图像文件
pub fn WritePNGImage(theFileName: &str, _theImage: *mut Image) -> bool {
    let _ = theFileName;
    false  // 暂未实现
}

/// 写入 TGA 图像文件
pub fn WriteTGAImage(theFileName: &str, _theImage: *mut Image) -> bool {
    let _ = theFileName;
    false  // 暂未实现
}

/// 加载图像文件（支持 PNG/JPEG/TGA）
pub fn GetImage(theFileName: &str, _lookForAlphaImage: bool) -> *mut Image {
    let _ = theFileName;
    ptr::null_mut()  // 暂未实现
}

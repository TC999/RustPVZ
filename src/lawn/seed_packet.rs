// [TRANSLATION_NOTE]: SeedPacket.h -> Rust 模块

use crate::const_enums::*;

pub struct SeedPacket {
    pub mIndex: i32,
    pub mX: i32,
    pub mY: i32,
    pub mPacketType: SeedType,
    pub mPacketType2: SeedType,
    pub mRefreshCounter: i32,
    pub mRefreshTime: i32,
    pub mTimerActive: bool,
    pub mFadeCount: i32,
    pub mXOff: f32,
    pub mYOff: f32,
}

impl SeedPacket {
    pub fn new() -> Self {
        SeedPacket {
            mIndex: 0,
            mX: 0,
            mY: 0,
            mPacketType: SeedType::SEED_NONE,
            mPacketType2: SeedType::SEED_NONE,
            mRefreshCounter: 0,
            mRefreshTime: 0,
            mTimerActive: false,
            mFadeCount: 0,
            mXOff: 0.0,
            mYOff: 0.0,
        }
    }

    pub fn SetPacketType(&mut self, theType: SeedType) {
        self.mPacketType = theType;
    }
}

impl Default for SeedPacket {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SeedBank {
    pub mNumPackets: i32,
    pub mSeedPackets: [SeedPacket; 10],
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
}

impl SeedBank {
    pub fn new() -> Self {
        SeedBank {
            mNumPackets: 0,
            mSeedPackets: [
                SeedPacket::new(), SeedPacket::new(), SeedPacket::new(), SeedPacket::new(), SeedPacket::new(),
                SeedPacket::new(), SeedPacket::new(), SeedPacket::new(), SeedPacket::new(), SeedPacket::new(),
            ],
            mX: 0, mY: 0, mWidth: 0, mHeight: 0,
        }
    }

    pub fn UpdateWidth(&mut self) {
        // stub
    }
}

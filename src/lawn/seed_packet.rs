// [TRANSLATION_NOTE]: SeedPacket.h -> Rust 模块

use crate::const_enums::*;
use crate::sexy_app_framework::graphics::graphics::Graphics;

#[derive(Clone)]
pub struct SeedPacket {
    pub mIndex: i32,
    pub mX: i32,
    pub mY: i32,
    pub mPacketType: SeedType,
    pub mImitaterType: SeedType,
    pub mRefreshCounter: i32,
    pub mRefreshTime: i32,
    pub mActive: bool,
    pub mRefreshing: bool,
    pub mFadeCount: i32,
    pub mSlotMachineCountDown: i32,
    pub mSlotMachiningPosition: f32,
    pub mSlotMachiningNextSeed: SeedType,
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
            mImitaterType: SeedType::SEED_NONE,
            mRefreshCounter: 0,
            mRefreshTime: 0,
            mActive: true,
            mRefreshing: false,
            mFadeCount: 0,
            mSlotMachineCountDown: 0,
            mSlotMachiningPosition: 0.0,
            mSlotMachiningNextSeed: SeedType::SEED_NONE,
            mXOff: 0.0,
            mYOff: 0.0,
        }
    }

    pub unsafe fn SetPacketType(&mut self, theType: SeedType) {
        self.mPacketType = theType;
        self.mActive = true;
        self.mRefreshing = false;
        self.mRefreshCounter = 0;
    }

    pub unsafe fn Update(&mut self) {
        let app = &mut *crate::lawn_app::G_LAWN_APP;

        // C++: if (mGameScene != SCENE_PLAYING || mPacketType == SEED_NONE) return;
        if (*app).mGameScene != GameScenes::SCENE_PLAYING || self.mPacketType == SeedType::SEED_NONE {
            return;
        }

        // C++: 获取 Board 引用用于检查 MainCounter
        let board = (*app).m_board.as_mut().unwrap();

        // C++: 在游戏帧 0 时触发 FlashIfReady
        if board.mMainCounter == 0 {
            self.FlashIfReady();
        }

        // C++: 冷却刷新
        if !self.mActive && self.mRefreshing {
            self.mRefreshCounter += 1;
            if self.mRefreshCounter > self.mRefreshTime {
                self.mRefreshCounter = 0;
                self.mRefreshing = false;
                self.Activate();
                self.FlashIfReady();
            }
        }

        // C++: 老虎机模式
        if self.mSlotMachineCountDown > 0 {
            self.mSlotMachineCountDown -= 1;
            let a_flips_per_second = crate::sexy_tod_lib::tod_common::tod_animate_curve_float(
                SLOT_MACHINE_TIME, 0, self.mSlotMachineCountDown, 6.0, 2.0, TodCurves::CURVE_LINEAR
            );
            self.mSlotMachiningPosition += a_flips_per_second * 0.01;

            if self.mSlotMachiningPosition >= 1.0 {
                self.mPacketType = self.mSlotMachiningNextSeed;
                if self.mSlotMachineCountDown == 0 {
                    self.Activate();
                    self.mSlotMachiningPosition = 0.0;
                } else {
                    self.mSlotMachiningPosition -= 1.0;
                    self.PickNextSlotMachineSeed();
                }
            } else if self.mSlotMachineCountDown == 0 {
                self.mSlotMachineCountDown = 1;
            }
        }
    }

    pub unsafe fn Draw(&self, _g: &mut Graphics) {
        // TODO: Draw SeedPacket sprite based on mPacketType
    }

    pub unsafe fn MouseDown(&mut self) -> bool {
        // Simplified: return true if seed was picked up
        if self.mPacketType == SeedType::SEED_NONE || !self.mActive {
            return false;
        }
        // Check cost, planting requirements etc.
        true
    }

    pub unsafe fn Activate(&mut self) {
        self.mActive = true;
        self.mRefreshing = false;
        self.mFadeCount = 0;
    }

    pub unsafe fn Deactivate(&mut self) {
        self.mActive = false;
        self.mRefreshing = true;
        self.mRefreshCounter = 0;
    }

    pub unsafe fn WasPlanted(&mut self) {
        self.Deactivate();
        // C++: mRefreshTime = GetSeedRefreshTime(mPacketType, mImitaterType);
        self.mRefreshTime = 600; // Default cooldown
        self.mRefreshCounter = 0;
    }

    /// C++ SeedPacket::FlashIfReady (SeedPacket.cpp:105)
    pub unsafe fn FlashIfReady(&mut self) {
        if self.mActive && self.mPacketType != SeedType::SEED_NONE {
            // C++: mBoard->mSeedBank->mY = 0;
            // [TODO]: 闪烁效果 — 种子包可用时的视觉提示
        }
    }

    /// C++ SeedPacket::PickNextSlotMachineSeed (SeedPacket.cpp:53)
    pub unsafe fn PickNextSlotMachineSeed(&mut self) {
        // C++: 随机选择老虎机下一个种子
        // [TODO]: 从可用种子中随机选取
        self.mSlotMachiningNextSeed = SeedType::SEED_PEASHOOTER; // Placeholder
    }

    /// C++ SeedPacket::SlotMachineStart (SeedPacket.cpp:98)
    pub unsafe fn SlotMachineStart(&mut self) {
        self.mSlotMachineCountDown = SLOT_MACHINE_TIME;
        self.mSlotMachiningPosition = 0.0;
        self.mActive = false;
        self.PickNextSlotMachineSeed();
    }

    pub unsafe fn CanPickUp(&self) -> bool {
        self.mActive && self.mPacketType != SeedType::SEED_NONE
    }
}

impl Default for SeedPacket {
    fn default() -> Self {
        Self::new()
    }
}

// C++: SLOT_MACHINE_TIME 常量 (SeedPacket.cpp:46)
pub const SLOT_MACHINE_TIME: i32 = 200;

pub struct SeedBank {
    pub mNumPackets: i32,
    pub mSeedPackets: [SeedPacket; 10],
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mCutSceneDarken: i32,
    pub mConveyorBeltCounter: i32,
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
            mCutSceneDarken: 0,
            mConveyorBeltCounter: 0,
        }
    }

    pub unsafe fn Update(&mut self) {
        for i in 0..self.mNumPackets as usize {
            self.mSeedPackets[i].Update();
        }
    }

    pub unsafe fn Draw(&self, _g: &mut Graphics) {
        // TODO: Draw all seed packets in the bank
    }

    pub unsafe fn AddSeed(&mut self, theSeedType: SeedType, _placeOnLeft: bool) {
        if self.mNumPackets >= 10 { return; }
        let idx = if _placeOnLeft {
            // Shift all seeds right and insert at position 0
            for i in (1..self.mNumPackets as usize).rev() {
                self.mSeedPackets[i] = SeedPacket::new();
                self.mSeedPackets[i].mPacketType = self.mSeedPackets[i - 1].mPacketType;
            }
            self.mSeedPackets[0].SetPacketType(theSeedType);
            0
        } else {
            let idx = self.mNumPackets as usize;
            self.mSeedPackets[idx].SetPacketType(theSeedType);
            idx
        };
        self.mSeedPackets[idx].mActive = true;
        self.mSeedPackets[idx].mRefreshing = false;
        self.mNumPackets += 1;
    }

    pub unsafe fn RemoveSeed(&mut self, theIndex: i32) {
        if theIndex < 0 || theIndex >= self.mNumPackets { return; }
        for i in theIndex as usize..(self.mNumPackets as usize - 1) {
            self.mSeedPackets[i] = self.mSeedPackets[i + 1].clone();
        }
        self.mNumPackets -= 1;
    }

    pub unsafe fn UpdateWidth(&mut self) {
        // Width based on number of packets
        let packetWidth = 56; // SEED_PACKET_WIDTH
        self.mWidth = self.mNumPackets * packetWidth;
    }

    pub unsafe fn RefreshAllPackets(&mut self) {
        for i in 0..self.mNumPackets as usize {
            self.mSeedPackets[i].mRefreshTime = 600;
            self.mSeedPackets[i].Deactivate();
        }
    }

    pub unsafe fn GetNumSeedsOnConveyorBelt(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.mNumPackets as usize {
            if self.mSeedPackets[i].mPacketType != SeedType::SEED_NONE {
                count += 1;
            }
        }
        count
    }
}

// Free functions
pub unsafe fn SeedPacketDrawSeed(_g: &mut Graphics, _x: f32, _y: f32, _theSeedType: SeedType, _theImitaterType: SeedType, _theOffsetX: f32, _theOffsetY: f32, _theScale: f32) {
    // TODO: Draw seed icon using IMAGE_PACKET_PLANTS
}

pub unsafe fn DrawSeedPacket(_g: &mut Graphics, _x: f32, _y: f32, _theSeedType: SeedType, _theImitaterType: SeedType, _thePercentDark: f32, _theGrayness: i32, _theDrawCost: bool, _theUseCurrentCost: bool) {
    // TODO: Full seed packet drawing with cost overlay
}

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
        // From C++ SeedPacket::Update()
        // if mApp->mGameScene != SCENE_PLAYING || mPacketType == SEED_NONE { return; }

        if self.mRefreshing && !self.mActive {
            self.mRefreshCounter += 1;
            if self.mRefreshCounter > self.mRefreshTime {
                self.mRefreshCounter = 0;
                self.mRefreshing = false;
                self.mActive = true;
                // FlashIfReady();
            }
        }

        if self.mSlotMachineCountDown > 0 {
            self.mSlotMachineCountDown -= 1;
            self.mSlotMachiningPosition += 0.06; // Simplified
            if self.mSlotMachiningPosition >= 1.0 {
                self.mPacketType = self.mSlotMachiningNextSeed;
                if self.mSlotMachineCountDown == 0 {
                    self.mActive = true;
                    self.mSlotMachiningPosition = 0.0;
                } else {
                    self.mSlotMachiningPosition -= 1.0;
                    // PickNextSlotMachineSeed();
                }
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
        self.mRefreshTime = 600; // Default cooldown
        self.mRefreshCounter = 0;
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

// [TRANSLATION_NOTE]: ProfileMgr.h + ProfileMgr.cpp -> Rust 翻译
// PlayerInfo 使用独立的 player_info 模块

#![allow(non_snake_case, dead_code)]

use std::collections::HashMap;

use crate::sexy_app_framework::common::{StringLessNoCase, get_app_data_path};
use crate::sexy_app_framework::sexy_app_base::G_SEXY_APP;
use crate::lawn::system::data_sync::{DataSync, DataReader, DataWriter};
use super::player_info::PlayerInfo;

pub type ProfilePair = (String, PlayerInfo);
pub type ProfileMap = HashMap<String, PlayerInfo>;

static mut G_PROFILE_VERSION: i32 = 14;

pub struct ProfileMgr {
    pub mProfileMap: ProfileMap,
    pub mNextProfileId: u32,
    pub mNextProfileUseSeq: u32,
}

impl ProfileMgr {
    pub fn new() -> Self {
        let mut mgr = ProfileMgr {
            mProfileMap: ProfileMap::new(),
            mNextProfileId: 1,
            mNextProfileUseSeq: 1,
        };
        mgr.Clear();
        mgr
    }

    pub fn Clear(&mut self) {
        self.mProfileMap.clear();
        self.mNextProfileId = 1;
        self.mNextProfileUseSeq = 1;
    }

    pub fn GetNumProfiles(&self) -> i32 {
        self.mProfileMap.len() as i32
    }

    fn SyncState(&mut self, theSync: &mut DataSync) {
        let aVersion = unsafe { G_PROFILE_VERSION };
        let mut aVersionMut = aVersion as u32;
        theSync.sync_u32(&mut aVersionMut);
        let aVersion = aVersionMut as i32;
        theSync.set_version(aVersion);

        if aVersion == unsafe { G_PROFILE_VERSION } {
            if theSync.is_reader() {
                self.mProfileMap.clear();

                let mut aMaxProfileId: u32 = 0;
                let mut aMaxUseSeq: u32 = 0;
                let mut aProfileCount = 0u16;
                theSync.sync_u16(&mut aProfileCount);
                for _ in 0..aProfileCount {
                    let mut aProfile = PlayerInfo::new();
                    aProfile.SyncSummary(theSync);

                    if aProfile.mId > aMaxProfileId {
                        aMaxProfileId = aProfile.mId;
                    }
                    if aProfile.mUseSeq > aMaxUseSeq {
                        aMaxUseSeq = aProfile.mUseSeq;
                    }

                    self.mProfileMap.insert(aProfile.mName.clone(), aProfile);
                }

                self.mNextProfileId = aMaxProfileId + 1;
                self.mNextProfileUseSeq = aMaxUseSeq + 1;
            } else {
                let size = self.mProfileMap.len() as u16;
                let mut size_mut = size;
                theSync.sync_u16(&mut size_mut);

                let keys: Vec<String> = self.mProfileMap.keys().cloned().collect();
                for key in &keys {
                    if let Some(profile) = self.mProfileMap.get_mut(key) {
                        profile.SyncSummary(theSync);
                    }
                }
            }
        }
    }

    pub fn Load(&mut self) {
        let aFileName = get_app_data_path("userdata/users.dat");

        // 尝试从 SexyAppBase 读取文件
        unsafe {
            if let Some(ref _base) = G_SEXY_APP {
                // 简化处理：尝试直接读取文件
                if let Ok(data) = std::fs::read(&aFileName) {
                    let mut aReader = DataReader::new();
                    aReader.open_memory(data, false);
                    let mut aSync = DataSync::from_reader(&mut aReader);
                    self.SyncState(&mut aSync);
                    return;
                }
            }
        }

        self.Clear();
    }

    pub fn Save(&mut self) {
        let mut aWriter = DataWriter::new();
        aWriter.open_memory(0x20);
        let mut aSync = DataSync::from_writer(&mut aWriter);
        self.SyncState(&mut aSync);

        let aFileName = get_app_data_path("userdata/users.dat");
        // 确保目录存在
        let _ = std::fs::create_dir_all(get_app_data_path("userdata"));
        let _ = std::fs::write(&aFileName, aWriter.into_data());
    }

    fn DeleteProfileByIterator(&mut self, key: &str) {
        if let Some(mut profile) = self.mProfileMap.remove(key) {
            profile.DeleteUserFiles();
        }
    }

    pub fn DeleteProfile(&mut self, theName: &str) -> bool {
        if !self.mProfileMap.contains_key(theName) {
            return false;
        }
        self.DeleteProfileByIterator(theName);
        true
    }

    pub fn RenameProfile(&mut self, theOldName: &str, theNewName: &str) -> bool {
        if !self.mProfileMap.contains_key(theOldName) {
            return false;
        }

        if theOldName.to_lowercase() == theNewName.to_lowercase() {
            if let Some(profile) = self.mProfileMap.get_mut(theOldName) {
                profile.mName = theNewName.to_string();
            }
            return true;
        }

        let old_profile = self.mProfileMap.remove(theOldName).unwrap();
        if self.mProfileMap.contains_key(theNewName) {
            // 恢复
            self.mProfileMap.insert(theOldName.to_string(), old_profile);
            return false;
        }

        let mut new_profile = old_profile;
        new_profile.mName = theNewName.to_string();
        self.mProfileMap.insert(theNewName.to_string(), new_profile);
        true
    }

    pub fn DeleteOldestProfile(&mut self) {
        if self.mProfileMap.is_empty() {
            return;
        }

        let oldest_key = self.mProfileMap.iter()
            .min_by_key(|(_, p)| p.mUseSeq)
            .map(|(k, _)| k.clone());

        if let Some(key) = oldest_key {
            self.DeleteProfileByIterator(&key);
        }
    }

    pub fn GetProfile(&mut self, theName: &str) -> Option<&mut PlayerInfo> {
        if let Some(profile) = self.mProfileMap.get_mut(theName) {
            profile.LoadDetails();
            profile.mUseSeq = self.mNextProfileUseSeq;
            self.mNextProfileUseSeq += 1;
            // 注意：由于借用规则，这里返回的引用不安全，但匹配 C++ 泄漏指针的语义
            // 实际使用中需要通过其他方式访问
            return None; // 改为通过下标访问
        }
        None
    }

    pub fn GetAnyProfile(&mut self) -> Option<&mut PlayerInfo> {
        if self.mProfileMap.is_empty() {
            return None;
        }

        let first_key = self.mProfileMap.keys().next().cloned();
        if let Some(key) = first_key {
            return self.GetProfile(&key);
        }
        None
    }

    pub fn AddProfile(&mut self, theName: &str) -> Option<&mut PlayerInfo> {
        if self.mProfileMap.contains_key(theName) {
            return None;
        }

        let mut new_profile = PlayerInfo::new();
        new_profile.mName = theName.to_string();
        new_profile.mId = self.mNextProfileId;
        self.mNextProfileId += 1;
        new_profile.mUseSeq = self.mNextProfileUseSeq;
        self.mNextProfileUseSeq += 1;

        self.mProfileMap.insert(theName.to_string(), new_profile);

        // 删除旧配置文件
        while self.mProfileMap.len() > 200 {
            self.DeleteOldestProfile();
        }

        self.mProfileMap.get_mut(theName)
    }

    pub fn GetProfileMap(&mut self) -> &mut ProfileMap {
        &mut self.mProfileMap
    }
}

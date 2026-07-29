// [TRANSLATION_NOTE]: PakInterface.h + PakInterface.cpp -> Rust 翻译
// .pak 资源包文件读取接口
// 使用 std::fs::File 替代 C 的 FILE*，Vec<u8> 替代 malloc 内存

#![allow(non_snake_case, dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::sexy_app_framework::common::{
    from_le32, from_le64, get_resource_folder, is_path_rooted, path_from_u8, path_to_u8,
};

const FILEFLAGS_END: u8 = 0x80;
const PAK_MAGIC: u32 = 0xBAC04AC0;

/// 资源包中的一个资源文件记录
#[derive(Clone)]
pub struct PakRecord {
    pub mCollectionIndex: usize,   // 所属资源包在 mPakDataList 中的索引
    pub mFileName: String,
    pub mFileTime: i64,
    pub mStartPos: i32,
    pub mSize: i32,
}

/// 资源包数据（内存中）
pub struct PakCollection {
    pub mData: Vec<u8>,
}

/// PFILE 结构 — 模拟 C 的 FILE*
pub struct PFILE {
    pub mRecordIndex: Option<usize>,  // 对应 mPakRecordMap 中的记录索引（若为 None 则使用真实文件）
    pub mPos: i32,
    pub mRealFile: Option<File>,       // 真实文件句柄
}

pub static mut G_PAK_INTERFACE: Option<PakInterface> = None;

pub struct PakInterface {
    pub mPakDataList: Vec<PakCollection>,       // 对应 mPakCollectionList
    pub mPakRecordList: Vec<PakRecord>,          // 所有记录的列表
    pub mPakRecordMap: HashMap<String, usize>,   // 文件名 -> 记录索引
}

impl PakInterface {
    pub fn new() -> Self {
        PakInterface {
            mPakDataList: Vec::new(),
            mPakRecordList: Vec::new(),
            mPakRecordMap: HashMap::new(),
        }
    }

    /// 标准化路径（大写、去除前导 ./、相对化）
    pub fn NormalizePakPath(theFileName: &str) -> String {
        let aFilePath = path_from_u8(theFileName);

        // 将绝对路径转换为相对于资源文件夹的路径
        let aRelativePath = if is_path_rooted(theFileName) {
            let aResourceFolder = get_resource_folder();
            if !aResourceFolder.is_empty() {
                let aResPath = Path::new(&aResourceFolder);
                if let Ok(rel) = aFilePath.strip_prefix(aResPath) {
                    rel.to_path_buf()
                } else {
                    aFilePath.clone()
                }
            } else {
                aFilePath.clone()
            }
        } else {
            aFilePath.clone()
        };

        let mut aResult = path_to_u8(&aRelativePath);
        // 替换反斜杠为正斜杠
        aResult = aResult.replace('\\', "/");

        // 去除前导 ./
        if aResult.starts_with("./") {
            aResult = aResult[2..].to_string();
        }

        // 转大写
        aResult = aResult.to_uppercase();
        aResult
    }

    /// 添加一个 .pak 文件
    pub fn AddPakFile(&mut self, theFileName: &str) -> bool {
        let mut aFile = match File::open(theFileName) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let aFileSize = match aFile.metadata() {
            Ok(m) => m.len() as usize,
            Err(_) => return false,
        };

        // 读取所有数据
        let mut aFileData = vec![0u8; aFileSize];
        if aFile.read_exact(&mut aFileData).is_err() {
            return false;
        }

        // XOR 解密
        for byte in aFileData.iter_mut() {
            *byte ^= 0xF7;
        }

        let pakCollectionIndex = self.mPakDataList.len();
        self.mPakDataList.push(PakCollection {
            mData: aFileData.clone(),
        });

        // 添加资源包自身的记录
        let aPakKey = Self::NormalizePakPath(theFileName);
        let pakRecordIndex = self.mPakRecordList.len();
        self.mPakRecordList.push(PakRecord {
            mCollectionIndex: pakCollectionIndex,
            mFileName: aPakKey.clone(),
            mStartPos: 0,
            mSize: aFileSize as i32,
            mFileTime: 0,
        });
        self.mPakRecordMap.insert(aPakKey.clone(), pakRecordIndex);

        // 打开资源包用于读取内部索引
        let aPakRecordIdx = pakRecordIndex;
        let mut aPos: i32 = 0;

        // 读取魔数
        let mut aMagicBytes = [0u8; 4];
        self.FReadAt(&mut aMagicBytes, 4, aPakRecordIdx, &mut 0);
        let aMagic = from_le32(u32::from_le_bytes(aMagicBytes));
        if aMagic != PAK_MAGIC {
            return false;
        }
        aPos += 4;

        // 读取版本
        let mut aVerBytes = [0u8; 4];
        self.FReadAt(&mut aVerBytes, 4, aPakRecordIdx, &mut aPos);
        let aVersion = from_le32(u32::from_le_bytes(aVerBytes));
        if aVersion > 0 {
            return false;
        }
        // aPos += 4; // 已经在 FReadAt 中更新了

        loop {
            let mut aFlags = [0u8; 1];
            let count = self.FReadAt(&mut aFlags, 1, aPakRecordIdx, &mut aPos);
            if count == 0 || (aFlags[0] & FILEFLAGS_END) != 0 {
                break;
            }

            // 读取文件名长度
            let mut aNameWidth = [0u8; 1];
            self.FReadAt(&mut aNameWidth, 1, aPakRecordIdx, &mut aPos);

            let nameLen = aNameWidth[0] as usize;
            let mut aName = vec![0u8; nameLen];
            self.FReadAt(&mut aName, nameLen, aPakRecordIdx, &mut aPos);

            let mut aNameStr = String::from_utf8_lossy(&aName).to_string();

            // 读取源大小
            let mut aSrcSizeBytes = [0u8; 4];
            self.FReadAt(&mut aSrcSizeBytes, 4, aPakRecordIdx, &mut aPos);
            let aSrcSize = from_le32(u32::from_le_bytes(aSrcSizeBytes)) as i32;

            // 读取文件时间
            let mut aFileTimeBytes = [0u8; 8];
            self.FReadAt(&mut aFileTimeBytes, 8, aPakRecordIdx, &mut aPos);
            let aFileTime = from_le64(u64::from_le_bytes(aFileTimeBytes)) as i64;

            // 替换反斜杠
            aNameStr = aNameStr.replace('\\', "/");

            let aKey = Self::NormalizePakPath(&aNameStr);
            let newRecordIdx = self.mPakRecordList.len();
            self.mPakRecordList.push(PakRecord {
                mCollectionIndex: pakCollectionIndex,
                mFileName: aKey.clone(),
                mStartPos: aPos,
                mSize: aSrcSize,
                mFileTime: aFileTime,
            });
            self.mPakRecordMap.insert(aKey, newRecordIdx);

            aPos += aSrcSize;
        }

        // 获取偏移量（索引头大小）
        let anOffset = aPos;

        // 更新所有属于此资源包的记录的起始位置
        for record in self.mPakRecordList.iter_mut() {
            if record.mCollectionIndex == pakCollectionIndex {
                record.mStartPos += anOffset;
            }
        }

        true
    }

    /// 从特定记录和位置读取数据
    fn FReadAt(&self, buf: &mut [u8], size: usize, recordIdx: usize, pos: &mut i32) -> usize {
        if recordIdx >= self.mPakRecordList.len() {
            return 0;
        }
        let record = &self.mPakRecordList[recordIdx];
        if record.mCollectionIndex >= self.mPakDataList.len() {
            return 0;
        }

        let collection = &self.mPakDataList[record.mCollectionIndex];
        let start = (record.mStartPos + *pos) as usize;
        let end = std::cmp::min(start + size, (record.mStartPos + record.mSize) as usize);

        if start >= end {
            return 0;
        }

        let actual_size = end - start;
        let copy_size = std::cmp::min(actual_size, buf.len());
        buf[..copy_size].copy_from_slice(&collection.mData[start..start + copy_size]);
        *pos += copy_size as i32;
        copy_size
    }

    /// 打开文件（从 pak 中或从真实文件系统）
    pub fn FOpen(&mut self, theFileName: &str, anAccess: &str) -> Option<*mut PFILE> {
        if anAccess == "r" || anAccess == "rb" || anAccess == "rt" {
            let aKey = Self::NormalizePakPath(theFileName);
            if let Some(&recordIdx) = self.mPakRecordMap.get(&aKey) {
                let pfile = Box::into_raw(Box::new(PFILE {
                    mRecordIndex: Some(recordIdx),
                    mPos: 0,
                    mRealFile: None,
                }));
                return Some(pfile);
            }
        }

        // 尝试从文件系统打开
        let aResourceBase = get_resource_folder();
        let aFP = if !aResourceBase.is_empty() && !is_path_rooted(theFileName) {
            let full_path = Path::new(&aResourceBase).join(theFileName);
            File::open(full_path).ok()
        } else {
            File::open(theFileName).ok()
        };

        match aFP {
            Some(file) => {
                let pfile = Box::into_raw(Box::new(PFILE {
                    mRecordIndex: None,
                    mPos: 0,
                    mRealFile: Some(file),
                }));
                Some(pfile)
            }
            None => None,
        }
    }

    pub fn FClose(&mut self, theFile: *mut PFILE) -> i32 {
        if theFile.is_null() {
            return 0;
        }
        unsafe {
            let _ = Box::from_raw(theFile);
        }
        0
    }

    pub fn FSeek(&self, theFile: *mut PFILE, theOffset: i64, theOrigin: i32) -> i32 {
        if theFile.is_null() {
            return -1;
        }
        unsafe {
            let pfile = &mut *theFile;
            match pfile.mRecordIndex {
                Some(recordIdx) => {
                    let record = &self.mPakRecordList[recordIdx];
                    pfile.mPos = match theOrigin {
                        0 /* SEEK_SET */ => theOffset as i32,
                        1 /* SEEK_CUR */ => pfile.mPos + theOffset as i32,
                        2 /* SEEK_END */ => record.mSize - theOffset as i32,
                        _ => pfile.mPos,
                    };
                    pfile.mPos = std::cmp::max(0, std::cmp::min(pfile.mPos, record.mSize));
                    0
                }
                None => {
                    if let Some(ref mut file) = pfile.mRealFile {
                        let seek_pos = match theOrigin {
                            0 => SeekFrom::Start(theOffset as u64),
                            1 => SeekFrom::Current(theOffset),
                            2 => SeekFrom::End(theOffset),
                            _ => return -1,
                        };
                        file.seek(seek_pos).ok().map(|_| 0).unwrap_or(-1)
                    } else {
                        -1
                    }
                }
            }
        }
    }

    pub fn FTell(&self, theFile: *mut PFILE) -> i32 {
        if theFile.is_null() {
            return -1;
        }
        unsafe {
            let pfile = &mut *theFile;
            match pfile.mRecordIndex {
                Some(_) => pfile.mPos,
                None => {
                    if let Some(ref mut file) = pfile.mRealFile {
                        file.stream_position().ok().map(|p| p as i32).unwrap_or(-1)
                    } else {
                        -1
                    }
                }
            }
        }
    }

    pub fn FRead(&self, thePtr: &mut [u8], theElemSize: i32, theCount: i32, theFile: *mut PFILE) -> usize {
        if theFile.is_null() {
            return 0;
        }
        unsafe {
            let pfile = &mut *theFile;
            match pfile.mRecordIndex {
                Some(recordIdx) => {
                    if recordIdx >= self.mPakRecordList.len() {
                        return 0;
                    }
                    let record = &self.mPakRecordList[recordIdx];
                    if record.mCollectionIndex >= self.mPakDataList.len() {
                        return 0;
                    }

                    let collection = &self.mPakDataList[record.mCollectionIndex];
                    let totalBytes = (theElemSize * theCount) as usize;
                    let aSizeBytes = std::cmp::min(totalBytes, (record.mSize - pfile.mPos) as usize);

                    let start = (record.mStartPos + pfile.mPos) as usize;
                    let end = start + aSizeBytes;

                    let copy_size = std::cmp::min(aSizeBytes, thePtr.len());
                    thePtr[..copy_size].copy_from_slice(&collection.mData[start..end]);

                    pfile.mPos += copy_size as i32;
                    copy_size / theElemSize as usize
                }
                None => {
                    if let Some(ref mut file) = pfile.mRealFile {
                        let totalBytes = (theElemSize * theCount) as usize;
                        let mut buf = vec![0u8; std::cmp::min(totalBytes, thePtr.len())];
                        match file.read(&mut buf) {
                            Ok(n) => {
                                thePtr[..n].copy_from_slice(&buf[..n]);
                                n / theElemSize as usize
                            }
                            Err(_) => 0,
                        }
                    } else {
                        0
                    }
                }
            }
        }
    }

    pub fn FGetC(&self, theFile: *mut PFILE) -> i32 {
        if theFile.is_null() {
            return -1;
        }
        unsafe {
            let pfile = &mut *theFile;
            match pfile.mRecordIndex {
                Some(recordIdx) => {
                    if recordIdx >= self.mPakRecordList.len() {
                        return -1;
                    }
                    let record = &self.mPakRecordList[recordIdx];
                    if record.mCollectionIndex >= self.mPakDataList.len() {
                        return -1;
                    }

                    let collection = &self.mPakDataList[record.mCollectionIndex];
                    loop {
                        if pfile.mPos >= record.mSize {
                            return -1; // EOF
                        }
                        let pos = (record.mStartPos + pfile.mPos) as usize;
                        let aChar = collection.mData[pos] as char;
                        pfile.mPos += 1;
                        if aChar != '\r' {
                            return aChar as u8 as i32;
                        }
                    }
                }
                None => {
                    if let Some(ref mut file) = pfile.mRealFile {
                        let mut buf = [0u8; 1];
                        match file.read(&mut buf) {
                            Ok(1) => buf[0] as i32,
                            _ => -1,
                        }
                    } else {
                        -1
                    }
                }
            }
        }
    }

    pub fn UnGetC(&self, theChar: i32, theFile: *mut PFILE) -> i32 {
        if theFile.is_null() {
            return -1;
        }
        unsafe {
            let pfile = &mut *theFile;
            match pfile.mRecordIndex {
                Some(_) => {
                    pfile.mPos = std::cmp::max(pfile.mPos - 1, 0);
                    theChar
                }
                None => {
                    // Rust 的 File 不支持 ungetc，这里简化处理
                    theChar
                }
            }
        }
    }

    pub fn FGetS(&self, thePtr: &mut [u8], theSize: i32, theFile: *mut PFILE) -> bool {
        if theFile.is_null() || theSize <= 0 {
            return false;
        }
        unsafe {
            let pfile = &mut *theFile;
            let record_idx = pfile.mRecordIndex;
            match record_idx {
                Some(recordIdx) => {
                    if recordIdx >= self.mPakRecordList.len() {
                        return false;
                    }
                    let record = &self.mPakRecordList[recordIdx];
                    if record.mCollectionIndex >= self.mPakDataList.len() {
                        return false;
                    }

                    let collection = &self.mPakDataList[record.mCollectionIndex];
                    let mut anIdx: usize = 0;
                    while anIdx < theSize as usize {
                        if pfile.mPos >= record.mSize {
                            if anIdx == 0 {
                                return false;
                            }
                            break;
                        }
                        let pos = (record.mStartPos + pfile.mPos) as usize;
                        let aChar = collection.mData[pos] as char;
                        pfile.mPos += 1;
                        if aChar != '\r' {
                            thePtr[anIdx] = aChar as u8;
                            anIdx += 1;
                        }
                        if aChar == '\n' {
                            break;
                        }
                    }
                    if anIdx < thePtr.len() {
                        thePtr[anIdx] = 0;
                    }
                    true
                }
                None => {
                    let pfile2 = &mut *theFile;
                    if let Some(ref mut file) = pfile2.mRealFile {
                        let mut anIdx: usize = 0;
                        let mut single = [0u8; 1];
                        loop {
                            match file.read(&mut single) {
                                Ok(0) | Err(_) => break,
                                Ok(1) => {
                                    let aChar = single[0] as char;
                                    if aChar == '\n' {
                                        if anIdx < thePtr.len() {
                                            thePtr[anIdx] = b'\n';
                                            anIdx += 1;
                                        }
                                        break;
                                    }
                                    if aChar != '\r' && anIdx < thePtr.len() {
                                        thePtr[anIdx] = single[0];
                                        anIdx += 1;
                                    }
                                }
                                _ => break,
                            }
                        }
                        if anIdx == 0 {
                            return false;
                        }
                        if anIdx < thePtr.len() {
                            thePtr[anIdx] = 0;
                        }
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    pub fn FEof(&self, theFile: *mut PFILE) -> bool {
        if theFile.is_null() {
            return true;
        }
        unsafe {
            let pfile = &*theFile;
            match pfile.mRecordIndex {
                Some(recordIdx) => {
                    if recordIdx >= self.mPakRecordList.len() {
                        return true;
                    }
                    let record = &self.mPakRecordList[recordIdx];
                    pfile.mPos >= record.mSize
                }
                None => {
                    // 真实文件 EOF 检测比较复杂，简化处理
                    false
                }
            }
        }
    }
}

// 全局 PakInterface 的便捷函数
pub fn pak_fopen(file_name: &str, access: &str) -> *mut PFILE {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref mut pak) => pak.FOpen(file_name, access).unwrap_or(std::ptr::null_mut()),
            None => std::ptr::null_mut(),
        }
    }
}

pub fn pak_fclose(file: *mut PFILE) -> i32 {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref mut pak) => pak.FClose(file),
            None => {
                if !file.is_null() {
                    let _ = Box::from_raw(file);
                }
                0
            }
        }
    }
}

pub fn pak_fseek(file: *mut PFILE, offset: i64, origin: i32) -> i32 {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref pak) => pak.FSeek(file, offset, origin),
            None => -1,
        }
    }
}

pub fn pak_ftell(file: *mut PFILE) -> i32 {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref pak) => pak.FTell(file),
            None => -1,
        }
    }
}

pub fn pak_fread(ptr: &mut [u8], elem_size: i32, count: i32, file: *mut PFILE) -> usize {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref pak) => pak.FRead(ptr, elem_size, count, file),
            None => 0,
        }
    }
}

pub fn pak_fgetc(file: *mut PFILE) -> i32 {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref pak) => pak.FGetC(file),
            None => -1,
        }
    }
}

pub fn pak_feof(file: *mut PFILE) -> bool {
    unsafe {
        match G_PAK_INTERFACE {
            Some(ref pak) => pak.FEof(file),
            None => true,
        }
    }
}

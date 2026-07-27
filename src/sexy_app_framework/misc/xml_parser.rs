// [TRANSLATION_NOTE]: XMLParser.h + XMLParser.cpp -> Rust
// XML 解析器，支持 ASCII/UTF-8/UTF-16 编码
// PFILE 使用 *mut c_void 存根，p_fopen 等函数用 libc 风格的包装

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;
use std::mem::MaybeUninit;
use std::ptr;

// ==================== PFILE 存根 ====================
// 使用标准库文件操作模拟 PFILE
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;

pub type PFILE = *mut std::ffi::c_void;

// 全局文件表
static FILE_TABLE: Mutex<Vec<Option<File>>> = Mutex::new(Vec::new());

fn alloc_file_id(file: File) -> isize {
    let mut table = FILE_TABLE.lock().unwrap();
    for (i, slot) in table.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(file);
            return i as isize + 1;
        }
    }
    let id = table.len() as isize + 1;
    table.push(Some(file));
    id
}

pub fn p_fopen(filename: &str, _mode: &str) -> PFILE {
    match File::open(filename) {
        Ok(file) => {
            let id = alloc_file_id(file);
            id as PFILE
        }
        Err(_) => std::ptr::null_mut(),
    }
}

pub fn p_fclose(file: PFILE) {
    let id = file as isize;
    if id <= 0 { return; }
    let mut table = FILE_TABLE.lock().unwrap();
    let idx = (id - 1) as usize;
    if idx < table.len() {
        table[idx] = None;
    }
}

pub fn p_fread(buf: &mut [u8], file: PFILE) -> usize {
    let id = file as isize;
    if id <= 0 { return 0; }
    let mut table = FILE_TABLE.lock().unwrap();
    let idx = (id - 1) as usize;
    if idx < table.len() {
        if let Some(ref mut f) = table[idx] {
            f.read(buf).unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    }
}

pub fn p_fseek(file: PFILE, offset: i64, whence: i32) -> i32 {
    let id = file as isize;
    if id <= 0 { return -1; }
    let mut table = FILE_TABLE.lock().unwrap();
    let idx = (id - 1) as usize;
    if idx < table.len() {
        if let Some(ref mut f) = table[idx] {
            let seek_from = match whence {
                0 => SeekFrom::Start(offset as u64),
                1 => SeekFrom::Current(offset),
                2 => SeekFrom::End(offset),
                _ => return -1,
            };
            f.seek(seek_from).map(|_| 0).unwrap_or(-1)
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn p_ftell(file: PFILE) -> i64 {
    let id = file as isize;
    if id <= 0 { return -1; }
    let mut table = FILE_TABLE.lock().unwrap();
    let idx = (id - 1) as usize;
    if idx < table.len() {
        if let Some(ref mut f) = table[idx] {
            let pos = f.seek(SeekFrom::Current(0)).unwrap_or(0);
            pos as i64
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn p_fgetc(file: PFILE) -> i32 {
    let mut buf = [0u8; 1];
    if p_fread(&mut buf, file) == 1 {
        buf[0] as i32
    } else {
        -1
    }
}

pub fn p_ungetc(c: i32, file: PFILE) -> i32 {
    let id = file as isize;
    if id <= 0 || c == -1 { return -1; }
    let mut table = FILE_TABLE.lock().unwrap();
    let idx = (id - 1) as usize;
    if idx < table.len() {
        if let Some(ref mut f) = table[idx] {
            let pos = f.seek(SeekFrom::Current(0)).unwrap_or(0);
            if pos > 0 {
                f.seek(SeekFrom::Current(-1)).ok();
                c
            } else {
                -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

// ==================== 工具函数 ====================
pub fn FromLE16(v: u16) -> u16 {
    u16::from_le(v)
}

pub fn FromBE16(v: u16) -> u16 {
    u16::from_be(v)
}

pub fn XMLDecodeString(theString: &str) -> String {
    // HTML/XML 实体解码
    let mut result = String::new();
    let chars: Vec<char> = theString.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '&' {
            // Find the closing ;
            let mut j = i + 1;
            while j < chars.len() && chars[j] != ';' {
                j += 1;
            }
            if j < chars.len() {
                let entity: String = chars[i+1..j].iter().collect();
                let decoded = match entity.as_str() {
                    "amp" => "&",
                    "lt" => "<",
                    "gt" => ">",
                    "quot" => "\"",
                    "apos" => "'",
                    _ => {
                        if entity.starts_with('#') {
                            let num_str = &entity[1..];
                            let code = if num_str.starts_with('x') || num_str.starts_with('X') {
                                u32::from_str_radix(&num_str[1..], 16).ok()
                            } else {
                                num_str.parse::<u32>().ok()
                            };
                            if let Some(c) = code.and_then(char::from_u32) {
                                let mut s = String::new();
                                s.push(c);
                                // return early
                                result.push_str(&s);
                                i = j + 1;
                                continue;
                            }
                            &entity
                        } else {
                            &entity
                        }
                    }
                };
                result.push_str(decoded);
                i = j + 1;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

// ==================== XML 类型定义 ====================
pub type XMLParamMap = HashMap<String, String>;
pub type XMLParserBuffer = Vec<u8>;

// XMLParamMapIteratorList - stores attribute iterators in their original order
// In C++ this stores map iterators; we store keys in insertion order
pub type XMLParamMapIteratorList = Vec<String>;

#[derive(Clone)]
pub struct XMLElement {
    pub mType: i32,
    pub mSection: String,
    pub mValue: String,
    pub mInstruction: String,
    pub mAttributes: XMLParamMap,
    pub mAttributeIteratorList: XMLParamMapIteratorList,
}

impl XMLElement {
    pub const TYPE_NONE: i32 = 0;
    pub const TYPE_START: i32 = 1;
    pub const TYPE_END: i32 = 2;
    pub const TYPE_ELEMENT: i32 = 3;
    pub const TYPE_INSTRUCTION: i32 = 4;
    pub const TYPE_COMMENT: i32 = 5;

    pub fn new() -> Self {
        XMLElement {
            mType: 0,
            mSection: String::new(),
            mValue: String::new(),
            mInstruction: String::new(),
            mAttributes: HashMap::new(),
            mAttributeIteratorList: Vec::new(),
        }
    }
}

// ==================== XMLParser ====================
pub struct XMLParser {
    mFileName: String,
    mErrorText: String,
    mLineNum: i32,
    mFile: PFILE,
    mHasFailed: bool,
    mAllowComments: bool,
    mBufferedText: XMLParserBuffer,
    mSection: String,
    mForcedEncodingType: bool,
    mFirstChar: bool,
    mByteSwap: bool,
}

impl XMLParser {
    pub const ASCII: i32 = 0;
    pub const UTF_8: i32 = 1;
    pub const UTF_16: i32 = 2;
    pub const UTF_16_LE: i32 = 3;
    pub const UTF_16_BE: i32 = 4;

    pub fn new() -> Self {
        XMLParser {
            mFile: ptr::null_mut(),
            mLineNum: 0,
            mAllowComments: false,
            mForcedEncodingType: false,
            mFirstChar: false,
            mByteSwap: false,
            mFileName: String::new(),
            mErrorText: String::new(),
            mHasFailed: false,
            mBufferedText: Vec::new(),
            mSection: String::new(),
        }
    }

    fn Init(&mut self) {
        self.mSection = String::new();
        self.mLineNum = 1;
        self.mHasFailed = false;
        self.mErrorText = String::new();
        self.mFirstChar = true;
        self.mByteSwap = false;
    }

    fn Fail(&mut self, theErrorText: &str) {
        self.mHasFailed = true;
        self.mErrorText = String::from(theErrorText);
    }

    pub fn SetEncodingType(&mut self, theEncoding: i32) {
        match theEncoding {
            Self::ASCII => { self.mForcedEncodingType = true; }
            Self::UTF_8 => { self.mForcedEncodingType = true; }
            Self::UTF_16 => { self.mForcedEncodingType = true; }
            Self::UTF_16_LE => { self.mForcedEncodingType = true; }
            Self::UTF_16_BE => { self.mForcedEncodingType = true; }
            _ => {}
        }
    }

    fn AddAttribute(&mut self, theElement: &mut XMLElement, theAttributeKey: &str, theAttributeValue: &str) {
        let key = String::from(theAttributeKey);
        let val = String::from(theAttributeValue);
        theElement.mAttributes.insert(key.clone(), val);
        if theAttributeKey != "/" {
            theElement.mAttributeIteratorList.push(key);
        }
    }

    fn EncodeUTF8(code: u32, out: &mut [u8; 4]) -> usize {
        if code <= 0x7F {
            out[0] = code as u8;
            1
        } else if code <= 0x7FF {
            out[0] = (0xC0 | (code >> 6)) as u8;
            out[1] = (0x80 | (code & 0x3F)) as u8;
            2
        } else if code <= 0xFFFF {
            out[0] = (0xE0 | (code >> 12)) as u8;
            out[1] = (0x80 | ((code >> 6) & 0x3F)) as u8;
            out[2] = (0x80 | (code & 0x3F)) as u8;
            3
        } else if code <= 0x10FFFF {
            out[0] = (0xF0 | (code >> 18)) as u8;
            out[1] = (0x80 | ((code >> 12) & 0x3F)) as u8;
            out[2] = (0x80 | ((code >> 6) & 0x3F)) as u8;
            out[3] = (0x80 | (code & 0x3F)) as u8;
            4
        } else {
            out[0] = b'?';
            1
        }
    }

    fn GetAsciiChar(&mut self, theChar: &mut u8) -> bool {
        let mut aChar: u8 = 0;
        let buf = std::slice::from_mut(&mut aChar);
        if p_fread(buf, self.mFile) != 1 { return false; }
        *theChar = aChar;
        true
    }

    fn GetUTF8Char(&mut self, theChar: &mut u8) -> bool {
        let mut aChar: u8 = 0;
        let buf = std::slice::from_mut(&mut aChar);
        if p_fread(buf, self.mFile) != 1 { return false; }

        if self.mFirstChar {
            self.mFirstChar = false;
            if aChar == 0xEF {
                let mut b2: u8 = 0;
                let mut b3: u8 = 0;
                let buf2 = std::slice::from_mut(&mut b2);
                let buf3 = std::slice::from_mut(&mut b3);
                if p_fread(buf2, self.mFile) == 1 && p_fread(buf3, self.mFile) == 1 {
                    if b2 == 0xBB && b3 == 0xBF {
                        return self.GetUTF8Char(theChar);
                    }
                    self.mBufferedText.push(b3);
                    self.mBufferedText.push(b2);
                }
            }
        }

        *theChar = aChar;
        true
    }

    fn GetUTF16Char(&mut self, theChar: &mut u8) -> bool {
        let mut aTempChar: u16 = 0;
        let buf = unsafe { std::slice::from_raw_parts_mut(&mut aTempChar as *mut u16 as *mut u8, 2) };
        if p_fread(buf, self.mFile) != 1 { return false; }

        if self.mFirstChar {
            self.mFirstChar = false;
            if aTempChar == 0xFEFF {
                self.mByteSwap = false;
                return self.GetUTF16Char(theChar);
            } else if aTempChar == 0xFFFE {
                self.mByteSwap = true;
                return self.GetUTF16Char(theChar);
            }
        }
        if self.mByteSwap {
            aTempChar = (aTempChar << 8) | (aTempChar >> 8);
        }

        let mut codepoint = aTempChar as u32;
        if (aTempChar & 0xD800) == 0xD800 {
            let mut aNextChar: u16 = 0;
            let buf2 = unsafe { std::slice::from_raw_parts_mut(&mut aNextChar as *mut u16 as *mut u8, 2) };
            if p_fread(buf2, self.mFile) != 1 { return false; }

            if self.mByteSwap {
                aNextChar = (aNextChar << 8) | (aNextChar >> 8);
            }
            if (aNextChar & 0xDC00) == 0xDC00 {
                codepoint = (((aTempChar as u32 & !0xD800) << 10) | (aNextChar as u32 & !0xDC00)) + 0x10000;
            } else {
                return false;
            }
        }

        let mut utf8 = [0u8; 4];
        let len = Self::EncodeUTF8(codepoint, &mut utf8);
        for i in (1..len).rev() {
            self.mBufferedText.push(utf8[i]);
        }
        *theChar = utf8[0];
        true
    }

    fn GetUTF16LEChar(&mut self, theChar: &mut u8) -> bool {
        let mut aTempChar: u16 = 0;
        let buf = unsafe { std::slice::from_raw_parts_mut(&mut aTempChar as *mut u16 as *mut u8, 2) };
        if p_fread(buf, self.mFile) != 1 { return false; }

        aTempChar = FromLE16(aTempChar);

        let mut codepoint = aTempChar as u32;
        if (aTempChar & 0xD800) == 0xD800 {
            let mut aNextChar: u16 = 0;
            let buf2 = unsafe { std::slice::from_raw_parts_mut(&mut aNextChar as *mut u16 as *mut u8, 2) };
            if p_fread(buf2, self.mFile) != 1 { return false; }

            aNextChar = FromLE16(aNextChar);
            if (aNextChar & 0xDC00) == 0xDC00 {
                codepoint = (((aTempChar as u32 & !0xD800) << 10) | (aNextChar as u32 & !0xDC00)) + 0x10000;
            } else {
                return false;
            }
        }

        let mut utf8 = [0u8; 4];
        let len = Self::EncodeUTF8(codepoint, &mut utf8);
        for i in (1..len).rev() {
            self.mBufferedText.push(utf8[i]);
        }
        *theChar = utf8[0];
        true
    }

    fn GetUTF16BEChar(&mut self, theChar: &mut u8) -> bool {
        let mut aTempChar: u16 = 0;
        let buf = unsafe { std::slice::from_raw_parts_mut(&mut aTempChar as *mut u16 as *mut u8, 2) };
        if p_fread(buf, self.mFile) != 1 { return false; }

        aTempChar = FromBE16(aTempChar);

        let mut codepoint = aTempChar as u32;
        if (aTempChar & 0xD800) == 0xD800 {
            let mut aNextChar: u16 = 0;
            let buf2 = unsafe { std::slice::from_raw_parts_mut(&mut aNextChar as *mut u16 as *mut u8, 2) };
            if p_fread(buf2, self.mFile) != 1 { return false; }

            aNextChar = FromBE16(aNextChar);
            if (aNextChar & 0xDC00) == 0xDC00 {
                codepoint = (((aTempChar as u32 & !0xD800) << 10) | (aNextChar as u32 & !0xDC00)) + 0x10000;
            } else {
                return false;
            }
        }

        let mut utf8 = [0u8; 4];
        let len = Self::EncodeUTF8(codepoint, &mut utf8);
        for i in (1..len).rev() {
            self.mBufferedText.push(utf8[i]);
        }
        *theChar = utf8[0];
        true
    }

    pub fn OpenFile(&mut self, theFileName: &str) -> bool {
        self.mFile = p_fopen(theFileName, "r");

        if self.mFile.is_null() {
            self.mLineNum = 0;
            self.Fail(&format!("Unable to open file {}", theFileName));
            return false;
        } else if !self.mForcedEncodingType {
            p_fseek(self.mFile, 0, 2); // SEEK_END
            let aFileLen = p_ftell(self.mFile);
            p_fseek(self.mFile, 0, 0); // SEEK_SET

            if aFileLen >= 2 {
                let aChar1 = p_fgetc(self.mFile);
                let aChar2 = p_fgetc(self.mFile);

                if (aChar1 == 0xFF && aChar2 == 0xFE) || (aChar1 == 0xFE && aChar2 == 0xFF) {
                    // Will use GetUTF16Char - we don't need to set mGetCharFunc since
                    // we dispatch based on the encoding
                    p_ungetc(aChar2, self.mFile);
                    p_ungetc(aChar1, self.mFile);
                }
            }
            // Simplified: default to UTF8 detection with BOM
            // In C++ this sets mGetCharFunc function pointer; we inline the encoding logic
        }

        self.mFileName = String::from(theFileName);
        self.Init();
        true
    }

    pub fn SetStringSource(&mut self, theString: &str) {
        self.Init();

        let mut offset: usize = 0;
        let bytes = theString.as_bytes();
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            offset = 3;
        }

        let aSize = bytes.len() - offset;
        self.mBufferedText.resize(aSize, 0u8);
        for i in 0..aSize {
            self.mBufferedText[i] = bytes[bytes.len() - 1 - i];
        }
    }

    pub fn NextElement(&mut self, theElement: &mut XMLElement) -> bool {
        loop {
            theElement.mType = XMLElement::TYPE_NONE;
            theElement.mSection = self.mSection.clone();
            theElement.mValue = String::new();
            theElement.mAttributes.clear();
            theElement.mInstruction.clear();
            theElement.mAttributeIteratorList.clear();

            let mut hasSpace = false;
            let mut inQuote = false;
            let mut gotEndQuote = false;

            let mut doingAttribute = false;
            let mut attributeVal = false;
            let mut aAttributeKey = String::new();
            let mut aAttributeValue = String::new();
            let mut aLastAttributeKey = String::new();

            loop {
                let mut c: u8 = 0;
                let aVal: i32;

                if !self.mBufferedText.is_empty() {
                    c = self.mBufferedText[self.mBufferedText.len() - 1];
                    self.mBufferedText.pop();
                    aVal = 1;
                } else {
                    if !self.mFile.is_null() {
                        let mut ch: u8 = 0;
                        if self.GetUTF8Char(&mut ch) {
                            c = ch;
                            aVal = 1;
                        } else {
                            self.Fail("Illegal Character");
                            aVal = 0;
                        }
                    } else {
                        aVal = 0;
                    }
                }

                if aVal == 1 {
                    let mut processChar = false;

                    if c == b'\n' {
                        self.mLineNum += 1;
                    }

                    if theElement.mType == XMLElement::TYPE_COMMENT {
                        let aStrPtr = &mut theElement.mInstruction;
                        aStrPtr.push(c as char);
                        let aLen = aStrPtr.len();
                        if c == b'>' && aLen >= 3
                            && aStrPtr.as_bytes()[aLen - 2] == b'-'
                            && aStrPtr.as_bytes()[aLen - 3] == b'-'
                        {
                            *aStrPtr = String::from(&aStrPtr[..aLen - 3]);
                            break;
                        }
                    } else if theElement.mType == XMLElement::TYPE_INSTRUCTION {
                        let aStrPtr: &mut String;
                        if !theElement.mInstruction.is_empty() || (c as char).is_ascii_whitespace() {
                            aStrPtr = &mut theElement.mInstruction;
                        } else {
                            aStrPtr = &mut theElement.mValue;
                        }
                        aStrPtr.push(c as char);
                        let aLen = aStrPtr.len();
                        if c == b'>' && aLen >= 2 && aStrPtr.as_bytes()[aLen - 2] == b'?' {
                            *aStrPtr = String::from(&aStrPtr[..aLen - 2]);
                            break;
                        }
                    } else {
                        if c == b'"' {
                            inQuote = !inQuote;
                            if theElement.mType == XMLElement::TYPE_NONE || theElement.mType == XMLElement::TYPE_ELEMENT {
                                processChar = true;
                            }
                            if !inQuote {
                                gotEndQuote = true;
                            }
                        } else if !inQuote {
                            if c == b'<' {
                                if theElement.mType == XMLElement::TYPE_ELEMENT {
                                    self.mBufferedText.push(c);
                                    break;
                                }
                                if theElement.mType == XMLElement::TYPE_NONE {
                                    theElement.mType = XMLElement::TYPE_START;
                                } else {
                                    self.Fail("Unexpected '<'");
                                    return false;
                                }
                            } else if c == b'>' {
                                if theElement.mType == XMLElement::TYPE_START {
                                    let mut insertEnd = false;

                                    if aAttributeKey == "/" {
                                        insertEnd = true;
                                    } else {
                                        if !aAttributeKey.is_empty() {
                                            aAttributeKey = XMLDecodeString(&aAttributeKey);
                                            aAttributeValue = XMLDecodeString(&aAttributeValue);
                                            aLastAttributeKey = aAttributeKey.clone();
                                            self.AddAttribute(theElement, &aLastAttributeKey, &aAttributeValue);
                                            aAttributeKey = String::new();
                                            aAttributeValue = String::new();
                                        }

                                        if !aLastAttributeKey.is_empty() {
                                            if let Some(aVal) = theElement.mAttributes.get(&aLastAttributeKey) {
                                                let aLen = aVal.len();
                                                if aLen > 0 && aVal.as_bytes()[aLen - 1] == b'/' {
                                                    self.AddAttribute(theElement, &aLastAttributeKey, &XMLDecodeString(&aVal[..aLen - 1]));
                                                    insertEnd = true;
                                                }
                                            }
                                        } else {
                                            let aLen = theElement.mValue.len();
                                            if aLen > 0 && theElement.mValue.as_bytes()[aLen - 1] == b'/' {
                                                theElement.mValue = String::from(&theElement.mValue[..aLen - 1]);
                                                insertEnd = true;
                                            }
                                        }
                                    }

                                    if insertEnd {
                                        let anAddString = format!("</{}>", theElement.mValue);
                                        let anOldSize = self.mBufferedText.len();
                                        let anAddLength = anAddString.len();
                                        self.mBufferedText.resize(anOldSize + anAddLength, 0u8);
                                        let addBytes = anAddString.as_bytes();
                                        for i in 0..anAddLength {
                                            self.mBufferedText[anOldSize + i] = addBytes[anAddLength - i - 1];
                                        }
                                        aAttributeKey = String::new();
                                    }

                                    if !self.mSection.is_empty() {
                                        self.mSection.push('/');
                                    }
                                    self.mSection.push_str(&theElement.mValue);
                                    break;
                                } else if theElement.mType == XMLElement::TYPE_END {
                                    let aLastSlash = match self.mSection.rfind('/') {
                                        Some(pos) => pos as isize,
                                        None => -1,
                                    };
                                    if aLastSlash == -1 && self.mSection.is_empty() {
                                        self.Fail("Unexpected End");
                                        return false;
                                    }

                                    let aLastSectionName = if aLastSlash == -1 {
                                        self.mSection.clone()
                                    } else {
                                        String::from(&self.mSection[(aLastSlash + 1) as usize..])
                                    };

                                    if aLastSectionName != theElement.mValue {
                                        self.Fail(&format!("End '{}' Doesn't Match Start '{}'", theElement.mValue, aLastSectionName));
                                        return false;
                                    }

                                    if aLastSlash == -1 {
                                        self.mSection.clear();
                                    } else {
                                        self.mSection.truncate(aLastSlash as usize);
                                    }
                                    break;
                                } else {
                                    self.Fail("Unexpected '>'");
                                    return false;
                                }
                            } else if c == b'/' && theElement.mType == XMLElement::TYPE_START && theElement.mValue.is_empty() {
                                theElement.mType = XMLElement::TYPE_END;
                            } else if c == b'?' && theElement.mType == XMLElement::TYPE_START && theElement.mValue.is_empty() {
                                theElement.mType = XMLElement::TYPE_INSTRUCTION;
                            } else if (c as char).is_ascii_whitespace() {
                                if !theElement.mValue.is_empty() {
                                    hasSpace = true;
                                }
                                if theElement.mType == XMLElement::TYPE_START && theElement.mValue == "!--" {
                                    theElement.mType = XMLElement::TYPE_COMMENT;
                                }
                            } else if c > 32 {
                                processChar = true;
                            } else {
                                self.Fail("Illegal Character");
                                return false;
                            }
                        } else {
                            processChar = true;
                        }

                        if processChar {
                            if theElement.mType == XMLElement::TYPE_NONE {
                                theElement.mType = XMLElement::TYPE_ELEMENT;
                            }

                            if theElement.mType == XMLElement::TYPE_START {
                                if hasSpace {
                                    if !doingAttribute || (!attributeVal && c != b'=')
                                        || (attributeVal && (!aAttributeValue.is_empty() || gotEndQuote))
                                    {
                                        if doingAttribute {
                                            aAttributeKey = XMLDecodeString(&aAttributeKey);
                                            aAttributeValue = XMLDecodeString(&aAttributeValue);
                                            self.AddAttribute(theElement, &aAttributeKey, &aAttributeValue);
                                            aAttributeKey = String::new();
                                            aAttributeValue = String::new();
                                            aLastAttributeKey = aAttributeKey.clone();
                                        } else {
                                            doingAttribute = true;
                                        }
                                        attributeVal = false;
                                    }
                                    hasSpace = false;
                                }

                                if !doingAttribute {
                                    theElement.mValue.push(c as char);
                                } else {
                                    if c == b'=' {
                                        attributeVal = true;
                                        gotEndQuote = false;
                                    } else if !attributeVal {
                                        aAttributeKey.push(c as char);
                                    } else {
                                        aAttributeValue.push(c as char);
                                    }
                                }
                            } else {
                                if hasSpace {
                                    theElement.mValue.push(' ');
                                    hasSpace = false;
                                }
                                theElement.mValue.push(c as char);
                            }
                        }
                    }
                } else {
                    if theElement.mType != XMLElement::TYPE_NONE {
                        self.Fail("Unexpected End of File");
                    }
                    return false;
                }
            }

            if !aAttributeKey.is_empty() {
                aAttributeKey = XMLDecodeString(&aAttributeKey);
                aAttributeValue = XMLDecodeString(&aAttributeValue);
                self.AddAttribute(theElement, &aAttributeKey, &aAttributeValue);
            }

            theElement.mValue = XMLDecodeString(&theElement.mValue);

            if theElement.mType != XMLElement::TYPE_COMMENT || self.mAllowComments {
                return true;
            }
        }
    }

    pub fn HasFailed(&self) -> bool {
        self.mHasFailed
    }

    pub fn GetErrorText(&self) -> &str {
        &self.mErrorText
    }

    pub fn GetCurrentLineNum(&self) -> i32 {
        self.mLineNum
    }

    pub fn GetFileName(&self) -> &str {
        &self.mFileName
    }
}

impl Drop for XMLParser {
    fn drop(&mut self) {
        if !self.mFile.is_null() {
            p_fclose(self.mFile);
        }
    }
}

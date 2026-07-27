// [TRANSLATION_NOTE]: ResourceManager.h + ResourceManager.cpp -> Rust
// 资源管理器 - 使用 *mut c_void 匹配 C++ 的 void* 风格

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

use crate::sexy_app_framework::graphics::graphics::{Font, Image};
use crate::sexy_app_framework::misc::xml_parser::{XMLElement, XMLParser, XMLParamMap};

// ==================== AnimInfo ====================
#[derive(Clone)]
pub struct AnimInfo {
    pub mAnimType: i32,
    pub mFrameDelay: i32,
    pub mPerFrameDelay: Vec<i32>,
    pub mFrameMap: Vec<i32>,
}

impl AnimInfo {
    pub const AnimType_None: i32 = 0;
    pub const AnimType_Once: i32 = 1;
    pub const AnimType_Loop: i32 = 2;
    pub const AnimType_PingPong: i32 = 3;

    pub fn new() -> Self {
        AnimInfo {
            mAnimType: Self::AnimType_None,
            mFrameDelay: 0,
            mPerFrameDelay: Vec::new(),
            mFrameMap: Vec::new(),
        }
    }

    pub fn Compute(&mut self, _aNumCels: i32, _aBeginDelay: i32, _anEndDelay: i32) {}
}

// ==================== 资源类型枚举 ====================
#[derive(Clone)]
pub enum ResType {
    Image(ImageRes),
    Sound(SoundRes),
    Font(FontRes),
}

#[derive(Clone)]
pub struct BaseResInfo {
    pub mType: i32,
    pub mId: String,
    pub mResGroup: String,
    pub mPath: String,
    pub mXMLAttributes: XMLParamMap,
    pub mFromProgram: bool,
}

impl BaseResInfo {
    pub const ResType_Image: i32 = 0;
    pub const ResType_Sound: i32 = 1;
    pub const ResType_Font: i32 = 2;
}

#[derive(Clone)]
pub struct ImageRes {
    pub base: BaseResInfo,
    pub mImage: *mut Image,
    pub mAlphaImage: String,
    pub mAlphaGridImage: String,
    pub mVariant: String,
    pub mAutoFindAlpha: bool,
    pub mPalletize: bool,
    pub mA4R4G4B4: bool,
    pub mA8R8G8B8: bool,
    pub mDDSurface: bool,
    pub mPurgeBits: bool,
    pub mMinimizeSubdivisions: bool,
    pub mRows: i32,
    pub mCols: i32,
    pub mAlphaColor: u32,
    pub mAnimInfo: AnimInfo,
}

#[derive(Clone)]
pub struct SoundRes {
    pub base: BaseResInfo,
    pub mSoundId: isize,
    pub mVolume: f64,
    pub mPanning: i32,
}

#[derive(Clone)]
pub struct FontRes {
    pub base: BaseResInfo,
    pub mFont: *mut Font,
    pub mImage: *mut Image,
    pub mImagePath: String,
    pub mTags: String,
    pub mSysFont: bool,
    pub mBold: bool,
    pub mItalic: bool,
    pub mUnderline: bool,
    pub mShadow: bool,
    pub mSize: i32,
}

// ==================== 异常 ====================
#[derive(Clone, Debug)]
pub struct ResourceManagerException {
    pub what: String,
}

impl ResourceManagerException {
    pub fn new(the_what: &str) -> Self {
        ResourceManagerException { what: String::from(the_what) }
    }
}

// ==================== 资源管理器 ====================
pub struct ResourceManager {
    pub mLoadedGroups: Vec<String>,
    pub mImageMap: HashMap<String, ImageRes>,
    pub mSoundMap: HashMap<String, SoundRes>,
    pub mFontMap: HashMap<String, FontRes>,
    pub mXMLParser: Option<Box<XMLParser>>,
    pub mError: String,
    pub mHasFailed: bool,
    pub mApp: *mut std::ffi::c_void,
    pub mCurResGroup: String,
    pub mDefaultPath: String,
    pub mDefaultIdPrefix: String,
    pub mAllowMissingProgramResources: bool,
    pub mAllowAlreadyDefinedResources: bool,
    pub mHadAlreadyDefinedError: bool,
    pub mResGroupMap: HashMap<String, Vec<String>>,
}

impl ResourceManager {
    pub fn new(theApp: *mut std::ffi::c_void) -> Self {
        ResourceManager {
            mApp: theApp,
            mHasFailed: false,
            mXMLParser: None,
            mAllowMissingProgramResources: false,
            mAllowAlreadyDefinedResources: false,
            mLoadedGroups: Vec::new(),
            mImageMap: HashMap::new(),
            mSoundMap: HashMap::new(),
            mFontMap: HashMap::new(),
            mError: String::new(),
            mCurResGroup: String::new(),
            mDefaultPath: String::new(),
            mDefaultIdPrefix: String::new(),
            mHadAlreadyDefinedError: false,
            mResGroupMap: HashMap::new(),
        }
    }

    fn fail(&mut self, text: &str) -> bool {
        if self.mHasFailed { return false; }
        self.mHasFailed = true;
        self.mError = String::from(text);
        false
    }

    fn fail_with_parser(&mut self, text: &str) -> bool {
        if self.mHasFailed { return false; }
        self.mHasFailed = true;

        let mut err = String::from(text);
        if let Some(ref parser) = self.mXMLParser {
            let line = parser.GetCurrentLineNum();
            if line > 0 { err += &format!(" on Line {}", line); }
            let fname = parser.GetFileName();
            if !fname.is_empty() { err += &format!(" in File '{}'", fname); }
        }
        self.mError = err;
        false
    }

    pub fn IsGroupLoaded(&self, group: &str) -> bool {
        self.mLoadedGroups.contains(&String::from(group))
    }

    pub fn DeleteResources(&mut self, group: &str) {
        self.mImageMap.retain(|_, v| v.base.mResGroup != group);
        self.mSoundMap.retain(|_, v| v.base.mResGroup != group);
        self.mFontMap.retain(|_, v| v.base.mResGroup != group);
        self.mLoadedGroups.retain(|g| g != group);
    }

    pub fn GetErrorText(&self) -> &str { &self.mError }
    pub fn HadError(&self) -> bool { self.mHasFailed }

    // ==================== XML 解析 ====================

    fn parse_common(&mut self, el: &XMLElement, res: &mut BaseResInfo) -> bool {
        self.mHadAlreadyDefinedError = false;

        let a_path = match el.mAttributes.get("path") {
            Some(p) => p.clone(),
            None => return self.fail("No path specified."),
        };

        res.mXMLAttributes = el.mAttributes.clone();
        res.mFromProgram = false;

        if a_path.starts_with('!') {
            res.mPath = a_path.clone();
            if a_path == "!program" { res.mFromProgram = true; }
        } else {
            res.mPath = self.mDefaultPath.clone() + &a_path;
        }

        let an_id = match el.mAttributes.get("id") {
            Some(id) => self.mDefaultIdPrefix.clone() + id,
            None => self.mDefaultIdPrefix.clone() + &Self::get_file_name(&res.mPath),
        };

        res.mResGroup = self.mCurResGroup.clone();
        res.mId = an_id;
        true
    }

    fn parse_sound(&mut self, el: &XMLElement) -> bool {
        let mut res = SoundRes {
            base: BaseResInfo { mType: BaseResInfo::ResType_Sound, ..BaseResInfo::default() },
            mSoundId: -1, mVolume: -1.0, mPanning: 0,
        };

        if !self.parse_common(el, &mut res.base) { return false; }
        let id = res.base.mId.clone();

        if self.mSoundMap.contains_key(&id) {
            return self.fail("Resource already defined.");
        }

        if let Some(v) = el.mAttributes.get("volume") {
            if let Ok(vf) = v.parse::<f64>() { res.mVolume = vf; }
        }
        if let Some(v) = el.mAttributes.get("pan") {
            if let Ok(vi) = v.parse::<i32>() { res.mPanning = vi; }
        }

        self.mSoundMap.insert(id.clone(), res);
        self.mResGroupMap.entry(self.mCurResGroup.clone()).or_insert_with(Vec::new).push(id);
        true
    }

    fn parse_image(&mut self, el: &XMLElement) -> bool {
        let mut res = ImageRes {
            base: BaseResInfo { mType: BaseResInfo::ResType_Image, ..BaseResInfo::default() },
            mImage: std::ptr::null_mut(),
            mAlphaImage: String::new(),
            mAlphaGridImage: String::new(),
            mVariant: String::new(),
            mAutoFindAlpha: true,
            mPalletize: true,
            mA4R4G4B4: false,
            mA8R8G8B8: false,
            mDDSurface: false,
            mPurgeBits: false,
            mMinimizeSubdivisions: false,
            mRows: 1,
            mCols: 1,
            mAlphaColor: 0xFFFFFF,
            mAnimInfo: AnimInfo::new(),
        };

        if !self.parse_common(el, &mut res.base) { return false; }
        let id = res.base.mId.clone();

        if self.mImageMap.contains_key(&id) {
            return self.fail("Resource already defined.");
        }

        res.mPalletize = el.mAttributes.get("nopal").is_none();
        res.mA4R4G4B4 = el.mAttributes.contains_key("a4r4g4b4");
        res.mDDSurface = el.mAttributes.contains_key("ddsurface");
        res.mAutoFindAlpha = el.mAttributes.get("noalpha").is_none();

        if let Some(ai) = el.mAttributes.get("alphaimage") {
            res.mAlphaImage = self.mDefaultPath.clone() + ai;
        }
        if let Some(ac) = el.mAttributes.get("alphacolor") {
            if let Ok(c) = u32::from_str_radix(ac.trim_start_matches("0x"), 16) { res.mAlphaColor = c; }
        }
        if let Some(v) = el.mAttributes.get("variant") { res.mVariant = v.clone(); }
        if let Some(ag) = el.mAttributes.get("alphagrid") {
            res.mAlphaGridImage = self.mDefaultPath.clone() + ag;
        }
        res.mRows = el.mAttributes.get("rows").and_then(|v| v.parse().ok()).unwrap_or(1);
        res.mCols = el.mAttributes.get("cols").and_then(|v| v.parse().ok()).unwrap_or(1);

        if let Some(at) = el.mAttributes.get("anim") {
            let atype = match at.as_str() {
                "once" => AnimInfo::AnimType_Once,
                "loop" => AnimInfo::AnimType_Loop,
                "pingpong" => AnimInfo::AnimType_PingPong,
                _ => AnimInfo::AnimType_None,
            };
            res.mAnimInfo.mAnimType = atype;
            if atype != AnimInfo::AnimType_None {
                if let Some(fd) = el.mAttributes.get("framedelay") {
                    if let Ok(v) = fd.parse() { res.mAnimInfo.mFrameDelay = v; }
                }
                res.mAnimInfo.Compute(std::cmp::max(res.mRows, res.mCols), 0, 0);
            }
        }

        self.mImageMap.insert(id.clone(), res);
        self.mResGroupMap.entry(self.mCurResGroup.clone()).or_insert_with(Vec::new).push(id);
        true
    }

    fn parse_font(&mut self, el: &XMLElement) -> bool {
        let mut res = FontRes {
            base: BaseResInfo { mType: BaseResInfo::ResType_Font, ..BaseResInfo::default() },
            mFont: std::ptr::null_mut(),
            mImage: std::ptr::null_mut(),
            mImagePath: String::new(),
            mTags: String::new(),
            mSysFont: false,
            mBold: false,
            mItalic: false,
            mUnderline: false,
            mShadow: false,
            mSize: 0,
        };

        if !self.parse_common(el, &mut res.base) { return false; }
        let id = res.base.mId.clone();

        if self.mFontMap.contains_key(&id) {
            return self.fail("Resource already defined.");
        }

        if let Some(img) = el.mAttributes.get("image") { res.mImagePath = img.clone(); }
        if let Some(tags) = el.mAttributes.get("tags") { res.mTags = tags.clone(); }

        if res.base.mPath.starts_with("!sys:") {
            res.mSysFont = true;
            res.base.mPath = res.base.mPath[5..].to_string();
            res.mSize = match el.mAttributes.get("size") {
                Some(s) => s.parse().unwrap_or(0),
                None => return self.fail("SysFont needs point size"),
            };
            if res.mSize <= 0 { return self.fail("SysFont needs point size"); }
            res.mBold = el.mAttributes.contains_key("bold");
            res.mItalic = el.mAttributes.contains_key("italic");
            res.mShadow = el.mAttributes.contains_key("shadow");
            res.mUnderline = el.mAttributes.contains_key("underline");
        }

        self.mFontMap.insert(id.clone(), res);
        self.mResGroupMap.entry(self.mCurResGroup.clone()).or_insert_with(Vec::new).push(id);
        true
    }

    fn parse_set_defaults(&mut self, el: &XMLElement) -> bool {
        if let Some(p) = el.mAttributes.get("path") {
            self.mDefaultPath = Self::remove_trailing_slash(p) + "/";
        }
        if let Some(p) = el.mAttributes.get("idprefix") {
            self.mDefaultIdPrefix = Self::remove_trailing_slash(p);
        }
        true
    }

    fn parse_resources_block(&mut self) -> bool {
        loop {
            let mut xml = XMLElement::new();
            if !self.mXMLParser.as_mut().unwrap().NextElement(&mut xml) { return false; }

            if xml.mType == XMLElement::TYPE_START {
                match xml.mValue.as_str() {
                    "Image" => {
                        if !self.parse_image(&xml) { return false; }
                        let mut end = XMLElement::new();
                        if !self.mXMLParser.as_mut().unwrap().NextElement(&mut end) { return false; }
                        if end.mType != XMLElement::TYPE_END { return self.fail_with_parser("Unexpected element found."); }
                    }
                    "Sound" => {
                        if !self.parse_sound(&xml) { return false; }
                        let mut end = XMLElement::new();
                        if !self.mXMLParser.as_mut().unwrap().NextElement(&mut end) { return false; }
                        if end.mType != XMLElement::TYPE_END { return self.fail_with_parser("Unexpected element found."); }
                    }
                    "Font" => {
                        if !self.parse_font(&xml) { return false; }
                        let mut end = XMLElement::new();
                        if !self.mXMLParser.as_mut().unwrap().NextElement(&mut end) { return false; }
                        if end.mType != XMLElement::TYPE_END { return self.fail_with_parser("Unexpected element found."); }
                    }
                    "SetDefaults" => {
                        if !self.parse_set_defaults(&xml) { return false; }
                        let mut end = XMLElement::new();
                        if !self.mXMLParser.as_mut().unwrap().NextElement(&mut end) { return false; }
                        if end.mType != XMLElement::TYPE_END { return self.fail_with_parser("Unexpected element found."); }
                    }
                    _ => return self.fail_with_parser(&format!("Invalid Section '{}'", xml.mValue)),
                }
            } else if xml.mType == XMLElement::TYPE_ELEMENT {
                return self.fail_with_parser(&format!("Element Not Expected '{}'", xml.mValue));
            } else if xml.mType == XMLElement::TYPE_END {
                return true;
            }
        }
    }

    fn do_parse(&mut self) -> bool {
        let parser = self.mXMLParser.as_ref().unwrap();
        if !parser.HasFailed() {
            loop {
                let mut xml = XMLElement::new();
                if !self.mXMLParser.as_mut().unwrap().NextElement(&mut xml) { break; }

                if xml.mType == XMLElement::TYPE_START {
                    if xml.mValue == "Resources" {
                        self.mCurResGroup = match xml.mAttributes.get("id") {
                            Some(id) => id.clone(),
                            None => { self.fail_with_parser("No id specified."); break; }
                        };
                        self.mResGroupMap.entry(self.mCurResGroup.clone()).or_insert_with(Vec::new);
                        if !self.parse_resources_block() { break; }
                    } else {
                        self.fail_with_parser(&format!("Invalid Section '{}'", xml.mValue));
                        break;
                    }
                }
            }
        }

        let has_err = self.mXMLParser.as_ref().map(|p| p.HasFailed()).unwrap_or(false);
        if has_err {
            let err_text = self.mXMLParser.as_ref().unwrap().GetErrorText().to_string();
            self.fail_with_parser(&err_text);
        }

        self.mXMLParser = None;
        !self.mHasFailed
    }

    pub fn ParseResourcesFile(&mut self, filename: &str) -> bool {
        let mut parser = Box::new(XMLParser::new());
        if !parser.OpenFile(filename) {
            return self.fail(&format!("Resource file not found: {}", filename));
        }

        let mut xml = XMLElement::new();
        while !parser.HasFailed() {
            if !parser.NextElement(&mut xml) {
                self.fail_with_parser(parser.GetErrorText());
                break;
            }
            if xml.mType == XMLElement::TYPE_START {
                if xml.mValue != "ResourceManifest" { break; }
                self.mXMLParser = Some(parser);
                return self.do_parse();
            }
        }

        self.fail_with_parser("Expecting ResourceManifest tag");
        self.mXMLParser = Some(parser);
        self.do_parse()
    }

    pub fn ReparseResourcesFile(&mut self, filename: &str) -> bool {
        let old = self.mAllowAlreadyDefinedResources;
        self.mAllowAlreadyDefinedResources = true;
        let r = self.ParseResourcesFile(filename);
        self.mAllowAlreadyDefinedResources = old;
        r
    }

    // ==================== 资源获取 ====================

    pub fn GetImage(&self, _id: &str) -> *mut Image {
        std::ptr::null_mut()
    }

    pub fn GetSound(&self, _id: &str) -> isize { -1 }

    pub fn GetFont(&self, _id: &str) -> *mut Font {
        std::ptr::null_mut()
    }

    pub fn GetImageThrow(&self, id: &str) -> Result<*mut Image, ResourceManagerException> {
        let img = self.GetImage(id);
        if img.is_null() {
            Err(ResourceManagerException::new(&format!("Image not found: {}", id)))
        } else {
            Ok(img)
        }
    }

    pub fn GetSoundThrow(&self, id: &str) -> Result<isize, ResourceManagerException> {
        let snd = self.GetSound(id);
        if snd < 0 {
            Err(ResourceManagerException::new(&format!("Sound not found: {}", id)))
        } else {
            Ok(snd)
        }
    }

    pub fn GetFontThrow(&self, id: &str) -> Result<*mut Font, ResourceManagerException> {
        let fnt = self.GetFont(id);
        if fnt.is_null() {
            Err(ResourceManagerException::new(&format!("Font not found: {}", id)))
        } else {
            Ok(fnt)
        }
    }

    pub fn ReplaceImage(&mut self, _id: &str, _img: *mut Image) -> bool { false }
    pub fn ReplaceSound(&mut self, _id: &str, _snd: isize) -> bool { false }
    pub fn ReplaceFont(&mut self, _id: &str, _fnt: *mut Font) -> bool { false }

    // ==================== 工具方法 ====================

    fn get_file_name(path: &str) -> String {
        let p = path.replace('\\', "/");
        match p.rfind('/') {
            Some(pos) => p[pos + 1..].to_string(),
            None => p,
        }
    }

    fn remove_trailing_slash(path: &str) -> String {
        let mut s = String::from(path);
        while s.ends_with('/') || s.ends_with('\\') { s.pop(); }
        s
    }
}

impl Default for BaseResInfo {
    fn default() -> Self {
        BaseResInfo {
            mType: 0,
            mId: String::new(),
            mResGroup: String::new(),
            mPath: String::new(),
            mXMLAttributes: HashMap::new(),
            mFromProgram: false,
        }
    }
}

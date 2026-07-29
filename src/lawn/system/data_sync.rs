// [TRANSLATION_NOTE]: DataSync.h + DataSync.cpp -> Rust
// 数据读写与同步模块：DataReader, DataWriter, DataSync
// C++ 中使用的 FILE* / fcaseopen 替换为 std::fs::File / 标准路径处理
// 小端转换使用标准库的 from_le/to_le 方法

use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct DataReaderException;

pub struct DataReader {
    m_file: Option<File>,
    m_data: Vec<u8>,
    m_data_pos: usize,
    m_own_data: bool,
}

impl DataReader {
    pub fn new() -> Self {
        DataReader {
            m_file: None,
            m_data: Vec::new(),
            m_data_pos: 0,
            m_own_data: false,
        }
    }

    pub fn open_file(&mut self, the_file_name: &str) -> bool {
        match File::open(the_file_name) {
            Ok(f) => {
                self.m_file = Some(f);
                true
            }
            Err(_) => false,
        }
    }

    pub fn open_memory(&mut self, the_data: Vec<u8>, take_ownership: bool) {
        self.close_file();
        self.m_data = the_data;
        self.m_data_pos = 0;
        self.m_own_data = take_ownership;
    }

    fn close_file(&mut self) {
        self.m_file = None;
    }

    pub fn close(&mut self) {
        self.close_file();
    }

    fn read_bytes_into(&mut self, buf: &mut [u8]) {
        if !self.m_data.is_empty() {
            let end = self.m_data_pos + buf.len();
            if end > self.m_data.len() {
                panic!("DataReaderException");
            }
            buf.copy_from_slice(&self.m_data[self.m_data_pos..end]);
            self.m_data_pos = end;
        } else if let Some(ref mut f) = self.m_file {
            f.read_exact(buf).expect("DataReaderException: read failed");
        } else {
            panic!("DataReaderException: no data source");
        }
    }

    pub fn read_bytes(&mut self, the_num_bytes: u32) -> Vec<u8> {
        let mut buf = vec![0u8; the_num_bytes as usize];
        if !self.m_data.is_empty() {
            let end = self.m_data_pos + the_num_bytes as usize;
            if end > self.m_data.len() {
                panic!("DataReaderException");
            }
            buf.copy_from_slice(&self.m_data[self.m_data_pos..end]);
            self.m_data_pos = end;
        } else if let Some(ref mut f) = self.m_file {
            f.read_exact(&mut buf).expect("DataReaderException: read failed");
        } else {
            panic!("DataReaderException: no data source");
        }
        buf
    }

    pub fn rewind(&mut self, the_num_bytes: u32) {
        let num = the_num_bytes.min(self.m_data_pos as u32);
        self.m_data_pos -= num as usize;
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.read_bytes_into(&mut buf);
        u16::from_le_bytes(buf)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_bytes_into(&mut buf);
        u32::from_le_bytes(buf)
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.read_bytes_into(&mut buf);
        u64::from_le_bytes(buf)
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.read_bytes_into(&mut buf);
        buf[0]
    }

    pub fn read_bool(&mut self) -> bool {
        let mut buf = [0u8; 1];
        self.read_bytes_into(&mut buf);
        buf[0] != 0
    }

    pub fn read_float(&mut self) -> f32 {
        let raw = self.read_u32();
        f32::from_bits(raw)
    }

    pub fn read_double(&mut self) -> f64 {
        let raw = self.read_u64();
        f64::from_bits(raw)
    }

    pub fn read_string(&mut self) -> String {
        let a_str_len = self.read_u16() as usize;
        let bytes = self.read_bytes(a_str_len as u32);
        String::from_utf8_lossy(&bytes).to_string()
    }
}

pub struct DataWriter {
    m_file: Option<File>,
    m_data: Vec<u8>,
    m_capacity: usize,
}

impl DataWriter {
    pub fn new() -> Self {
        DataWriter {
            m_file: None,
            m_data: Vec::new(),
            m_capacity: 0,
        }
    }

    pub fn open_file(&mut self, the_file_name: &str) -> bool {
        match File::create(the_file_name) {
            Ok(f) => {
                self.m_file = Some(f);
                true
            }
            Err(_) => false,
        }
    }

    pub fn close(&mut self) {
        self.m_file = None;
    }

    pub fn write_to_file(&mut self, the_file_name: &str) -> bool {
        if let Ok(mut f) = File::create(the_file_name) {
            f.write_all(&self.m_data).is_ok()
        } else {
            false
        }
    }

    fn ensure_capacity(&mut self, the_num_bytes: u32) {
        let needed = self.m_data.len() + the_num_bytes as usize;
        while self.m_capacity < needed {
            if self.m_capacity == 0 {
                self.m_capacity = 32;
            } else {
                self.m_capacity <<= 1;
            }
        }
        self.m_data.reserve(self.m_capacity - self.m_data.len());
    }

    pub fn open_memory(&mut self, the_reserve_amount: u32) {
        self.close();
        let reserve = std::cmp::max(the_reserve_amount, 32) as usize;
        self.m_data = Vec::with_capacity(reserve);
        self.m_capacity = reserve;
    }

    pub fn write_bytes(&mut self, the_data: &[u8]) {
        if !self.m_data.is_empty() || self.m_file.is_none() {
            self.ensure_capacity(the_data.len() as u32);
            self.m_data.extend_from_slice(the_data);
        } else if let Some(ref mut f) = self.m_file {
            let _ = f.write_all(the_data);
        }
    }

    pub fn write_u32(&mut self, the_u32: u32) {
        self.write_bytes(&the_u32.to_le_bytes());
    }

    pub fn write_u64(&mut self, the_u64: u64) {
        self.write_bytes(&the_u64.to_le_bytes());
    }

    pub fn write_u16(&mut self, the_u16: u16) {
        self.write_bytes(&the_u16.to_le_bytes());
    }

    pub fn write_u8(&mut self, the_u8: u8) {
        self.write_bytes(&[the_u8]);
    }

    pub fn write_bool(&mut self, the_bool: bool) {
        self.write_bytes(&[the_bool as u8]);
    }

    pub fn write_float(&mut self, the_float: f32) {
        self.write_u32(the_float.to_bits());
    }

    pub fn write_double(&mut self, the_double: f64) {
        self.write_u64(the_double.to_bits());
    }

    pub fn write_string(&mut self, the_str: &str) {
        let a_str_len = the_str.len() as u16;
        self.write_u16(a_str_len);
        self.write_bytes(the_str.as_bytes());
    }

    pub fn get_pos(&self) -> u32 {
        self.m_data.len() as u32
    }

    pub fn get_data_ptr(&self) -> *const u8 {
        self.m_data.as_ptr()
    }

    pub fn get_data_len(&self) -> u32 {
        self.m_data.len() as u32
    }

    pub fn into_data(self) -> Vec<u8> {
        self.m_data
    }
}

pub struct DataSync {
    m_reader: Option<*mut DataReader>,
    m_writer: Option<*mut DataWriter>,
    m_version: i32,
}

impl DataSync {
    pub fn from_reader(the_reader: &mut DataReader) -> Self {
        DataSync {
            m_reader: Some(the_reader as *mut DataReader),
            m_writer: None,
            m_version: 0,
        }
    }

    pub fn from_writer(the_writer: &mut DataWriter) -> Self {
        DataSync {
            m_reader: None,
            m_writer: Some(the_writer as *mut DataWriter),
            m_version: 0,
        }
    }

    pub fn set_reader(&mut self, the_reader: &mut DataReader) {
        self.m_reader = Some(the_reader as *mut DataReader);
    }

    pub fn set_writer(&mut self, the_writer: &mut DataWriter) {
        self.m_writer = Some(the_writer as *mut DataWriter);
    }

    pub fn get_version(&self) -> i32 {
        self.m_version
    }

    pub fn set_version(&mut self, the_version: i32) {
        self.m_version = the_version;
    }

    pub fn is_reader(&self) -> bool {
        self.m_reader.is_some()
    }

    fn get_reader(&self) -> &mut DataReader {
        unsafe { &mut *self.m_reader.unwrap() }
    }

    fn get_writer(&self) -> &mut DataWriter {
        unsafe { &mut *self.m_writer.unwrap() }
    }

    pub fn sync_bytes(&mut self, the_data: &mut [u8]) {
        if self.m_reader.is_some() {
            self.get_reader().read_bytes_into(the_data);
        } else {
            self.get_writer().write_bytes(the_data);
        }
    }

    pub fn sync_u64(&mut self, the_num: &mut u64) {
        if self.m_reader.is_some() {
            *the_num = self.get_reader().read_u64();
        } else {
            self.get_writer().write_u64(*the_num);
        }
    }

    pub fn sync_u32(&mut self, the_num: &mut u32) {
        if self.m_reader.is_some() {
            *the_num = self.get_reader().read_u32();
        } else {
            self.get_writer().write_u32(*the_num);
        }
    }

    pub fn sync_u32_from_i32(&mut self, the_num: &mut i32) {
        let mut a_num = *the_num as u32;
        self.sync_u32(&mut a_num);
        if self.m_reader.is_some() {
            *the_num = a_num as i32;
        }
    }

    pub fn sync_u32_from_u8(&mut self, the_num: &mut u8) {
        let mut a_num = *the_num as u32;
        self.sync_u32(&mut a_num);
        if self.m_reader.is_some() {
            *the_num = a_num as u8;
        }
    }

    pub fn sync_u16(&mut self, the_num: &mut u16) {
        if self.m_reader.is_some() {
            *the_num = self.get_reader().read_u16();
        } else {
            self.get_writer().write_u16(*the_num);
        }
    }

    pub fn sync_u16_from_i32(&mut self, the_num: &mut i32) {
        let mut a_num = *the_num as u16;
        self.sync_u16(&mut a_num);
        if self.m_reader.is_some() {
            *the_num = a_num as i32;
        }
    }

    pub fn sync_u16_from_u8(&mut self, the_num: &mut u8) {
        let mut a_num = *the_num as u16;
        self.sync_u16(&mut a_num);
        if self.m_reader.is_some() {
            *the_num = a_num as u8;
        }
    }

    pub fn sync_u8(&mut self, the_char: &mut u8) {
        if self.m_reader.is_some() {
            *the_char = self.get_reader().read_u8();
        } else {
            self.get_writer().write_u8(*the_char);
        }
    }

    pub fn sync_bool(&mut self, the_bool: &mut bool) {
        if self.m_reader.is_some() {
            *the_bool = self.get_reader().read_bool();
        } else {
            self.get_writer().write_bool(*the_bool);
        }
    }

    pub fn sync_float(&mut self, the_float: &mut f32) {
        if self.m_reader.is_some() {
            *the_float = self.get_reader().read_float();
        } else {
            self.get_writer().write_float(*the_float);
        }
    }

    pub fn sync_double(&mut self, the_double: &mut f64) {
        if self.m_reader.is_some() {
            *the_double = self.get_reader().read_double();
        } else {
            self.get_writer().write_double(*the_double);
        }
    }

    pub fn sync_string(&mut self, the_str: &mut String) {
        if self.m_reader.is_some() {
            *the_str = self.get_reader().read_string();
        } else {
            self.get_writer().write_string(the_str);
        }
    }
}

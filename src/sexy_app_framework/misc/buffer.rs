// [TRANSLATION_NOTE]: Buffer.h + Buffer.cpp -> Rust
// SexyAppFramework 的 Buffer 类，支持位级读写、WebString 编解码、CRC32 等

use crate::sexy_app_framework::common::Uchar;
use std::cell::Cell;

pub type ByteVector = Vec<Uchar>;

pub struct Buffer {
    pub m_data: ByteVector,
    pub m_data_bit_size: i32,
    pub m_read_bit_pos: Cell<i32>,
    pub m_write_bit_pos: Cell<i32>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            m_data: ByteVector::new(),
            m_data_bit_size: 0,
            m_read_bit_pos: Cell::new(0),
            m_write_bit_pos: Cell::new(0),
        }
    }

    pub fn seek_front(&self) {
        self.m_read_bit_pos.set(0);
        self.m_write_bit_pos.set(0);
    }

    pub fn clear(&mut self) {
        self.m_data.clear();
        self.m_data_bit_size = 0;
        self.m_read_bit_pos.set(0);
        self.m_write_bit_pos.set(0);
    }

    pub fn from_web_string(&mut self, the_string: &str) {
        self.clear();
        let bytes = the_string.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                let hi = hex_val(bytes[i + 1]);
                let lo = hex_val(bytes[i + 2]);
                if hi >= 0 && lo >= 0 {
                    self.write_byte((hi << 4 | lo) as Uchar);
                    i += 3;
                    continue;
                }
            }
            self.write_byte(bytes[i] as Uchar);
            i += 1;
        }
    }

    pub fn write_byte(&mut self, the_byte: Uchar) {
        self.m_data.push(the_byte);
        self.m_data_bit_size += 8;
    }

    pub fn write_num_bits(&mut self, the_num: i32, the_bits: i32) {
        if the_bits <= 0 {
            return;
        }
        let a_num = the_num;
        let mut a_bit_pos = self.m_write_bit_pos.get();
        for _ in 0..the_bits {
            let a_byte_index = (a_bit_pos >> 3) as usize;
            let a_bit_index = a_bit_pos & 7;
            while self.m_data.len() <= a_byte_index {
                self.m_data.push(0);
            }
            self.m_data[a_byte_index] |= (((a_num >> (the_bits - 1)) & 1) as Uchar) << (7 - a_bit_index);
            a_bit_pos += 1;
        }
        self.m_write_bit_pos.set(a_bit_pos);
        if a_bit_pos > self.m_data_bit_size {
            self.m_data_bit_size = a_bit_pos;
        }
    }

    pub fn get_bits_required(the_num: i32, is_signed: bool) -> i32 {
        if the_num == 0 {
            return if is_signed { 1 } else { 0 };
        }
        let mut a_num = if is_signed && the_num < 0 { -the_num } else { the_num };
        let mut a_bits = if is_signed { 1 } else { 0 };
        while a_num != 0 {
            a_num >>= 1;
            a_bits += 1;
        }
        a_bits
    }

    pub fn write_boolean(&mut self, the_bool: bool) {
        self.write_byte(if the_bool { 1 } else { 0 });
    }

    pub fn write_short(&mut self, the_short: i16) {
        self.write_byte((the_short & 0xFF) as Uchar);
        self.write_byte(((the_short >> 8) & 0xFF) as Uchar);
    }

    pub fn write_u32(&mut self, the_value: u32) {
        self.write_byte((the_value & 0xFF) as Uchar);
        self.write_byte(((the_value >> 8) & 0xFF) as Uchar);
        self.write_byte(((the_value >> 16) & 0xFF) as Uchar);
        self.write_byte(((the_value >> 24) & 0xFF) as Uchar);
    }

    pub fn write_i32(&mut self, the_value: i32) {
        self.write_u32(the_value as u32);
    }

    pub fn write_string(&mut self, the_string: &str) {
        let bytes = the_string.as_bytes();
        self.write_short(bytes.len() as i16);
        for &b in bytes {
            self.write_byte(b);
        }
    }

    pub fn write_line(&mut self, the_string: &str) {
        for &b in the_string.as_bytes() {
            self.write_byte(b);
        }
        self.write_byte(b'\n');
    }

    pub fn write_buffer(&mut self, the_buffer: &[Uchar]) {
        for &b in the_buffer {
            self.write_byte(b);
        }
    }

    pub fn write_bytes(&mut self, the_byte: &[Uchar], the_count: i32) {
        for i in 0..the_count {
            if (i as usize) < the_byte.len() {
                self.write_byte(the_byte[i as usize]);
            } else {
                break;
            }
        }
    }

    pub fn set_data_from_vec(&mut self, the_buffer: ByteVector) {
        self.m_data = the_buffer;
        self.m_data_bit_size = self.m_data.len() as i32 * 8;
        self.m_read_bit_pos.set(0);
        self.m_write_bit_pos.set(0);
    }

    pub fn set_data_from_ptr(&mut self, the_ptr: *const Uchar, the_count: i32) {
        let slice = unsafe { std::slice::from_raw_parts(the_ptr, the_count as usize) };
        self.m_data = slice.to_vec();
        self.m_data_bit_size = the_count * 8;
        self.m_read_bit_pos.set(0);
        self.m_write_bit_pos.set(0);
    }

    pub fn to_web_string(&self) -> String {
        let mut result = String::new();
        for &b in &self.m_data {
            if b == b'%' {
                result.push_str("%25");
            } else if b == b' ' {
                result.push_str("%20");
            } else if b == b'\n' {
                result.push_str("%0A");
            } else if b == b'\r' {
                result.push_str("%0D");
            } else if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                result.push(b as char);
            } else {
                result.push_str(&format!("%{:02X}", b));
            }
        }
        result
    }

    pub fn to_utf8_string(&self, the_string: &mut String) -> bool {
        the_string.clear();
        // C++: Buffer::ToString — 将原始字节复制到 std::string。
        // [TRANSLATION_NOTE]: Rust String 必须是合法 UTF-8，此处按 UTF-8 字节序列解释
        // （与原版一致，XML/文本资源均为 UTF-8）。此前的 b as char 逐字节 Latin-1
        // 转换会破坏 UTF-8 多字节序列（如 UTF-8 BOM），故改为 from_utf8_lossy。
        let bytes: Vec<u8> = self.m_data.iter().map(|&b| b as u8).collect();
        *the_string = String::from_utf8_lossy(&bytes).into_owned();
        true
    }

    pub fn read_byte(&self) -> Uchar {
        let pos = self.m_read_bit_pos.get();
        if (pos >> 3) as usize >= self.m_data.len() {
            return 0;
        }
        let val = self.m_data[(pos >> 3) as usize];
        self.m_read_bit_pos.set(pos + 8);
        val
    }

    pub fn read_num_bits(&self, the_bits: i32, is_signed: bool) -> i32 {
        let mut a_value = 0i32;
        let mut a_bit_pos = self.m_read_bit_pos.get();
        for _ in 0..the_bits {
            let a_byte_index = (a_bit_pos >> 3) as usize;
            let a_bit_index = a_bit_pos & 7;
            if a_byte_index < self.m_data.len() {
                let bit = (self.m_data[a_byte_index] >> (7 - a_bit_index)) & 1;
                a_value = (a_value << 1) | bit as i32;
            }
            a_bit_pos += 1;
        }
        self.m_read_bit_pos.set(a_bit_pos);

        if is_signed && (a_value & (1 << (the_bits - 1))) != 0 {
            a_value |= !((1 << the_bits) - 1);
        }
        a_value
    }

    pub fn read_boolean(&self) -> bool {
        self.read_byte() != 0
    }

    pub fn read_short(&self) -> i16 {
        let lo = self.read_byte() as i16;
        let hi = self.read_byte() as i16;
        lo | (hi << 8)
    }

    pub fn read_u32(&self) -> u32 {
        let b0 = self.read_byte() as u32;
        let b1 = self.read_byte() as u32;
        let b2 = self.read_byte() as u32;
        let b3 = self.read_byte() as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    pub fn read_i32(&self) -> i32 {
        self.read_u32() as i32
    }

    pub fn read_string(&self) -> String {
        let len = self.read_short() as usize;
        let mut s = String::with_capacity(len);
        for _ in 0..len {
            s.push(self.read_byte() as char);
        }
        s
    }

    pub fn read_line(&self) -> String {
        let mut s = String::new();
        loop {
            let b = self.read_byte();
            if b == b'\n' || b == 0 {
                break;
            }
            s.push(b as char);
        }
        s
    }

    pub fn read_bytes(&self, the_data: &mut [Uchar], the_len: i32) {
        for i in 0..the_len {
            if (i as usize) < the_data.len() {
                the_data[i as usize] = self.read_byte();
            } else {
                break;
            }
        }
    }

    pub fn read_buffer(&self, the_byte_vector: &mut ByteVector) {
        the_byte_vector.clear();
        loop {
            let b = self.read_byte();
            if b == 0 {
                break;
            }
            the_byte_vector.push(b);
        }
    }

    pub fn get_data_ptr(&self) -> *const Uchar {
        self.m_data.as_ptr()
    }

    pub fn get_data_len(&self) -> i32 {
        self.m_data.len() as i32
    }

    pub fn get_data_len_bits(&self) -> i32 {
        self.m_data_bit_size
    }

    pub fn get_crc32(&self, the_seed: u32) -> u32 {
        let mut crc = !the_seed;
        for &b in &self.m_data {
            crc = CRC32_TABLE[((crc as u8) ^ b) as usize] ^ (crc >> 8);
        }
        !crc
    }

    pub fn at_end(&self) -> bool {
        self.m_read_bit_pos.get() >= self.m_data_bit_size
    }

    pub fn past_end(&self) -> bool {
        self.m_read_bit_pos.get() > self.m_data_bit_size
    }
}

fn hex_val(c: u8) -> i32 {
    match c {
        b'0'..=b'9' => (c - b'0') as i32,
        b'A'..=b'F' => (c - b'A' + 10) as i32,
        b'a'..=b'f' => (c - b'a' + 10) as i32,
        _ => -1,
    }
}

const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = 0xEDB88320 ^ (crc >> 1);
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

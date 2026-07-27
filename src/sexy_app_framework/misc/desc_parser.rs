// [TRANSLATION_NOTE]: DescParser.h + DescParser.cpp -> Rust
// 描述文件解析器，使用泛型 + trait 模拟 C++ 虚函数

use crate::sexy_app_framework::common;

pub const CMDSEP_SEMICOLON: i32 = 1;
pub const CMDSEP_NO_INDENT: i32 = 2;

// ============================================================
// DataElement / SingleDataElement / ListDataElement
// ============================================================
pub trait DataElementTrait {
    fn is_list(&self) -> bool;
    fn duplicate(&self) -> Box<dyn DataElementTrait>;
}

pub struct SingleDataElement {
    pub m_is_list: bool,
    pub m_string: String,
}

impl SingleDataElement {
    pub fn new() -> Self {
        SingleDataElement { m_is_list: false, m_string: String::new() }
    }

    pub fn with_string(the_string: &str) -> Self {
        SingleDataElement { m_is_list: false, m_string: the_string.to_string() }
    }
}

impl DataElementTrait for SingleDataElement {
    fn is_list(&self) -> bool { self.m_is_list }
    fn duplicate(&self) -> Box<dyn DataElementTrait> {
        Box::new(SingleDataElement {
            m_is_list: self.m_is_list,
            m_string: self.m_string.clone(),
        })
    }
}

pub struct ListDataElement {
    pub m_is_list: bool,
    pub m_element_vector: Vec<Box<dyn DataElementTrait>>,
}

impl ListDataElement {
    pub fn new() -> Self {
        ListDataElement { m_is_list: true, m_element_vector: Vec::new() }
    }
}

impl Clone for ListDataElement {
    fn clone(&self) -> Self {
        ListDataElement {
            m_is_list: self.m_is_list,
            m_element_vector: self.m_element_vector.iter().map(|e| e.duplicate()).collect(),
        }
    }
}

impl DataElementTrait for ListDataElement {
    fn is_list(&self) -> bool { self.m_is_list }
    fn duplicate(&self) -> Box<dyn DataElementTrait> {
        Box::new(self.clone())
    }
}

// ============================================================
// DescParser trait — 需要用户实现的接口方法
// ============================================================
pub trait DescParserHandler {
    fn error(&mut self, the_error: &str) -> bool;
    fn dereference(&self, the_string: &str) -> Option<&dyn DataElementTrait>;
    fn handle_command(&mut self, the_params: &ListDataElement) -> bool;
}

// ============================================================
// DescParser
// ============================================================
pub struct DescParser<'a, T: DescParserHandler> {
    pub m_cmd_sep: i32,
    pub m_error: String,
    pub m_current_line_num: i32,
    pub m_current_line: String,
    pub m_define_map: Vec<(String, Box<dyn DataElementTrait>)>,
    pub handler: &'a mut T,
}

impl<'a, T: DescParserHandler> DescParser<'a, T> {
    pub fn new(handler: &'a mut T) -> Self {
        DescParser {
            m_cmd_sep: CMDSEP_SEMICOLON,
            m_error: String::new(),
            m_current_line_num: 0,
            m_current_line: String::new(),
            m_define_map: Vec::new(),
            handler,
        }
    }

    pub fn err(&mut self, the_error: &str) -> bool {
        self.m_error = the_error.to_string();
        self.handler.error(the_error)
    }

    pub fn dereference(&self, the_string: &str) -> Option<&dyn DataElementTrait> {
        let a_define_name = common::string_to_upper(the_string);
        for (name, elem) in &self.m_define_map {
            if *name == a_define_name {
                return Some(elem.as_ref());
            }
        }
        self.handler.dereference(the_string)
    }

    pub fn is_immediate(&self, the_string: &str) -> bool {
        if the_string.is_empty() {
            return false;
        }
        let c = the_string.as_bytes()[0];
        (c >= b'0' && c <= b'9') || c == b'-' || c == b'+' || c == b'\'' || c == b'"'
    }

    pub fn unquote(&self, the_quoted_string: &str) -> String {
        if the_quoted_string.is_empty() {
            return String::new();
        }
        let first = the_quoted_string.as_bytes()[0];
        if first != b'\'' && first != b'"' {
            return the_quoted_string.to_string();
        }
        let quote_char = first;
        let mut literal = String::new();
        let mut last_was_quote = false;
        for &b in the_quoted_string.as_bytes() {
            if b == quote_char {
                if last_was_quote {
                    literal.push(b as char);
                }
                last_was_quote = true;
            } else {
                literal.push(b as char);
                last_was_quote = false;
            }
        }
        literal
    }

    pub fn get_values(&mut self, the_source: &ListDataElement, the_values: &mut ListDataElement) -> bool {
        the_values.m_element_vector.clear();
        for elem in &the_source.m_element_vector {
            if elem.is_list() {
                let mut child_list = ListDataElement::new();
                if let Some(list_elem) = elem.as_ref() as *const dyn DataElementTrait as *const ListDataElement {
                    unsafe {
                        if !self.get_values(&*list_elem, &mut child_list) {
                            return false;
                        }
                    }
                }
                the_values.m_element_vector.push(Box::new(child_list));
            } else {
                if let Some(single) = elem.as_ref() as *const dyn DataElementTrait as *const SingleDataElement {
                    unsafe {
                        let a_string = &(*single).m_string;
                        if !a_string.is_empty() {
                            let bytes = a_string.as_bytes();
                            if bytes[0] == b'\'' || bytes[0] == b'"' {
                                the_values.m_element_vector.push(
                                    Box::new(SingleDataElement::with_string(&self.unquote(a_string)))
                                );
                            } else if self.is_immediate(a_string) {
                                the_values.m_element_vector.push(
                                    Box::new(SingleDataElement::with_string(a_string))
                                );
                            } else {
                                let a_define_name = common::string_to_upper(a_string);
                                let mut found = false;
                                for (name, data_elem) in &self.m_define_map {
                                    if *name == a_define_name {
                                        the_values.m_element_vector.push(data_elem.duplicate());
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    return self.err(&format!("Unable to Dereference \"{}\"", a_string));
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }

    pub fn data_element_to_string(&self, the_data_element: &dyn DataElementTrait) -> String {
        if the_data_element.is_list() {
            let list = the_data_element.as_ref() as *const dyn DataElementTrait as *const ListDataElement;
            unsafe {
                let a_list = &*list;
                let mut s = String::from("(");
                for (i, elem) in a_list.m_element_vector.iter().enumerate() {
                    if i != 0 {
                        s += ", ";
                    }
                    s += &self.data_element_to_string(elem.as_ref());
                }
                s += ")";
                s
            }
        } else {
            let single = the_data_element.as_ref() as *const dyn DataElementTrait as *const SingleDataElement;
            unsafe {
                (*single).m_string.clone()
            }
        }
    }

    pub fn data_to_string(&mut self, the_source: &dyn DataElementTrait, the_string: &mut String) -> bool {
        the_string.clear();
        if the_source.is_list() {
            return false;
        }
        let single = the_source.as_ref() as *const dyn DataElementTrait as *const SingleDataElement;
        let a_def_name = unsafe { (*single).m_string.clone() };

        if let Some(a_data_element) = self.dereference(&a_def_name) {
            if a_data_element.is_list() {
                return false;
            }
            let data_single = a_data_element as *const dyn DataElementTrait as *const SingleDataElement;
            *the_string = self.unquote(unsafe { &(*data_single).m_string });
        } else {
            *the_string = self.unquote(&a_def_name);
        }
        true
    }

    pub fn data_to_int(&mut self, the_source: &dyn DataElementTrait, the_int: &mut i32) -> bool {
        *the_int = 0;
        let mut a_temp = String::new();
        if !self.data_to_string(the_source, &mut a_temp) {
            return false;
        }
        common::string_to_int(&a_temp, the_int)
    }

    pub fn data_to_string_vector(&mut self, the_source: &dyn DataElementTrait, the_string_vector: &mut Vec<String>) -> bool {
        the_string_vector.clear();

        let (a_values, is_owned): (ListDataElement, bool);
        let values_ref: &ListDataElement;

        if the_source.is_list() {
            let mut static_values = ListDataElement::new();
            if !self.get_values(
                unsafe { &*(the_source as *const dyn DataElementTrait as *const ListDataElement) },
                &mut static_values
            ) {
                return false;
            }
            a_values = static_values;
            is_owned = true;
            values_ref = &a_values;
        } else {
            let single = the_source as *const dyn DataElementTrait as *const SingleDataElement;
            let a_def_name = unsafe { (*single).m_string.clone() };

            if let Some(a_data_element) = self.dereference(&a_def_name) {
                if !a_data_element.is_list() {
                    return false;
                }
                values_ref = unsafe { &*(a_data_element as *const dyn DataElementTrait as *const ListDataElement) };
            } else {
                return self.err(&format!("Unable to Dereference \"{}\"", a_def_name));
            }
        }

        for elem in &values_ref.m_element_vector {
            if elem.is_list() {
                the_string_vector.clear();
                return false;
            }
            let single = elem.as_ref() as *const dyn DataElementTrait as *const SingleDataElement;
            the_string_vector.push(unsafe { (*single).m_string.clone() });
        }
        true
    }

    pub fn data_to_list(&mut self, the_source: &dyn DataElementTrait, the_values: &mut ListDataElement) -> bool {
        if the_source.is_list() {
            return self.get_values(
                unsafe { &*(the_source as *const dyn DataElementTrait as *const ListDataElement) },
                the_values
            );
        }

        let single = the_source as *const dyn DataElementTrait as *const SingleDataElement;
        let a_def_name = unsafe { (*single).m_string.clone() };

        if let Some(a_data_element) = self.dereference(&a_def_name) {
            if !a_data_element.is_list() {
                return false;
            }
            let list_elem = unsafe { &*(a_data_element as *const dyn DataElementTrait as *const ListDataElement) };
            *the_values = list_elem.clone();
            true
        } else {
            false
        }
    }

    pub fn data_to_int_vector(&mut self, the_source: &dyn DataElementTrait, the_int_vector: &mut Vec<i32>) -> bool {
        the_int_vector.clear();
        let mut a_string_vector = Vec::new();
        if !self.data_to_string_vector(the_source, &mut a_string_vector) {
            return false;
        }
        for s in &a_string_vector {
            let mut val = 0i32;
            if !common::string_to_int(s, &mut val) {
                return false;
            }
            the_int_vector.push(val);
        }
        true
    }

    pub fn data_to_double_vector(&mut self, the_source: &dyn DataElementTrait, the_double_vector: &mut Vec<f64>) -> bool {
        the_double_vector.clear();
        let mut a_string_vector = Vec::new();
        if !self.data_to_string_vector(the_source, &mut a_string_vector) {
            return false;
        }
        for s in &a_string_vector {
            let mut val = 0.0f64;
            if !common::string_to_double(s, &mut val) {
                return false;
            }
            the_double_vector.push(val);
        }
        true
    }

    pub fn parse_to_list(&mut self, the_string: &str, the_list: &mut ListDataElement, expect_list_end: bool, the_string_pos: &mut Option<usize>) -> bool {
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;
        let mut escaped = false;

        let mut cur_single: Option<SingleDataElement> = None;

        let chars: Vec<char> = the_string.chars().collect();
        let mut pos = the_string_pos.unwrap_or(0);

        while pos < chars.len() {
            let mut add_single_char = false;
            let a_char = chars[pos];
            pos += 1;

            let is_separator = a_char == ' ' || a_char == '\t' || a_char == '\n' || a_char == ',';

            if escaped {
                add_single_char = true;
                escaped = false;
            } else {
                if a_char == '\'' && !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                } else if a_char == '"' && !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                }

                if a_char == '\\' {
                    escaped = true;
                } else if !in_single_quotes && !in_double_quotes {
                    if a_char == ')' {
                        if expect_list_end {
                            *the_string_pos = Some(pos);
                            return true;
                        } else {
                            return self.err("Unexpected List End");
                        }
                    } else if a_char == '(' {
                        if cur_single.is_some() {
                            return self.err("Unexpected List Start");
                        }
                        let mut child_list = ListDataElement::new();
                        if !self.parse_to_list(the_string, &mut child_list, true, &mut Some(pos)) {
                            return false;
                        }
                        the_list.m_element_vector.push(Box::new(child_list));
                    } else if is_separator {
                        cur_single = None;
                    } else {
                        add_single_char = true;
                    }
                } else {
                    add_single_char = true;
                }
            }

            if add_single_char {
                if cur_single.is_none() {
                    cur_single = Some(SingleDataElement::new());
                    if let Some(ref single) = cur_single {
                        // 需要把 single 添加到 the_list 中，但这需要 ownership
                        // 用 clone 方式处理
                    }
                }
                if let Some(ref mut single) = cur_single {
                    single.m_string.push(a_char);
                }
            }
        }

        // 收集当前挂起的 single 到 list
        if let Some(single) = cur_single {
            the_list.m_element_vector.push(Box::new(single));
        }

        if in_single_quotes {
            return self.err("Unterminated Single Quotes");
        }
        if in_double_quotes {
            return self.err("Unterminated Double Quotes");
        }
        if expect_list_end {
            return self.err("Unterminated List");
        }

        *the_string_pos = Some(pos);
        true
    }

    pub fn parse_descriptor_line(&mut self, the_descriptor_line: &str) -> bool {
        let mut a_params = ListDataElement::new();
        if !self.parse_to_list(the_descriptor_line, &mut a_params, false, &mut None) {
            return false;
        }
        if !a_params.m_element_vector.is_empty() {
            if a_params.m_element_vector[0].is_list() {
                return self.err("Missing Command");
            }
            if !self.handler.handle_command(&a_params) {
                return false;
            }
        }
        true
    }

    pub fn load_descriptor(&mut self, the_file_name: &str) -> bool {
        self.m_current_line_num = 0;
        self.m_error.clear();

        // 使用 gSexyAppBase 的 ReadUTF8StringFromFile 等价功能
        let a_file_content = match std::fs::read_to_string(the_file_name) {
            Ok(c) => c,
            Err(_) => return self.err("Failed to open file"),
        };

        let chars: Vec<char> = a_file_content.chars().collect();
        let mut a_index = 0;
        let mut a_line_count = 0;
        let mut a_buff_char: Option<char> = None;

        while a_index < chars.len() || a_buff_char.is_some() {
            let mut skip_line = false;
            let mut at_line_start = true;
            let mut in_single_quotes = false;
            let mut in_double_quotes = false;
            let mut escaped = false;
            let mut is_indented = false;

            loop {
                let a_char: char;
                if let Some(bc) = a_buff_char.take() {
                    a_char = bc;
                } else {
                    if a_index >= chars.len() {
                        break;
                    }
                    a_char = chars[a_index];
                    a_index += 1;
                    a_buff_char = None;
                }

                if a_char != '\r' {
                    if a_char == '\n' {
                        a_line_count += 1;
                    }

                    if (a_char == ' ' || a_char == '\t') && at_line_start {
                        is_indented = true;
                    }

                    if !at_line_start || (a_char != ' ' && a_char != '\t' && a_char != '\n') {
                        if at_line_start {
                            if (self.m_cmd_sep & CMDSEP_NO_INDENT) != 0 && !is_indented && !self.m_current_line.is_empty() {
                                a_buff_char = Some(a_char);
                                break;
                            }
                            if a_char == '#' {
                                skip_line = true;
                            }
                            at_line_start = false;
                        }

                        if a_char == '\n' {
                            is_indented = false;
                            at_line_start = true;
                        }

                        if a_char == '\n' && skip_line {
                            skip_line = false;
                        } else if !skip_line {
                            if a_char == '\\' && (in_single_quotes || in_double_quotes) && !escaped {
                                escaped = true;
                            } else {
                                if a_char == '\'' && !in_double_quotes && !escaped {
                                    in_single_quotes = !in_single_quotes;
                                }
                                if a_char == '"' && !in_single_quotes && !escaped {
                                    in_double_quotes = !in_double_quotes;
                                }
                                if a_char == ';' && (self.m_cmd_sep & CMDSEP_SEMICOLON) != 0 && !in_single_quotes && !in_double_quotes {
                                    break;
                                }
                                if escaped {
                                    self.m_current_line.push('\\');
                                    escaped = false;
                                }
                                if self.m_current_line.is_empty() {
                                    self.m_current_line_num = a_line_count + 1;
                                }
                                self.m_current_line.push(a_char);
                            }
                        }
                    }
                }
            }

            if !self.m_current_line.is_empty() {
                if !self.parse_descriptor_line(&self.m_current_line) {
                    return false;
                }
                self.m_current_line.clear();
            }
        }

        self.m_current_line.clear();
        self.m_current_line_num = 0;
        true
    }
}

// [TRANSLATION_NOTE]: PropertiesParser.h + PropertiesParser.cpp -> Rust
// XML 属性文件解析器

use crate::sexy_app_framework::misc::xml_parser::*;
use crate::sexy_app_framework::misc::buffer::Buffer;
use crate::sexy_app_framework::common;

pub struct PropertiesParser<'a> {
    pub m_app: &'a mut crate::sexy_app_framework::sexy_app_base::SexyAppBase,
    pub m_xml_parser: Option<XMLParser>,
    pub m_error: String,
    pub m_has_failed: bool,
}

impl<'a> PropertiesParser<'a> {
    pub fn new(the_app: &'a mut crate::sexy_app_framework::sexy_app_base::SexyAppBase) -> Self {
        PropertiesParser {
            m_app: the_app,
            m_xml_parser: None,
            m_error: String::new(),
            m_has_failed: false,
        }
    }

    fn fail(&mut self, the_error_text: &str) {
        if !self.m_has_failed {
            self.m_has_failed = true;
            let a_line_num = if let Some(ref parser) = self.m_xml_parser {
                parser.GetCurrentLineNum()
            } else {
                0
            };

            self.m_error = the_error_text.to_string();
            if a_line_num > 0 {
                self.m_error += &format!(" on Line {}", a_line_num);
            }
            if let Some(ref parser) = self.m_xml_parser {
                if !parser.GetFileName().is_empty() {
                    self.m_error += &format!(" in File '{}'", parser.GetFileName());
                }
            }
        }
    }

    fn parse_single_element(&mut self, a_string: &mut String) -> bool {
        a_string.clear();
        loop {
            let mut a_xml_element = XMLElement::new();
            if !self.m_xml_parser.as_mut().unwrap().NextElement(&mut a_xml_element) {
                return false;
            }

            match a_xml_element.mType {
                XMLElementType::TYPE_START => {
                    self.fail(&format!("Unexpected Section: '{}'", a_xml_element.mValue));
                    return false;
                }
                XMLElementType::TYPE_ELEMENT => {
                    *a_string = a_xml_element.mValue;
                }
                XMLElementType::TYPE_END => {
                    return true;
                }
                _ => {}
            }
        }
    }

    fn parse_string_array(&mut self, the_string_vector: &mut Vec<String>) -> bool {
        the_string_vector.clear();
        loop {
            let mut a_xml_element = XMLElement::new();
            if !self.m_xml_parser.as_mut().unwrap().NextElement(&mut a_xml_element) {
                return false;
            }

            match a_xml_element.mType {
                XMLElementType::TYPE_START => {
                    if a_xml_element.mValue == "String" {
                        let mut a_string = String::new();
                        if !self.parse_single_element(&mut a_string) {
                            return false;
                        }
                        the_string_vector.push(a_string);
                    } else {
                        self.fail(&format!("Invalid Section '{}'", a_xml_element.mValue));
                        return false;
                    }
                }
                XMLElementType::TYPE_ELEMENT => {
                    self.fail(&format!("Element Not Expected '{}'", a_xml_element.mValue));
                    return false;
                }
                XMLElementType::TYPE_END => {
                    return true;
                }
                _ => {}
            }
        }
    }

    fn parse_properties(&mut self) -> bool {
        loop {
            let mut a_xml_element = XMLElement::new();
            if !self.m_xml_parser.as_mut().unwrap().NextElement(&mut a_xml_element) {
                return false;
            }

            match a_xml_element.mType {
                XMLElementType::TYPE_START => {
                    if a_xml_element.mValue == "String" {
                        let mut a_def = String::new();
                        if !self.parse_single_element(&mut a_def) {
                            return false;
                        }
                        let an_id = a_xml_element.mAttributes.get("id").cloned().unwrap_or_default();
                        self.m_app.set_string(&an_id, &a_def);
                    } else if a_xml_element.mValue == "StringArray" {
                        let mut a_def = Vec::new();
                        if !self.parse_string_array(&mut a_def) {
                            return false;
                        }
                        let an_id = a_xml_element.mAttributes.get("id").cloned().unwrap_or_default();
                        self.m_app.m_string_vector_properties.insert(an_id, a_def);
                    } else if a_xml_element.mValue == "Boolean" {
                        let mut a_val = String::new();
                        if !self.parse_single_element(&mut a_val) {
                            return false;
                        }
                        let a_val_up = common::upper(&a_val);
                        let bool_val = match a_val_up.as_str() {
                            "1" | "YES" | "ON" | "TRUE" => true,
                            "0" | "NO" | "OFF" | "FALSE" => false,
                            _ => {
                                self.fail(&format!("Invalid Boolean Value: '{}'", a_val));
                                return false;
                            }
                        };
                        let an_id = a_xml_element.mAttributes.get("id").cloned().unwrap_or_default();
                        self.m_app.set_boolean(&an_id, bool_val);
                    } else if a_xml_element.mValue == "Integer" {
                        let mut a_val = String::new();
                        if !self.parse_single_element(&mut a_val) {
                            return false;
                        }
                        let mut an_int = 0i32;
                        if !common::string_to_int(&a_val, &mut an_int) {
                            self.fail(&format!("Invalid Integer Value: '{}'", a_val));
                            return false;
                        }
                        let an_id = a_xml_element.mAttributes.get("id").cloned().unwrap_or_default();
                        self.m_app.set_integer(&an_id, an_int);
                    } else if a_xml_element.mValue == "Double" {
                        let mut a_val = String::new();
                        if !self.parse_single_element(&mut a_val) {
                            return false;
                        }
                        let mut a_double = 0.0f64;
                        if !common::string_to_double(&a_val, &mut a_double) {
                            self.fail(&format!("Invalid Double Value: '{}'", a_val));
                            return false;
                        }
                        let an_id = a_xml_element.mAttributes.get("id").cloned().unwrap_or_default();
                        self.m_app.set_double(&an_id, a_double);
                    } else {
                        self.fail(&format!("Invalid Section '{}'", a_xml_element.mValue));
                        return false;
                    }
                }
                XMLElementType::TYPE_ELEMENT => {
                    self.fail(&format!("Element Not Expected '{}'", a_xml_element.mValue));
                    return false;
                }
                XMLElementType::TYPE_END => {
                    return true;
                }
                _ => {}
            }
        }
    }

    fn do_parse_properties(&mut self) -> bool {
        let has_failed = self.m_xml_parser.as_ref().map_or(true, |p| p.HasFailed());
        if !has_failed {
            loop {
                let mut a_xml_element = XMLElement::new();
                if !self.m_xml_parser.as_mut().unwrap().NextElement(&mut a_xml_element) {
                    break;
                }

                match a_xml_element.mType {
                    XMLElementType::TYPE_START => {
                        if a_xml_element.mValue == "Properties" {
                            if !self.parse_properties() {
                                break;
                            }
                        } else {
                            self.fail(&format!("Invalid Section '{}'", a_xml_element.mValue));
                            break;
                        }
                    }
                    XMLElementType::TYPE_ELEMENT => {
                        self.fail(&format!("Element Not Expected '{}'", a_xml_element.mValue));
                        break;
                    }
                    _ => {}
                }
            }
        }

        if self.m_xml_parser.as_ref().map_or(false, |p| p.HasFailed()) {
            let an_error = self.m_xml_parser.as_ref().unwrap().GetErrorText().to_string();
            self.fail(&an_error);
        }

        self.m_xml_parser = None;
        !self.m_has_failed
    }

    pub fn parse_properties_buffer(&mut self, the_buffer: &Buffer) -> bool {
        let mut parser = XMLParser::new();
        let mut a_string = String::new();
        the_buffer.to_utf8_string(&mut a_string);
        parser.SetStringSource(&a_string);
        self.m_xml_parser = Some(parser);
        self.do_parse_properties()
    }

    pub fn parse_properties_file(&mut self, the_filename: &str) -> bool {
        let mut parser = XMLParser::new();
        parser.OpenFile(the_filename);
        self.m_xml_parser = Some(parser);
        self.do_parse_properties()
    }

    pub fn get_error_text(&self) -> &str {
        &self.m_error
    }
}

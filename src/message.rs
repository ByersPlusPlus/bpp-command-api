use std::collections::{HashMap, HashSet};
use custom_error::custom_error;

use crate::collection;

custom_error! { pub StringViewError
    ExpectedClosingQuote{close_quote: String} = "Expected closing quote '{}'",
    UnexpectedQuote{quote: String} = "Unexpected quote '{}' in non-quoted string.",
    InvalidEndOfQuotedString{char: String} = "Expected space after closing quotation but received {}"
}

// Below is the implementation of Rapptz's StringView in Rust.
// Original license:
/*
The MIT License (MIT)

Copyright (c) 2015-2019 Rapptz

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

lazy_static! {
    static ref QUOTE_CHARS: HashMap<String, String> = {
        let m: HashMap<String, String> = collection! {
            "\"".to_string() => "\"".to_string(),
            "‘".to_string() => "’".to_string(),
            "‚".to_string() => "‛".to_string(),
            "“".to_string() => "”".to_string(),
            "„".to_string() => "‟".to_string(),
            "⹂".to_string() => "⹂".to_string(),
            "「".to_string() => "」".to_string(),
            "『".to_string() => "』".to_string(),
            "〝".to_string() => "〞".to_string(),
            "﹁".to_string() => "﹂".to_string(),
            "﹃".to_string() => "﹄".to_string(),
            "＂".to_string() => "＂".to_string(),
            "｢".to_string() => "｣".to_string(),
            "«".to_string() => "»".to_string(),
            "‹".to_string() => "›".to_string(),
            "《".to_string() => "》".to_string(),
            "〈".to_string() => "〉".to_string(),
        };
        m
    };

    static ref ALL_QUOTE_CHARS: HashSet<String> = {
        let mut v: HashSet<String> = HashSet::new();
        for (k, qv) in QUOTE_CHARS.iter() {
            v.insert(k.clone());
            v.insert(qv.clone());
        }
        v
    };
}

pub struct StringView {
    pub index: i32,
    pub buffer: String,
    pub end: i32,
    pub previous: i32
}

impl StringView {
    pub fn new(buffer: String) -> Self {
        let buffer_len = buffer.len() as i32;
        StringView {
            index: 0,
            buffer,
            end: buffer_len,
            previous: 0
        }
    }

    pub fn eof(&self) -> bool {
        self.index >= self.end
    }

    pub fn current(&self) -> Option<char> {
        if self.eof() {
            return None;
        } else {
            return Some(self.buffer.chars().nth(self.index as usize).unwrap());
        }
    }

    pub fn undo(&mut self) {
        self.index = self.previous;
    }

    pub fn skip_whitespace(&mut self) -> bool {
        let mut pos = 0;
        while !self.eof() {
            let current_pos = self.index + pos;
            let current_char = self.buffer.chars().nth(current_pos as usize);
            match current_char {
                Some(c) => {
                    if !c.is_whitespace() {
                        break;
                    }
                    pos += 1;
                },
                None => {
                    break;
                }
            }
        }

        self.previous = self.index;
        self.index += pos;
        return self.previous != self.index;
    }

    pub fn skip_string(&mut self, string: String) -> bool {
        let str_len = string.len() as i32;
        if self.buffer[self.index as usize..(self.index + str_len) as usize].eq(string.as_str()) {
            self.previous = self.index;
            self.index += str_len;
            return true;
        }
        return false;
    }

    pub fn read_rest(&mut self) -> String {
        let result = self.buffer[self.index as usize..].to_string();
        self.previous = self.index;
        self.index = self.end;
        return result;
    }

    pub fn read(&mut self, length: i32) -> String {
        let result = self.buffer[self.index as usize..(self.index + length) as usize].to_string();
        self.previous = self.index;
        self.index += length;
        return result;
    }

    pub fn get(&mut self) -> Option<char> {
        let result: Option<char>;
        let index_result = self.buffer.get((self.index + 1) as usize..(self.index + 2) as usize);
        if index_result.is_none() {
            result = None;
        } else {
            let unwrapped_index_result = index_result.unwrap();
            result = Some(unwrapped_index_result.chars().nth(0).unwrap());
        }
        self.previous = self.index;
        self.index += 1;
        return result;
    }

    pub fn get_word(&mut self) -> String {
        let mut pos = 0;
        while !self.eof() {
            let current_pos = self.index + pos;
            let current_char = self.buffer.chars().nth(current_pos as usize);
            match current_char {
                Some(c) => {
                    if c.is_whitespace() {
                        break;
                    }
                    pos += 1;
                },
                None => {
                    break;
                }
            }
        }

        self.previous = self.index;
        let result = self.buffer[self.index as usize..(self.index + pos) as usize].to_string();
        self.index += pos;
        return result;
    }

    pub fn get_parameters(&mut self) -> Result<Vec<String>, StringViewError> {
        let mut parameters: Vec<String> = Vec::new();
        while !self.eof() {
            let quoted_word = self.get_quoted_word();
            if quoted_word.is_err() {
                return Err(quoted_word.err().unwrap());
            }
            let quoted_word = quoted_word.unwrap();
            if quoted_word.is_none()  {
                break;
            }
            let quoted_word = quoted_word.unwrap();
            parameters.push(quoted_word);
        }
        return Ok(parameters);
    }

    pub fn get_quoted_word(&mut self) -> Result<Option<String>, StringViewError> {
        let current = self.current();
        if current.is_none() {
            return Ok(None);
        }
        let current = current.unwrap();

        let close_quote = QUOTE_CHARS.get(&current.to_string());
        let mut result: Vec<char>;
        let _escaped_quotes: HashSet<String>;
        let is_quoted = close_quote.is_some();
        if is_quoted {
            result = Vec::new();
            let close_quote = close_quote.unwrap().chars().nth(0).unwrap();
            _escaped_quotes = collection! { current.to_string(), close_quote.to_string() };
        } else {
            result = vec![current];
            _escaped_quotes = ALL_QUOTE_CHARS.clone();
        }

        while !self.eof() {
            let current_char = self.get();
            if current_char.is_none() {
                if is_quoted {
                    // unexpected EOF
                    return Err(StringViewError::ExpectedClosingQuote {
                        close_quote: close_quote.unwrap().clone()
                    });
                }
                return Ok(Some(result.into_iter().collect()));
            }
            let current_char = current_char.unwrap();
            if current_char == '\\' {
                let next_char = self.get();
                if next_char.is_none() {
                    // string ends with escape without character behind it
                    if is_quoted {
                        // we expect a closing quote
                        return Err(StringViewError::ExpectedClosingQuote {
                            close_quote: close_quote.unwrap().clone()
                        });
                    }
                    return Ok(Some(result.into_iter().collect()));
                }

                let next_char = next_char.unwrap();
                if _escaped_quotes.contains(&next_char.to_string()) {
                    // escaped quote
                    result.push(next_char);
                } else {
                    // escaped character
                    // ignoring it
                    self.undo();
                    result.push(current_char);
                }
                continue;
            }

            if !is_quoted && ALL_QUOTE_CHARS.contains(&current_char.to_string()) {
                // unexpected quote
                return Err(StringViewError::UnexpectedQuote {
                    quote: current_char.to_string()
                });
            }

            // closing quote
            if is_quoted {
                let close_quote = close_quote.unwrap().chars().nth(0).unwrap();
                if current_char == close_quote {
                    let next_char = self.get();
                    let valid_eof = next_char.is_none() || next_char.unwrap().is_whitespace();
                    if !valid_eof {
                        return Err(StringViewError::InvalidEndOfQuotedString {
                            char: next_char.unwrap().to_string()
                        });
                    }

                    return Ok(Some(result.into_iter().collect()));
                }
            }

            if current_char.is_whitespace() && !is_quoted {
                // end of word
                return Ok(Some(result.into_iter().collect()));
            }
            result.push(current_char);
        }

        return Ok(Some(result.into_iter().collect()));
    }
}
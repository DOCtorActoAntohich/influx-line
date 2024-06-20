use crate::line::InfluxLineParseError;

use super::{exclusive_split_at, Escaped};

#[derive(Debug)]
pub struct KeyParser {
    escaped: Escaped,
}

impl KeyParser {
    pub fn new() -> Self {
        Self {
            escaped: Escaped::No,
        }
    }

    pub fn process<'a>(
        mut self,
        line: &'a str,
    ) -> Result<(&'a str, &'a str), InfluxLineParseError> {
        for (index, character) in line.char_indices() {
            match (self.escaped, character) {
                (Escaped::No, '\\') => {
                    self.escaped = Escaped::Yes;
                }
                (Escaped::No, ' ' | ',') => {
                    return Err(InfluxLineParseError::UnescapedSpecialCharacter);
                }
                (Escaped::No, '=') => {
                    return Ok(exclusive_split_at(line, index));
                }
                (Escaped::No, _) => (),
                (Escaped::Yes, _) => {
                    self.escaped = Escaped::No;
                }
            }
        }

        Err(InfluxLineParseError::NoValue)
    }
}

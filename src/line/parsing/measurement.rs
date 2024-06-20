use crate::line::InfluxLineParseError;

use super::{exclusive_split_at, Escaped};

#[derive(Debug)]
pub struct MeasurementParser {
    escaped: Escaped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementTail<'a> {
    Tags(&'a str),
    Fields(&'a str),
}

impl MeasurementParser {
    pub fn new() -> Self {
        Self {
            escaped: Escaped::No,
        }
    }

    pub fn process<'a>(
        mut self,
        line: &'a str,
    ) -> Result<(&'a str, MeasurementTail<'a>), InfluxLineParseError> {
        for (index, character) in line.char_indices() {
            match (self.escaped, character) {
                (Escaped::No, ',' | ' ') if index == 0 => {
                    return Err(InfluxLineParseError::NoMeasurement);
                }
                (Escaped::No, ',') => {
                    let (measurement, tail) = exclusive_split_at(line, index);
                    return Ok((measurement, MeasurementTail::Tags(tail)));
                }
                (Escaped::No, ' ') => {
                    let (measurement, tail) = exclusive_split_at(line, index);
                    return Ok((measurement, MeasurementTail::Fields(tail)));
                }
                (Escaped::No, '\\') => {
                    self.escaped = Escaped::Yes;
                }
                (Escaped::No, _) => (),
                (Escaped::Yes, '\\') => (),
                (Escaped::Yes, _) => {
                    self.escaped = Escaped::No;
                }
            }
        }

        Err(InfluxLineParseError::NoWhitespaceDelimiter)
    }
}
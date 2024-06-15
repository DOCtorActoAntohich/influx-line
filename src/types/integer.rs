use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::From,
    derive_more::Display,
)]
#[from(types(i8, i16, i32))]
#[display(fmt = "{}i", _0)]
pub struct InfluxInteger(i64);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
)]
#[from(types(u8, u16, u32))]
#[display(fmt = "{}u", _0)]
pub struct InfluxUInteger(u64);

#[derive(Debug, thiserror::Error)]
pub enum NumberParseError {
    #[error("Wrong format: {0}")]
    Malformed(String),
    #[error("Failed to parse number: {0}")]
    Failed(String),
}

impl FromStr for InfluxInteger {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((int_slice, empty)) = s.split_once('i') else {
            return Err(NumberParseError::Malformed(s.into()));
        };
        if !empty.is_empty() {
            return Err(NumberParseError::Malformed(s.into()));
        }

        let integer = int_slice
            .parse::<i64>()
            .map_err(|_| NumberParseError::Failed(int_slice.into()))?;

        Ok(Self(integer))
    }
}

impl FromStr for InfluxUInteger {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((uint_slice, empty)) = s.split_once('u') else {
            return Err(NumberParseError::Malformed(s.into()));
        };
        if !empty.is_empty() {
            return Err(NumberParseError::Malformed(s.into()));
        }

        let uinteger = uint_slice
            .parse::<u64>()
            .map_err(|_| NumberParseError::Failed(uint_slice.into()))?;

        Ok(Self(uinteger))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{InfluxInteger, InfluxUInteger};

    #[rstest::rstest]
    #[case("123i", 123.into())]
    #[case("0i", 0.into())]
    #[case("-25565i", (-25565).into())]
    fn successful_int_parsing(#[case] input: &str, #[case] expected_integer: InfluxInteger) {
        let actual_integer = InfluxInteger::from_str(input).expect("Must parse here");

        assert_eq!(expected_integer, actual_integer)
    }

    #[rstest::rstest]
    #[case::uint("123u")]
    #[case::no_suffix_means_float("0")]
    #[case::actual_float("128.0")]
    #[case::empty("")]
    #[case::gibberish("randomi")]
    #[case::spaces("123 01i")]
    fn int_parse_error(#[case] input: &str) {
        let _parse_error = InfluxInteger::from_str(input).expect_err("Must return parse error");
    }

    #[rstest::rstest]
    #[case("123u", (123 as u32).into())]
    #[case("0u", (0 as u32).into())]
    fn successful_uint_parsing(#[case] input: &str, #[case] expected_integer: InfluxUInteger) {
        let actual_integer = InfluxUInteger::from_str(input).expect("Must parse here");

        assert_eq!(expected_integer, actual_integer)
    }

    #[rstest::rstest]
    #[case::positive_int("25565i")]
    #[case::negative_int("-25565i")]
    #[case::negative_uint("-25565u")]
    #[case::no_suffix_means_float("0")]
    #[case::actual_float("128.0")]
    #[case::empty("")]
    #[case::gibberish("randomu")]
    #[case::spaces("123 01u")]
    fn uint_parse_error(#[case] input: &str) {
        let _parse_error = InfluxUInteger::from_str(input).expect_err("Must return parse error");
    }
}
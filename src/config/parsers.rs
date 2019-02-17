/* anevicon: The most powerful UDP-based load generator, written in Rust.
 * Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * For more information see <https://github.com/Gymmasssorla/anevicon>.
 */

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::{NonZeroUsize, ParseIntError};

use super::{MAX_PACKET_LENGTH, MIN_PACKET_LENGTH};

pub fn parse_packet_length(length: &str) -> Result<NonZeroUsize, PacketLengthError> {
    let length: usize = length
        .parse()
        .map_err(|error| PacketLengthError::InvalidFormat(error))?;

    if length < MIN_PACKET_LENGTH {
        return Err(PacketLengthError::Underflow);
    } else if length > MAX_PACKET_LENGTH {
        return Err(PacketLengthError::Overflow);
    }

    NonZeroUsize::new(length).ok_or(PacketLengthError::Underflow)
}

pub fn parse_non_zero_usize(number: &str) -> Result<NonZeroUsize, NonZeroUsizeError> {
    let number: usize = number
        .parse()
        .map_err(|error| NonZeroUsizeError::InvalidFormat(error))?;

    NonZeroUsize::new(number).ok_or(NonZeroUsizeError::ZeroValue)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketLengthError {
    InvalidFormat(ParseIntError),
    Overflow,
    Underflow,
}

impl Display for PacketLengthError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            PacketLengthError::InvalidFormat(error) => write!(fmt, "{}", error),
            PacketLengthError::Overflow => write!(fmt, "The value is too big"),
            PacketLengthError::Underflow => write!(fmt, "The value is too small"),
        }
    }
}

impl Error for PacketLengthError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonZeroUsizeError {
    InvalidFormat(ParseIntError),
    ZeroValue,
}

impl Display for NonZeroUsizeError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            NonZeroUsizeError::InvalidFormat(error) => write!(fmt, "{}", error),
            NonZeroUsizeError::ZeroValue => write!(fmt, "The value equals to zero"),
        }
    }
}

impl Error for NonZeroUsizeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ordinary_lengths() {
        unsafe {
            // Check that ordinary values are parsed correctly
            assert_eq!(
                parse_packet_length("53251"),
                Ok(NonZeroUsize::new_unchecked(53251))
            );
            assert_eq!(
                parse_packet_length("26655"),
                Ok(NonZeroUsize::new_unchecked(26655))
            );
            assert_eq!(
                parse_packet_length("+75"),
                Ok(NonZeroUsize::new_unchecked(75))
            );

            // Check that the min and max values are still valid
            assert_eq!(
                parse_packet_length("1"),
                Ok(NonZeroUsize::new_unchecked(MIN_PACKET_LENGTH))
            );
            assert_eq!(
                parse_packet_length("65000"),
                Ok(NonZeroUsize::new_unchecked(MAX_PACKET_LENGTH))
            );
        }
    }

    #[test]
    fn parses_invalid_lengths() {
        let panic_if_invalid = |string| {
            if let Ok(_) = parse_packet_length(string) {
                panic!("Parses invalid formatted length correctly");
            }
        };

        // Invalid numbers must produce the invalid format error
        panic_if_invalid("   ");

        panic_if_invalid("abc5653odr!");
        panic_if_invalid("6485&02hde");

        panic_if_invalid("-565642");
        panic_if_invalid(&"2178".repeat(50));

        // Check that too big and too small values aren't valid
        assert_eq!(parse_packet_length("0"), Err(PacketLengthError::Underflow));
        assert_eq!(
            parse_packet_length("533987721456"),
            Err(PacketLengthError::Overflow)
        );
    }

    #[test]
    fn parses_ordinary_non_zero_usize() {
        unsafe {
            // Check that ordinary values are parsed correctly
            assert_eq!(
                parse_non_zero_usize("1"),
                Ok(NonZeroUsize::new_unchecked(1))
            );
            assert_eq!(
                parse_non_zero_usize("3"),
                Ok(NonZeroUsize::new_unchecked(3))
            );
            assert_eq!(
                parse_non_zero_usize("26655"),
                Ok(NonZeroUsize::new_unchecked(26655))
            );
            assert_eq!(
                parse_non_zero_usize("+75"),
                Ok(NonZeroUsize::new_unchecked(75))
            );
        }
    }

    #[test]
    fn parses_invalid_non_zero_usize() {
        let panic_if_invalid = |string| {
            if let Ok(_) = parse_non_zero_usize(string) {
                panic!("Parses invalid formatted usize correctly");
            }
        };

        // Invalid numbers must produce the invalid format error
        panic_if_invalid("   ");

        panic_if_invalid("abc5653odr!");
        panic_if_invalid("6485&02hde");

        panic_if_invalid("-565642");
        panic_if_invalid(&"2178".repeat(50));

        // Check that the zero value is not allowed
        assert_eq!(parse_non_zero_usize("0"), Err(NonZeroUsizeError::ZeroValue));
    }
}

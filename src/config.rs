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
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::time::Duration;

use humantime::parse_duration;
use structopt::StructOpt;

const MIN_PACKET_LENGTH: usize = 1;
const MAX_PACKET_LENGTH: usize = 65000;

#[derive(Debug, StructOpt)]
#[structopt(
    author = "Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>",
    about = "An UDP-based server stress-testing tool, written in Rust.",
    after_help = "For more information see <https://github.com/Gymmasssorla/anevicon>.",
    set_term_width = 80
)]
pub struct ArgsConfig {
    /// A receiver of generated traffic, specified as an IP-address
    /// and a port number, separated by the colon character.
    #[structopt(
        short = "r",
        long = "receiver",
        takes_value = true,
        value_name = "ADDRESS",
        required = true
    )]
    pub receiver: SocketAddr,

    /// A sender of generated traffic, specified as an IP-address
    /// and a port number, separated by the colon character.
    #[structopt(
        short = "s",
        long = "sender",
        takes_value = true,
        value_name = "ADDRESS",
        default_value = "0.0.0.0:0"
    )]
    pub sender: SocketAddr,

    /// A program working time. The default value is too big, that
    /// is, an attack will be performed until you explicitly stop
    /// the process.
    #[structopt(
        short = "d",
        long = "duration",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "64years 64hours 64secs",
        parse(try_from_str = "parse_duration")
    )]
    pub duration: Duration,

    /// A size of each UDP-packet in the range of [1; 65000],
    /// specified in bytes. Note that your system or a victim server
    /// might not be able to handle the default value.
    #[structopt(
        short = "l",
        long = "length",
        takes_value = true,
        value_name = "BYTES",
        default_value = "65000",
        parse(try_from_str = "parse_length")
    )]
    pub length: usize,

    /// A waiting time before an attack execution. It is mainly
    /// used to prevent a launch of an erroneous (unwanted) attack.
    #[structopt(
        short = "w",
        long = "waiting",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "5secs",
        parse(try_from_str = "parse_duration")
    )]
    pub waiting: Duration,

    /// A periodicity of sending packets. The default value equals
    /// to zero seconds, that is, all packets will be sent
    /// momentarily.
    #[structopt(
        short = "p",
        long = "periodicity",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "0secs",
        parse(try_from_str = "parse_duration")
    )]
    pub periodicity: Duration,

    /// A file in which all logging information will be printed.
    /// Note that even with this option, logs will even still be
    /// written to a terminal too.
    #[structopt(
        short = "o",
        long = "output",
        takes_value = true,
        value_name = "FILENAME"
    )]
    pub output: Option<PathBuf>,
}

fn parse_length(length: &str) -> Result<usize, PacketLengthError> {
    let length: usize = length
        .parse()
        .map_err(|error| PacketLengthError::InvalidFormat(error))?;

    if length < MIN_PACKET_LENGTH {
        return Err(PacketLengthError::Underflow);
    } else if length > MAX_PACKET_LENGTH {
        return Err(PacketLengthError::Overflow);
    }

    Ok(length)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketLengthError {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ordinary_values() {
        // Check that ordinary values are parsed correctly
        assert_eq!(parse_length("53251"), Ok(53251));
        assert_eq!(parse_length("26655"), Ok(26655));
        assert_eq!(parse_length("+75"), Ok(75));

        // Check that the min and max values are still valid
        assert_eq!(parse_length("1"), Ok(MIN_PACKET_LENGTH));
        assert_eq!(parse_length("65000"), Ok(MAX_PACKET_LENGTH));
    }

    #[test]
    fn parses_invalid_values() {
        let panic_if_invalid = |string| {
            if let Ok(_) = parse_length(string) {
                panic!("Parses invalid formatted number correctly");
            }
        };

        // Invalid numbers must produce the invalid format error
        panic_if_invalid("   ");

        panic_if_invalid("abc5653odr!");
        panic_if_invalid("6485&02hde");

        panic_if_invalid("-565642");
        panic_if_invalid(&"2178".repeat(50));

        // Check that too big and too small values aren't valid
        assert_eq!(parse_length("0"), Err(PacketLengthError::Underflow));
        assert_eq!(
            parse_length("533987721456"),
            Err(PacketLengthError::Overflow)
        );
    }
}

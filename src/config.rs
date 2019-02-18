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
use std::num::{NonZeroUsize, ParseIntError};
use std::time::Duration;

use humantime::{format_duration, parse_duration};
use structopt::StructOpt;

#[derive(Debug, Eq, PartialEq, StructOpt)]
#[structopt(
    author = "Temirkhan Myrzamadi <gymmasssorla@gmail.com>",
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

    /// A size of each UDP-packet, specified in bytes. Note that
    /// your system or a victim server might not be able to handle
    /// the default value.
    #[structopt(
        short = "l",
        long = "length",
        takes_value = true,
        value_name = "BYTES",
        default_value = "65000",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub length: NonZeroUsize,

    /// A waiting time before an attack execution. It is mainly
    /// used to prevent a launch of an erroneous (unwanted) attack.
    #[structopt(
        short = "w",
        long = "wait",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "5secs",
        parse(try_from_str = "parse_duration")
    )]
    pub wait: Duration,

    /// A periodicity of sending packets. The default value equals
    /// to zero seconds, that is, all packets will be sent
    /// momentarily.
    #[structopt(
        long = "send-periodicity",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "0secs",
        parse(try_from_str = "parse_duration")
    )]
    pub send_periodicity: Duration,

    /// A count of packets per displaying attack summaries. It is
    /// not recommended to set this option to a small value (say, 6)
    /// for the performance reasons.
    #[structopt(
        long = "display-periodicity",
        takes_value = true,
        value_name = "PACKETS",
        default_value = "300",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub display_periodicity: NonZeroUsize,

    /// A count of packets for sending. The default value equals to
    /// the largest number available for the inner data type.
    #[structopt(
        short = "p",
        long = "packets",
        takes_value = true,
        value_name = "COUNT",
        default_value = "18446744073709551615",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub packets: NonZeroUsize,

    /// If sending calls will continue longer than this timeout, the
    /// program will exit with an error. By default, all the sending
    /// calls will continue indefinitely.
    #[structopt(
        long = "send-timeout",
        takes_value = true,
        value_name = "TIME-SPAN",
        parse(try_from_str = "parse_duration")
    )]
    pub send_timeout: Option<Duration>,

    /// Enable the debugging mode
    #[structopt(long = "debug")]
    pub debug: bool,
}

impl Display for ArgsConfig {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let send_timeout = match self.send_timeout {
            Some(given_timeout) => format_duration(given_timeout).to_string(),
            None => String::from("infinity"),
        };

        write!(
            fmt,
            "receiver: {receiver}, \
             sender: {sender}, \
             duration: {duration}, \
             length: {length}, \
             wait: {wait}, \
             send-periodicity: {send_periodicity}, \
             display-periodicity: {display_periodicity}, \
             packets: {packets}, \
             send-timeout: {send_timeout}, \
             debug: {debug}",
            receiver = self.receiver,
            sender = self.sender,
            duration = format_duration(self.duration),
            length = self.length,
            wait = format_duration(self.wait),
            send_periodicity = format_duration(self.send_periodicity),
            display_periodicity = self.display_periodicity,
            packets = self.packets,
            send_timeout = send_timeout,
            debug = self.debug,
        )
    }
}

pub fn parse_non_zero_usize(number: &str) -> Result<NonZeroUsize, NonZeroUsizeError> {
    let number: usize = number
        .parse()
        .map_err(|error| NonZeroUsizeError::InvalidFormat(error))?;

    NonZeroUsize::new(number).ok_or(NonZeroUsizeError::ZeroValue)
}

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
    fn parses_valid_non_zero_usize() {
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

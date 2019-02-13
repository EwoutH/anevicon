/* anevicon: An UDP-based server stress-testing tool, written in Rust.
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
use std::time::Duration;

use clap::ArgMatches;
use humantime::parse_duration;

pub const MIN_PACKET_LENGTH: usize = 1;
pub const MAX_PACKET_LENGTH: usize = 65000;

#[derive(Debug, Clone)]
pub struct ArgsConfig {
    receiver: SocketAddr,
    sender: SocketAddr,
    duration: Duration,
    length: usize,
    waiting: Duration,
    periodicity: Duration,
}

impl ArgsConfig {
    pub fn from_matches(matches: &ArgMatches) -> Result<ArgsConfig, ArgsConfigError> {
        Ok(ArgsConfig {
            receiver: ArgsConfig::parse_receiver(matches)?,
            sender: ArgsConfig::parse_sender(matches)?,
            duration: ArgsConfig::parse_duration(matches)?,
            length: ArgsConfig::parse_length(matches)?,
            waiting: ArgsConfig::parse_waiting(matches)?,
            periodicity: ArgsConfig::parse_periodicity(matches)?,
        })
    }

    fn parse_receiver(matches: &ArgMatches) -> Result<SocketAddr, ArgsConfigError> {
        Ok(matches
            .value_of("receiver")
            .unwrap()
            .parse()
            .map_err(|_| ArgsConfigError::Receiver)?)
    }

    fn parse_sender(matches: &ArgMatches) -> Result<SocketAddr, ArgsConfigError> {
        Ok(matches
            .value_of("sender")
            .unwrap()
            .parse()
            .map_err(|_| ArgsConfigError::Sender)?)
    }

    fn parse_duration(matches: &ArgMatches) -> Result<Duration, ArgsConfigError> {
        Ok(parse_duration(matches.value_of("duration").unwrap())
            .map_err(|_| ArgsConfigError::Duration)?)
    }

    fn parse_length(matches: &ArgMatches) -> Result<usize, ArgsConfigError> {
        let length = matches
            .value_of("length")
            .unwrap()
            .parse()
            .map_err(|_| ArgsConfigError::Length)?;

        if (length < MIN_PACKET_LENGTH) || (length > MAX_PACKET_LENGTH) {
            Err(ArgsConfigError::Length)
        } else {
            Ok(length)
        }
    }

    fn parse_waiting(matches: &ArgMatches) -> Result<Duration, ArgsConfigError> {
        Ok(parse_duration(matches.value_of("waiting").unwrap())
            .map_err(|_| ArgsConfigError::Waiting)?)
    }

    fn parse_periodicity(matches: &ArgMatches) -> Result<Duration, ArgsConfigError> {
        Ok(parse_duration(matches.value_of("periodicity").unwrap())
            .map_err(|_| ArgsConfigError::Periodicity)?)
    }
}

#[derive(Debug, Clone)]
pub enum ArgsConfigError {
    Receiver,
    Sender,
    Duration,
    Length,
    Waiting,
    Periodicity,
}

impl Display for ArgsConfigError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ArgsConfigError::Receiver => write!(fmt, "An invalid receiver address was specified!"),
            ArgsConfigError::Sender => write!(fmt, "An invalid sender address was specified!"),
            ArgsConfigError::Duration => write!(fmt, "An invalid duration format was specified!"),
            ArgsConfigError::Length => write!(fmt, "An invalid packet length was specified!"),
            ArgsConfigError::Waiting => write!(fmt, "An invalid waiting duration was specified!"),
            ArgsConfigError::Periodicity => {
                write!(fmt, "An invalid periodicity format was specified!")
            }
        }
    }
}

impl Error for ArgsConfigError {}

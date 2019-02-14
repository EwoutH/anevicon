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

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use humantime::parse_duration;
use structopt::StructOpt;

pub const MIN_PACKET_LENGTH: usize = 1;
pub const MAX_PACKET_LENGTH: usize = 65000;

#[derive(Debug, Clone, StructOpt)]
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
    receiver: SocketAddr,

    /// A sender of generated traffic, specified as an IP-address
    /// and a port number, separated by the colon character.
    #[structopt(
        short = "s",
        long = "sender",
        takes_value = true,
        value_name = "ADDRESS",
        default_value = "0.0.0.0:0"
    )]
    sender: SocketAddr,

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
    duration: Duration,

    /// A size of each UDP-packet in the range of [1; 65000],
    /// specified in bytes. Note that your system or a victim server
    /// might not be able to handle the default value.
    #[structopt(
        short = "l",
        long = "length",
        takes_value = true,
        value_name = "BYTES",
        default_value = "65000"
    )]
    length: usize,

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
    waiting: Duration,

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
    periodicity: Duration,

    /// A file in which all logging information will be printed.
    /// Note that even with this option, logs will even still be
    /// written to a terminal too.
    #[structopt(
        short = "o",
        long = "output",
        takes_value = true,
        value_name = "FILENAME"
    )]
    output: Option<PathBuf>,
}

impl ArgsConfig {
    pub fn receiver(&self) -> SocketAddr {
        self.receiver
    }

    pub fn sender(&self) -> SocketAddr {
        self.sender
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn waiting(&self) -> Duration {
        self.waiting
    }

    pub fn periodicity(&self) -> Duration {
        self.periodicity
    }

    pub fn output(&self) -> &Option<PathBuf> {
        &self.output
    }
}

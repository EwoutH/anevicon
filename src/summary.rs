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

use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

use humantime::format_duration;

#[derive(Debug, Clone)]
pub struct AttackSummary {
    bytes_sent: usize,
    packets_sent: usize,
    initial_time: Instant,
}

impl AttackSummary {
    pub fn new() -> AttackSummary {
        AttackSummary {
            bytes_sent: 0,
            packets_sent: 0,
            initial_time: Instant::now(),
        }
    }

    pub fn update(&mut self, additional_bytes: usize, additional_packets: usize) {
        self.bytes_sent += additional_bytes;
        self.packets_sent += additional_packets;
    }

    pub fn megabytes_sent(&self) -> usize {
        self.bytes_sent / 1024
    }

    pub fn packets_sent(&self) -> usize {
        self.packets_sent
    }

    pub fn time_passed(&self) -> Duration {
        self.initial_time.elapsed()
    }

    pub fn megabytes_in_sec(&self) -> usize {
        let secs_passed = self.time_passed().as_secs() as usize;

        if secs_passed == 0 {
            0
        } else {
            self.megabytes_sent() / secs_passed
        }
    }
}

impl Display for AttackSummary {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "{} packets with {}MB and the average speed of {} MB/s were sent just in {}",
            self.packets_sent(),
            self.megabytes_sent(),
            self.megabytes_in_sec(),
            format_duration(self.time_passed())
        )
    }
}

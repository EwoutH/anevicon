/* anevicon: The fastest UDP-based server stress-testing tool, written in Rust.
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
        self.bytes_sent / 1024 / 1024
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn is_nondecreasing_clock() {
        let summary = AttackSummary::new();

        let mut last_time = summary.time_passed();
        for _ in 0..50 {
            // Wait for about 30 milliseconds to see real results
            sleep(Duration::from_millis(30));

            let current_time = summary.time_passed();

            // Check that out clock is monotonically nondecreasing
            assert!(last_time <= current_time);

            last_time = current_time;
        }
    }

    #[test]
    fn is_correct_initial_value() {
        let summary = AttackSummary::new();

        assert_eq!(summary.megabytes_sent(), 0);
        assert_eq!(summary.packets_sent(), 0);
        assert_eq!(summary.megabytes_in_sec(), 0);
    }

    #[test]
    fn ordinary_updates_work() {
        let mut summary = AttackSummary::new();

        summary.update(1024 * 1024 * 23, 2698);
        assert_eq!(summary.megabytes_sent(), 23);
        assert_eq!(summary.packets_sent(), 2698);

        summary.update(1024 * 1024 * 85, 4258);
        assert_eq!(summary.megabytes_sent(), 85 + 23);
        assert_eq!(summary.packets_sent(), 2698 + 4258);
    }

    #[test]
    fn truncate_megabytes_correctly() {
        let mut summary = AttackSummary::new();

        summary.update(1024 * 1023, 5338);
        assert_eq!(
            summary.megabytes_sent(),
            0,
            "'AttackSummary' truncates megabytes incorrectly"
        );
        assert_eq!(summary.packets_sent(), 5338);

        // However, we must have one megabyte sent after this update
        summary.update(1024, 19);
        assert_eq!(summary.megabytes_sent(), 1);
    }

    #[test]
    fn zero_update_works() {
        let mut summary = AttackSummary::new();
        summary.update(1024 * 1024 * 85, 2698);

        summary.update(0, 0);
        assert_eq!(
            summary.megabytes_sent(),
            85,
            "'AttackSummary' hasn't the same megabytes after zero-update"
        );
        assert_eq!(
            summary.packets_sent(),
            2698,
            "'AttackSummary' hasn't the same packets after zero-update"
        );
    }

    #[test]
    fn time_passed_works() {
        let mut summary = AttackSummary::new();
        let initial_time = Instant::now();

        // Do an arbitrary updates and sleep that take some time
        for _ in 0..100 {
            summary.update(1024 * 1024 * 563, 54138);
            sleep(Duration::from_millis(20));
        }

        assert!(summary.time_passed() <= initial_time.elapsed());
    }
}

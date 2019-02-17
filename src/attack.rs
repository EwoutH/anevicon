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

use std::io;
use std::net::UdpSocket;
use std::num::NonZeroUsize;
use std::thread;

use super::config::ArgsConfig;
use super::summary::AttackSummary;

use log::info;
use rand::{thread_rng, RngCore};

#[derive(Debug)]
pub struct Attacker<'a> {
    socket: UdpSocket,
    buffer: Vec<u8>,
    args_config: &'a ArgsConfig,
}

impl<'a> Attacker<'a> {
    pub fn from_args_config(args_config: &'a ArgsConfig) -> io::Result<Attacker<'a>> {
        // Complete any necessary stuff with the specified socket
        let socket = UdpSocket::bind(args_config.sender)?;
        socket.connect(args_config.receiver)?;
        socket.set_write_timeout(args_config.send_timeout)?;

        Ok(Attacker {
            socket,
            buffer: Attacker::random_buffer(args_config.length),
            args_config,
        })
    }

    fn random_buffer(length: NonZeroUsize) -> Vec<u8> {
        // Create a sending buffer without an unnecessary initialization
        // because we'll fill this buffer with random values next.
        let mut buffer = Vec::with_capacity(length.get());
        unsafe {
            buffer.set_len(length.get());
        }

        thread_rng().fill_bytes(buffer.as_mut_slice());
        buffer
    }

    pub fn attack(&self) -> io::Result<AttackSummary> {
        info!(
            "The program is starting to attack with {}.",
            self.args_config
        );

        thread::sleep(self.args_config.wait);
        let mut summary = AttackSummary::new();

        loop {
            for _ in 0..self.args_config.display_periodicity.get() {
                summary.update(self.socket.send(&self.buffer)?, 1);

                if self.check_end_cond(&summary) {
                    return Ok(summary);
                }

                thread::sleep(self.args_config.send_periodicity);
            }

            info!("The attack is running with {}.", summary);
        }
    }

    fn check_end_cond(&self, summary: &AttackSummary) -> bool {
        if summary.time_passed() >= self.args_config.duration {
            info!(
                "The program is stopping the packet sending because \
                 the allotted time has passed. The total result is: {}.",
                summary
            );
            return true;
        }
        if summary.packets_sent() == self.args_config.packets.get() {
            info!(
                "The program is stopping the packet sending because \
                 all the required packets were sent. The total result is: {}.",
                summary
            );
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use structopt::StructOpt;

    fn setup_server() -> UdpSocket {
        UdpSocket::bind("0.0.0.0:0")
            .expect("Cannot setup the testing server with the address 0.0.0.0:0")
    }

    fn default_config(server: &UdpSocket) -> ArgsConfig {
        let server_addr = server
            .local_addr()
            .expect("Cannot get the testing server local address")
            .to_string();

        // The first command-line argument doesn't have any meaning for CLAP
        ArgsConfig::from_iter_safe(vec!["anevicon", "--receiver", &server_addr])
            .expect("The command-line arguments are incorrectly specified")
    }

    #[test]
    fn generates_random_buffer() {
        let length = unsafe { NonZeroUsize::new_unchecked(35684) };
        let buffer = Attacker::random_buffer(length);

        // Check that we've got the correctly length and capacity
        assert_eq!(buffer.len(), length.get());
        assert!(buffer.capacity() >= length.get());
    }

    #[test]
    fn sends_all_packets() {
        // Assign a very low required packets count to prevent our
        // lovely Travis CI and your computer for a shameful breaking :)))
        const REQUIRED_PACKETS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(25) };

        // Setup the testing server and modify the default config
        let server = setup_server();
        let mut config = default_config(&server);
        config.packets = REQUIRED_PACKETS;

        assert_eq!(
            Attacker::from_args_config(&config)
                .expect("Cannot setup the testing attacker")
                .attack()
                .expect("The testing attack has returned an error")
                .packets_sent(),
            REQUIRED_PACKETS.get()
        );
    }
}

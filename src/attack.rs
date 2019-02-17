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

        // Generate a random set of bytes into the sending buffer
        let mut buffer = vec![0; args_config.length];
        thread_rng().fill_bytes(buffer.as_mut_slice());

        Ok(Attacker {
            socket,
            buffer,
            args_config,
        })
    }

    pub fn attack(&self) -> io::Result<AttackSummary> {
        info!(
            "The program is starting to attack with {}. Press <CTRL+C> to exit.",
            self.args_config
        );

        thread::sleep(self.args_config.wait);
        let mut summary = AttackSummary::new();

        loop {
            for _ in 0..self.args_config.display_periodicity.get() {
                summary.update(self.socket.send(&self.buffer)?, 1);

                if summary.time_passed() >= self.args_config.duration {
                    info!(
                        "The program is stopping the packet sending because \
                         the allotted time has passed. The total result is: {}.",
                        summary
                    );
                    return Ok(summary);
                }
                if summary.packets_sent() == self.args_config.packets.get() {
                    info!(
                        "The program is stopping the packet sending because \
                         all the required packets were sent. The total result is: {}.",
                        summary
                    );
                    return Ok(summary);
                }

                thread::sleep(self.args_config.send_periodicity);
            }

            info!("The attack is running with {}.", summary);
        }
    }
}

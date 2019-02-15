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

use humantime::format_duration;
use log::{error, info};
use rand::{thread_rng, RngCore};

const ATTACKER_DISPLAY_CYCLICALITY: usize = 500;

#[derive(Debug)]
pub struct Attacker<'a> {
    socket: UdpSocket,
    buffer: Vec<u8>,
    args_config: &'a ArgsConfig,
}

impl<'a> Attacker<'a> {
    pub fn from_args_config(args_config: &'a ArgsConfig) -> io::Result<Attacker<'a>> {
        let socket = UdpSocket::bind(args_config.sender)?;
        socket.connect(args_config.receiver)?;

        let mut buffer = vec![0; args_config.length];
        thread_rng().fill_bytes(buffer.as_mut_slice());

        Ok(Attacker {
            socket,
            buffer,
            args_config,
        })
    }

    pub fn attack(&self) {
        let mut summary = AttackSummary::new();
        self.display_entry_message();
        thread::sleep(self.args_config.waiting);

        loop {
            for _ in 0..ATTACKER_DISPLAY_CYCLICALITY {
                match self.socket.send(&self.buffer) {
                    Err(error) => {
                        error!("Cannot send the packet to the receiver: {}", error);
                    }
                    Ok(bytes_sent) => {
                        summary.update(bytes_sent, 1);
                    }
                }

                if summary.time_passed() >= self.args_config.duration {
                    info!("Stopping the packet sending due to the expired time");
                    return;
                }

                thread::sleep(self.args_config.periodicity);
            }

            info!("{}", summary);
        }
    }

    fn display_entry_message(&self) {
        let output = match self.args_config.output {
            Some(ref filename) => String::from(
                filename
                    .as_path()
                    .to_str()
                    .expect("The output file contains invalid unicode"),
            ),
            None => String::from("<unspecified output>"),
        };

        info!(
            "The program is starting to attack using receiver: {receiver}, \
             sender: {sender}, length: {length}, periodicity: {periodicity}, \
             duration: {duration}, waiting: {waiting}, output: {output}",
            receiver = self.args_config.receiver,
            sender = self.args_config.sender,
            length = self.args_config.length,
            periodicity = format_duration(self.args_config.periodicity),
            duration = format_duration(self.args_config.duration),
            waiting = format_duration(self.args_config.waiting),
            output = output
        );
    }
}

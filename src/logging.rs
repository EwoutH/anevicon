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

use std::io::{stderr, stdout};

use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::Level;
use time::{self, strftime};

pub fn setup_logging(debug: bool) {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta)
        .trace(Color::Cyan);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{anevicon}] {level} [{date_time}]: {message}",
                anevicon = "anevicon".magenta().bold(),
                level = colors.color(record.level()).to_string().underline(),
                date_time = strftime("%x %X %z", &time::now()).unwrap().cyan(),
                message = message,
            ));
        })
        // Print all traces and debugging information to stderr
        .chain(
            Dispatch::new()
                .filter(move |metadata| {
                    if !debug {
                        return false;
                    }

                    match metadata.level() {
                        Level::Info | Level::Warn | Level::Error => false,
                        Level::Debug | Level::Trace => true,
                    }
                })
                .chain(stderr()),
        )
        // Print all notifications, warnings and errors to stdout
        .chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => true,
                    Level::Debug | Level::Trace => false,
                })
                .chain(stdout()),
        )
        .apply()
        .expect("Cannot correctly setup the logging system");
}

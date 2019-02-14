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

use std::fmt::Arguments;
use std::io::{self, stderr, stdout};
use std::path::PathBuf;

use fern::colors::{Color, ColoredLevelConfig};
use fern::{log_file, Dispatch, FormatCallback};
use log::{Level, Record};
use termion::color::{self, Cyan, Fg};
use termion::style::{self, Underline};
use time::{self, strftime};

pub fn setup_logging(output: &Option<PathBuf>) -> io::Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta)
        .trace(Color::Cyan);

    // The terminal formatter for user messages and debugging
    let fmt = move |out: FormatCallback, message: &Arguments, record: &Record| {
        out.finish(format_args!(
            "{style}{level}{no_style} [{cyan}{date_time}{no_cyan}]: {message}",
            style = Underline,
            level = colors.color(record.level()),
            no_style = style::Reset,
            cyan = Fg(Cyan),
            date_time = strftime("%x %X %z", &time::now()).unwrap(),
            no_cyan = Fg(color::Reset),
            message = message,
        ));
    };

    let mut dispatch = Dispatch::new()
        // Print all notices, warnings, and errors to stdout
        .chain(
            Dispatch::new()
                .format(fmt)
                .filter(|metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => true,
                    Level::Debug | Level::Trace => false,
                })
                .chain(stdout()),
        )
        // Print all traces and debugging information to stderr
        .chain(
            Dispatch::new()
                .format(fmt)
                .filter(|metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => false,
                    Level::Debug | Level::Trace => true,
                })
                .chain(stderr()),
        );

    // Add an output logging file if it was specified by a user
    if let Some(filename) = output {
        dispatch = dispatch.chain(
            Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[anevicon] {level} [{date_time}]: {message}",
                        level = record.level(),
                        date_time = strftime("%x %X %z", &time::now()).unwrap(),
                        message = message,
                    ));
                })
                .chain(log_file(filename)?),
        );
    }

    dispatch
        .apply()
        .expect("Cannot correctly setup the logging system");
    Ok(())
}

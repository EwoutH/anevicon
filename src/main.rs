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

use colored::Colorize;
use time::{self, strftime};

use config::ArgsConfig;
use logging::setup_logging;
use options::setup_options;

mod config;
mod logging;
mod options;

fn main() {
    let matches = setup_options();

    let config = match ArgsConfig::from_matches(&matches) {
        Ok(config) => config,
        Err(error) => {
            raw_exit_with_error(format_args!("{}", error));
        }
    };

    if let Err(error) = setup_logging(matches.value_of("output")) {
        raw_exit_with_error(format_args!("{}", error));
    }

    display_title();
}

fn raw_exit_with_error(message: Arguments) -> ! {
    println!(
        "{level} [{time}]: {message}",
        level = "ERROR".underline().red(),
        time = strftime("%x %X %z", &time::now()).unwrap().cyan(),
        message = message,
    );
    std::process::exit(1)
}

fn display_title() {
    println!(
        "       {}",
        r"                        _                 ".red()
    );
    println!(
        "       {}",
        r"  __ _ _ __   _____   _(_) ___ ___  _ __  ".red()
    );
    println!(
        "       {}",
        r" / _` | '_ \ / _ \ \ / / |/ __/ _ \| '_ \ ".red()
    );
    println!(
        "       {}",
        r"| (_| | | | |  __/\ V /| | (_| (_) | | | |".red()
    );
    println!(
        "       {}",
        r" \__,_|_| |_|\___| \_/ |_|\___\___/|_| |_|".red()
    );
    println!(
        "{}",
        "An UDP-based server stress-testing tool, written in Rust\n"
            .yellow()
            .underline()
    );
}

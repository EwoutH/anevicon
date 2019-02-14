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
use structopt::StructOpt;

use config::{ArgsConfig, MAX_PACKET_LENGTH, MIN_PACKET_LENGTH};
use logging::setup_logging;

mod config;
mod logging;

fn main() {
    let config = ArgsConfig::from_args();

    if config.length() < MIN_PACKET_LENGTH {
        option_error("--length <BYTES>", format_args!("The value is too small"));
    } else if config.length() > MAX_PACKET_LENGTH {
        option_error("--length <BYTES>", format_args!("The value is too big"));
    }

    if let Err(error) = setup_logging(config.output()) {
        option_error("--output <FILENAME>", format_args!("{}", error));
    }

    display_title();
}

// Prints CLAP-like error message to the standard error stream and then
// exists the current process with the exit code of 1.
fn option_error(option: &str, message: Arguments) {
    eprintln!(
        "{error} Invalid value for '{option}': {message}",
        error = "error:".bold().red(),
        option = option.yellow(),
        message = message
    );
    std::process::exit(1);
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

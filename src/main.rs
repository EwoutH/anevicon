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

use colored::Colorize;
use structopt::StructOpt;

use config::ArgsConfig;
use logging::{raw_exit_with_error, setup_logging};

mod config;
mod logging;
mod summary;

fn main() {
    let config = ArgsConfig::from_args();

    if let Err(error) = setup_logging(&config.output) {
        raw_exit_with_error(format_args!("Cannot open the output file: {}", error));
    }

    display_title();
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

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

use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use time::{self, strftime};

use config::ArgsConfig;
use logging::setup_logging;

mod config;
mod logging;

fn main() {
    // If this line succeeds, then we'll have correct options specified
    let matches = setup_options();

    // If this line succeeds, then we'll have a correct configuration
    let config = match ArgsConfig::from_matches(&matches) {
        Ok(config) => config,
        Err(error) => {
            raw_exit_with_error(format_args!("{}", error));
        }
    };

    // If this line succeeds, then we'll have a configured logging system
    if let Err(error) = setup_logging(matches.value_of("output")) {
        raw_exit_with_error(format_args!("{}", error));
    }

    // Finally, display the title because we have all things configured
    // correctly
    display_title();
}

// This function imitates a fancy log message even without a configured
// logging
fn raw_exit_with_error(message: Arguments) -> ! {
    println!(
        "{level} [{time}]: {message}",
        level = "ERROR".underline().red(),
        time = strftime("%x %X %z", &time::now()).unwrap().cyan(),
        message = message,
    );
    std::process::exit(1)
}

// This function returns correct argument matches and fails if those
// are incorrect
fn setup_options<'a>() -> ArgMatches<'a> {
    App::new("anevicon")
        .author("Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>")
        .about("An UDP-based server stress-testing tool, written in Rust.")
        .version("0.1.0")
        .set_term_width(80)
        .arg(
            Arg::with_name("receiver")
                .short("r")
                .long("receiver")
                .takes_value(true)
                .value_name("ADDRESS")
                .required(true)
                .help(
                    "A receiver of generated traffic, specified as an IP-address \
                     and a port number, separated by the colon character.",
                ),
        )
        .arg(
            Arg::with_name("sender")
                .short("s")
                .long("sender")
                .takes_value(true)
                .value_name("ADDRESS")
                .default_value("0.0.0.0:0")
                .help(
                    "A sender of generated traffic, specified as an IP-address \
                     and a port number, separated by the colon character.",
                ),
        )
        .arg(
            Arg::with_name("duration")
                .short("d")
                .long("duration")
                .takes_value(true)
                .value_name("TIME-SPAN")
                .default_value("64years 64hours 64secs")
                .help(
                    "A program working time. The default value is too big, that \
                     is, an attack will be performed until you explicitly stop \
                     the process.",
                ),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .takes_value(true)
                .value_name("BYTES")
                .default_value("65000")
                .help(
                    "A size of each UDP-packet in the range of [1; 65000], \
                     specified in bytes. Note that your system or a victim server \
                     might not be able to handle the default value.",
                ),
        )
        .arg(
            Arg::with_name("waiting")
                .short("w")
                .long("waiting")
                .takes_value(true)
                .value_name("TIME-SPAN")
                .default_value("5secs")
                .help(
                    "A waiting time before an attack execution. It is mainly \
                     used to prevent a launch of an erroneous (unwanted) attack.",
                ),
        )
        .arg(
            Arg::with_name("periodicity")
                .short("p")
                .long("periodicity")
                .takes_value(true)
                .value_name("TIME-SPAN")
                .default_value("0secs")
                .help(
                    "A periodicity of sending packets. The default value equals \
                     to zero seconds, that is, all packets will be sent \
                     momentarily.",
                ),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILENAME")
                .help(
                    "A file in which all logging information will be printed. \
                     Note that even with this option, logs will even still be \
                     written to a terminal too.",
                ),
        )
        .after_help("For more information see <https://github.com/Gymmasssorla/anevicon>.")
        .get_matches()
}

// This function displays the fancy application title with the
// underlined description
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

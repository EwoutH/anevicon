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

use std::io::stdout;

use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::error;
use termion::color::{Cyan, Fg, Reset};
use termion::style::{NoUnderline, Underline};
use time::{self, strftime};

use config::ArgsConfig;

mod config;

fn main() {
    setup_logging();

    let config = match ArgsConfig::from_matches(&setup_options()) {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error);
            std::process::exit(1);
        }
    };

    display_title();
}

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
                    "A size of each UDP-packet, specified in bytes. Note that \
                     your system or a victim server might not be able to handle \
                     the default value.",
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
        .after_help("For more information see <https://github.com/Gymmasssorla/anevicon>.")
        .get_matches()
}

fn setup_logging() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta)
        .trace(Color::Cyan);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{underline}{level}{no_underline} [{cyan}{date_time}{no_cyan}]: {message}",
                underline = Underline,
                level = colors.color(record.level()),
                no_underline = NoUnderline,
                cyan = Fg(Cyan),
                date_time = strftime("%x %X %z", &time::now()).unwrap(),
                no_cyan = Fg(Reset),
                message = message,
            ))
        })
        .chain(stdout())
        .apply()
        .expect("Cannot correctly setup the logging system");
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

/* anevicon: An UDP-based server stress-testing tool, written in Rust
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

use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use termion::color::{Cyan, Fg, Reset};
use termion::style::{NoUnderline, Underline};
use time::{self, strftime};

fn main() {
    setup_logging();
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

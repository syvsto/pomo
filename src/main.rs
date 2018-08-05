#[macro_use]
extern crate quicli;

use quicli::prelude::*;
use std::process::Command;
use std::{thread, time, fmt};

const ERROR_MESSAGE: &'static str =
    "Couldn't run notify-send. Is it installed and on the system path?";
const SHORTBREAK_MESSAGE: &'static str = "Break time!";
const LONGBREAK_MESSAGE: &'static str = "Looooong Break time!";
const WORK_MESSAGE: &'static str = "Work work!";
const TITLE_MESSAGE: &'static str = "Pomo";
const STARTUP_MESSAGE: &'static str = "Time to focus!";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(
        long = "worktime",
        short = "w",
        default_value = "25",
        help = "Length of each work session in minutes"
    )]
    worktime: usize,
    #[structopt(
        long = "shortbreak",
        short = "s",
        default_value = "5",
        help = "Length of each short break in minutes"
    )]
    shortbreak: usize,
    #[structopt(
        long = "longbreak",
        short = "l",
        default_value = "20",
        help = "Length of each long break in minutes"
    )]
    longbreak: usize,
    #[structopt(
        long = "numbreaks",
        short = "n",
        default_value = "4",
        help = "The number of short breaks before a long one. Set to 0 to disable long breaks"
    )]
    number_of_short_breaks: usize,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[derive(Debug, PartialEq)]
enum State {
    ShortBreak,
    LongBreak,
    Work,
    Initial,
}

impl fmt::Display for State {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       use State::*;
       match self {
		ShortBreak => write!(f, "{} {}", TITLE_MESSAGE, SHORTBREAK_MESSAGE),
		LongBreak => write!(f, "{} {}", TITLE_MESSAGE, LONGBREAK_MESSAGE),
		Work => write!(f, "{} {}", TITLE_MESSAGE, WORK_MESSAGE),
		Initial => write!(f, "{} {}", TITLE_MESSAGE, STARTUP_MESSAGE),
       }
   }
}

main!(|args: Cli, log_level: verbosity| {
    use State::*;
    let short_break_duration = time::Duration::from_millis(1000 * 60 * args.shortbreak as u64);
    let long_break_duration = time::Duration::from_millis(1000 * 60 * args.longbreak as u64);
    let work_duration = time::Duration::from_millis(1000 * 60 * args.worktime as u64);

    let using_long_breaks = if args.number_of_short_breaks != 0 {
        Some(args.number_of_short_breaks * 2)
    } else {
        None
    };
    let mut counter = 1;
    let mut state = Initial;

    println!("The clock is ticking! Using the following settings:\nWork time: {} mins\nShort breaks: {} mins\nLong breaks: {} mins\n",
            work_duration.as_secs() / 60, short_break_duration.as_secs() / 60, long_break_duration.as_secs() / 60);

    loop {
        let text = format!("{}", &state);
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &format!("notify-send {}", text)])
                .output()
                .expect(ERROR_MESSAGE);
        } else {
            Command::new("sh")
                .args(&["-c", &format!("notify-send.exe {}", text)])
                .output()
                .expect(ERROR_MESSAGE);
        }

        if using_long_breaks.is_some() && counter % using_long_breaks.unwrap() == 0 && state == Work
        {
            info!("Taking a long break");
            state = LongBreak;
            thread::sleep(long_break_duration);
        } else if state == Work {
            info!("Taking a short break");
            state = ShortBreak;
            thread::sleep(short_break_duration);
        } else {
            info!("Working");
            state = Work;
            thread::sleep(work_duration);
        }

        counter += 1;
    }
});

#[macro_use]
extern crate quicli;

use quicli::prelude::*;
use std::process::Command;
use std::{thread, time};

const ERROR_MESSAGE: &'static str =
    "Couldn't run notify-send. Is it installed and on the system path?";
const SHORTBREAK_MESSAGE: &'static str = "Break time!";
const LONGBREAK_MESSAGE: &'static str = "Looooong Break time!";
const WORK_MESSAGE: &'static str = "Work work!";
const TITLE_MESSAGE: &'static str = "Pomo";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "worktime", short = "w", default_value = "25")]
    worktime: usize,
    #[structopt(long = "shortbreak", short = "s", default_value = "5")]
    shortbreak: usize,
    #[structopt(long = "longbreak", short = "l", default_value = "20")]
    longbreak: usize,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[derive(Debug, PartialEq)]
enum State {
    LongBreak,
    ShortBreak,
    Work,
    Initial,
}

main!(|args: Cli, log_level: verbosity| {
    use State::*;
    let short_break_duration = time::Duration::from_millis(1000 * 60 * args.shortbreak as u64);
    let long_break_duration = time::Duration::from_millis(1000 * 60 * args.longbreak as u64);
    let work_duration = time::Duration::from_millis(1000 * 60 * args.worktime as u64);

    let mut counter = 0;
    let mut state = Initial;

    loop {
        if counter % 8 == 0 && state == Work {
            state = LongBreak;
            thread::sleep(long_break_duration);
            info!("Taking a long break");
        } else if state == Work {
            state = ShortBreak;
            thread::sleep(short_break_duration);
            info!("Taking a short break");
        } else {
            state = Work;
            thread::sleep(work_duration);
            info!("Working");
        }

        let text = match state {
            LongBreak => format!("{} {}", TITLE_MESSAGE, LONGBREAK_MESSAGE),
            ShortBreak => format!("{} {}", TITLE_MESSAGE, SHORTBREAK_MESSAGE),
            Work => format!("{} {}", TITLE_MESSAGE, WORK_MESSAGE),
            Initial => unreachable!(),
        };
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

        counter += 1;
    }
});

#[macro_use]
extern crate quicli;

use quicli::prelude::*;
use std::process::Command;
use std::{thread, time};

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
     let shortbreak_text = "Break time!";
     let longbreak_text = "Looooong Break time!";
     let work_text = "Work work!";
     let title = "Pomo";
     let short_break = time::Duration::from_millis(1000 * 60 * args.shortbreak as u64);
     let long_break = time::Duration::from_millis(1000 * 60 * args.longbreak as u64);
     let work = time::Duration::from_millis(1000 * 60 * args.worktime as u64);

	 let mut counter = 0;
	 let mut state = Initial;

     loop {
         if counter % 8 == 0 && state == Work {
             state = LongBreak;
             thread::sleep(long_break);
         } else if state == Work {
             state = ShortBreak;
             thread::sleep(short_break);
         } else {
             state = Work;
             thread::sleep(work);
         }


        let text = match state {
            LongBreak => format!("{} {}", title, longbreak_text),
            ShortBreak => format!("{} {}", title, shortbreak_text),
            Work => format!("{} {}", title, work_text),
            Initial => unreachable!()
        };
         if cfg!(target_os = "windows") {
             Command::new("cmd")
             	.args(&["/C", &format!("notify-send {}", text)])
             	.output()
             	.expect("Couldn't run notify-send");
         } else {
             Command::new("sh")
             	.args(&["-c", &format!("notify-send.exe {}", text)])
             	.output()
             	.expect("Couldn't run notify-send");
         }
         
       counter += 1;  
     }
});

use std::env;
use std::ops::Add;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};
use humantime;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut  args: Vec<String> = env::args().collect();
    // NOTE: first argument is the program name
    args.remove(0);

    if args.len() <= 0 {
        eprintln!("No duration specified");
        exit(1)
    }

    let duration_input = args.join(" ");
    let duration = humantime::parse_duration(duration_input.as_str());
    let duration = match duration {
        Ok(duration) => {
            duration
        }
        Err(_) => {
            eprintln!("Invalid duration");
            exit(1)
        }
    };
    let complete_at = Instant::now().add(duration);
    let duration_in_units = duration.as_millis() as u64;

    let bar = ProgressBar::new(duration_in_units);
    bar.set_style(ProgressStyle::with_template("{wide_bar} {msg}").unwrap());
    bar.set_message(humantime::format_duration(remainder(complete_at)).to_string());
    bar.set_position(duration_in_units);

    while Instant::now() < complete_at {
        let remaining = remainder(complete_at);
        let msg = if remaining >= Duration::from_secs(1) {
            humantime::format_duration(
                Duration::from_secs(remaining.as_secs())
            ).to_string()
        } else {
            humantime::format_duration(
                Duration::from_millis(remaining.as_millis() as u64)
            ).to_string()
        };
        bar.set_message(msg);
        bar.set_position(duration_in_units - (remaining.as_millis() as u64));
        sleep(Duration::from_millis(10));
    }

    bar.finish_and_clear()
}

fn remainder(complete_at: Instant) -> Duration {
    complete_at - Instant::now()
}



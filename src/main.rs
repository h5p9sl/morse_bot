#![feature(test)]

use clap::{crate_version, Arg};
use inputbot::MouseButton;
use std::time::Duration;

mod morse;

fn main() {
    let matches = clap::App::new("Morse Bot")
        .version(crate_version!())
        .args(&[
            Arg::with_name("MESSAGE")
                .required_unless("file")
                ,
            Arg::with_name("wpm")
                .default_value("15")
                .help("Words per minute - how fast the virtual keyer goes")
                .long_help("Words per minute - how long a single time unit is (one 'dit'/'.'), which defines how fast the virtual keyer \"types\" out the morse code message.")
                .short("w")
                .long("wpm")
                .takes_value(true)
                ,
            Arg::with_name("delay")
                .help("Delay before starting")
                .long_help("Specifies how long the program should wait before starting the virtual keyer (the playback of the morse code)")
                .short("d")
                .long("delay")
                .takes_value(true)
                ,
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Load message from file")
                .takes_value(true)
        ])
        .after_help("To view a summary of a flag/option, use `-h`. To view a detailed description of a flag/option, use `--help`")
        .get_matches();

    let mut msg = String::from(matches.value_of("MESSAGE").unwrap_or(""));
    let alphabet = morse::Alphabet::new();

    use std::str::FromStr;
    if let Some(delay) = matches.value_of("delay") {
        std::thread::sleep(Duration::from_secs_f32(f32::from_str(delay).unwrap()));
    }
    if let Some(file) = matches.value_of("file") {
        msg = std::fs::read_to_string(file).unwrap();
    }
    let wpm = u32::from_str(matches.value_of("wpm").unwrap()).unwrap();
    morse::Executor::new()
        .with_message(alphabet.to_morse(msg))
        .with_wpm(wpm)
        .with_callback(|state| {
            if state == morse::State::Down {
                MouseButton::LeftButton.press();
            } else {
                MouseButton::LeftButton.release();
            }
        })
        .execute();
}

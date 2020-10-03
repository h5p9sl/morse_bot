#![feature(test)]

use inputbot::MouseButton;
use std::{str::FromStr, time::Duration};

mod app;
mod morse;

fn main() {
    let matches = app::create_app();
    let alphabet = morse::Alphabet::new();

    let msg = {
        if let Some(file) = matches.value_of("file") {
            std::fs::read_to_string(file).unwrap()
        } else {
            matches.value_of("MESSAGE").unwrap().to_string()
        }
    };

    // Wait for `delay` seconds if specified in app args
    if let Some(delay) = matches.value_of("delay") {
        std::thread::sleep(Duration::from_secs_f32(f32::from_str(delay).unwrap()));
    }

    let wpm = u32::from_str(matches.value_of("wpm").unwrap()).unwrap();

    let button =
        MouseButton::OtherButton(u32::from_str(matches.value_of("button").unwrap()).unwrap());

    morse::Executor::new()
        .with_message(alphabet.to_morse(msg))
        .with_wpm(wpm)
        .with_callback(move |state| {
            if state == morse::State::Down {
                button.press();
            } else {
                button.release();
            }
        })
        .execute();
}

#![feature(test)]

#[macro_use]
extern crate log;

use inputbot::{MouseButton, KeybdKey};
use std::{str::FromStr, time::Duration};

mod app;
mod morse;

fn main() {
    let _ = app::init_logger();
    let matches = app::create_app();
    let alphabet = morse::Alphabet::new();

    let msg = {
        if let Some(file) = matches.value_of("file") {
            std::fs::read_to_string(file).unwrap()
        } else {
            matches.value_of("MESSAGE").unwrap().to_string()
        }
    };
    debug!("Message = \"{}\"", msg);

    let wpm = u32::from_str(matches.value_of("wpm").unwrap()).unwrap();
    debug!("Words Per Minute = {}", wpm);

    // Wait for `delay` seconds if specified in app args
    if let Some(delay) = matches.value_of("delay") {
        info!("Delaying {} second(s)", delay);
        std::thread::sleep(Duration::from_secs_f32(f32::from_str(delay).unwrap()));
    }

    let button = {
        if let Some(button) = matches.value_of("mouse") {
            Some(MouseButton::OtherButton(u32::from_str(button).unwrap()))
        } else {
            None
        }
    };
    debug!("button = {:?}", button);

    let key = {
        if let Some(key) = matches.value_of("key") {
            Some(KeybdKey::from_str(&key).unwrap())
        } else {
            None
        }
    };
    debug!("key = {:?}", key);

    info!("Sending message");
    morse::Executor::new()
        .with_message(alphabet.to_morse(msg))
        .with_wpm(wpm)
        .with_callback(move |state| {
            if state == morse::State::Down {
                if let Some(key) = key {
                    key.press();
                } else {
                    button.unwrap().press();
                }
            } else {
                if let Some(key) = key {
                    key.release();
                } else {
                    button.unwrap().release();
                }
            }
        })
        .execute();

    info!("Finished");
}

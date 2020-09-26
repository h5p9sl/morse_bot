use clap::{crate_version, Arg};
use inputbot::MouseButton;
use std::time::Duration;

mod morse {
    use std::time::Duration;

    /// Describes the state of the virtual morse keyer
    #[derive(Debug, PartialEq)]
    pub enum State {
        Down,
        Up,
    }

    #[derive(Debug)]
    /// Dynamic Morse-code alphabet loader & converter
    pub struct Alphabet {
        alphabet: Vec<(char, String)>,
    }

    impl Alphabet {
        pub fn new() -> Alphabet {
            let ronfile = std::fs::read_to_string("morse_alphabet.ron").unwrap();
            Alphabet {
                alphabet: ron::from_str(&ronfile).unwrap(),
            }
        }

        /// Converts a string to morse code
        pub fn to_morse<S: Into<String>>(&self, message: S) -> Vec<String> {
            let msg = message.into();
            let mut result = Vec::<String>::new();
            for c in msg.to_uppercase().chars() {
                for m in &self.alphabet {
                    if c == m.0 {
                        result.push(m.1.clone());
                    }
                }
            }
            result
        }
    }

    /// The virtual morse keyer.
    /// This struct handles the timing and execution of the morse code
    pub struct Executor {
        message: Vec<String>,
        unit_duration: Duration,
        callback: Box<dyn Fn(State)>,
    }

    impl Executor {
        #[inline]
        fn wpm_to_unit(wpm: u32) -> Duration {
            Duration::from_secs_f32((1200.0 / wpm as f32) / 1000.0)
        }

        pub fn new() -> Executor {
            Executor {
                message: Default::default(),
                unit_duration: Self::wpm_to_unit(20),
                callback: Box::new(|state| {
                    println!("{:?}", state);
                }),
            }
        }

        pub fn with_message(mut self, message: Vec<String>) -> Executor {
            self.message = message;
            self
        }

        pub fn with_wpm(mut self, wpm: u32) -> Executor {
            self.unit_duration = Self::wpm_to_unit(wpm);
            self
        }

        pub fn with_callback<F: 'static>(mut self, callback: F) -> Executor
        where
            F: Fn(State),
        {
            self.callback = Box::from(callback);
            self
        }

        pub fn execute(&self) {
            for morse_char in &self.message {
                for c in morse_char.chars() {
                    // Signal on
                    std::thread::sleep(
                        match c {
                            '/' => Some(self.unit_duration * 4),
                            '.' => {
                                (self.callback)(State::Down);
                                // Short signal: 1 unit
                                Some(self.unit_duration)
                            }
                            '-' | '_' => {
                                (self.callback)(State::Down);
                                // Long signal: 3 units
                                Some(self.unit_duration * 3)
                            }
                            _ => None,
                        }
                        .unwrap_or(Duration::from_secs(0)),
                    );

                    // Signal off
                    std::thread::sleep(
                        match c {
                            '.' | '-' | '_' => {
                                (self.callback)(State::Up);
                                // Symbol space: 1 unit
                                Some(self.unit_duration)
                            }
                            _ => None,
                        }
                        .unwrap_or(Duration::from_secs(0)),
                    );
                }
                // Letter space: 3 units
                std::thread::sleep(self.unit_duration * 2);
            }
        }
    }
}

fn main() {
    let matches = clap::App::new("Morse Bot")
        .version(crate_version!())
        .args(&[
            Arg::with_name("MESSAGE").required(true),
            Arg::with_name("wpm")
                .default_value("15")
                .help("Words per minute - how fast the virtual keyer goes")
                .long_help("Words per minute - how long a single time unit is (one 'dit'/'.'), which defines how fast the virtual keyer \"types\" out the morse code message.")
                .short("w")
                .takes_value(true)
                ,
            Arg::with_name("delay")
                .help("Delay before starting")
                .long_help("Specifies how long the program should wait before starting the virtual keyer (the playback of the morse code)")
                .short("d")
                .takes_value(true)
                ,
        ])
        .after_help("To view a summary of a flag/option, use `-h`. To view a detailed description of a flag/option, use `--help`")
        .get_matches();

    let msg = matches.value_of("MESSAGE").unwrap();
    let alphabet = morse::Alphabet::new();

    use std::str::FromStr;
    if let Some(delay) = matches.value_of("delay") {
        std::thread::sleep(Duration::from_secs_f32(f32::from_str(delay).unwrap()));
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

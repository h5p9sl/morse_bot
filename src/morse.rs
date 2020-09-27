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
        let mut space_needed = false;
        for morse_char in &self.message {
            if space_needed {
                // Letter space: 3 units
                std::thread::sleep(self.unit_duration * 3);
            }
            for c in morse_char.chars() {
                std::thread::sleep(
                    match c {
                        '/' => {
                            // Word space: 7 units
                            Some(self.unit_duration * 7)
                        }
                        '.' | '-' | '_' => {
                            if space_needed {
                                // Symbol space: 1 unit
                                std::thread::sleep(self.unit_duration);
                            }
                            // Signal on
                            (self.callback)(State::Down);
                            match c {
                                '.' => Some(self.unit_duration),
                                '-' | '_' => Some(self.unit_duration * 3),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                    .unwrap_or(Duration::from_secs(0)),
                );
                space_needed = match c {
                    '.' | '-' | '_' => {
                        (self.callback)(State::Up);
                        true
                    }
                    _ => false,
                };
            }
            space_needed = true;
        }
    }
}

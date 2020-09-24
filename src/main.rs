use clap::{crate_version, Arg};

#[derive(Debug)]
struct MorseAlphabet {
    alphabet: Vec<(char, String)>,
}

impl MorseAlphabet {
    pub fn new() -> MorseAlphabet {
        let ronfile = std::fs::read_to_string("morse_alphabet.ron").unwrap();
        MorseAlphabet {
            alphabet: ron::from_str(&ronfile).unwrap(),
        }
    }

    /// Converts a string to morse code
    pub fn to_morse<S: Into<String>>(&self, message: S) -> String {
        let msg = message.into();
        let mut result = String::new();
        for c in msg.to_uppercase().chars() {
            for m in &self.alphabet {
                if c == m.0 {
                    result.push_str(&m.1);
                    result.push(' ');
                }
            }
        }
        result.trim_end().to_string()
    }
}

fn main() {
    let matches = clap::App::new("Morse Bot")
        .version(crate_version!())
        .args(&[Arg::with_name("MESSAGE").required(true)])
        .get_matches();

    let msg = matches.value_of("MESSAGE").unwrap();
    let alphabet = MorseAlphabet::new();

    println!("{}", alphabet.to_morse(msg));
}

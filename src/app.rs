use clap::{crate_version, AppSettings, Arg, ArgMatches};
use inputbot::strum::VariantNames;

pub fn init_logger() {
    use env_logger::fmt::TimestampPrecision;
    env_logger::builder()
        .filter(Some("morsebot"), log::LevelFilter::Info)
        .format_timestamp(Some(TimestampPrecision::Millis))
        .init();
}

pub fn create_app() -> ArgMatches<'static> {
    clap::App::new("Morse Bot")
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .args(&[
            Arg::with_name("MESSAGE")
                .required_unless_one(&["file", "list_keys"])
                ,
            Arg::with_name("wpm")
                .default_value("15")
                .help("Words per minute - how fast the virtual keyer goes")
                .long_help("Words per minute - how long a single time unit is (one \"dot\"/\"dit\"), which defines how fast the virtual keyer \"types\" out the morse code message.")
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
            Arg::with_name("mouse")
                .short("m")
                .long("mouse")
                .help("Defines which mouse button to use for sending morse, possible values are non-negative numbers")
                .takes_value(true)
                .default_value("1")
                .required_unless("key")
                ,
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("Defines which keyboard key to use for sending morse")
                .takes_value(true)
                .possible_values(&inputbot::KeybdKey::VARIANTS)
                .hide_possible_values(true)
                ,
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Load message from file")
                .takes_value(true)
        ])
        .after_help("To view a summary of a flag/option, use `-h`. To view a detailed description of a flag/option, use `--help`")
        .get_matches()
}

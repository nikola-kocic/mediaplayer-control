use std::time::Duration;

use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::{BlockingSender, Connection};
use dbus::{arg, Message};
#[macro_use]
extern crate clap;
use clap::{App, Arg};
use floating_duration::TimeAsFloat;

fn action(c: &Connection, dest: &str, command: &str, v: Option<i64>) {
    let mut m = Message::new_method_call(
        dest,
        "/org/mpris/MediaPlayer2",
        "org.mpris.MediaPlayer2.Player",
        command,
    )
    .unwrap();
    if let Some(v) = v {
        m = m.append1(v);
    }

    c.send_with_reply_and_block(m, Duration::from_millis(100))
        .unwrap();
}

fn get_player_property<T>(c: &Connection, dest: &str, name: &str) -> T
where
    for<'b> T: dbus::arg::Get<'b>,
{
    let p = c.with_proxy(dest, "/org/mpris/MediaPlayer2", Duration::from_millis(5000));
    p.get("org.mpris.MediaPlayer2.Player", name).unwrap()
}

// offset : value between -1 and 1
fn offset_volume(c: &Connection, dest: &str, offset: f64) {
    let p = c.with_proxy(dest, "/org/mpris/MediaPlayer2", Duration::from_millis(5000));
    let v: f64 = get_player_property(c, dest, "Volume");
    let newv = v + offset;
    p.set(
        "org.mpris.MediaPlayer2.Player",
        "Volume",
        arg::Variant(newv),
    )
    .unwrap();
}

fn main() {
    let matches = App::new("MediaPlayer Control")
        .version("0.1.1")
        .author("Nikola Kocić. <nikolakocic@gmail.com>")
        .about("Controls Media Player using DBus protocol")
        .arg(
            Arg::with_name("player")
                .long("player")
                .short("p")
                .help("Sets the player to use")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("command")
                .long("command")
                .short("c")
                .help(
                    "Sets the command to execute: \
                     Next, Previous, Pause, PlayPause, Stop, Play, \
                     Seek(nanoseconds), OffsetVolume(0.0 - 1.0), \
                     GetFormattedPosition",
                )
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("arg")
                .long("arg")
                .short("a")
                .help("Sets the command argument")
                .takes_value(true)
                .required(false),
        )
        .after_help(
            r#"
Examples:
    mediaplayer-control --player audacious --command Next
    mediaplayer-control --player audacious --command Seek --arg="-5000000"
    mediaplayer-control -p audacious -c OffsetVolume -a="-0.025""#,
        )
        .get_matches();

    let player = matches.value_of("player").unwrap();
    let command = matches.value_of("command").unwrap();
    let c = Connection::new_session().unwrap();
    let dest = "org.mpris.MediaPlayer2.".to_string() + player;
    let missing_arg_message = format!("No argument for {} command", command);
    match command {
        "OffsetVolume" => {
            let arg = value_t!(matches, "arg", f64).expect(&missing_arg_message);
            offset_volume(&c, &dest, arg);
        }
        "Seek" => {
            let arg = value_t!(matches, "arg", i64).expect(&missing_arg_message);
            action(&c, &dest, command, Some(arg));
        }
        "GetFormattedPosition" => {
            let v: i64 = get_player_property(&c, &dest, "Position");
            if v < 0 {
                panic!("Wrong retuned value for position: {}", v);
            }
            let duration = Duration::from_micros(v as u64);
            let total_seconds = duration.as_fractional_secs();
            let minutes = ((total_seconds / 60.0).floor() as i32) % 60;
            let seconds = total_seconds - f64::from(minutes * 60);
            println!("{:02}:{:05.2}", minutes, seconds);
        }
        _ => {
            action(&c, &dest, command, None);
        }
    }
}

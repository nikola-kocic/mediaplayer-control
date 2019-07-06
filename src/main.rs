extern crate chrono;
// use chrono::Duration;
use std::time::Duration;

extern crate dbus;
use dbus::stdintf::org_freedesktop_dbus::Properties;
use dbus::{BusType, Connection, Interface, Member, MessageItem, Props};
#[macro_use]

extern crate clap;
use clap::{App, Arg};

extern crate floating_duration;
use floating_duration::TimeAsFloat;

fn action(c: &Connection, dest: &str, member: &Member, v: Option<MessageItem>) {
    let p = c.with_path(dest, "/org/mpris/MediaPlayer2", 5000);
    let i = Interface::new("org.mpris.MediaPlayer2.Player").unwrap();
    p.method_call_with_args(&i, member, |m| {
        if let Some(val) = v {
            m.append_items(&[val]);
        }
    })
    .unwrap();
}

fn get_player_property<T>(c: &Connection, dest: &str, name: &str) -> T
where
    for<'b> T: dbus::arg::Get<'b>,
{
    let p = c.with_path(dest, "/org/mpris/MediaPlayer2", 5000);
    p.get("org.mpris.MediaPlayer2.Player", name).unwrap()
}

// offset : value between -1 and 1
fn offset_volume(c: &Connection, dest: &str, offset: f64) {
    let p = Props::new(
        &c,
        dest,
        "/org/mpris/MediaPlayer2",
        "org.mpris.MediaPlayer2.Player",
        10000,
    );
    let m: MessageItem = p.get("Volume").unwrap();
    let v: f64 = m.inner().unwrap();
    let newv = v + offset;
    p.set("Volume", MessageItem::Double(newv)).unwrap();
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
    let c = Connection::get_private(BusType::Session).unwrap();
    let dest = "org.mpris.MediaPlayer2.".to_string() + player;
    let missing_arg_message = format!("No argument for {} command", command);
    match command {
        "OffsetVolume" => {
            let arg = value_t!(matches, "arg", f64).expect(&missing_arg_message);
            offset_volume(&c, &dest, arg);
        }
        "Seek" => {
            let m = Member::new(command).unwrap();
            let arg = value_t!(matches, "arg", i64).expect(&missing_arg_message);
            action(&c, &dest, &m, Some(MessageItem::Int64(arg)));
        }
        "GetFormattedPosition" => {
            let v: i64 = get_player_property(&c, &dest, "Position");
            if v < 0 {
                panic!("Wrong retuned value for position: {}", v);
            }
            let duration = Duration::from_micros(v as u64);
            let total_seconds = duration.as_fractional_secs();
            let minutes = ((total_seconds / 60.0).floor() as i32) % 60;
            let seconds = total_seconds - ((minutes * 60) as f64);
            println!("{:02}:{:05.2}", minutes, seconds);
        }
        _ => {
            let m = Member::new(command).unwrap();
            action(&c, &dest, &m, None);
        }
    }
}

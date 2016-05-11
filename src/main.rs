extern crate dbus;
use dbus::{BusType, Connection, Message, MessageItem, Props};
#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn action(c: &Connection, dest: &str, name: &str, v: Option<MessageItem>) {
    let mut m = Message::new_method_call(dest,
                                          "/org/mpris/MediaPlayer2",
                                          "org.mpris.MediaPlayer2.Player",
                                          name)
        .unwrap();
    if let Some(val) = v {
        m = m.append(val);
    }
    c.send_with_reply_and_block(m, 2000).unwrap();
}

// offset : value between -1 and 1
fn offset_volume(c: &Connection, dest: &str, offset: f64) {
    let p = Props::new(&c,
                       dest,
                       "/org/mpris/MediaPlayer2",
                       "org.mpris.MediaPlayer2.Player",
                       10000);
    let m: MessageItem = p.get("Volume").unwrap();
    let v: f64 = m.inner().unwrap();
    let newv = v + offset;
    p.set("Volume", MessageItem::Double(newv)).unwrap();
}

fn main() {
    let matches = App::new("MediaPlayer Control")
        .version("0.1.0")
        .author("Nikola Kocić. <nikolakocic@gmail.com>")
        .about("Controls Media Player using DBus protocol")
        .arg(Arg::with_name("player")
            .long("player")
            .help("Sets the player to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("command")
            .long("command")
            .help("Sets the command to execute: \
                   Next, Previous, Pause, PlayPause, Stop, Play, \
                   Seek(nanoseconds), OffsetVolume(0.0 - 1.0)")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("arg")
            .long("arg")
            .help("Sets the command argument")
            .takes_value(true)
            .required(false))
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
            let arg = value_t!(matches, "arg", i64).expect(&missing_arg_message);
            action(&c, &dest, command, Some(MessageItem::Int64(arg)));
        }
        _ => {
            action(&c, &dest, command, None);
        }
    }
}

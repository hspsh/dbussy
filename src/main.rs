use std::time::Duration;

use dbus::{blocking::Connection, strings::{Interface, Member}};

fn main() -> Result<(), dbus::Error> {
    let conn = Connection::new_session().unwrap();

    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .unwrap();

    for name in names {
        println!("{}", name);
    }

    let proxy_player = conn.with_proxy(
        "org.mpris.MediaPlayer2.spotify",
        "/org/mpris/MediaPlayer2",
        Duration::from_secs(1),
    );

    let result: Result<(), dbus::Error> = proxy_player.method_call("org.mpris.MediaPlayer2.Player", "PlayPause", ());

    return result;
}

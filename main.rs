use serde::Deserialize;
use tray_item::{IconSource, TrayItem};

use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct RadioStation {
    name: String,
    url: String,
}

fn read_stations_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<RadioStation>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let stations: Vec<RadioStation> = serde_json::from_reader(reader)?;
    Ok(stations)
}

fn stop_playback() -> Result<std::process::Child, std::io::Error> {
    Command::new("playerctl").args(vec!["stop"]).spawn()
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("traydio").expect("xdg failed");
    let stations_file = xdg_dirs.get_config_file("stations.json");
    let errmsg = format!("unable to read {stations_file:?}");
    let stations = read_stations_from_file(stations_file).expect(&errmsg);

    gtk::init().unwrap();

    let mut tray = TrayItem::new(
        "traydio",
        IconSource::Resource("media-playback-start-symbolic"),
    )
    .unwrap();

    for station in stations {
        tray.add_menu_item(&station.name, move || {
            stop_playback().expect("unable to run playerctl");

            Command::new("mpv")
                .args(vec![&station.url])
                .spawn()
                .expect("unable to run mpv");
        })
        .unwrap();
    }

    tray.add_label("🎶🎶🎶🎶🎶🎶🎶").unwrap();

    tray.add_menu_item("Stop playback", || {
        stop_playback().expect("unable to run playerctl");
    })
    .unwrap();

    tray.add_menu_item("Quit", || {
        stop_playback().expect("unable to run playerctl");
        unsafe {
            gtk_sys::gtk_main_quit();
        }
    })
    .unwrap();

    gtk::main();
}

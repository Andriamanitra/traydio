use std::process::Command;
use tray_item::{IconSource, TrayItem};
use kdl::{KdlNode, KdlDocument};

struct RadioStation {
    name: String,
    url: String,
}

impl From<&KdlNode> for RadioStation {
    fn from(node: &KdlNode) -> Self {
        RadioStation {
            name: node.name().value().to_owned(),
            url: node
                .get("url")
                .expect("missing radio station url")
                .value()
                .as_string()
                .expect("radio station url should be a string")
                .to_owned(),
        }
    }
}

fn stop_playback() -> Result<std::process::Child, std::io::Error> {
    Command::new("playerctl")
        .args(vec!["--player", "mpv", "stop"])
        .spawn()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("traydio")?;
    let stations_file = xdg_dirs.get_config_file("stations.kdl");
    let doc = std::fs::read_to_string(&stations_file)
        .unwrap_or_else(|_| panic!("unable to read {stations_file:?}"))
        .parse::<KdlDocument>()?;
    let stations: Vec<RadioStation> = doc.nodes().iter().map(|n| n.into()).collect();
    let icon = IconSource::Resource("media-playback-start-symbolic");

    gtk::init()?;
    let mut tray = TrayItem::new("traydio", icon)?;

    for station in stations {
        tray.add_menu_item(&station.name, move || {
            stop_playback().expect("unable to run playerctl");

            Command::new("mpv")
                .args(vec![&station.url])
                .spawn()
                .expect("unable to run mpv");
        })?;
    }

    tray.add_label("🎶🎶🎶🎶🎶🎶🎶")?;

    tray.add_menu_item("Stop playback", || {
        stop_playback().expect("unable to run playerctl");
    })?;

    tray.add_menu_item("Quit", || {
        stop_playback().expect("unable to run playerctl");
        unsafe {
            gtk_sys::gtk_main_quit();
        }
    })?;

    gtk::main();
    Ok(())
}

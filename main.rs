#![feature(if_let_guard)]

use kdl::{KdlDocument, KdlNode};
use ksni;
use std::process::Command;

struct Traydio {
    current: Option<usize>,
    stations: Vec<RadioStation>,
}

impl Traydio {
    fn change_station(&mut self, idx: usize) {
        stop_playback().expect("unable to run playerctl");
        if let Some(station) = self.stations.get(idx) {
            self.current = Some(idx);
            Command::new("mpv")
                .args(vec![&station.url])
                .spawn()
                .expect("unable to run mpv");
        } else {
            eprintln!("Error: no station at index {}", idx);
            self.current = None;
        }
    }
}

impl ksni::Tray for Traydio {
    fn icon_name(&self) -> String {
        String::from("media-playback-start-symbolic")
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        println!("Changing to next station");
        self.change_station(match self.current {
            Some(i) => (i + 1) % self.stations.len(),
            None => 0
        });
        
    }

    fn title(&self) -> String {
        if let Some(idx) = self.current {
            let station = &self.stations[idx];
            format!("{} – playing {}", "Traydio", station.name)
        } else {
            String::from("Traydio")
        }
    }

    fn id(&self) -> String {
        String::from("traydio")
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            RadioGroup {
                selected: self.current.unwrap_or(9001),
                select: Box::new(Traydio::change_station),
                options: self
                    .stations
                    .iter()
                    .map(|st| RadioItem {
                        label: st.name.to_string(),
                        ..Default::default()
                    })
                    .collect(),
            }
            .into(),
            StandardItem {
                label: "Stop".into(),
                activate: Box::new(|this: &mut Self| {
                    stop_playback().expect("unable to run playerctl");
                    this.current = None;
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_: &mut Self| {
                    stop_playback().expect("unable to run playerctl");
                    std::process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

struct RadioStation {
    name: String,
    url: String,
}

#[derive(Debug)]
enum RadioStationParseError {
    MissingUrl(String),
    InvalidUrl(String),
}

impl TryFrom<&KdlNode> for RadioStation {
    type Error = RadioStationParseError;

    fn try_from(node: &KdlNode) -> Result<Self, Self::Error> {
        let name = node.name().value().to_owned();
        match node.get("url") {
            None => Err(RadioStationParseError::MissingUrl(name)),
            Some(entry) if let Some(url) = entry.value().as_string() => {
                Ok(RadioStation { name, url: url.to_owned() })
            },
            Some(_) => Err(RadioStationParseError::InvalidUrl(name))
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

    let stations: Vec<RadioStation> = doc
        .nodes()
        .iter()
        .filter_map(|n| match n.try_into() {
            Ok(station) => Some(station),
            Err(RadioStationParseError::MissingUrl(station_name)) => {
                eprintln!("Skipping {} (Error: missing url)", station_name);
                None
            },
            Err(RadioStationParseError::InvalidUrl(station_name)) => {
                eprintln!("Skipping {} (Error: invalid url)", station_name);
                None
            },
        })
        .collect();

    let service = ksni::TrayService::new(Traydio { stations, current: None });
    service.spawn();

    loop {
        std::thread::park();
    }
}

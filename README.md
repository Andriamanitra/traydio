# traydio

Play internet radios from the system tray on Linux.

## Configuration:

The program looks for `$XDG_CONFIG_HOME/traydio/stations.kdl` (`~/.config/traydio/stations.kdl` by default).
This [KDL](https://kdl.dev/) file should be a list of stations that have attribute "url" (any URL or even file path that mpv knows how to open is fine), mayhaps something like this:
```kdl
"HouseTime.fm" url="https://listen.housetime.fm/aac-hd.pls"
"TranceBase.fm" url="https://listen.trancebase.fm/aac-hd.pls"
```

## Installation:

First you need to make sure you have the requirements:

* [mpv](https://github.com/mpv-player/mpv) – to play music
* [mpv-mpris](https://github.com/hoyon/mpv-mpris) plugin – to make mpv listen to playerctl
* [playerctl](https://github.com/altdesktop/playerctl) – to control mpv
* Probably bunch of other less obvious library dependencies like maybe build-essentials and gtk-devel and cairo or something, I wish you the best of luck in figuring them all out

If you by some miracle made it this far you can try these commands and pray this house of cards doesn't fall down:

```
$ git clone https://github.com/andriamanitra/traydio
$ cd traydio
$ cargo install --path .
$ traydio
```

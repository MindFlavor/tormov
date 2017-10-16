# TORMOV

[![legal](https://img.shields.io/github/license/mindflavor/tormov.svg)](LICENSE)

[![Crate](https://img.shields.io/crates/v/tormov.svg)](https://crates.io/crates/tormov)  [![cratedown](https://img.shields.io/crates/d/tormov.svg)](https://crates.io/crates/tormov) [![cratelastdown](https://img.shields.io/crates/dv/tormov.svg)](https://crates.io/crates/tormov)

[![tag](https://img.shields.io/github/tag/mindflavor/tormov.svg)](https://github.com/MindFlavor/tormov/tree/v0.2.2)
[![release](https://img.shields.io/github/release/mindflavor/tormov.svg)](https://github.com/MindFlavor/tormov/tree/v0.2.2)
[![commitssince](https://img.shields.io/github/commits-since/mindflavor/tormov/v0.2.2.svg)](https://img.shields.io/github/commits-since/mindflavor/tormov/v0.2.2.svg)

## TORrentMOVer

Simple script program to move completed torrents to specific folders.

Sometimes it's easy to let torrent download files to a generic directory and then move it afterwards. This program automates this task by:

1. Looking for RegEx patterns in the folder. You can specify exactly which folder/file to handle.
1. Checking if the downloads are completed. Every file (if there are more than one) must be completed.
1. Moving or linking the file/folder to the designated directory.

The program is written in Rust so it's very quick and light on your system. The source code is very small so you can check yourself what it does in few moments.

## Installation

1. Make sure to have Rust installed. Grab it here [https://www.rust-lang.org/en-US/](https://www.rust-lang.org/en-US/) if you don't have it. This program is tested with rustc nightly but it should work with others versions too (I'm too lazy to test it myself, sorry :smile: ).
1. Install the tool with ```cargo install tormov```. This will install the latest published version. If you want the master branch use ```cargo install --git https://github.com/MindFlavor/tormov``` instead.
1. type ```tormov``` in the console to test the program execution. You'll get an error because of missing parameters. We'll cover them in the next section.

## Parameters

TORMOV expects, from the command line, two parameters:

1. Configuration file.
1. Folder to analyze.

So, for example, if you have a configuration file called ```tormov_config.json``` and you want to check the ```/var/lib/transmission-daemon/downloads/``` you can write:

```bash
$ tormov tormov_config.json /var/lib/transmission-daemon/downloads/
```

## Configuration file

A sample configuration file is available here: [example_config.json](https://github.com/MindFlavor/tormov/blob/master/example_config.json). The format, however, is simple.

```json
{
    "skipextension": "part",
    "matches": [
        {
            "regex": "Arrow",
            "destination": "/mnt/shows/Arrow",
	    "action": "Move"
        },
        {
            "regex": "Big.Bang",
            "destination": "/mnt/shows/The.big.bang.theory",
	    "action": "Move"
        },
        {
            "regex": "Marvels.Agents.of.S.H.I.E.L.D.*",
            "destination": "/mnt/shows/agents_of_the_shield",
	    "action": "Link"
        }
    ]
}
```

### skipextension

```skipextension``` is the extension appended whenever the file is not ready. Most torrent clients use ```part``` or ```incomplete``` but make sure to specify the right one.

### matches

The ```matches``` section is an array of entries you want to check. Each entry must have:

1. A regular expression to match in the field ```regex```.
1. A destination in the ```destination``` field. The destination is where the file/folder will be moved if the contents have been completely downloaded (that is, there is no file with ```skipextension``` anywhere in the folder).
1. An action to perform in case all the rules match. The supported actions are ```Move``` or ```Link```. The latter will create a symbolic link instead of moving the file. *Note*: right now linking is supported only on Linux, if you need Windows please drop a line.

## Scheduling

While TORMOV does not have a scheduler it's fairly easy to automate it with cron jobs or systemd. For example with systemd you can schedule it creating two files. The first is ```tormov.service``` with these contents:

```

[Unit]
Description=TORrent MOVer

[Service]
ExecStart=<full TORMOV bin path> <config> <folder>

```

And another one called ```tormow.timer``` with the schedule:

```

[Unit]
Description=Runs tormov every minute

[Timer]
OnBootSec=5min
OnUnitActiveSec=1min

Unit=tormov.service

[Install]
WantedBy=timers.target

```

Now simply start end optionally enable the timer with:

```bash

$ sudo systemctl start tormov.timer
$ sudo systemctl enable tormov.timer

```

You can then check the output with ```sudo systemctl status tormov.service``` as usual.


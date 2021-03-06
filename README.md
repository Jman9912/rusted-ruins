# Rusted Ruins [![Build Status](https://travis-ci.org/garkimasera/rusted-ruins.svg?branch=master)](https://travis-ci.org/garkimasera/rusted-ruins)
Extensible open world rogue like game with pixel art. Players can explore the wilderness and ruins.
This game is written in Rust.

## Screenshot
Ruin (Example of auto generated map)

![exploring-ruin](https://github.com/garkimasera/rusted-ruins/blob/master/screenshots/exploring-ruin.png)

Town (Example of created map by map-editor)

![town](https://github.com/garkimasera/rusted-ruins/blob/master/screenshots/town.png)

## Video

https://www.youtube.com/watch?v=CUMPWX-teaY

## Game Objective

The player arrives at a recently discovered continent where a lot of ruins remain. The player will explore ruins and fight against monsters. By collecting relics in ruins, the player can earn money and fame, and solve the mystery of the ruined nation.

## Status
This is a very early project. Many features for playing are not completed.

Binary format of pak files and save files may be changed before version 1.0.

## Changelog

[See this wiki.](https://github.com/garkimasera/rusted-ruins/wiki/Changelog)

## Design

* 2D graphics.
* Easy to extend by the pak file system. Most of assets are packaged as pak file. Pak file can be created by makepak. Users can add new characters, items and dungeons easily by pak file system.
* Map editor to create new map.
* Script to describe talks and events in game.
* Open world. Provide many playing style for players. The game objective will be different by players.

## Pak files
In this game, most of image data and many assets are handled as *XXObject*.
XXObject is packaged to pak files. Their file extension is "pak".
Pak files and the sources are under [rusted-ruins-pak](https://github.com/garkimasera/rusted-ruins-pak).

## How to try
Please install SDL2 libraries at first.
For Ubuntu users:
```shell
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

Rusted Ruins is written in Rust, so please install Rust compilation tools. You can use [rustup](https://www.rustup.rs/) to install Rust.

After that, clone this repository, download pak files, and run.

```shell
git clone https://github.com/garkimasera/rusted-ruins.git
cd rusted-ruins
./build-pak.sh
RUSTED_RUINS_ASSETS_DIR=./assets cargo run --release -p rusted-ruins
```

If you are a Windows user, you can download from [Releases page](https://github.com/garkimasera/rusted-ruins/releases).

## How to operate

Operate the player character with the keyboard and mouse.

Left click on tiles - Move to the tile, melee atack, or start talking.

Left click + Ctrl - Shoot by the ranged weapon.

Left click + Shift - Use the equipped tool.

Right click - Open action menu. Actions for specified tile are available through the menu. For example, you can use stairs and enter/exit the map by opening menu at the tile that player is on.

Arrow keys - Move

Enter key - Enter towns or dungeons, walk up/down stairs, and select an answer when talking.

### Sidebar

There are some icons on the sidebar. Click icons to open windows.

Icon list

* Inventory window
* Equipment window
* Status window
* Creation window
* Game information window
* Save / Exit game

### Shortcut keys

c - Open creation window

d - Drop items

e - Eat an item

f - Shot

g - Pick up items

h - Help

i - View inventory

o - Open game information window

q - Drink an item

s - Open status window

w - Open equipment window

escape - Open exit window

f12 - Debug command

## License
GPL v3

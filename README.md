# Arrata-APP

This repo contains the code for an [Arrata](https://github.com/kalebvonburris/Arrata-TTRPG) character sheet manager.

The application itself is written in Rust and therefore can be compiled for and used across multiple platforms if you would like to do so for yourself. See [Compiling Locally](#compiling-locally) for details if you're on an unusual platform.

## Installation

### Prerequisites

- For these operating systems:
  - Windows 10/11
    - [Webview2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/): This should be installed if you have Microsoft Edge.
  - Linux
    - WebkitGtk
      - `sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libappindicator3-dev`
      - on Debian/bullseye use: `sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev`
  - Mac OS
    - No needed dependencies

### Installing

TODO!

### Compiling Locally

To compile locally, you will need the following prerequisites:

- Rust Nightly

From the `Arrata-APP` directory, run `cargo build --release`. A new binary will be generated at `Arrata-APP/target/release/Arrata-Character-Sheet-Manager`.

## Usage

TODO!

## Roadmap

### Main features

- [ ] Create a functioning application
- [x] Establish character sheet paradigm - (`*.arrata`)
- [ ] Implement the character sheet
  - [x] Name/Stock/Miscellaneous
  - [x] Stats
  - [ ] Skills
  - [ ] Quirks
  - [ ] Argos
  - [ ] Equipment
- [ ] Implement NPC character sheets

### Miscellaneous QOL features

- [ ] Add dice-rolling functionality
  - [ ] Discord integration(?)
- [ ] Allow exporting into a printable format

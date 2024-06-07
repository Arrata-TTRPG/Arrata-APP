# Arrata-APP

Access the online version here: [https://kalebvonburris.github.io/Arrata-APP/](https://arrata-ttrpg.github.io/Arrata-APP/)

This repo contains the code for an [Arrata](https://github.com/kalebvonburris/Arrata-TTRPG) character sheet manager.

The application itself is written in Rust and therefore can be compiled for and used across multiple platforms if you would like to do so for yourself. See [Compiling Locally](#compiling-locally) for details if you're on an unusual platform.

## Installation

To install, go to the [latest release](https://github.com/kalebvonburris/Arrata-APP/releases/latest) and download the zip that matches your system. Extract into a folder of your choosing but maintain the file structure. You should be able to run the executable and use the app.

## Compiling Locally

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
- Rust Nightly
- Tailwind CSS
  - `npm install -D tailwindcss`

### Compilation

Clone the repo: `git clone https://github.com/Arrata-TTRPG/Arrata-APP.git`

`cd` into `Arrata-APP`

Run `npx tailwindcss -i input.css -o public/tailwind.css`.

#### Web

To compile and run the web version, install Dioxus CLI: `cargo install dioxus-cli`.

Then compile and run by `cd Arrata-APP` and `dx serve --platform web --features web`.

#### Desktop

To compile and run the desktop version, `cd` into `Arrata-APP` and run `cargo run --release --features desktop`

## Roadmap

### Main features

- [x] Create a functioning application
- [x] Establish character sheet paradigm - (`*.arrata`)
- [x] Implement the character sheet
  - [x] Name/Stock/Miscellaneous
  - [x] Stats
  - [x] Skills
  - [x] Quirks
  - [x] Argos
  - [x] Inventory
- [ ] Implement NPC character sheets

### Miscellaneous QOL features

- [x] Add dice-rolling functionality
  - [ ] Discord integration(?)
- [ ] Allow exporting into a printable format


## Troubleshooting

If you're on Linux and encounter the app being blank and/or the message: `AcceleratedSurfaceDMABuf was unable to construct a complete framebuffer`,
add `WEBKIT_DISABLE_DMABUF_RENDERER=1` to your environment variables. Usually this is under `/etc/environment` but it will differ based on your shell.

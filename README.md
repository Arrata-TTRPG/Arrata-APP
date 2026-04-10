# Arrata-APP

Access the online version here: [https://arrata-ttrpg.github.io/Arrata-APP/](https://arrata-ttrpg.github.io/Arrata-APP/)

This repo contains the code for an [Arrata](https://github.com/Arrata-TTRPG) character sheet manager.

The application is written in Rust using [Dioxus](https://dioxuslabs.com/) and can be compiled for web, desktop, and mobile. See [Compiling Locally](#compiling-locally) for details.

## Installation

To use the web version, visit the link above. No installation required.

To install a desktop build, go to the [latest release](https://github.com/Arrata-TTRPG/Arrata-APP/releases/latest) and download the zip that matches your system. Extract it and run the executable.

## Compiling Locally

### With Nix (recommended)

The project includes a Nix flake that provides the full development environment.

1. [Install Nix](https://nixos.org/download) with flakes enabled
2. Clone the repo: `git clone https://github.com/Arrata-TTRPG/Arrata-APP.git`
3. `cd Arrata-APP`
4. `nix develop` - this drops you into a shell with Rust nightly, Tailwind, and all dependencies

From there:

- **Web:** `dx serve --platform web`
- **Desktop:** `dx serve --platform desktop`
- **Release build:** `dx build --release --platform web`

### Without Nix

#### Prerequisites

- Rust Nightly (with `wasm32-unknown-unknown` target and `rust-src` component)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started): `cargo install dioxus-cli`
- [Tailwind CSS v4](https://tailwindcss.com/docs/installation)
- Platform-specific dependencies:
  - **Linux:** WebkitGtk - `sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev`
  - **Windows:** [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (included with Microsoft Edge)
  - **macOS:** No additional dependencies

#### Build

1. Clone the repo: `git clone https://github.com/Arrata-TTRPG/Arrata-APP.git`
2. `cd Arrata-APP`
3. `tailwindcss -i input.css -o public/tailwind.css`
4. `dx serve --platform web` or `dx serve --platform desktop`

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
add `WEBKIT_DISABLE_DMABUF_RENDERER=1` to your environment variables. Usually this is under `/etc/environment` but it will differ based on your shell.dd `WEBKIT_DISABLE_DMABUF_RENDERER=1` to your environment variables. Usually this is under `/etc/environment` but it will differ based on your shell.

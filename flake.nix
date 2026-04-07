{
  description = "Rust development for Dioxus Web";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # Combine nightly toolchain with wasm target
        toolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.complete.toolchain
          fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.rust-std
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            toolchain  # Includes nightly rustc, cargo, rust-src, etc.
            cargo-binstall
            lld
            openssl
            pkg-config
            webkitgtk_4_1
            libcanberra-gtk3
            xdotool
            libxkbcommon
            libepoxy
            cairo
            gdk-pixbuf
            atk
            gtk3
            tailwindcss_4
          ];

          OPENSSL_DIR = "${pkgs.openssl.out}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
            echo "Rust nightly development environment loaded."
            
            if ! command -v dx &> /dev/null; then
              echo "Installing dioxus-cli..."
              cargo binstall dioxus-cli --force --no-confirm
            fi
          '';
        };
      }
    );
}

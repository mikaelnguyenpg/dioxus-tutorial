{
  description = "Dioxus Desktop App - Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Dependencies cho Dioxus Desktop trên Linux
        desktopDeps = with pkgs; [
          # WebView & GTK
          webkitgtk_4_1
          gtk3
          glib
          cairo
          pango
          gdk-pixbuf
          atk
          libsoup_3

          # Crypto & build tools
          openssl
          pkg-config

          # Input simulation & X11
          xdotool
          libxtst
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          name = "dioxus-desktop-dev";

          buildInputs = [ rustToolchain ] ++ desktopDeps;

          # === Compile time ===
          PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" (with pkgs; [
            webkitgtk_4_1.dev
            gtk3.dev
            glib.dev
            libsoup_3.dev
            openssl.dev
          ]);

          # === Runtime ===
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath desktopDeps;

          GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";

          XDG_DATA_DIRS = pkgs.lib.concatStringsSep ":" [
            "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}"
            "${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}"
            "${pkgs.gtk3}/share"
          ];

          # Optional: Force X11 if you have issues with Wayland + NVIDIA
          # GDK_BACKEND = "x11";

          shellHook = ''
            echo "========================================"
            echo "🦀 Dioxus Desktop DevShell on NixOS"
            echo "Rust: $(rustc --version)"
            echo "========================================"

            # Thêm cargo bin vào PATH
            export PATH="$HOME/.cargo/bin:$PATH"

            # Cài dioxus-cli (dx) nếu chưa có
            if ! command -v dx >/dev/null 2>&1; then
              echo "→ Installing dioxus-cli..."
              cargo install dioxus-cli --version 0.7.5 --locked
            fi

            # Cài wasm-bindgen-cli nếu cần (dùng cho web target)
            if ! command -v wasm-bindgen >/dev/null 2>&1; then
              echo "→ Installing wasm-bindgen-cli..."
              cargo install wasm-bindgen-cli --version 0.2.117 --locked
            fi

            echo ""
            echo "Available commands:"
            echo "   cargo run                 → Build & run"
            echo "   cargo run --release       → Release mode"
            echo "   dx serve --platform desktop"
            echo ""
          '';
        };
      }
    );
}

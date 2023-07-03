let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "latest";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    rust-analyzer
    lazygit # Optional
    pkg-config # For GUI
    cairo # For GUI
    gtk4 # For GUI
  ];
}

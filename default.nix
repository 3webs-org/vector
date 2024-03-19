{ pkgs ? import <nixpkgs> { }}:
with pkgs;
rust.packages.stable.rustPlatform.buildRustPackage rec {
    name = "vanadium-browser";
    version = "0.1.0";
    src = lib.cleanSource ./.;

    buildInputs = [
        pkg-config
        gtk4
        glib
        libadwaita
        webkitgtk_6_0
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
        gst_all_1.gst-plugins-bad
    ];

    nativeBuildInputs = [
        pkg-config
    ];

    cargoLock = {
        lockFile = ./Cargo.lock;
    };
}

# Copyright 2024 3WEBS LLC
# SPDX-License-Identifier: GPL-3.0-or-later

{...}: {
  perSystem = {
    pkgs,
    config,
    ...
  }: let
    crateName = "vanadium-browser";
  in {
    # declare projects
    nci.projects."simple".path = ./.;
    # configure crates
    nci.crates.${crateName} = {
      depsDrvConfig = {
        mkDerivation = {
          buildInputs = with pkgs; [
            pkg-config
            gtk4
            glib
            libadwaita
            webkitgtk_6_0
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
            gst_all_1.gst-plugins-bad
          ];
        };
      };
    };
  };
}

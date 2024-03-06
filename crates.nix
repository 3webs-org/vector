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
          ];
        };
      };
    };
  };
}

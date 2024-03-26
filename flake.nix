let
  cargoToml = builtins.readFile ./Cargo.toml;
  packageData = nixpkgs.lib.getAttr "package" (builtins.fromTOML cargoToml);
  customPackageData = nixpkgs.lib.getAttr "metadata" packageData;
in {
  description = packageData.description;

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/23.11";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration/53af4303dda1fe6e575b2c5ee662ac9b23a18c9f";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { self, nixpkgs, parts, nci, ... }: parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" ];
    imports = [ nci.flakeModule ];
    perSystem = {
      pkgs,
      config,
      ...
    }: let
      crateOutputs = config.nci.outputs."${packageData.name}";
    in {
      nci = {
        projects."${packageData.name}" = {
          path = ./.;
        };
        crates."${packageData.name}" = rec {
          depsDrvConfig = {
            mkDerivation = with pkgs; {
              nativeBuildInputs = [ pkg-config ];
              buildInputs = [
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
          drvConfig = {
            mkDerivation = {
              nativeBuildInputs = [ pkgs.pkg-config ];
              buildInputs = depsDrvConfig.mkDerivation.buildInputs;
              postInstall = ''
                mkdir -p $out/share/applications $out/share/pixmaps

                cat > $out/share/applications/${packageData.name}.desktop <<EOF
                [Desktop Entry]
                Name=${customPackageData.human_readable_name}
                Exec=${packageData.name}
                Icon=${packageData.name}
                Type=Application
                Categories=Network;WebBrowser;
                EOF

                # TODO: Figure out how to not hardcode release
                cp ./icon.svg $out/share/pixmaps/${packageData.name}.svg
              '';
            };
          };
        };
      };
      devShells.default = crateOutputs.devShell;
      packages.default = crateOutputs.packages.release;
    };
  };
}

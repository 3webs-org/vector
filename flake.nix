# Copyright 2024 3WEBS LLC
# SPDX-License-Identifier: GPL-3.0-or-later

{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { self, nixpkgs, rust-overlay, parts, ... }: parts.lib.mkFlake { inherit inputs; } (
    let
      cargoToml = builtins.readFile ./Cargo.toml;
      packageData = nixpkgs.lib.getAttr "package" (builtins.fromTOML cargoToml);
      customPackageData = nixpkgs.lib.getAttr "metadata" packageData;
    in {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      
      perSystem = {
        system,
        pkgs,
        lib,
        inputs',
        ...
      }: let
        cargoNix = pkgs.callPackage (inputs.crate2nix.tools.${system}.generatedCargoNix {
          name = packageData.name;
          src = ./.;
        }) {
          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            gobject-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ gtk4 ];
            };
            javascriptcore6-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ webkitgtk_6_0 ];
            };
            gio-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ glib ];
            };
            soup3-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ webkitgtk_6_0 ];
            };
            gdk-pixbuf-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ gtk4 ];
            };
            libadwaita-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ libadwaita ];
            };
            webkit6-sys = attrs: {
              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [ webkitgtk_6_0 ];
            };
          };
        };
        defaultPackage = cargoNix.rootCrate.build.overrideAttrs (oldAttrs: {
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

            cp $src/assets/icon.svg $out/share/pixmaps/${packageData.name}.svg
          '';
        });
      in rec {
        packages = {
          default = defaultPackage;
        };

        checks = {
          rustnix = packages.default.override {
            runTests = true;
          };
        };

        devShells.default = defaultPackage.overrideAttrs (oldAttrs: {
          buildInputs = oldAttrs.buildInputs ++ (with pkgs; [
            rust-overlay.packages.${system}.default
            yamllint
            reuse
          ]);
        });
      };
    }
  );
}

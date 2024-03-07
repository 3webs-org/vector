# Copyright 2024 3WEBS LLC
# SPDX-License-Identifier: GPL-3.0-or-later

{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration/53af4303dda1fe6e575b2c5ee662ac9b23a18c9f";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [
        nci.flakeModule
        ./crates.nix
      ];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        # shorthand for accessing this crate's outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        crateOutputs = config.nci.outputs."vanadium-browser";
      in {
        # export the crate devshell as the default devshell
        devShells.default = crateOutputs.devShell;
        # export the release package of the crate as default package
        packages.default = crateOutputs.packages.release;
      };
    };
}

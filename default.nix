{ pkgs ? import <nixpkgs> { }, lib }: with pkgs; let
    cargoToml = builtins.readFile ./Cargo.toml;
    packageData = lib.getAttr "package" (builtins.fromTOML cargoToml);
    customPackageData = lib.getAttr "metadata" packageData;
in rust.packages.stable.rustPlatform.buildRustPackage rec {
    pname = packageData.name;
    version = packageData.version;
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

    buildPhase = ''
        cargo build --release --locked --all-features --target-dir=target
    '';

    installPhase = ''
        mkdir -p $out/share/applications $out/share/pixmaps $out/bin

        cat > $out/share/applications/${packageData.name}.desktop <<EOF
        [Desktop Entry]
        Name=${customPackageData.human_readable_name}
        Exec=${packageData.name}
        Icon=${packageData.name}
        Type=Application
        Categories=Network;WebBrowser;
        EOF

        cp target/release/${packageData.name} $out/bin/${packageData.name}
        cp ./icon.svg $out/share/pixmaps/${packageData.name}.svg
    '';

    meta = {
        description = packageData.description;
        homepage = packageData.homepage;
        platforms = lib.platforms.linux;
        license = packageData.license;
        maintainers = [ ]; # TODO
        mainProgram = "${packageData.name}";
    };
}

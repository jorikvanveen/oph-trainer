{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    utils.lib.eachSystem
      [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ]
      (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          version = (builtins.fromTOML (builtins.readFile ./backend/Cargo.toml)).package.version;

          frontend = pkgs.buildNpmPackage {
            pname = "oph-trainer-frontend";
            inherit version;
            src = ./frontend;
            npmDeps = pkgs.importNpmLock {
              npmRoot = ./frontend;
            };
            npmConfigHook = pkgs.importNpmLock.npmConfigHook;
            meta.platforms = [ "x86_64-linux" ];

            installPhase = ''
              runHook preInstall
              mkdir -p "$out"
              cp -r build/. "$out/"
              runHook postInstall
            '';
          };

          oph-trainer = pkgs.rustPlatform.buildRustPackage {
            pname = "oph-trainer";
            inherit version;
            src = pkgs.lib.cleanSource ./.;
            sourceRoot = "source/backend";
            cargoLock.lockFile = ./backend/Cargo.lock;
            cargoBuildFlags = [
              "--package"
              "oph-trainer-backend"
            ];
            cargoTestFlags = [ "--workspace" ];
            nativeBuildInputs = [ pkgs.makeWrapper ];
            passthru = { inherit frontend; };

            # Cargo's DTO export tests write to the sibling frontend tree.
            preCheck = ''
              chmod -R u+w ../frontend
            '';

            postInstall = ''
              mkdir -p "$out/share/oph-trainer"
              ln -s ${frontend} "$out/share/oph-trainer/static"
              wrapProgram "$out/bin/oph-trainer-backend" \
                --set-default OPH_STATIC_DIR "$out/share/oph-trainer/static"
            '';

            meta = {
              description = "Unofficial Orpheus interview trainer";
              homepage = "https://github.com/jorikvanveen/oph-trainer";
              mainProgram = "oph-trainer-backend";
              platforms = [ "x86_64-linux" ];
            };
          };
        in
        {
          packages = pkgs.lib.optionalAttrs (system == "x86_64-linux") {
            inherit oph-trainer;
            oph-trainer-frontend = frontend;
            default = oph-trainer;
          };

          devShell = pkgs.mkShell {
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
              nodejs
              sea-orm-cli
            ];
          };
        }
      );
}

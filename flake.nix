{
  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = import inputs.systems;

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        pkgsWithOverlays = inputs.nixpkgs.legacyPackages.${system}.extend inputs.rust-overlay.overlays.default;

        mkShellMini = import inputs.mini-shell pkgs;

        rust = pkgs.rust-bin.stable."1.77.0".default.override (orig: {
          extensions = ["rust-src"] ++ orig.extensions;
        });
      in {
        _module.args.pkgs = pkgsWithOverlays;

        formatter = pkgs.alejandra;

        devShells.default = mkShellMini {
          packages = builtins.attrValues {
            inherit (pkgs) cargo-nextest cargo-audit cargo-deny cargo-tarpaulin rust-analyzer-unwrapped;
            inherit (pkgs) nil pre-commit;
            inherit (pkgs) rune-languageserver rune;
            inherit rust;
          };

          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };
      };
    };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay.url = "github:oxalica/rust-overlay";

    mini-shell.url = "github:viperML/mkshell-minimal";
    mini-shell.flake = false;

    systems.url = "github:nix-systems/default-linux";
    systems.flake = false;
  };
}

{ sources ? import nix/sources.nix }:

let
  pkgs = import sources.nixpkgs {
    overlays = [ (import sources.rust-overlay) ];
  };
in pkgs.mkShell {
  nativeBuildInputs = [
    ((pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
      extensions = [ "rust-analyzer" "rust-src" ];
    })
  ];
}

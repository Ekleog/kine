let
  pkgs = import ./nix;
in
pkgs.stdenv.mkDerivation {
  name = "kine";
  buildInputs = (
    (with pkgs; [
      cargo-bolero
      cargo-nextest
      cargo-udeps
      niv

      (fenix.combine (with fenix; [
        complete.cargo
        complete.clippy
        complete.rust-analyzer
        complete.rustc
        complete.rustfmt
      ]))
    ])
  );
}

{
  pkgs ?
    import <nixpkgs> {
    },
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    exercism
    mob
    rustup
  ];

  RUST_BACKTRACE = 1;

  shellHook = ''
    echo "---"
    exercism version
    echo "mob $(mob --version)"
    rustc --version
    cargo --version
    echo "---"
  '';
}

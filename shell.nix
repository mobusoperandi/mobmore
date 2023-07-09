{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        # master on 2023-05-13
        (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/7ec9793168e4c328f08d10ab7ef4a1ada2dbf93e.tar.gz"))
      ];
    },
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    exercism
    mob
    (rust-bin.stable.latest.default.override {
      extensions = ["rust-src"];
    })
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

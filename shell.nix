let
  # nixos-22.11 on 2023-05-13
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/9656e85a15a0fe67847ee8cdb99a20d8df499962.tar.gz")) {
    overlays = [
      # master on 2023-05-13
      (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/7ec9793168e4c328f08d10ab7ef4a1ada2dbf93e.tar.gz"))
    ];
  };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    exercism
    mob
    (rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
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

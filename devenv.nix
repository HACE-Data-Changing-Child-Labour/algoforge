{ pkgs, lib, ... }:
let
  # NOTE: Outside of nix, there's no need to build this way
  # Just use your system's package manager to install it
  pdfiumBin = pkgs.stdenv.mkDerivation {
    pname = "pdfium";
    version = "latest";

    src = pkgs.fetchurl {
      url =
        "https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz";
      sha256 = "sha256-h4Pg5G2EYSUie3c11WJcKESiX/MVwL9gooNNRXYiuSE=";
    };

    sourceRoot = ".";

    unpackPhase = ''
      mkdir -p $out
      tar -xzf $src -C $out
    '';

    installPhase = ''
      patchelf --set-rpath $out/lib $out/lib/libpdfium.so
    '';
  };
in {
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  languages.python = {
    enable = true;
    package = pkgs.python312;
    venv = { 
      enable = true; 
      requirements = ''
        maturin
      '';
    };
  };

  packages = with pkgs; [
    openssl # Needed for cargo to compile packages
    file # On nix, this includes `libmagic`
    pdfiumBin # Needed for pdfium bindings
    rust-analyzer
    rustup
  ];

  enterShell = ''
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pdfiumBin}/lib
  '';
}

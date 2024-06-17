{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.fontconfig    # For yeslogic-fontconfig-sys
    pkgs.libopus       # For audiopus_sys
    pkgs.pkg-config    # For locating .pc files, updated name
    pkgs.cmake         # For building dependencies from source
    pkgs.gnumake       # Correct attribute for GNU Make
    pkgs.gcc           # C compiler, often needed for building native extensions
    pkgs.rustup        # Rust toolchain manager
  ];

  shellHook = ''
    # Set PKG_CONFIG_PATH to include the path to .pc files from the dependencies
    export PKG_CONFIG_PATH="${pkgs.fontconfig}/lib/pkgconfig:${pkgs.libopus}/lib/pkgconfig:$PKG_CONFIG_PATH"

    # Additional environment setup can be done here
    echo "Environment setup complete. Ready to build Rust projects."
  '';
}

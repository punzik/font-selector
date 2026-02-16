{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    pkg-config
    gtk4
    glib
    pango
    cairo
    gdk-pixbuf
    graphene
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}

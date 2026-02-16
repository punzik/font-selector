{ lib
, rustPlatform
, pkg-config
, wrapGAppsHook4
, gtk4
, glib
, pango
, cairo
, gdk-pixbuf
, graphene
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "font-selector";
  version = "0.2.0";

  src = fetchFromGitHub {
    owner = "punzik";
    repo = "font-selector";
    rev = "refs/tags/${version}";
    hash = "sha256-fVgFzMDVza7QKPp5aMg7NjVGFeI/N9gLH0FNq8UE5Pg=";
  };

  cargoHash = "sha256-1VTBajfyvnN5zEEtZj1g6pwc9yYEBq7/pM2ZTV+mdZA=";

  nativeBuildInputs = [
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = [
    gtk4
    glib
    pango
    cairo
    gdk-pixbuf
    graphene
  ];

  meta = with lib; {
    description = "Simple GTK4 app to preview installed system fonts";
    homepage = "https://github.com/punzik/font-selector";
    license = lib.licenses.mit;
    platforms = platforms.linux;
    mainProgram = "font-selector";
  };
}

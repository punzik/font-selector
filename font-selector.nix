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
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "punzik";
    repo = "font-selector";
    rev = "refs/tags/${version}";
    hash = "sha256-8ZCxm3eCG4YytQvb0OJiEzAQL+aO5kyTjuyqBje3cZc=";
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

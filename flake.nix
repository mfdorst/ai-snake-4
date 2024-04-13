{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      allSystems = [
        "x86_64-linux"
      ];

      forAllSystems = fn:
        nixpkgs.lib.genAttrs allSystems
        (system: fn { pkgs = import nixpkgs { inherit system; }; });

    in {
      devShells = forAllSystems ({ pkgs }: {
        default = with pkgs; pkgs.mkShell rec {
          nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [
            udev alsa-lib vulkan-loader
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      });
    };
}

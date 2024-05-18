{
  inputs = { nixpkgs.url = "nixpkgs/nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      systems =
        [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

      forAllSystems = fn:
        nixpkgs.lib.genAttrs systems
        (system: fn { pkgs = import nixpkgs { inherit system; }; });

    in {
      devShells = forAllSystems ({ pkgs }: {
        default = with pkgs;
          pkgs.mkShell rec {
            name = "snake";
            nativeBuildInputs = [ pkg-config ];
            buildInputs = [
              cargo
              clippy
              rustc
              rustfmt
              rust-analyzer

              udev
              alsa-lib
              vulkan-loader

              # To use the x11 feature
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr

              # To use the wayland feature
              libxkbcommon
              wayland
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
      });
      formatter = forAllSystems ({ pkgs }: pkgs.nixfmt-classic);
    };
}

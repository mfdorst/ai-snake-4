{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      overlays = [ (import rust-overlay) ];

      forAllSystems = fn:
        nixpkgs.lib.genAttrs systems
          (system: fn { pkgs = import nixpkgs { inherit system overlays; }; });

    in {
      devShells = forAllSystems ({ pkgs }: {
        default = with pkgs; pkgs.mkShell rec {
          name = "snake";
          nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [
            rust-bin.stable.latest.default
            udev alsa-lib vulkan-loader
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      });
    };
}

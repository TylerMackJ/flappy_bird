{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";

  outputs = inputs@{ parts, nci, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [ nci.flakeModule ./crates.nix ];
      perSystem = { pkgs, config, lib, ... }:
        let
          # shorthand for accessing this crate's outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          crateOutputs = config.nci.outputs."flappy_bird";
        in {
          # export the crate devshell as the default devshell
          devShells.default = crateOutputs.devShell.overrideAttrs (old: {
            nativeBuildInputs = (old.nativeBuildInputs or [ ])
              ++ [ pkgs.pkg-config ];
            buildInputs = (old.buildInputs or [ ]) ++ [
              pkgs.alsa-lib
              pkgs.libxkbcommon
              pkgs.udev
              pkgs.vulkan-loader
              pkgs.wayland
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
            ];
            packages = (old.packages or [ ])
              ++ [ pkgs.clang pkgs.mold pkgs.rust-analyzer ];
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              pkgs.alsa-lib
              pkgs.libxkbcommon
              pkgs.udev
              pkgs.vulkan-loader
              pkgs.wayland
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
            ];
          });
          # export the release package of the crate as default package
          packages.default = crateOutputs.packages.release;
        };
    };
}

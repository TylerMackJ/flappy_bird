{...}: {
  perSystem = {
    pkgs,
    config,
    ...
  }: let
    crateName = "flappy_bird";
  in {
    # declare projects
    nci.projects.${crateName}.path = ./.;
    # configure crates
    nci.crates.${crateName} = {};
  };
}

{
  system = "x86_64-linux";
  user = {
    username = "joao";
    homeDirectory = "/home/joao";
  };
  monitors = import ./monitors.nix;
}

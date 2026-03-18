{
  description = "NixOS config";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, home-manager, ... }@inputs: 
  let
    # Supported architectures
    supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
    
    # Helper to generate attrs for each system
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    
    # Default user configuration - override per host if needed
    defaultUser = {
      username = "joao";
      homeDirectory = "/home/joao";
    };
    
    # Path to this repo (where dotfiles are stored)
    # Dotfiles are at ${nixosConfigPath}/dotfiles
    nixosConfigPath = "/home/joao/nixos";

    # Host definitions - add new machines here
    hosts = {
      nixos = {
        system = "x86_64-linux";
        user = defaultUser;
      };
      # Example: add more hosts like this:
      # laptop = {
      #   system = "aarch64-linux";
      #   user = defaultUser;
      # };
    };

    # Function to create a NixOS configuration for a host
    mkHost = hostname: hostConfig: 
      nixpkgs.lib.nixosSystem {
        system = hostConfig.system;
        specialArgs = { 
          inherit inputs hostname nixosConfigPath;
          userConfig = hostConfig.user;
        };
        modules = [
          ./hosts/${hostname}/configuration.nix
          home-manager.nixosModules.home-manager
          {
            home-manager.extraSpecialArgs = { 
              inherit inputs nixosConfigPath;
              userConfig = hostConfig.user;
            };
            home-manager.users.${hostConfig.user.username} = {
              imports = [ ./home.nix ];
            };
          }
        ];
      };

  in {
    nixosConfigurations = nixpkgs.lib.mapAttrs mkHost hosts;
  };
}

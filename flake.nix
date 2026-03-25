{
  description = "Amber - NixOS Configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, home-manager, ... }@inputs: 
  let
    supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
    
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    
    defaultUser = {
      username = "joao";
      homeDirectory = "/home/joao";
    };
    
    amberPath = builtins.toString ./.;

    devMode = false;

    amberLib = import ./core/lib.nix { lib = nixpkgs.lib; };

    hosts = {
      personal = {
        system = "x86_64-linux";
        user = defaultUser;
      };
      work = {
        system = "x86_64-linux";
        user = defaultUser;
      };
      vm = {
        system = "x86_64-linux";
        user = defaultUser;
      };
    };

    mkHost = hostname: hostConfig: 
      nixpkgs.lib.nixosSystem {
        system = hostConfig.system;
        specialArgs = { 
          inherit inputs hostname amberPath devMode amberLib;
          userConfig = hostConfig.user;
        };
        modules = [
          ./core
          ./domains/wm/hyprland/system.nix
          ./hosts/${hostname}/configuration.nix
          home-manager.nixosModules.home-manager
          ({ config, ... }: {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.backupFileExtension = "backup";
            home-manager.extraSpecialArgs = { 
              inherit inputs amberPath devMode amberLib;
              userConfig = hostConfig.user;
              monitors = config.monitors;
            };
            home-manager.users.${hostConfig.user.username} = {
              imports = [ ./home.nix ];
            };
          })
        ];
      };

  in {
    nixosConfigurations = nixpkgs.lib.mapAttrs mkHost hosts;
  };
}

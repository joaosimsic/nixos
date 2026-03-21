{
  description = "NixOS config";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    elephant.url = "github:abenz1267/elephant";

    walker = {
      url = "github:abenz1267/walker";
      inputs.elephant.follows = "elephant";
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
    
    nixosConfigPath = builtins.toString ./.;

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
          inherit inputs hostname nixosConfigPath;
          userConfig = hostConfig.user;
        };
        modules = [
          ./modules/monitors.nix
          ./hosts/${hostname}/configuration.nix
          home-manager.nixosModules.home-manager
          ({ config, ... }: {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.backupFileExtension = "backup";
            home-manager.extraSpecialArgs = { 
              inherit inputs nixosConfigPath;
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

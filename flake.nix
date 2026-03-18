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
    supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
    
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    
    defaultUser = {
      username = "joao";
      homeDirectory = "/home/joao";
    };
    
    nixosConfigPath = "/home/joao/nixos";

    hosts = {
      nixos = {
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
          ./hosts/${hostname}/configuration.nix
          home-manager.nixosModules.home-manager
          {
            home-manager.useGlobalPkgs = true;
            home-manager.useUserPackages = true;
            home-manager.backupFileExtension = "backup";
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

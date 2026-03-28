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
    
    amberPath = "/home/joao/.config/amber";

    defaultMonitors = {
      primary = {
        name = "DP-1";
        resolution = "1920x1080";
        refreshRate = 144;
      };
      secondary = {
        name = "HDMI-A-1";
        resolution = "1920x1080";
        refreshRate = 60;
      };
    };

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
          inherit inputs hostname amberPath;
          userConfig = hostConfig.user;
        };
        modules = [
          ./core
          ./domains/wm/hyprland/system.nix
          ./hosts/${hostname}/configuration.nix
        ];
      };

  in {
    nixosConfigurations = nixpkgs.lib.mapAttrs mkHost hosts;

    homeConfigurations."joao" = home-manager.lib.homeManagerConfiguration {
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        config.allowUnfree = true;
      };
      extraSpecialArgs = {
        inherit inputs amberPath;
        userConfig = defaultUser;
        monitors = defaultMonitors;
      };
      modules = [
        ./home.nix
        {
          home.stateVersion = "24.05";
          programs.home-manager.enable = true;
          home.backupFileExtension = "backup";
        }
      ];
    };
  };
}

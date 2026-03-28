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
    amberPath = "/home/joao/.config/amber";

    hostNames = [ "personal" "work" "vm" ];

    getHostConfig = hostname: import ./hosts/${hostname};

    mkHost = hostname: 
      let
        hostConfig = getHostConfig hostname;
      in
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

    mkHome = hostname: 
      let
        hostConfig = getHostConfig hostname;
      in
      home-manager.lib.homeManagerConfiguration {
        pkgs = import nixpkgs {
          system = hostConfig.system;
          config.allowUnfree = true;
        };
        extraSpecialArgs = {
          inherit inputs amberPath;
          userConfig = hostConfig.user;
        };
        modules = [ ./home.nix ];
      };

  in {
    nixosConfigurations = nixpkgs.lib.genAttrs hostNames mkHost;

    homeConfigurations = 
      let
        perHost = nixpkgs.lib.genAttrs 
          (map (h: "joao@${h}") hostNames) 
          (name: mkHome (nixpkgs.lib.removePrefix "joao@" name));
      in
      perHost // { "joao" = mkHome "personal"; };
  };
}

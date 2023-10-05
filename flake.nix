{
  description = "ndent";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in let
      ndent = let
          manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
    in {
      packages.x86_64-linux = rec {
        inherit ndent;
        
        default = ndent;
      };
      
      devShells.x86_64-linux = {
        default = pkgs.mkShell {
          inputsFrom = [
            ndent
          ];
          
          buildInputs = with pkgs; [
            rust-analyzer
            clippy
          ];
        };
      };
    };
}
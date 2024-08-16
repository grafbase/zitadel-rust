{
  # Nix: https://nixos.org/download.html
  # How to activate flakes: https://nixos.wiki/wiki/Flakes
  # For seamless integration, consider using:
  # - direnv: https://github.com/direnv/direnv
  # - nix-direnv: https://github.com/nix-community/nix-direnv
  #
  # # .envrc
  # use flake
  # dotenv .env
  #
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: let
    inherit (nixpkgs.lib) optional;
    systems = flake-utils.lib.system;
  in
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      aarch64LinuxPkgs = import nixpkgs {
        inherit system;
        crossSystem = {
          config = "aarch64-unknown-linux-musl";
        };
      };
      aarch64LinuxBuildPkgs = aarch64LinuxPkgs.buildPackages;

      x86_64LinuxPkgs = import nixpkgs {
        inherit system;
        crossSystem = {
          config = "x86_64-unknown-linux-musl";
        };
      };
      x86_64LinuxBuildPkgs = x86_64LinuxPkgs.buildPackages;
    in {
      devShells.default =
        pkgs.mkShell
        {
          nativeBuildInputs = with pkgs;
            [
              # https://discourse.nixos.org/t/non-interactive-bash-errors-from-flake-nix-mkshell/33310
              bashInteractive
              just
              cmake
              openssl
              pkg-config
              rustup
              protobuf
              libiconv
              nodePackages.prettier
              cargo-nextest
              llvm
              buf
              protoc-gen-tonic
              protoc-gen-prost
              protoc-gen-prost-crate
              clang
            ]
            ++ optional (system == systems.aarch64-darwin) [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
              darwin.apple_sdk.frameworks.CoreFoundation
              darwin.apple_sdk.frameworks.CoreServices
            ]
            ++ optional (system == systems.x86_64-linux) [
              # Ideally we would be using stdenv to keep clang on Darwin, but I couldn't make the ring library cross
              # compile properly.
              aarch64LinuxBuildPkgs.gcc
              x86_64LinuxBuildPkgs.gcc
              # not useful to build them, but only useful when you can build lambdas in the first place.
              cargo-lambda
            ];
        };
    });
}

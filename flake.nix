{
  description = "Flake for building LLM-SEO project with standard nixpkgs Rust and OpenSSL support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            pkg-config
            sqlx-cli
            postgresql
            sqlite
            nodejs
            pnpm
            zip
            unzip
            rsync
            openssl.dev
            openssl

            chromedriver
            chromium
            xvfb-run

            typescript
            nodePackages.typescript

            rclone

            csvlens

            act
            docker
            docker-compose
          ];
        };

        devShell = self.devShells.${system}.default;
      }
    );
}

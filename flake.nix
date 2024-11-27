{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }@inputs:
    let
      inherit (self) outputs;
      forAllSystems = nixpkgs.lib.genAttrs [
        "aarch64-linux"
        "x86_64-linux"
      ];
    in
    {
      devShell = forAllSystems (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rust-toolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        pkgs.mkShell {
          buildInputs = [
            rust-toolchain
            pkgs.postgresql
            pkgs.sqlx-cli
            pkgs.pgadmin4-desktopmode
          ];

          shellHook = ''
            export PGDATA="$PWD/.pgdata"
            export PGPORT=5432
            export PGHOST="localhost"

            export POSTGRES_USER="recipe"
            export POSTGRES_PASSWORD="test"
            export POSTGRES_DBNAME="recipe"
            export DATABASE_URL="postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:5432/$POSTGRES_DBNAME"
          '';
        }
      );
    };
}

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
          ];

          shellHook = ''
            export PGDATA="$PWD/.pgdata"
            export PGPORT=5432
            export PGHOST="localhost"

            export POSTGRES_USER="recipe"
            export POSTGRES_PASSWORD="test"
            export POSTGRES_DBNAME="recipe"
            export DATABASE_URL="postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:5432/$POSTGRES_DBNAME"

            if [ ! -d "$PGDATA" ]; then 
              echo "Initializing PostgreSQL database..."
              initdb --username=postgres --encoding=UTF8 "$PGDATA"
              pg_ctl -D "$PGDATA" -o "-k /tmp" -l "$PGDATA/logfile" start
              psql -U postgres -c "CREATE ROLE $POSTGRES_USER WITH LOGIN PASSWORD '$POSTGRES_PASSWORD';"
              psql -U postgres -c "CREATE DATABASE $POSTGRES_DBNAME;"
              psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE $POSTGRES_DBNAME TO $POSTGRES_USER;"
              psql -U postgres -d $POSTGRES_DBNAME -c "GRANT ALL ON SCHEMA public TO $POSTGRES_USER;"
              pg_ctl -D "$PGDATA" stop
            fi

            echo "Starting PostgreSQL server..."
            pg_ctl -D "$PGDATA" -o "-k /tmp" -l "$PGDATA/logfile" start

            echo "PostgreSQL server running on port $PGPORT"

            trap "echo 'Stopping PostgreSQL server...'; pg_ctl -D '$PGDATA' stop" EXIT
          '';
        }
      );
    };
}

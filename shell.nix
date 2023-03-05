{ target ? "default", pkgs ? import <nixpkgs> {} }:
let
  dbStop = pkgs.writeShellScriptBin "db-stop" ''
      echo "Shutting down the database..."
      ${pkgs.postgresql_15}/bin/pg_ctl stop &> /dev/null
      echo "Done!"
  '';
  dbStart = pkgs.writeShellScriptBin "db-start" ''
      if [ ! -f $PGDATA/postmaster.pid ]; then
        ${pkgs.postgresql_15}/bin/pg_ctl -l $PGDATA/logfile start
      else
        echo "Postgres is already running"
      fi
  '';
in
  pkgs.mkShell rec {
    name = "rssc-intract-env";

    buildInputs = [
      pkgs.pkg-config
      pkgs.openssl
      pkgs.postgresql
      pkgs.diesel-cli
      dbStop
      dbStart
    ];

    PGHOST = "localhost";
    PGPORT = "5432";
    PGUSER = "postgres";
    PGDATABASE = "postgres";
    PGDATA = "${toString ./.}/.db/postgres";
    PGCONF = "${PGDATA}/postgresql.conf";
    SOCKET_DIRECTORIES = "${toString ./.}/.db/postgres-sockets";

    postgresConf =
      pkgs.writeText "postgresql.conf"
        ''
          log_min_messages = warning
          log_min_error_statement = error
          log_min_duration_statement = 100  # ms
          log_connections = on
          log_disconnections = on
          log_duration = on
          log_timezone = 'UTC'
          log_statement = 'all'
          log_directory = 'logs'
          log_filename = 'postgresql-%Y-%m-%d_%H%M%S.log'
          logging_collector = on
          log_min_error_statement = error
          port = ${PGPORT}
          listen_addresses = '${PGHOST}'
          unix_socket_directories = '${SOCKET_DIRECTORIES}'
      '';

    shellHook = ''
      mkdir -p ${PGDATA} && mkdir -p ${SOCKET_DIRECTORIES}

      if [ ! -f $PGCONF ];
      then
        ${pkgs.postgresql_14}/bin/pg_ctl initdb -o "-U $PGUSER"
        cat "$postgresConf" >> $PGCONF
        ${pkgs.postgresql_14}/bin/pg_ctl -l $PGDATA/logfile start
        ${pkgs.postgresql_14}/bin/psql -h localhost -c 'CREATE DATABASE intract'
      fi

      db-start
    '';
}

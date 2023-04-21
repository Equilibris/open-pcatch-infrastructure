{
  description = "Flake utils demo";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [ sqlx-cli insomnia ];
          shellHook =
            ''
              export DATABASE_URL="postgres://postgres:password@localhost/test"
            '';
        };
      }
    );
}

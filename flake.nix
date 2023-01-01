{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, fenix }: {
    devShells.x86_64-linux.default = let
      pkgs = nixpkgs.legacyPackages.x86_64-linux.extend fenix.overlays.default;
    in pkgs.mkShell {
      nativeBuildInputs = (with pkgs; [ nil nixfmt clang ])
        ++ (with fenix.packages.x86_64-linux; [
          (complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
          rust-analyzer
        ]);
      LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    };
  };
}

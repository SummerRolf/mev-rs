{ pkgs }:
with pkgs;
mkShell {
  buildInputs = lib.optionals pkgs.stdenv.isLinux [
    openssl
  ] ++ lib.optionals pkgs.stdenv.isDarwin [
    libiconv
    darwin.apple_sdk.frameworks.CFNetwork
    darwin.apple_sdk.frameworks.SystemConfiguration
  ] ++ [
    cargo-udeps
    just
    mdbook
  ];
}

{ pkgs ? import <nixpkgs> {} }:

with pkgs;
mkShell {
  buildInputs = [
      corepack_20
      nodejs_20
      deno
      openssl
      protobuf
      rustup
      llvmPackages.libclang
    ];
  shellHook =
    let versionParts = builtins.split "\\." llvmPackages.libclang.version;
        majorVersion = builtins.head versionParts;
    in ''
      export LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib"
      export BINDGEN_EXTRA_CLANG_ARGS="-isystem ${llvmPackages.libclang.lib}/lib/clang/${majorVersion}/include"

      rustup update nightly
      rustup target add wasm32-unknown-unknown
      rustup target add wasm32-unknown-unknown --toolchain nightly

      rustup component add rust-src
      rustup component add rust-src --toolchain nightly

      rustup default nightly
    '';
}

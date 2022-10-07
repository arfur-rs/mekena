{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = {
    self,
    nci,
    ...
  } @ inputs:
    nci.lib.makeOutputs {
      root = ./.;

      overrides.shell = common: prev: {
        packages =
          prev.packages
          ++ (with common.pkgs; [
            pkg-config
            openssl.dev
            glibc_multi
            rust-analyzer
            cargo-outdated
            cargo-audit
            cargo-release
            cargo-tarpaulin
            cargo-nextest
            git-cliff
          ]);
      };
    };
}

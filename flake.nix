{
  inputs.flakelight-rust.url = "github:accelbread/flakelight-rust";
  outputs = { flakelight-rust, ... }: flakelight-rust ./. ({ lib, src, ... }: {
    devShell.packages = pkgs: [ pkgs.pkg-config pkgs.aria2 ];
    devShell.env = pkgs: {
      OPENSSL_DIR = "${pkgs.openssl.dev}";
      OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    };

    package = lib.mkForce ({ craneLib, defaultMeta, pkgs, stdenv }:
      craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        # inherit cargoArtifacts;
        doCheck = false;
        strictDeps = true;
        meta = defaultMeta;

        nativeBuildInputs = [ pkgs.pkg-config ];

        # Dependencies which need to be built for the platform on which
        # the binary will run. In this case, we need to compile openssl
        # so that it can be linked with our executable.
        buildInputs = [
          # Add additional build inputs here
          pkgs.openssl
        ];
      });

    packages.linux-generic = { pkgs, stdenv }:
      stdenv.mkDerivation {
        name = "hf-aria-dl-linux-generic";

        src = pkgs.default;

        buildPhase = ''
          mkdir -p $out
          cp -r . $out
          
          patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 $out/bin/hf-aria-dl
        '';
      };
  });
}

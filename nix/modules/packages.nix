{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      lib,
      system,
      ...
    }:
    let
      mkDez = import ../toolchain.nix { inherit inputs; };
      dez-editor = mkDez pkgs;
    in
    {
      packages = {
        default = dez-editor;
        debug = dez-editor.override { profile = "dev"; };
      };
    }
    // lib.optionalAttrs (lib.hasSuffix "linux" system) {
      checks.a11y-test = import ../tests/a11y.nix {
        inherit pkgs inputs;
      };
    };
}

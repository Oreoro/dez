{ inputs, ... }:
{
  flake.overlays.default =
    final: _:
    let
      mkDez = import ../toolchain.nix { inherit inputs; };
    in
    {
      dez-editor = mkDez final;
    };
}

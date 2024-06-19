{ pkgs, ... }:
{
  packages = with pkgs; [
    sqlx-cli
    nixfmt-rfc-style
    cargo-expand
  ];

  dotenv.enable = true;
  languages.rust = {
    enable = true;
    channel = "nightly";
  };
}

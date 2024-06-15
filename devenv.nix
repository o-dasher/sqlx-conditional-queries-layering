{ pkgs, ... }:
{
  packages = with pkgs; [
	sqlx-cli
	nixfmt-rfc-style
  ];
  dotenv.enable = true;
  languages.rust = {
	  enable = true;
	  channel = "nightly";
  };
}

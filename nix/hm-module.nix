self: { config, pkgs, lib, ... }: with lib; let
  cfg = config.programs.steam-deck-remapper;
  format = pkgs.formats.toml {};
in {
  options.programs.steam-deck-remapper = {
    enable = mkEnableOption "Steam Deck input remapper";

    package = mkOption {
      type = types.nullOr types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = literalExpression "inputs.steam-deck-remapper.packages.${pkgs.stdenv.hostPlatform.system}.default";
      description = ''
        The steam-deck-remapper package to use.
                
        By default, this option is set to the steam-deck-remapper package for your host system.
      '';
    };

    settings = mkOption {
      inherit (format) type;
      default = {};
      example = builtins.fromTOML (builtins.readFile (self + /config.example.toml));
      description = "Settings for Steam Deck input remapper";
    };
  };

  config = mkIf cfg.enable {
    home.packages = [
      cfg.package
    ];
    
    xdg.configFile."steam-deck-remapper/config.toml".source = format.generate "config.toml" cfg.settings;
  };
} 

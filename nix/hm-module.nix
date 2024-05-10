self: { config, pkgs, lib, ... }: with lib; let
  cfg = config.services.steam-deck-remapper;
  format = pkgs.formats.toml {};
in {
  options = {
    services.steam-deck-remapper = {
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
  };

  config = mkIf cfg.enable {
    home.packages = [
      cfg.package
    ];

    systemd.user.services."steam-deck-remapper" = lib.mkIf cfg.enable {
      Unit = {
        Description = "Systemd service for Steam Deck input remapper";
        Requires = ["graphical-session.target"];
      };

      Service = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/steam-deck-remapper";
      };

      Install.WantedBy = [
        (lib.mkIf config.wayland.windowManager.hyprland.systemd.enable "hyprland-session.target")
        (lib.mkIf config.wayland.windowManager.sway.systemd.enable "sway-session.target")
      ];
    };
    
    xdg.configFile."steam-deck-remapper/config.toml".source = format.generate "config.toml" cfg.settings;
  };
} 

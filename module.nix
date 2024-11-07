flake:
{
  config,
  lib,
  pkgs,
  ...
}:
with lib;

let
  service_name = "divera-reports";
  cfg = config.services.${service_name};
  pkg = (flake.defaultPackage.${pkgs.stdenv.hostPlatform.system});
in
{
  options.services.${service_name} = {
    enable = mkEnableOption "enable ${service_name} services";
    config_path = mkOption {
      type = types.path;
      description = "The config path";
    };
    timer = mkOption {
      type = types.str;
      description = "The timer value to set";
    };
  };

  config = mkIf cfg.enable {
    systemd.services."${service_name}-station" = {
      description = "Uploads station divera reports";
      path = [ "${pkg}" ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "divera-reports --config ${cfg.config_path} report station --upload Verbesserungsvorschläge_Feuerwehrhaus.xlsx";
        ProtectHome = "read-only";
        partOf = "${service_name}.target";
      };
    };

    systemd.services."${service_name}-roster" = {
      description = "Uploads roster divera reports";
      path = [ "${pkg}" ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "divera-reports --config ${cfg.config_path} report roster --upload Vorschläge_Dienstplan.xlsx";
        ProtectHome = "read-only";
        partOf = "${service_name}.target";
      };
    };

    systemd.services."${service_name}-absences" = {
      description = "Uploads absences divera reports";
      path = [ "${pkg}" ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "divera-reports --config ${cfg.config_path} report absences --upload Abwesenheiten.xlsx";
        ProtectHome = "read-only";
        partOf = "${service_name}.target";
      };
    };

    systemd.services."${service_name}-fire-operation" = {
      description = "Uploads absences divera reports";
      path = [ "${pkg}" ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "divera-reports --config ${cfg.config_path} report fire-operation --upload Atemschutz_Kurzbericht.xlsx";
        ProtectHome = "read-only";
        partOf = "${service_name}.target";
      };
    };

    # Define the systemd target to group multiple services
    systemd.targets.${service_name} = {
      description = "Group of divera reports services";
    };

    systemd.timers.${service_name} = {
      description = "${service_name} timer";
      wantedBy = [ "timers.target" ]; # Ensure the timer is activated at boot
      timerConfig = {
        OnCalendar = cfg.timer;
        Persistent = true; # Ensures the job runs after missed events (e.g., after reboot)
        Unit = "${service_name}.target";
      };
    };
  };
}

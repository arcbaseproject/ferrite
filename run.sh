#!/bin/sh
# Run Ferrite from this checkout. Pass --launcher for the separate desktop launcher.
set -e
cd "$(dirname "$0")"
if [ "${1:-}" = "--launcher" ]; then
    shift
    cd launcher
    exec cargo run --release -- "$@"
fi

case "$(uname -s)" in
    Darwin) ferrite_data_dir="$HOME/Library/Application Support" ;;
    MINGW*|MSYS*|CYGWIN*) ferrite_data_dir="${APPDATA:-$HOME/AppData/Roaming}" ;;
    *)
        case "${XDG_DATA_HOME:-}" in
            /*) ferrite_data_dir="$XDG_DATA_HOME" ;;
            *) ferrite_data_dir="$HOME/.local/share" ;;
        esac ;;
esac

export HORIZON_HOME="${HORIZON_HOME:-$ferrite_data_dir/ferrite}"
export HORIZON_TELEMETRY_URL=off HORIZON_NO_HEARTBEAT=1
exec cargo run --profile fast -p horizon-game --bin horizon-client -- \
    --game-dir "$HORIZON_HOME" --resources "$PWD/.local/horizon-client/26.3" "$@"

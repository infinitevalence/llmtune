#!/usr/bin/env sh
# pkg.sh — agnostic OS package manager dispatcher.
# Detects the available package manager on the system and runs it.
#
# Usage:
#   ./pkg.sh install <pkg> [pkg ...]
#   ./pkg.sh remove <pkg> [pkg ...]
#   ./pkg.sh list
#   ./pkg.sh search <query>
#
# Auto-detect order: pacman > apk > apt > dnf > yum
# Calls `sudo` when the detected manager requires root.
#
# POSIX sh, no external dependencies beyond the PM itself.
set -eu

pkg_detect() {
    if command -v pacman >/dev/null 2>&1; then
        echo "pacman"
    elif command -v apk >/dev/null 2>&1; then
        echo "apk"
    elif command -v apt >/dev/null 2>&1; then
        echo "apt"
    elif command -v dnf >/dev/null 2>&1;      then
        echo "dnf"
    elif command -v yum >/dev/null 2>&1; then
        echo "yum"
    else
        printf '%s' "unsupported: no package manager found" >&2
        exit 1
    fi
}

privesc() {
    if command -v doas >/dev/null 2>&1; then echo "doas"; return 0; fi
    if command -v sudo >/dev/null 2>&1; then echo "sudo"; return 0; fi
    # fallback: run without elevation (assumes caller is already root)
    echo ""
}

pkg_install() {
    manager="$1"; shift
    case "$manager" in
        pacman) "$manager" -Su --noconfirm --needed "$@"  ;;
        apk)     "$manager" update && "$manager" add "$@"    ;;
        apt)     "$manager" update && "$manager" install "$@" ;;
        dnf|yum) "$manager" install "$@"                      ;;
    esac
}

pkg_remove() {
    manager="$1"; shift
    case "$manager" in
        pacman) "$manager" -Scc && "$manager" -Rsu --noconfirm "$@" ;;
        apk)     "$manager" del "$@"                                   ;;
        apt)     "$manager" remove "$@"                                ;;
        dnf|yum) "$manager" remove "$@"                               ;;
    esac
}

if [ $# -lt 2 ]; then
    echo "usage: $0 {install|remove|list|search} <args...>" >&2
    exit 1
fi

action="$1" ; shift
manager="$(pkg_detect)"
esc="$(privesc)"

case "$action" in
    install)
        if [ -n "$esc" ]; then
            $esc sh -c 'manager="$1"; shift; case "$manager" in pacman) "$manager" -Su --noconfirm --needed "$@" ;; apk) "$manager" update && "$manager" add "$@" ;; apt) "$manager" update && "$manager" install "$@" ;; dnf|yum) "$manager" install "$@" ;; esac' _ "$manager" "$@"
        else
            pkg_install "$manager" "$@"
        fi
        ;;
    remove)
        if [ -n "$esc" ]; then
            $esc sh -c 'manager="$1"; shift; case "$manager" in pacman) "$manager" -Scc && "$manager" -Rsu --noconfirm "$@" ;; apk) "$manager" del "$@" ;; apt) "$manager" remove "$@" ;; dnf|yum) "$manager" remove "$@" ;; esac' _ "$manager" "$@"
        else
            pkg_remove "$manager" "$@"
        fi
        ;;
    list)
        case "$manager" in
            pacman) pacman -Pa ;;
            apk)     apk list ;;
            apt)     apt list --installed ;;
            dnf|yum)  "$manager" list installed ;;
        esac
        ;;
    search)
        case "$manager" in
            pacman) pacman -Ss "$1" ;;
            apk)     apk search "$1" ;;
            apt)     apt search "$1" ;;
            dnf|yum)  "$manager" search "$1" ;;
        esac
        ;;
    *)
        echo "unknown action: $action" >&2
        exit 1
        ;;
esac

#!/usr/bin/env sh
set -eu

# Downloads a release tarball from GitHub Releases
# (https://github.com/shenghsi/dez/releases) and unpacks it into ~/.local/.
# Set ZED_VERSION to a tag (e.g. v0.3.7) to pin a version; it defaults to the
# latest release for the selected channel.
# Set ZED_CHANNEL=nightly to install the latest nightly build.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    channel="${ZED_CHANNEL:-stable}"
    ZED_VERSION="${ZED_VERSION:-latest}"
    case "$channel" in
        stable | nightly) ;;
        *)
            echo "Unsupported release channel: $channel" >&2
            exit 1
            ;;
    esac
    # Use TMPDIR if available (for environments with non-standard temp directories)
    if [ -n "${TMPDIR:-}" ] && [ -d "${TMPDIR}" ]; then
        temp="$(mktemp -d "$TMPDIR/dez-XXXXXX")"
    else
        temp="$(mktemp -d "/tmp/dez-XXXXXX")"
    fi

    if [ "$platform" = "Darwin" ]; then
        platform="macos"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$platform-$arch" in
        macos-arm64* | linux-arm64* | linux-armhf | linux-aarch64)
            arch="aarch64"
            ;;
        macos-x86* | linux-x86* | linux-i686*)
            arch="x86_64"
            ;;
        *)
            echo "Unsupported platform or architecture"
            exit 1
            ;;
    esac

    if command -v curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif command -v wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    "$platform" "$@"

    if [ "$(command -v dez)" = "$HOME/.local/bin/dez" ]; then
        echo "dez has been installed. Run with 'dez'"
    else
        echo "To run dez from your terminal, you must add ~/.local/bin to your PATH"
        echo "Run:"

        case "$SHELL" in
            *zsh)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
                echo "   source ~/.zshrc"
                ;;
            *fish)
                echo "   fish_add_path -U $HOME/.local/bin"
                ;;
            *)
                echo "   echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
                echo "   source ~/.bashrc"
                ;;
        esac

        echo "To run dez now, '~/.local/bin/dez'"
    fi
}

# Builds the GitHub Releases download URL for a given asset filename. Uses the
# "latest" redirect unless ZED_VERSION pins a specific tag (with or without the
# leading "v"). Nightly uses its moving tag.
github_release_url() {
    asset="$1"
    if [ "$ZED_VERSION" = "latest" ]; then
        if [ "$channel" = "nightly" ]; then
            echo "https://github.com/shenghsi/dez/releases/download/nightly/$asset"
        else
            echo "https://github.com/shenghsi/dez/releases/latest/download/$asset"
        fi
    else
        case "$ZED_VERSION" in
            v*) tag="$ZED_VERSION" ;;
            *) tag="v$ZED_VERSION" ;;
        esac
        echo "https://github.com/shenghsi/dez/releases/download/$tag/$asset"
    fi
}

linux() {
    if [ -n "${ZED_BUNDLE_PATH:-}" ]; then
        cp "$ZED_BUNDLE_PATH" "$temp/dez-linux-$arch.tar.gz"
    else
        echo "Downloading dez version: $ZED_VERSION"
        curl "$(github_release_url "dez-linux-$arch.tar.gz")" > "$temp/dez-linux-$arch.tar.gz"
    fi

    suffix=""
    if [ "$channel" != "stable" ]; then
        suffix="-$channel"
    fi

    appid=""
    case "$channel" in
      stable)
        appid="dev.dez.dez"
        ;;
      nightly)
        appid="dev.dez.dez-Nightly"
        ;;
      dev)
        appid="dev.dez.dez-Dev"
        ;;
      *)
        echo "Unknown release channel: ${channel}. Using stable app ID."
        appid="dev.dez.dez"
        ;;
    esac

    # Unpack
    rm -rf "$HOME/.local/dez$suffix.app"
    mkdir -p "$HOME/.local/dez$suffix.app"
    tar -xzf "$temp/dez-linux-$arch.tar.gz" -C "$HOME/.local/"

    # Setup ~/.local directories
    mkdir -p "$HOME/.local/bin" "$HOME/.local/share/applications"

    # Link the binary
    if [ -f "$HOME/.local/dez$suffix.app/bin/dez" ]; then
        ln -sf "$HOME/.local/dez$suffix.app/bin/dez" "$HOME/.local/bin/dez"
    else
        # support for versions before 0.139.x.
        ln -sf "$HOME/.local/dez$suffix.app/bin/cli" "$HOME/.local/bin/dez"
    fi

    # Copy .desktop file
    desktop_file_path="$HOME/.local/share/applications/${appid}.desktop"
    src_dir="$HOME/.local/dez$suffix.app/share/applications"
    if [ -f "$src_dir/${appid}.desktop" ]; then
        cp "$src_dir/${appid}.desktop" "${desktop_file_path}"
    else
        # Fallback for older tarballs
        cp "$src_dir/dez$suffix.desktop" "${desktop_file_path}"
    fi
    sed -i "s|Icon=dez|Icon=$HOME/.local/dez$suffix.app/share/icons/hicolor/512x512/apps/dez.png|g" "${desktop_file_path}"
    sed -i "s|Exec=dez|Exec=$HOME/.local/dez$suffix.app/bin/dez|g" "${desktop_file_path}"
}

macos() {
    echo "Downloading dez version: $ZED_VERSION"
    curl "$(github_release_url "dez-$arch.dmg")" > "$temp/dez-$arch.dmg"
    hdiutil attach -quiet "$temp/dez-$arch.dmg" -mountpoint "$temp/mount"
    app="$(cd "$temp/mount/"; echo *.app)"
    echo "Installing $app"
    if [ -d "/Applications/$app" ]; then
        echo "Removing existing $app"
        rm -rf "/Applications/$app"
    fi
    ditto "$temp/mount/$app" "/Applications/$app"
    hdiutil detach -quiet "$temp/mount"

    mkdir -p "$HOME/.local/bin"
    # Link the binary
    ln -sf "/Applications/$app/Contents/MacOS/cli" "$HOME/.local/bin/dez"
}

main "$@"

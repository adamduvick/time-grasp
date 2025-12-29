#!/bin/zsh
set -eo pipefail

# Safe defaults:
# PLAT="${PLATFORM_DISPLAY_NAME:-${PLATFORM_NAME:-iOS Simulator}}"
# SDK="${SDKROOT:?SDKROOT missing}"
# FR="${FRAMEWORK_SEARCH_PATHS:-}"
# HDR="${HEADER_SEARCH_PATHS:-}"
# DEF="${GCC_PREPROCESSOR_DEFINITIONS:-}"
# CFG="${CONFIGURATION:-debug}"

# # Derive a sane arch if Xcode didn’t set one.
# # On Apple-silicon hosts this will be arm64; on Intel it’s x86_64.
# ARCH_SAFE="${ARCHS:-$(uname -m)}"

# # Never pass the broken 'arm64-sim'
# [[ "$ARCH_SAFE" == "arm64-sim" ]] && ARCH_SAFE="arm64"

# # Call Tauri’s helper exactly as intended
# exec cargo tauri ios xcode-script -v \
#   --platform "$PLAT" \
#   --sdk-root "$SDK" \
#   --framework-search-paths "$FR" \
#   --header-search-paths "$HDR" \
#   --gcc-preprocessor-definitions "$DEF" \
#   --configuration "$CFG" \
#   ${FORCE_COLOR:-} "$ARCH_SAFE"

# export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"

cargo tauri ios xcode-script -v --platform ${PLATFORM_DISPLAY_NAME:?} --sdk-root ${SDKROOT:?} --framework-search-paths "${FRAMEWORK_SEARCH_PATHS:?}" --header-search-paths "${HEADER_SEARCH_PATHS:?}" --gcc-preprocessor-definitions "${GCC_PREPROCESSOR_DEFINITIONS:-}" --configuration ${CONFIGURATION:?} ${FORCE_COLOR} ${ARCHS:?}

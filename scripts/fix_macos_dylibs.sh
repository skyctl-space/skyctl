#!/bin/bash
set -euo pipefail

# Redirect stdout and stderr to a log file
# exec > >(tee -a /tmp/fix_macos_dylibs.log) 2>&1

# Verify that required tools are available
if ! command -v otool &> /dev/null; then
  echo "Error: otool is not installed or not in PATH. Please install it to proceed."
  exit 1
fi

if ! command -v install_name_tool &> /dev/null; then
  echo "Error: install_name_tool is not installed or not in PATH. Please install it to proceed."
  exit 1
fi


fixup() {
  BIN="$1"
  DYLIB_REGEX="libcfitsio(\.[0-9]+)?\.dylib"

  # Ensure the binary and Frameworks directory exist
  if [[ ! -f "$BIN" ]]; then
    return 0
  fi

  # echo "Checking for $DYLIB_REGEX in $BIN..."

  # Use otool to find the original linked path of libcfitsio.dylib
  ORIGINAL_PATH=$(otool -L "$BIN" | grep -E "$DYLIB_REGEX" | awk '{print $1}')
  if [[ "$ORIGINAL_PATH" == @executable_path* ]]; then
    echo "Skipping patching for $ORIGINAL_PATH as it already starts with @executable_path"
    return 0
  fi

  echo "Original path: $ORIGINAL_PATH"

  if [[ -z "$ORIGINAL_PATH" ]]; then
    echo "Error: Could not find linked path for $DYLIB_REGEX"
    exit 1
  fi

  DYLIB_NAME=$(basename "$ORIGINAL_PATH")
  BUNDLED_PATH="@executable_path/../Frameworks/$DYLIB_NAME"

  echo "Patching $DYLIB_NAME:"
  echo "  From: $ORIGINAL_PATH"
  echo "  To:   $BUNDLED_PATH"

  # Patch the binary
  install_name_tool -change "$ORIGINAL_PATH" "$BUNDLED_PATH" "$BIN"

  # Vendor the library
  FRAMEWORKDIR=$(dirname "$BIN")/Frameworks
  mkdir -p $FRAMEWORKDIR
  cp -v "$ORIGINAL_PATH" "$FRAMEWORKDIR/$DYLIB_NAME"
}

for BIN in src-tauri/target/release/skyctl src-tauri/target/debug/skyctl; do
  fixup "$BIN"
done
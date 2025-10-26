#!/usr/bin/env bash
set -euo pipefail

REGISTRY_DIR=".registry"
INDEX_DIR="$REGISTRY_DIR/index"
PACKAGES_DIR="$REGISTRY_DIR/packages"

echo "📦 Preparing local registry..."
mkdir -p "$INDEX_DIR" "$PACKAGES_DIR"

# Initialize index git repo if missing
if [ ! -d "$INDEX_DIR/.git" ]; then
  git -C "$INDEX_DIR" init -q
fi

# Package and publish each crate
for crate in $(cargo metadata --no-deps --format-version=1 | jq -r '.packages[].manifest_path' | grep '^crates/'); do
  crate_dir=$(dirname "$crate")
  echo "🚀 Publishing $crate_dir"
  (cd "$crate_dir" && cargo package --allow-dirty --no-verify --target-dir "$PACKAGES_DIR")
  (cd "$crate_dir" && cargo publish --registry local --no-verify)
done

echo "✅ Local registry updated"

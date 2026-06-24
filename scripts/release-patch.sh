#!/usr/bin/env bash
set -euo pipefail

cargo set-version --bump patch

VERSION=$(cargo pkgid -p lore-cli | cut -d# -f2)

git add .
git commit -m "chore: release v${VERSION}"
git tag "v${VERSION}"

echo "Created tag v${VERSION}"

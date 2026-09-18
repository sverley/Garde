#!/usr/bin/env bash
# Les crochets vivent dans le dépôt (annexe C) ; cette commande les branche.
set -euo pipefail
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
echo "Crochets branchés sur .githooks"

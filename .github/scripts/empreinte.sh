#!/usr/bin/env bash
# Empreinte de l'état validé (§4.4).
#
# Rend une empreinte courte de l'arbre restreint à ce qui compte. Deux états qui
# ne diffèrent que par de la documentation ou des harnais rendent la même
# empreinte : une validation n'est donc pas annulée par ce qui ne la concerne pas.
#
# Déterministe : mêmes fichiers, même empreinte, quelle que soit la machine.
# N'exécute rien de l'arbre qu'il lit.
#
#   empreinte.sh <répertoire de l'arbre> <fichier d'exclusions>

set -euo pipefail

arbre="${1:?usage: empreinte.sh <arbre> <exclusions>}"
exclusions="${2:?usage: empreinte.sh <arbre> <exclusions>}"

motifs=()
while IFS= read -r ligne || [ -n "$ligne" ]; do
  ligne="${ligne%%#*}"
  ligne="${ligne#"${ligne%%[![:space:]]*}"}"
  ligne="${ligne%"${ligne##*[![:space:]]}"}"
  [ -n "$ligne" ] && motifs+=("$ligne")
done < "$exclusions"

retenu() {
  local chemin="$1" motif
  for motif in "${motifs[@]}"; do
    # shellcheck disable=SC2254
    case "$chemin" in
      $motif) return 1 ;;
    esac
  done
  return 0
}

lignes=$(
  cd "$arbre"
  git ls-tree -r --format='%(objectname) %(path)' HEAD | LC_ALL=C sort -k2
)

sortie=""
while IFS= read -r ligne; do
  [ -n "$ligne" ] || continue
  objet="${ligne%% *}"
  chemin="${ligne#* }"
  if retenu "$chemin"; then
    sortie+="$objet $chemin"$'\n'
  fi
done <<< "$lignes"

printf '%s' "$sortie" | sha256sum | cut -c1-16

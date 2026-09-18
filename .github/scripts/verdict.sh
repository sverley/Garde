#!/usr/bin/env bash
# Verdict de forme d'une intégration (§4.4).
#
#   verdict.sh <chemins> <modifications> <corps>
#
#   <chemins>        la liste d'exclusions du plancher, prise dans l'arbre du
#                    juge — jamais dans l'arbre jugé (§1, non-intrusion)
#   <modifications>  un chemin par ligne, produit par comparaison des deux
#                    arbres ; ce script ne sait pas comment
#   <corps>          la description de l'intégration
#
# Rend les refus, un par ligne, chacun nommant le chemin ou la section en cause.
# Une ligne ouverte par « validation : » dit qu'une validation est due ; c'est à
# l'appelant de la lever s'il constate cette validation, qui ne vit pas ici.
#
#   0  aucun refus
#   1  au moins un refus
#   2  l'appel est mal formé — le script lui-même n'a rien pu juger
#
# Déterministe, hors ligne, sans forge et sans git : il ne lit que les trois
# fichiers qu'on lui donne, et n'exécute rien.
#
# Échafaudage : préfiguration de `garde juger`. La garde tirera le plancher des
# chemins portés par les entrées, et demandera les vérifications à partir des
# engagements déclarés (tranches B et C).

set -uo pipefail

if [ "$#" -ne 3 ]; then
  echo "verdict.sh : trois arguments attendus — <chemins> <modifications> <corps>" >&2
  exit 2
fi

for fichier in "$@"; do
  if [ ! -f "$fichier" ] || [ ! -r "$fichier" ]; then
    echo "verdict.sh : fichier illisible — $fichier" >&2
    exit 2
  fi
done

chemins="$1"
modifications="$2"
corps="$3"

# --- La liste d'exclusions ---------------------------------------------------
# Lue ligne à ligne. Ce qu'elle ne reconnaît pas est ignoré plutôt que refusé :
# une ligne mal formée fait alors perdre une exclusion, jamais en gagner une, et
# l'erreur fait jouer plus (annexe C). Sont reconnus : une ligne nue, un point de
# liste, la première cellule d'un tableau, avec ou sans accents graves.

motifs=()
fence=0

elaguer() {  # elaguer <texte> -> le texte sans blanc de tête ni de queue
  local t="$1"
  t="${t#"${t%%[![:space:]]*}"}"
  t="${t%"${t##*[![:space:]]}"}"
  printf '%s' "$t"
}

while IFS= read -r ligne || [ -n "$ligne" ]; do
  ligne="${ligne%$'\r'}"
  ligne=$(elaguer "$ligne")
  [ -n "$ligne" ] || continue

  case "$ligne" in
    '```'*) fence=$((1 - fence)); continue ;;
    '#'*) continue ;;
  esac

  segment="$ligne"
  case "$ligne" in
    '|'*)
      segment="${ligne#|}"
      segment="${segment%%|*}"
      ;;
    '- '* | '+ '* | \*' '*)
      segment="${ligne#??}"
      ;;
  esac

  segment=$(elaguer "$segment")
  if [[ "$segment" == *'`'* ]]; then
    segment="${segment#*\`}"
    segment="${segment%%\`*}"
  fi
  candidat="${segment%%[[:space:]]*}"

  # Un motif ne porte pas de blanc, et ressemble à un chemin : sans quoi une
  # ligne de prose deviendrait une exclusion.
  case "$candidat" in
    '' | *'<'*) continue ;;
    */* | *.* | *'*'*) motifs+=("$candidat") ;;
  esac
done < "$chemins"

exclu() {  # exclu <chemin> -> 0 s'il est exclu du plancher
  local chemin="$1" motif
  [ "${#motifs[@]}" -eq 0 ] && return 1
  for motif in "${motifs[@]}"; do
    # shellcheck disable=SC2254
    case "$chemin" in
      $motif) return 0 ;;
    esac
  done
  return 1
}

# --- La description ----------------------------------------------------------
# Les commentaires HTML sont retirés d'abord : le gabarit en porte, et un
# commentaire laissé en place ne vaut ni analyse ni justification.

corps_nu=$(awk '
{
  ligne = $0
  while (1) {
    if (dedans) {
      p = index(ligne, "-->")
      if (p == 0) { ligne = ""; break }
      ligne = substr(ligne, p + 3); dedans = 0
    } else {
      p = index(ligne, "<!--")
      if (p == 0) break
      avant = substr(ligne, 1, p - 1)
      reste = substr(ligne, p + 4)
      q = index(reste, "-->")
      if (q == 0) { ligne = avant; dedans = 1; break }
      ligne = avant substr(reste, q + 3)
    }
  }
  print ligne
}' "$corps")

refus=()

printf '%s\n' "$corps_nu" | grep -qE '^##[[:space:]]+Déclaration' \
  || refus+=("forme : la section « ## Déclaration » manque.")

section_vm=0
if printf '%s\n' "$corps_nu" | grep -qE '^##[[:space:]]+Vérifications manuelles'; then
  section_vm=1
else
  refus+=("forme : la section « ## Vérifications manuelles » manque.")
fi

demandees=$(printf '%s\n' "$corps_nu" | grep -cE '^###[[:space:]]*VM-' || true)

# Une vérification déclarée sans analyse ne peut pas être validée.
sans_analyse=$(printf '%s\n' "$corps_nu" | awk '
  /^###[[:space:]]*VM-/ {
    if (nom != "" && vu == 0) print nom
    nom = $0; sub(/^###[[:space:]]*/, "", nom); sub(/[[:space:]]+$/, "", nom)
    vu = 0; next
  }
  /^#/ { if (nom != "" && vu == 0) print nom; nom = ""; next }
  NF   { if (nom != "") vu = 1 }
  END  { if (nom != "" && vu == 0) print nom }
')
if [ -n "$sans_analyse" ]; then
  while IFS= read -r nom; do
    [ -n "$nom" ] || continue
    refus+=("forme : la vérification « $nom » ne porte pas d'analyse.")
  done <<< "$sans_analyse"
fi

# Aucune vérification demandée n'est plus un refus : §4.4, consignes
# proportionnées. Ce qui est exigé, c'est de dire pourquoi le produit n'est pas
# atteint. La présence de cette justification est constatée, jamais jugée (§1).
if [ "$section_vm" -eq 1 ] && [ "$demandees" -eq 0 ]; then
  if ! printf '%s\n' "$corps_nu" | awk '
    /^##[[:space:]]+Vérifications manuelles/ { dans = 1; next }
    /^##[[:space:]]/ { dans = 0 }
    dans && NF && $0 !~ /^#/ { trouve = 1 }
    END { exit (trouve ? 0 : 1) }
  '; then
    refus+=("forme : aucune vérification manuelle demandée, et la section « ## Vérifications manuelles » ne dit pas pourquoi le produit n'est pas atteint.")
  fi
fi

# --- Le plancher -------------------------------------------------------------

while IFS= read -r chemin || [ -n "$chemin" ]; do
  chemin="${chemin%$'\r'}"
  chemin=$(elaguer "$chemin")
  [ -n "$chemin" ] || continue
  exclu "$chemin" && continue
  refus+=("validation : « $chemin » n'est pas exclu du plancher — une validation est due.")
done < <(LC_ALL=C sort -u "$modifications")

if [ "${#refus[@]}" -eq 0 ]; then
  exit 0
fi

printf '%s\n' "${refus[@]}"
exit 1

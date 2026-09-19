#!/usr/bin/env bash
# Les documents à lire à chaque session — la liste est-elle complète et vivante ?
#
# SOURCE : le porteur — *les documents fondateurs doivent être lus dans chaque
# session*. La liste vit dans `CLAUDE.md`, premier bloc encadré.
#
# Ce qui se mesure ici, et ce qui ne se mesure pas. Qu'une session ait lu ne se
# code pas : rien dans l'arbre n'en garde trace. Ce qui se code, c'est que la
# liste soit **applicable** — que chaque document nommé existe, et qu'aucun
# catalogue n'en soit absent. Un catalogue né après la liste serait invisible de
# toute session suivante, et l'instruction serait inapplicable sans que rien ne
# le dise. C'est ce silence-là qu'on retire.
#
# Deux mesures, chacune sur un arbre qu'on lui donne :
#
#   nommes_existent      un document listé qui n'existe pas
#   catalogues_tous_listes  un catalogue du dépôt absent de la liste
#
# Témoins : deux arbres fabriqués, chacun violant une mesure et une seule ; le
# dépôt lui-même sert de témoin vert.
#
# Codes : 0 tout est vert ; 1 une mesure rougit sur le dépôt ; 2 un témoin ment
# — le harnais est alors en cause, ce qui est plus grave qu'un rouge.

set -uo pipefail

racine=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$racine" || exit 2

# --- Les deux mesures --------------------------------------------------------
# Chacune rend 0 si l'arbre donné est conforme, 1 sinon, et écrit ce qu'elle
# reproche. Aucune ne sort du texte qu'on lui donne.

liste() {
  awk '/^```/ { dedans = !dedans; next } dedans && NF { print }' "$1/CLAUDE.md"
}

nommes_existent() {
  local arbre="$1" faute=0 doc
  while IFS= read -r doc; do
    case "$doc" in
      */\*\*)
        doc="${doc%/**}"
        if [ ! -d "$arbre/$doc" ] || [ -z "$(ls -A "$arbre/$doc" 2>/dev/null)" ]; then
          echo "  $doc/** est à lire mais ne contient rien"
          faute=1
        fi
        ;;
      *) [ -f "$arbre/$doc" ] || { echo "  $doc est à lire mais n'existe pas"; faute=1; } ;;
    esac
  done < <(liste "$arbre")
  return "$faute"
}

catalogues_tous_listes() {
  local arbre="$1" faute=0 cat nom
  for cat in "$arbre"/docs/catalogues/*.md; do
    [ -e "$cat" ] || continue
    nom="${cat#"$arbre"/}"
    # Nommé un par un, ou couvert par le répertoire : les deux valent, mais
    # aucun catalogue ne doit échapper aux deux.
    liste "$arbre" | grep -qxF "$nom" && continue
    liste "$arbre" | grep -qxF "$(dirname "$nom")/**" && continue
    echo "  $nom est un catalogue et n'est couvert par aucune ligne de la liste"
    faute=1
  done
  return "$faute"
}

mesures=(nommes_existent catalogues_tous_listes)

# --- Les témoins -------------------------------------------------------------
# Chaque arbre fabriqué nomme la mesure qu'il doit faire rougir, et ne doit
# faire rougir qu'elle. Un témoin qui ment arrête tout.

fabriquer() {
  local ou="$1" defaut="$2" omis=""
  # Rien n'est copié du dépôt : un inventé qui dépendrait du réel cesserait
  # d'être constructible le jour où le réel est en faute, et le harnais
  # s'accuserait lui-même d'une faute qui n'est pas la sienne.
  mkdir -p "$ou/docs/catalogues"
  : > "$ou/docs/document-fondateur.md"
  : > "$ou/docs/chemins.md"
  : > "$ou/docs/catalogues/premier.md"
  : > "$ou/docs/catalogues/second.md"
  {
    printf '# Instructions\n\n```\n'
    printf 'docs/document-fondateur.md\ndocs/chemins.md\n'
    for c in "$ou"/docs/catalogues/*.md; do
      nom="docs/catalogues/$(basename "$c")"
      # On omet le premier catalogue venu, quel qu'il soit : coder son nom en
      # dur ferait taire le témoin le jour où ce catalogue change de nom.
      [ "$defaut" = "catalogue-absent" ] && [ "$omis" = "" ] && { omis="$nom"; continue; }
      :
      printf '%s\n' "$nom"
    done
    [ "$defaut" = "document-absent" ] && printf 'docs/document-qui-n-existe-pas.md\n'
    printf '```\n'
  } > "$ou/CLAUDE.md"
}

atelier=$(mktemp -d)
trap 'rm -rf "$atelier"' EXIT

declare -A attendu=(
  [document-absent]=nommes_existent
  [catalogue-absent]=catalogues_tous_listes
)

echo "· témoins"
menteurs=0
for cas in $(printf '%s\n' "${!attendu[@]}" | LC_ALL=C sort); do
  fabriquer "$atelier/$cas" "$cas"
  for mesure in "${mesures[@]}"; do
    if "$mesure" "$atelier/$cas" > /dev/null; then rouge=0; else rouge=1; fi
    if [ "$mesure" = "${attendu[$cas]}" ]; then
      if [ "$rouge" -eq 0 ]; then
        echo "  TÉMOIN ROUGE MUET : $cas ne fait pas rougir $mesure"
        menteurs=$((menteurs + 1))
      fi
    elif [ "$rouge" -eq 1 ]; then
      echo "  TÉMOIN VERT SALI : $cas fait rougir $mesure, qu'il ne vise pas"
      "$mesure" "$atelier/$cas"
      menteurs=$((menteurs + 1))
    fi
  done
done
# Toute mesure doit être visée par au moins un témoin : sans quoi une mesure
# entrerait au harnais sans qu'on ait prouvé qu'elle sait échouer.
for mesure in "${mesures[@]}"; do
  vise=0
  for cas in "${!attendu[@]}"; do [ "${attendu[$cas]}" = "$mesure" ] && vise=1; done
  [ "$vise" -eq 1 ] || { echo "  MESURE SANS TÉMOIN : $mesure"; menteurs=$((menteurs + 1)); }
done
if [ "$menteurs" -ne 0 ]; then
  echo "  $menteurs témoin(s) en défaut — le harnais est en cause, pas le dépôt." >&2
  exit 2
fi
echo "  ${#attendu[@]} témoins tiennent"

echo "· la liste du dépôt"
rouges=0
for mesure in "${mesures[@]}"; do
  "$mesure" "$racine" || rouges=$((rouges + 1))
done
if [ "$rouges" -ne 0 ]; then
  echo "  la liste à lire de CLAUDE.md n'est pas applicable telle quelle." >&2
  exit 1
fi
echo "  tout est vert"

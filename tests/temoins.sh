#!/usr/bin/env bash
# Les témoins des harnais Rust — exécutés, non racontés.
#
# §4.2 : *chaque harnais prouve qu'il sait rougir, par un témoin rouge — une
# mutation désignée —, et qu'il sait verdir : il est vert sur la référence, ou
# une dette ouverte dit pourquoi il ne l'est pas.*
#
# Une prose qui affirme qu'une mutation ferait rougir ne se relit pas et dérive
# en silence. Ce harnais applique la mutation et regarde.
#
# Chaque harnais déclare la sienne en tête de son propre fichier :
#
#   //! TÉMOIN fichier src/main.rs
#   //! TÉMOIN ancien         eprintln!("{erreur}");
#   //! TÉMOIN nouveau         println!("{erreur}");
#
# `ancien` et `nouveau` se répètent pour un bloc de plusieurs lignes. Le texte
# qui suit le mot-clé et une espace est pris verbatim, indentation comprise.
# Sans `ancien`, le fichier est créé avec les lignes `nouveau` : c'est la forme
# des mutations qui ajoutent au lieu de remplacer.
#
# Trois états par harnais, et non deux :
#
#   tenu   vert sans la mutation, rouge avec — le témoin est établi
#   dû     déjà rouge sans la mutation — rien à prouver tant que le codage
#          n'est pas fait ; ce n'est pas un défaut, c'est une dette visible
#   muet   vert sans la mutation, vert avec — LE HARNAIS NE GARDE RIEN
#
# Codes : 0 aucun témoin muet ; 2 un témoin ment ou manque. Il n'y a pas de
# code 1 : ce harnais ne juge pas le dépôt, il juge les harnais.
#
# Il recompile à chaque mutation. Sa place est le passage complet, pas le
# crochet, dont le budget est de dix secondes.
#
# Son propre témoin : le cas `sentinelle` ci-dessous applique une mutation
# volontairement inoffensive à un harnais vert. Elle DOIT être rapportée muette.
# Si elle ne l'est pas, le mécanisme ne sait pas détecter un témoin muet, et
# c'est lui qui est en défaut.

set -uo pipefail

racine=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$racine" || exit 2

if ! command -v cargo > /dev/null 2>&1; then
  echo "cargo absent — le passage complet l'exige (§4.2)" >&2
  exit 2
fi

copie=$(mktemp -d)
travail=$(mktemp -d)
trap 'rm -rf "$copie" "$travail"' EXIT

# L'arbre est copié : aucune mutation ne touche le dépôt. Un harnais qui laisse
# une mutation derrière lui en cas d'interruption serait pire que pas de harnais.
git ls-files -z | while IFS= read -r -d '' f; do
  mkdir -p "$copie/$(dirname "$f")"
  cp "$f" "$copie/$f"
done
export CARGO_TARGET_DIR="$travail/cible"

# --- Lire la déclaration d'un harnais ---------------------------------------
# Rend, sur la sortie standard : la première ligne est le fichier visé, puis les
# lignes `ancien`, puis un séparateur, puis les lignes `nouveau`.

declaration() {
  awk '
    /^\/\/! TÉMOIN fichier / { sub(/^\/\/! TÉMOIN fichier /, ""); fichier = $0; next }
    /^\/\/! TÉMOIN ancien/   { ligne = $0; sub(/^\/\/! TÉMOIN ancien ?/, "", ligne); a[++na] = ligne; next }
    /^\/\/! TÉMOIN nouveau/  { ligne = $0; sub(/^\/\/! TÉMOIN nouveau ?/, "", ligne); n[++nn] = ligne; next }
    END {
      if (fichier == "") exit 1
      print fichier
      for (i = 1; i <= na; i++) print "A" a[i]
      for (i = 1; i <= nn; i++) print "N" n[i]
    }
  ' "$1"
}

# --- Appliquer une mutation dans la copie -----------------------------------
# Remplacement d'un bloc littéral, ou création du fichier s'il n'y a pas
# d'ancien. Rend 1 si l'ancien est introuvable ou n'est pas unique : un témoin
# dont l'ancre a bougé ne ment pas, il est périmé, et ça doit se voir.

appliquer() {
  local cible="$1" ancien="$2" nouveau="$3"
  if [ ! -s "$ancien" ]; then
    mkdir -p "$(dirname "$copie/$cible")"
    cp "$nouveau" "$copie/$cible"
    return 0
  fi
  [ -f "$copie/$cible" ] || return 1
  awk -v fa="$ancien" -v fn="$nouveau" '
    BEGIN {
      while ((getline l < fa) > 0) a[++na] = l
      while ((getline l < fn) > 0) n[++nn] = l
    }
    { t[++nt] = $0 }
    END {
      trouve = 0
      for (i = 1; i <= nt - na + 1; i++) {
        ok = 1
        for (j = 1; j <= na; j++) if (t[i + j - 1] != a[j]) { ok = 0; break }
        if (ok) { depart[++trouve] = i }
      }
      if (trouve != 1) exit 3
      d = depart[1]
      for (i = 1; i < d; i++) print t[i]
      for (j = 1; j <= nn; j++) print n[j]
      for (i = d + na; i <= nt; i++) print t[i]
    }
  ' "$copie/$cible" > "$travail/mute" || return 1
  cp "$travail/mute" "$copie/$cible"
}

restaurer() {
  local cible="$1"
  if git ls-files --error-unmatch "$cible" > /dev/null 2>&1; then
    mkdir -p "$(dirname "$copie/$cible")"
    cp "$cible" "$copie/$cible"
  else
    rm -f "$copie/$cible"
  fi
}

jouer() { ( cd "$copie" && cargo test --quiet --test "$1" > /dev/null 2>&1 ); }

# --- Les harnais ------------------------------------------------------------

harnais=()
for f in tests/*.rs; do harnais+=("$(basename "$f" .rs)"); done

echo "· état de départ"
declare -A vert
for h in "${harnais[@]}"; do
  if jouer "$h"; then vert[$h]=1; else vert[$h]=0; fi
done
echo "  $(for h in "${harnais[@]}"; do [ "${vert[$h]}" = 1 ] && echo x; done | wc -l) verts sur ${#harnais[@]}"

# Rend un verdict sur la sortie standard : tenu, muet, ou périmé. La sentinelle
# passe par ici comme les autres — sans quoi la branche qui détecte un témoin
# muet ne serait exercée par rien, et ce harnais affirmerait sans mesurer.
eprouver() {
  local h="$1" cible="$2" ancien="$3" nouveau="$4"
  if ! appliquer "$cible" "$ancien" "$nouveau"; then
    restaurer "$cible"
    echo "périmé"
    return
  fi
  if jouer "$h"; then echo "muet"; else echo "tenu"; fi
  restaurer "$cible"
}

echo "· témoins"
menteurs=0
tenus=0
dus=0
for h in "${harnais[@]}"; do
  if ! declaration "tests/$h.rs" > "$travail/decl" 2>/dev/null; then
    echo "  TÉMOIN MANQUANT : tests/$h.rs ne déclare aucune mutation"
    menteurs=$((menteurs + 1))
    continue
  fi
  cible=$(head -1 "$travail/decl")
  sed -n '2,$p' "$travail/decl" | sed -n 's/^A//p' > "$travail/ancien"
  sed -n '2,$p' "$travail/decl" | sed -n 's/^N//p' > "$travail/nouveau"

  if [ "${vert[$h]}" = 0 ]; then
    echo "  dû     $h — déjà rouge sans la mutation ; le témoin vert est dû au codage"
    dus=$((dus + 1))
    continue
  fi

  case "$(eprouver "$h" "$cible" "$travail/ancien" "$travail/nouveau")" in
    tenu)   tenus=$((tenus + 1)) ;;
    muet)   echo "  TÉMOIN ROUGE MUET : $h reste vert sous sa mutation — il ne garde rien"
            menteurs=$((menteurs + 1)) ;;
    périmé) echo "  TÉMOIN PÉRIMÉ : $h — l'ancre n'est plus dans $cible, ou n'y est pas unique"
            menteurs=$((menteurs + 1)) ;;
  esac
done

# --- Le témoin de ce harnais-ci ---------------------------------------------
# Une mutation inoffensive sur un harnais vert : elle doit être vue muette. Si
# elle ne l'est pas, ce script ne sait pas détecter un témoin muet.

echo "· sentinelle"
# Une mutation volontairement sans effet, éprouvée par le chemin ci-dessus. Le
# verdict DOIT être « muet ». S'il ne l'est pas, ce script ne sait pas voir un
# témoin qui ment, et tous les « tenus » qu'il vient d'annoncer ne valent rien.
sentinelle=""
for h in "${harnais[@]}"; do [ "${vert[$h]}" = 1 ] && { sentinelle="$h"; break; }; done
if [ -z "$sentinelle" ]; then
  echo "  aucun harnais vert — la sentinelle ne peut pas se poser, et rien ne garde ce script"
  menteurs=$((menteurs + 1))
else
  : > "$travail/ancien"
  printf '// mutation sans effet, posee par la sentinelle\n' > "$travail/nouveau"
  verdict=$(eprouver "$sentinelle" "src/sentinelle-temoin.rs" "$travail/ancien" "$travail/nouveau")
  if [ "$verdict" = "muet" ]; then
    echo "  muette comme il se doit — la détection d'un témoin qui ment est exercée"
  else
    echo "  SENTINELLE EN DÉFAUT : verdict « $verdict » au lieu de « muet »." >&2
    echo "  Ce script ne sait pas reconnaître un témoin muet ; ses verdicts ne valent rien." >&2
    menteurs=$((menteurs + 1))
  fi
fi

echo
if [ "$menteurs" -ne 0 ]; then
  echo "  $menteurs témoin(s) en défaut — le harnais est en cause, pas le produit." >&2
  exit 2
fi
echo "  $tenus témoin(s) tenus, $dus dû(s) au codage"

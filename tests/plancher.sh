#!/usr/bin/env bash
# Harnais du plancher de validation (#20).
#
# Il établit qu'une validation est exigée **là où l'intégration touche un
# engagement, et nulle part ailleurs** (§4.4). Deux façons de mesurer, parce
# qu'il y a deux choses à garder :
#
#   1. la RÈGLE — jouée, sur des projets inventés : on donne au verdict des
#      entrées connues, on compare ce qu'il rend à ce qu'on attend, exactement.
#   2. la PLOMBERIE — lue, sans rien exécuter : que le contrôle demande bien son
#      verdict, et qu'il lui donne les chemins du JUGE et non ceux du jugé.
#
# La règle est jouable parce qu'elle vit dans un script et non dans le YAML :
#
#   .github/scripts/verdict.sh <chemins> <modifications> <corps>
#
#   <chemins>        la liste d'exclusions du plancher, prise dans l'arbre du juge
#   <modifications>  un chemin par ligne, produit par comparaison des deux arbres
#   <corps>          la description de l'intégration
#
# Sa sortie standard porte zéro, une ou plusieurs exigences, une par ligne, dans
# un vocabulaire fermé :
#
#   plancher <chemin>        ce chemin modifié appelle une validation
#   justification-absente    aucune vérification demandée, et rien qui dise pourquoi
#   analyse-absente <nom>    une vérification déclarée sans son analyse
#
# Codes : 0 aucune exigence ; 1 au moins une ; 2 appel mal formé. Ce que la forge
# seule sait — la validation a-t-elle été postée pour cette empreinte — reste au
# workflow : le verdict dit ce qui est DÛ, pas ce qui a été fait.
#
# Un motif de `<chemins>` est un glob à la manière de `case` ; `*` franchit un
# `/`, donc `src/*` couvre tout le sous-arbre.
#
# Déterministe et hors ligne (§5) : aucun appel de forge, aucun réseau, aucun
# git. Mêmes entrées, même verdict, sur n'importe quelle machine.
#
# Échafaudage : la garde reprendra ces mesures quand elle existera.
#
#   tests/plancher.sh                    les mesures
#   tests/inventes/plancher/             les projets inventés — la règle
#   tests/inventes/plancher-workflow/    les workflows inventés — la plomberie
#   tests/TEMOINS.md                     les mutations désignées
#
# Codes : 0 tout est vert ; 1 une mesure rougit sur le dépôt ; 2 un témoin ment
# — le harnais lui-même est en défaut, ce qui est plus grave qu'un rouge.

set -uo pipefail
export LC_ALL=C

racine=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$racine"

verdict=".github/scripts/verdict.sh"
chemins_du_depot="docs/chemins.md"
controle=".github/workflows/garde.yml"

menteurs=0
rouges=0

normaliser() { grep -v '^[[:space:]]*$' | sort; }

# --- 1. La plomberie : deux mesures lexicales -------------------------------
# Chacune rend 0 si le fichier est conforme, 1 sinon, et écrit ce qu'elle
# reproche. Aucune ne sort du texte qu'on lui donne.

# Les chemins qui font le plancher viennent de l'arbre du juge. Les prendre dans
# l'arbre jugé rendrait la garde désarmable par le projet qu'elle juge : il lui
# suffirait de s'exclure lui-même (§1, §5).
#
# Le texte est d'abord préparé : les lignes de commentaire sont retirées — un
# commentaire ne s'exécute pas, et le reprocher est un faux rouge — et les
# continuations de ligne sont recollées, un appel coupé par un `\` restant un
# seul appel. Sans cela la mesure jugeait des fragments, et reprochait à un appel
# correct de l'être sur deux lignes.
chemins_pris_dans_la_base() {
  local texte
  texte=$(sed '/^[[:space:]]*#/d' "$1" | sed -e :a -e '/\\$/N; s/\\\n[[:space:]]*/ /; ta')
  printf '%s\n' "$texte" \
    | grep -E 'arbre/[^[:space:]"]*chemins' \
    | sed 's/^[[:space:]]*/  chemins pris dans l'"'"'arbre jugé : /'
  printf '%s\n' "$texte" | awk '
    /verdict\.sh/ {
      if ($0 !~ /verdict\.sh[[:space:]]+"?base\//)
        print "  verdict.sh appelé sans chemins pris dans base/ :" substr($0, 1, 60)
    }
  '
}

# Le contrôle ne juge plus dans le YAML : il demande son verdict au script, qui
# est ce que le harnais sait jouer. Un contrôle qui rejuge sur place redevient
# invérifiable.
le_controle_appelle_le_verdict() {
  grep -q 'verdict\.sh' "$1" || echo "  le contrôle n'appelle pas verdict.sh"
}

# La règle ne connaît ni forge ni système de versions (§4.4, §5). Elle reçoit des
# fichiers et rend un verdict : appeler git, la forge ou le réseau ferait dépendre
# ce verdict de ce qui n'est pas dans ses entrées, et le rendrait non rejouable.
aucune_forge() {
  grep -n -E '(^|[^-[:alnum:]_])(git|gh|curl|wget)[[:space:]]' "$1" \
    | grep -v -E '^[[:space:]]*[0-9]+:[[:space:]]*#' \
    | sed 's/^/  ligne /'
}

mesures=(chemins_pris_dans_la_base le_controle_appelle_le_verdict)

juger() { # juger <mesure> <fichier> -> 0 vert, 1 rouge ; écrit le détail
  local sortie
  sortie=$("$1" "$2")
  [ -z "$sortie" ] && return 0
  printf '%s\n' "$sortie"
  return 1
}

declare -A attendu_workflow=(
  [conforme.yml]=""
  [chemins-de-l-arbre.yml]="chemins_pris_dans_la_base"
  [sans-appel.yml]="le_controle_appelle_le_verdict"
)

echo "· témoins de la plomberie"
for cas in $(printf '%s\n' "${!attendu_workflow[@]}" | sort); do
  fichier="tests/inventes/plancher-workflow/$cas"
  if [ ! -f "$fichier" ]; then
    echo "  TÉMOIN MANQUANT : $fichier"
    menteurs=$((menteurs + 1))
    continue
  fi
  for mesure in "${mesures[@]}"; do
    if juger "$mesure" "$fichier" > /dev/null; then rouge=0; else rouge=1; fi
    if [ "$mesure" = "${attendu_workflow[$cas]}" ]; then
      if [ "$rouge" -eq 0 ]; then
        echo "  TÉMOIN ROUGE MUET : $cas ne fait pas rougir $mesure"
        menteurs=$((menteurs + 1))
      fi
    elif [ "$rouge" -eq 1 ]; then
      echo "  TÉMOIN VERT SALI : $cas fait rougir $mesure, qu'il ne vise pas"
      juger "$mesure" "$fichier"
      menteurs=$((menteurs + 1))
    fi
  done
done
[ "$menteurs" -eq 0 ] && echo "  ${#attendu_workflow[@]} témoins tiennent"

# --- 2. La règle : les projets inventés --------------------------------------
# Chaque cas porte ses trois entrées et le verdict attendu, exactement. Exact
# veut dire : ni exigence en moins — la mesure ne garderait rien —, ni exigence
# en trop — le cas salirait les autres.

cas_dir="tests/inventes/plancher"
requis=(chemins.md modifications corps.md attendu)

echo "· forme des cas inventés"
for cas in "$cas_dir"/*/; do
  [ -d "$cas" ] || continue
  for fichier in "${requis[@]}"; do
    if [ ! -f "$cas$fichier" ]; then
      echo "  CAS INCOMPLET : $cas$fichier manque"
      menteurs=$((menteurs + 1))
    fi
  done
done
[ "$menteurs" -eq 0 ] && echo "  les cas sont formés"

if [ "$menteurs" -ne 0 ]; then
  echo "  $menteurs témoin(s) en défaut — le harnais est en cause, pas le dépôt." >&2
  exit 2
fi

echo "· la règle"
if [ ! -f "$verdict" ]; then
  echo "  ABSENT : $verdict"
  echo "  La règle n'existe pas encore : aucune exigence ne peut être mesurée."
  echo "  C'est l'état attendu à la fin de l'audit ; le témoin vert est dû au codage."
  rouges=$((rouges + 1))
else
  for cas in "$cas_dir"/*/; do
    [ -d "$cas" ] || continue
    nom=$(basename "$cas")
    obtenu=$(bash "$verdict" "$cas/chemins.md" "$cas/modifications" "$cas/corps.md" 2>&1)
    code=$?

    att=$(normaliser < "$cas/attendu")
    obt=$(printf '%s\n' "$obtenu" | normaliser)

    if [ "$att" != "$obt" ]; then
      echo "  ROUGE $nom — exigences rendues ≠ attendues"
      diff <(printf '%s\n' "$att") <(printf '%s\n' "$obt") \
        | sed 's/^</    attendu : /; s/^>/    rendu   : /' | grep -E 'attendu|rendu'
      rouges=$((rouges + 1))
      continue
    fi

    if [ -z "$att" ]; then attendu_code=0; else attendu_code=1; fi
    if [ "$code" -ne "$attendu_code" ]; then
      echo "  ROUGE $nom — code $code, attendu $attendu_code"
      rouges=$((rouges + 1))
    fi
  done

  # Un appel mal formé se distingue d'un refus : sans cela, le workflow ne peut
  # pas savoir s'il a été mal écrit ou si l'intégration est rouge.
  bash "$verdict" > /dev/null 2>&1
  code=$?
  if [ "$code" -ne 2 ]; then
    echo "  ROUGE appel-mal-forme — code $code, attendu 2"
    rouges=$((rouges + 1))
  fi

  if ! juger aucune_forge "$verdict"; then
    echo "  ROUGE aucune_forge — $verdict"
    rouges=$((rouges + 1))
  fi
fi

# --- 3. Le dépôt --------------------------------------------------------------

echo "· plomberie du dépôt"
if [ ! -f "$controle" ]; then
  echo "  ABSENT : $controle"
  rouges=$((rouges + 1))
else
  for mesure in "${mesures[@]}"; do
    if ! juger "$mesure" "$controle"; then
      echo "  ROUGE $mesure — $controle"
      rouges=$((rouges + 1))
    fi
  done
fi

echo "· liste de chemins du dépôt"
if [ ! -f "$chemins_du_depot" ]; then
  echo "  ABSENT : $chemins_du_depot"
  echo "  C'est la source destinée à être embarquée dans l'exécutable ; rien n'est"
  echo "  demandé au projet jugé (§1)."
  rouges=$((rouges + 1))
elif [ -f "$verdict" ]; then
  corps="$cas_dir/sans-vm-justifie/corps.md"
  liste=$(mktemp)

  exige() { # exige <chemin> <due|non-due>
    printf '%s\n' "$1" > "$liste"
    local sortie
    sortie=$(bash "$verdict" "$chemins_du_depot" "$liste" "$corps" 2>&1)
    case "$sortie" in
      *"plancher $1"*) obtenu=due ;;
      *)               obtenu=non-due ;;
    esac
    if [ "$obtenu" != "$2" ]; then
      echo "  ROUGE $1 — $obtenu, attendu $2"
      rouges=$((rouges + 1))
    fi
  }

  for chemin in src/lib.rs Cargo.toml Cargo.lock .gitignore README.md \
                docs/application.md docs/plan-de-construction.md; do
    exige "$chemin" non-due
  done
  for chemin in docs/document-fondateur.md docs/chemins.md tests/plancher.sh \
                .github/workflows/garde.yml .github/pull_request_template.md \
                .githooks/pre-commit .garde/empreinte-exclusions \
                truc/imprevu.txt; do
    exige "$chemin" due
  done
  rm -f "$liste"
else
  echo "  passé : la règle n'existe pas encore"
fi

if [ "$rouges" -ne 0 ]; then
  echo "  $rouges rouge(s)." >&2
  exit 1
fi
echo "  tout est vert"

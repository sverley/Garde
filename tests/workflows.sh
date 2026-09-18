#!/usr/bin/env bash
# Harnais des workflows (#13).
#
# Quatre mesures, toutes lexicales : ce harnais LIT du YAML, il ne l'exécute
# pas, ne l'interprète pas et n'appelle aucun outil de la forge. Déterministe
# et hors ligne (§5) : mêmes fichiers, même verdict, sur n'importe quelle
# machine.
#
# Il s'exerce d'abord sur des workflows INVENTÉS — `tests/inventes/` : un
# conforme, et un par violation — puis sur les workflows réels du dépôt. Les
# inventés sont les témoins : le conforme prouve que chaque mesure sait verdir,
# chaque violation prouve que la mesure qui la vise sait rougir, et que les
# autres ne rougissent pas avec elle.
#
# Échafaudage : la garde reprendra ces mesures quand elle existera.
#
#   tests/workflows.sh              inventés, puis réels
#   tests/inventes/                 les témoins
#
# Codes : 0 tout est vert ; 1 une mesure rougit sur le dépôt ; 2 un témoin ment
# — le harnais lui-même est en défaut, ce qui est plus grave qu'un rouge.

set -uo pipefail

racine=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$racine"

reels=(.github/workflows/*.yml)
scripts=(.github/scripts/*.sh .githooks/pre-commit)

# --- Les quatre mesures ------------------------------------------------------
# Chacune rend 0 si le fichier est conforme, 1 sinon, et écrit sur la sortie
# standard ce qu'elle reproche. Aucune ne sort du texte qu'on lui donne.

# Toute étape `actions/checkout` déclare qu'elle ne persiste pas ses
# identifiants. Sans cette ligne, l'action écrit l'en-tête d'authentification
# dans la configuration locale du dépôt récupéré — un jeton sur disque, que la
# contrainte `jeton hors du disque` exclut.
identifiants_non_persistes() {
  awk '
    # Une nouvelle étape commence : on juge la précédente.
    /^[[:space:]]*-[[:space:]]/ {
      if (checkout && !declare) print "  étape ligne " depart " : checkout sans persist-credentials: false"
      checkout = 0; declare = 0; depart = NR
    }
    /uses:[[:space:]]*actions\/checkout/ { checkout = 1; if (!depart) depart = NR }
    /persist-credentials:[[:space:]]*false/ { declare = 1 }
    END {
      if (checkout && !declare) print "  étape ligne " depart " : checkout sans persist-credentials: false"
    }
  ' "$1"
}

# Aucune URL ne porte d'identifiants. Le motif vise `//quelque-chose:secret@`,
# la forme qui survit dans un `remote` ou dans la configuration d'un clone.
jeton_hors_des_url() {
  grep -n -E '://[^/[:space:]"]+:[^/[:space:]"]+@' "$1" \
    | sed 's/^/  ligne /' \
    | sed 's/:[^:]*@/:***@/'
}

# L'arbre de l'intégration n'est pas récupéré avant qu'on sache qu'il y a lieu
# de juger. Une étape qui touche `refs/pull/` porte donc une condition.
arbre_conditionne_au_brouillon() {
  awk '
    /^[[:space:]]*-[[:space:]]/ {
      if (tete && !condition) print "  étape ligne " depart " : récupère refs/pull sans condition"
      tete = 0; condition = 0; depart = NR
    }
    /refs\/pull\// { tete = 1; if (!depart) depart = NR }
    /^[[:space:]]*if:/ { condition = 1 }
    END {
      if (tete && !condition) print "  étape ligne " depart " : récupère refs/pull sans condition"
    }
  ' "$1"
}

# Le contrôle d'intégration n'exécute, ne construit et n'installe rien venu de
# l'intégration (§5). Deux interdits : un gestionnaire de paquets ou un
# constructeur dans le fichier du contrôle, et l'exécution d'un chemin de
# l'arbre. Passer `arbre` en ARGUMENT d'un script de la base reste permis —
# c'est ce que fait l'empreinte.
rien_de_l_integration_n_est_execute() {
  case "$1" in
    *construction.yml) return 0 ;;  # celui-ci joue le code, et c'est son rôle
  esac
  grep -n -E '(^|[^-[:alnum:]])(cargo|npm|yarn|pnpm|pip|pip3|make|docker)[[:space:]]' "$1" \
    | sed 's/^/  ligne /'
  grep -n -E '(^|[[:space:]])(bash|sh|source|exec|\.)[[:space:]]+arbre/|\./arbre/' "$1" \
    | sed 's/^/  ligne /'
  grep -n -E 'uses:[[:space:]]*actions/setup-' "$1" | sed 's/^/  ligne /'
}

mesures=(identifiants_non_persistes jeton_hors_des_url
         arbre_conditionne_au_brouillon rien_de_l_integration_n_est_execute)

juger() {  # juger <mesure> <fichier> -> 0 vert, 1 rouge ; écrit le détail
  local sortie
  sortie=$("$1" "$2")
  [ -z "$sortie" ] && return 0
  printf '%s\n' "$sortie"
  return 1
}

# --- Les témoins -------------------------------------------------------------
# Chaque invention nomme la mesure qu'elle doit faire rougir ; `conforme` n'en
# fait rougir aucune. Un témoin qui ment arrête tout : une mesure qui ne sait
# pas rougir ne garde rien, et une mesure qui rougit sur le conforme rougira
# partout.

declare -A attendu=(
  [conforme.yml]=""
  [persiste-identifiants.yml]="identifiants_non_persistes"
  [url-avec-jeton.yml]="jeton_hors_des_url"
  [arbre-inconditionnel.yml]="arbre_conditionne_au_brouillon"
  [execute-l-integration.yml]="rien_de_l_integration_n_est_execute"
)

menteurs=0
echo "· témoins"
for cas in $(printf '%s\n' "${!attendu[@]}" | LC_ALL=C sort); do
  fichier="tests/inventes/$cas"
  if [ ! -f "$fichier" ]; then
    echo "  TÉMOIN MANQUANT : $fichier"
    menteurs=$((menteurs + 1))
    continue
  fi
  for mesure in "${mesures[@]}"; do
    if juger "$mesure" "$fichier" > /dev/null; then rouge=0; else rouge=1; fi
    if [ "$mesure" = "${attendu[$cas]}" ]; then
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

if [ "$menteurs" -ne 0 ]; then
  echo "  $menteurs témoin(s) en défaut — le harnais est en cause, pas le dépôt." >&2
  exit 2
fi
echo "  ${#attendu[@]} témoins tiennent"

# --- Le dépôt ----------------------------------------------------------------

rouges=0
echo "· workflows"
for fichier in "${reels[@]}"; do
  for mesure in "${mesures[@]}"; do
    if ! juger "$mesure" "$fichier"; then
      echo "  ROUGE $mesure — $fichier"
      rouges=$((rouges + 1))
    fi
  done
done

echo "· scripts et crochets"
for fichier in "${scripts[@]}"; do
  [ -f "$fichier" ] || continue
  if ! juger jeton_hors_des_url "$fichier"; then
    echo "  ROUGE jeton_hors_des_url — $fichier"
    rouges=$((rouges + 1))
  fi
done

if [ "$rouges" -ne 0 ]; then
  echo "  $rouges rouge(s)." >&2
  exit 1
fi
echo "  tout est vert"

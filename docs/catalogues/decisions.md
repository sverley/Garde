# Décisions

Comment le produit est fait (§3). Préfiguration du catalogue des décisions : les identifiants seront
attribués par la garde à la tranche B ; ici les entrées sont nommées par ce qu'elles tranchent.

---

# Dépendances

**La règle : chaque caisse du noyau est une décision justifiée.** Un outil dont le métier est la
certification n'a pas de raison de s'exempter de ce qu'il impose. Le harnais `dependances` mesure la
bijection dans les deux sens : aucune caisse déclarée dans `Cargo.toml` sans entrée ici, aucune
entrée ici sans caisse déclarée.

## Caisses retenues

Aucune. Le noyau ne dépend que de la bibliothèque standard.

## Caisses écartées

### clap

**Écartée.** L'analyse de la ligne de commande est écrite à la main dans `src/appel.rs`.

`clap` est confortable et tire une dizaine de caisses transitives. Ce que A1 demande de la ligne de
commande — un verbe, deux options à valeur, l'aide et la version — s'écrit en une cinquantaine de
lignes qu'on lit d'un seul tenant, et qui n'ont ni dérivation, ni macro, ni surface d'évolution
qu'on ne contrôle pas.

Trois raisons, dans l'ordre où elles pèsent :

1. **§5 — déterministe, hors ligne, sans dépendance.** C'est la propriété que la garde promet au
   projet qu'elle juge. Une dizaine de caisses transitives dans le noyau la rend plus difficile à
   tenir : chacune est une mise à jour à suivre et une surface qu'on n'audite pas.
2. **La règle du document.** Chaque caisse doit être une décision justifiée. Le confort n'est pas
   une justification quand le coût de l'écrire à la main est d'une cinquantaine de lignes.
3. **Le prix est connu et borné.** Ce qu'on perd — les messages d'usage générés, les complétions de
   shell, les sous-commandes imbriquées — n'est demandé par aucune tranche du plan.

**Ce qui la rouvrirait.** Le jour où la ligne de commande demande des sous-commandes imbriquées, des
complétions, ou dépasse ce qu'on relit d'un seul tenant, cette entrée s'amende — elle ne se
contourne pas.

---

# Ligne de commande

## convention d'appel GNU

**Retenu.** Options longues (`--projet`), **valeur au mot suivant**, et `--` qui ferme les options.

- Une option se donne en toutes lettres, précédée de deux tirets. Aucune option d'un seul caractère
  n'est servie.
- La valeur d'une option est le **mot suivant, quel qu'il soit** — tiret compris. Seule la fin de la
  ligne prive une option de sa valeur.
- `--` ferme les options : ce qui suit est un opérande, fût-il le sosie d'une option. Il n'est pas
  lui-même un opérande ; un second `--`, lui, en est un. Il protège les opérandes, **pas** les
  valeurs d'option : consommé comme valeur au mot d'avant, il ne ferme rien.

**Source.** Le porteur, dans l'intégration #12.

La référence n'est pas POSIX, qui ne connaît que les options d'un seul caractère : ce sont les *GNU
Coding Standards* et le comportement de `getopt_long`. Ce que POSIX apporte ici est sa **ligne
directrice 7** — la valeur d'une option est l'argument suivant —, que GNU ne lève pas.

**Écarté : la forme `--nom=valeur`.** Elle est une seconde façon d'écrire ce que la première dit
déjà, et deux formes pour une même chose sont deux chemins à analyser, à documenter et à garder.

Étant une forme **voisine d'une option servie** et non une option inconnue, elle se refuse en
nommant l'option (§4.1) : `--projet=/ailleurs` répond que la forme `--projet=valeur` n'est pas
servie, jamais que `--projet=/ailleurs` serait une option qu'on ne connaît pas. Une forme voisine
lue comme une variante est précisément ce que §4.1 interdit.

**Conséquence assumée.** Sans la forme `=`, une valeur qui commence par un tiret se donne quand
même au mot suivant : `--projet --` donne `--` pour valeur à `--projet`, et `--projet --anterieur`
donne `--anterieur`. C'est surprenant à la première lecture et c'est le comportement de tous les
outils GNU — `ls --format --help` prend `--help` pour valeur et le dit. Le refuser demanderait de
décider à la place de l'appelant qu'un mot en tiret ne peut pas être une valeur, ce qui est faux :
un chemin peut commencer par un tiret.

**Ce qui la rouvrirait.** Le jour où une valeur en tiret devient assez courante pour qu'on se
trompe souvent, la forme `=` redevient le moyen de lever l'ambiguïté et cette entrée s'amende. De
même si des sous-commandes imbriquées apparaissent : la convention qui les porte n'est pas
celle-ci. L'entrée s'amende, elle ne se contourne pas.

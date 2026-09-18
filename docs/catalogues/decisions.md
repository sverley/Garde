# Décisions — dépendances

Préfiguration du catalogue des décisions (§3). Les identifiants seront attribués par la garde à la
tranche B ; ici les entrées sont nommées par la caisse qu'elles tranchent.

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

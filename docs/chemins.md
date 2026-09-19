# Les chemins de ce projet

Les chemins qui disent **quand une validation est due** (#20, §4.4).

Ce fichier est la source des chemins **de ce projet-ci**, destinée à être embarquée dans son
exécutable au moment de la livraison (§5), dans la même forme que les entrées des harnais-cœur.
Il n'est pas un fichier que la garde réclame à un projet jugé : §1 veut qu'un projet conforme au
formalisme soit jugeable tel quel, sans rien ajouter à son dépôt. Tant que l'exécutable n'existe
pas, le contrôle le lit **dans l'arbre du juge** — jamais dans l'arbre jugé.

Ce que la garde prendra à sa charge : un chemin est un attribut d'entrée de catalogue (§3), et le
plancher se tirera des chemins portés par les entrées. Ce fichier tient leur place tant que les
catalogues n'existent pas — échafaudage retiré en tranche B.

## Le plancher s'écrit par exclusion

Un chemin modifié qui n'est **pas** dans la liste ci-dessous impose une validation. Un chemin
nouveau que personne n'a pensé à exclure en impose donc une, au lieu de passer en silence : *un
oubli fait jouer plus, jamais moins* (annexe C).

Une ligne est un motif à la manière de `case`, comme pour l'empreinte. Une ligne vide ou ouverte
par un croisillon est ignorée.

```
src/**
Cargo.toml
Cargo.lock
.gitignore
README.md
docs/application.md
docs/plan-de-construction.md
```

## Pourquoi ceux-là

| Exclu | Fondement |
|---|---|
| Le produit et son manifeste | §6, premier membre : *ajouter, en respectant la garde en place — rien de plus n'est demandé*. Le jour où le produit fait rougir un harnais, c'est le rouge qui parle, pas une validation exigée d'avance |
| La documentation simple | §4.5 : ni catalogue, ni garde. La garde vérifie qu'elle ne contredit pas ce qu'elle garde ; elle n'en fait pas un point de validation |

## Pourquoi tout le reste est dedans

| Au plancher | Fondement |
|---|---|
| La description et les catalogues | §5, contenu gardé hors décisions ; §4.3, conformité par rang dès qu'une intégration touche un catalogue |
| Les harnais | §4.2 : *amender ou retirer une garde demande la même analyse et la même validation qu'en ajouter une* |
| La garde en place — crochets, workflows, scripts, ce fichier | §6, second membre : modifier un comportement de la garde. Tant qu'il n'y a pas d'artefact, la garde en place, c'est eux |

## À ne pas confondre avec l'autre axe

L'empreinte dit **quand une validation tombe** ; ce fichier dit **quand elle est due**. Les deux
listes ne se recouvrent pas et ne doivent pas être alignées : les harnais sont exclus de la
première — les corriger n'annule pas une validation — et inclus dans le second — les amender en
appelle une.

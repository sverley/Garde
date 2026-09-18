# Contraintes

Ce que l'environnement impose (§3). Une contrainte ne se discute pas au fil d'un développement :
les décisions s'y conforment, elle ne cède à aucune. Elle ne se subordonne qu'aux principes de la
description, qu'elle ne peut pas contredire.

Une entrée porte une ligne `Cibles :` qui nomme les cibles qu'elle concerne ; absente, elle vaut
pour toutes.

Les identifiants sont attribués à la main tant que la garde ne les attribue pas elle-même —
échafaudage « Catalogues maintenus à la main » de `docs/application.md`.

---

## `jeton hors du disque`

Un jeton d'accès n'existe qu'en mémoire de process et dans les instructions de la session qui le
porte. Il n'est écrit dans aucun fichier : ni configuration de dépôt, ni fichier d'environnement,
ni journal, ni arbre de travail, ni artefact.

Ce qui en découle, partout où un jeton sert à lire ou à écrire :

- une URL de dépôt ne porte jamais d'identifiants — ni dans un `remote`, ni dans un `clone` dont la
  configuration les garderait ;
- une action qui s'authentifie pour le compte du projet déclare qu'elle ne persiste pas ses
  identifiants, y compris quand rien n'est fait du dépôt ensuite ;
- un jeton passé en argument ou en variable d'environnement est admis : il ne survit pas au process.

La contrainte vient de l'environnement, pas d'un choix : un jeton écrit sur disque survit à l'usage
qui l'a justifié, et le contexte qui l'écrit est précisément celui qui porte le plus de droits.

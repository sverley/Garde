# Plan de construction

Ce plan est subordonné au document fondateur : il dit *comment on y arrive*, jamais *ce qu'on
construit*. Toute contradiction entre les deux se tranche en faveur du document.

L'ordre se défend en une phrase : **sans catalogues intègres, la couverture n'a rien à couvrir ;
sans couverture, le contrôle d'une intégration n'a rien à exiger.**

---

## Ce qui est déjà tranché

Pour mémoire, et pour ne pas rouvrir en chemin.

| | |
|---|---|
| Langage | Rust |
| Cibles | Windows, macOS, Linux, compilées nativement |
| Distribution | canaux en ligne de commande, signature ad-hoc, pas de Developer ID ni d'Authenticode |
| Emplois | `juger` n'exécute rien du projet ; `jouer` exécute les harnais |
| Entrée | deux arbres ; git en commodité d'appel |
| Harnais-cœur | embarqués dans l'artefact avec leurs entrées ; le cible n'a rien à copier |
| Harnais-garde | écrits par le cible, au registre, avec leurs témoins |
| Formes de harnais | commande, test nommé |
| Issues d'exécution | passé, rouge, n'a pas pu tourner (code réservé), interrompu |
| Témoin rouge | obligation à l'écriture, redue quand l'empreinte du corps change |
| Sémantique | au seuil lâche, en proposition ; jamais au seuil qui décide |
| Preuve | ce que l'intégration n'a pas pu produire ; le reste est du signalement |

Mise de côté volontairement : la montée de version du schéma (§8), et le socle dérivé pour plusieurs
projets (§5).

---

## Étape 0 — Fondations

Avant toute ligne de code.

- Écrire **les catalogues de la garde elle-même**. Le document fondateur *est* sa description ; en
  extraire les invariants, avec leur source et leur mesure. Le tri de §9.1 s'applique d'abord à elle.
- Fixer le **contrat d'appel** : les verbes, l'état courant et l'état antérieur en arguments, le
  verdict, les codes de retour, le **code réservé** — dont la valeur sera confirmée en étape 3 en
  observant ce que rendent les lanceurs visés.
- Fixer le **schéma** : les huit catalogues, leurs rangs, leurs renvois, l'emplacement conventionnel,
  et la version du schéma consignée dès maintenant pour la migration future.
- Fixer la forme d'un **projet fabriqué** : un répertoire, plus un second pour l'état antérieur.
- Poser la **règle de dépendances** : chaque caisse du noyau est une décision justifiée.

**Preuve de sortie** : les catalogues de la garde existent et se lisent ; le contrat est écrit.

## Étape 1 — Le socle : schéma, intégrité, premiers harnais-cœur

C'est la vraie fondation, et elle est plus grosse qu'un parseur : la garde s'applique à elle-même dès
la première ligne.

- Modèle des huit catalogues et de la priorité, ses deux sortes d'entrées comprises.
- **Lecteur ligne à ligne, pas un parseur Markdown générique** : les catalogues ont une forme
  restreinte, et un lecteur qui refuse ce qu'il ne reconnaît pas sert mieux l'intégrité qu'un parseur
  qui accepte tout.
- Description répartie sur plusieurs documents : un extrait cité est cherché dans tous.
- Intégrité complète (§4.1), y compris le refus exact de l'invariant dont la source est une décision.
- Les premiers **harnais-cœur** portant leurs entrées, embarquées dans l'artefact.
- Verdict exact et court ; le juge affiche la référence qu'il a retenue.

**Preuve de sortie** : un projet fabriqué sain passe ; un projet fabriqué par motif de refus échoue
sur son motif et sur lui seul. Aucun fichier n'a été écrit chez le projet jugé.

## Étape 2 — Les verbes d'écriture

- Ajouter et amender par catalogue, conformité par rang, attribution de l'identifiant par
  translittération déterministe.
- Refus qui réaffiche la prose refusée avec sa raison ; la garde ne réécrit jamais la prose.
- **Rejeu d'une édition externe** à partir des deux arbres.
- Normalisation Unicode, avec sa **version de données épinglée** et attestée par la livraison.

**Preuve de sortie** : prose acceptée consignée à l'identique ; prose refusée non consignée et
montrée ; édition externe rejouée sur les seules entrées touchées.

## Étape 3 — Couverture et exécution

Le cœur du travail, et l'étape la plus longue.

- Registre : entrée → harnais, vérification manuelle, renvoi, ou dette.
- **Les deux formes de harnais.** La forme *commande* d'abord, qui ne demande aucune analyse
  lexicale ; la forme *test nommé* ensuite, avec l'épluchage (commentaires effacés, chaînes et
  expressions régulières masquées, marqueurs de désactivation).
- **`jouer`** : lancement, répertoire de travail, collecte des quatre issues, rapport attaché à
  l'empreinte de l'état joué.
- **Mode strict** comme argument du passage ; criticité par couple harnais × outil ; le strict
  l'emporte.
- **Empreinte du corps** d'un harnais, calculée des deux côtés, jamais consignée.
- Témoins : rouge désigné et localisé, redû quand l'empreinte diverge ; vert sur la référence ou
  dette qui l'explique.

**Preuve de sortie** : un harnais qui rougit, un qui passe, un qui ne peut pas tourner et le dit, un
qui est interrompu — chacun rendant l'issue attendue dans les deux modes. Un corps modifié redemande
son témoin.

## Étape 4 — Conformité, rapprochement, promotions

- Comparaison lexicale : texte normalisé, n‑grammes de mots et de caractères, recouvrement, distance
  d'édition. Seuil exigeant, rapprochement automatique et consigné, raison explicable.
- Seuil lâche, proposition avec score, ouvert aux mesures sémantiques — **locales d'abord**, cosinus
  calculé sur les catalogues du projet, sans modèle embarqué. Scores arrondis avant comparaison.
- Signaux de promotion, rendus au même passage que la conformité.
- Vérifications manuelles de conformité demandées par rang dès qu'une intégration touche un catalogue.
- Rapprochements écartés notés pour ne pas revenir.

**Preuve de sortie** : deux entrées jumelles rapprochées automatiquement avec leur raison ; deux
entrées synonymes sans mot commun proposées avec leur score ; aucune décision prise sur un score
sémantique.

## Étape 5 — Contrôle d'une intégration

- Plancher de déclaration tiré de la comparaison des deux arbres et des chemins des entrées.
- Vérifications demandées, analysées, validées ; rouge tant qu'il en manque une.
- **Empreinte de l'état validé** — arbre restreint, documentation et harnais exclus ; annulation par
  divergence d'empreinte, jamais par liste de cas.
- Format et verbe d'enregistrement d'une validation.
- Signal de passer-outre.
- Premiers **harnais-garde** de référence, pour servir de modèle aux cibles : empreinte de l'artefact,
  existence des crochets.

**Preuve de sortie** : une intégration qui demande ses vérifications, une validation enregistrée puis
annulée par un changement d'état, un signal quand on passe outre.

## Étape 6 — Application git/GitHub, et bascule du premier client

- Les workflows de l'annexe C : contrôle depuis la référence, enregistrement, alerte.
- Dépôt protégé pour l'artefact ; harnais-garde d'empreinte chez le client.
- Distribution : compilation sur les trois runners, signature ad-hoc, canaux en ligne de commande.
- Bascule : tri des catalogues du client, extraction des invariants, réécriture des 29 fichiers
  d'amorçage en tests de la garde.

**Preuve de sortie** : les sept conditions de §9.

---

## Ce qui se vérifie à chaque étape

Quatre propriétés qui ne sont pas d'une étape mais de toutes. À défaut, elles se perdent en chemin.

1. **Non-intrusion** — un projet conforme au formalisme est jugeable sans qu'on lui ajoute ni ne lui
   écrive quoi que ce soit.
2. **Séparation des emplois** — `juger` n'ouvre, n'exécute et ne charge rien du projet jugé.
3. **Déterminisme** — rien qui sorte ne dépend d'un ordre de parcours non spécifié, d'une horloge, ou
   d'une version de données non épinglée.
4. **La garde ne décide pas** — tout ce qui relève du sens sort en proposition ou en vérification
   manuelle, jamais en verdict.

## Risques nommés

- **L'épluchage lexical** de l'étape 3 est la partie la plus fragile. Atténuation : la forme
  *commande* la contourne entièrement, et le projet qui veut de la robustesse peut l'employer.
- **Le lien par le titre** entre un invariant et son harnais se rompt à un renommage. La garde le
  verra comme absent — verdict exact, cause obscure. Atténuation : proposer le titre le plus proche
  dans le même fichier, ce qui est le rapprochement de l'étape 4 appliqué ailleurs.
- **Le code réservé** doit ne pas pouvoir être rendu par accident. Bandes à éviter : les petits codes
  des lanceurs, 64–78, 123–127, 128 et au-delà. À confirmer par un test de la garde qui observe ce
  que rendent les lanceurs visés.
- **L'arbitrage flottant** près d'un seuil sémantique. Atténuation : arrondi ou quantification avant
  comparaison, et le sémantique cantonné à la proposition.

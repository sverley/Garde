# Plan de construction

Documentation simple, subordonnée au document fondateur : elle dit *dans quel ordre on construit*,
jamais *ce qu'on construit*. Toute contradiction se tranche en faveur du document.

Elle est jetable : quand l'outil existe, elle ne sert plus. Son pendant durable est
`docs/application.md`, qui dit comment les concepts sont appliqués **maintenant**. Les deux partagent
une seule liste — celle des échafaudages. **L'application dit lesquels restent ; le plan dit dans
quel ordre ils tombent.**

---

## Le principe d'ordonnancement

Non pas l'ordre du constructeur — socle, puis verbes, puis couverture — mais **l'ordre du
soulagement** : ce qui retire le plus de charge humaine par unité de travail passe devant.

Trois règles en découlent.

**La garde ne réduit pas le jugement, elle réduit la recherche.** §1 lui interdit de décider. Ce
qu'elle retire au porteur, c'est de chercher ce que son intégration touche et quelles vérifications
en découlent. Il cesse de chercher et se met à juger. C'est là qu'est le gain, et c'est pourquoi
`déclarer` et `demander` passent avant la couverture et la conformité.

**La première version publiée vient le plus tôt possible.** §6 fait commencer l'auto-garde là : tant
qu'elle n'existe pas, tout se contrôle à la main. Une version qui ne fait qu'une chose, mais qui est
épinglée et appelée depuis la référence, ferme déjà la chaîne complète — construction, livraison,
épinglage, invocation — sur un objet où rien ne coûte cher à rater. **La fenêtre non gardée de §6
dure une tranche, pas tout le chantier.**

**Les catalogues grossissent avec l'outil.** Ils n'ont pas à être complets pour servir : cinq
invariants avec leurs chemins produisent déjà un plancher de déclaration utile. Le tri du document
fondateur se fait à mesure, sous la garde qui existe déjà, et non d'un coup avant elle.

### Chaque tranche

- se termine par une **version publiée et épinglée**, qui garde la tranche suivante ;
- **retire au moins un échafaudage** de `docs/application.md` ;
- est utilisable bien qu'incomplète.

---

## Ce qui est déjà tranché

Pour mémoire, et pour ne pas rouvrir en chemin.

| | |
|---|---|
| Langage | Rust |
| Cibles | Windows, macOS, Linux, compilées nativement |
| Distribution | canaux en ligne de commande, signature ad-hoc |
| Emplois | `juger` n'exécute rien du projet ; `jouer` exécute les harnais |
| Entrée | deux arbres ; git en commodité d'appel |
| Harnais-cœur | embarqués dans l'artefact avec leurs entrées |
| Formes de harnais | commande, test nommé |
| Issues d'exécution | passé, rouge, n'a pas pu tourner (code réservé), interrompu |
| Témoin rouge | obligation à l'écriture, redue quand l'empreinte du corps change |
| Sémantique | au seuil lâche, en proposition ; jamais au seuil qui décide |
| Preuve | ce que l'intégration n'a pas pu produire ; le reste est du signalement |

---

## Tranche A — La chaîne, sur le plus petit objet possible

**Ce qu'on livre** : un projet Rust, un seul verbe, trois cibles, une version publiée.

Le verbe est `garde empreinte` : l'arbre restreint, la liste d'exclusions, une empreinte. Rien
d'autre. Il est choisi parce qu'il est le plus petit objet qui existe déjà — le script shell le fait
aujourd'hui — et parce qu'un écart se voit immédiatement.

Tout le risque de la tranche est dans la chaîne, pas dans le verbe : compilation sur les trois
runners, signature ad-hoc appliquée par l'éditeur de liens, publication, dépôt protégé, épinglage,
et appel depuis `garde.yml` en contexte privilégié. Le rater sur un verbe trivial coûte une heure ;
le rater plus tard coûte une tranche.

**Échafaudage retiré** : `empreinte.sh`.

**Preuve de sortie** : `garde.yml` n'appelle plus de script du dépôt mais l'artefact épinglé, et le
verdict est inchangé. Le même arbre rend la même empreinte sur les trois cibles.

À faire à la main à ce moment-là, et pas avant : la protection de la référence, et le dépôt protégé
de l'artefact. Sans eux, ce qui suit indique sans barrer.

## Tranche B — Les catalogues, et la déclaration qui se calcule

**Ce qu'on livre** : la garde commence à se garder, et le porteur cesse d'écrire sa déclaration.

- Lecteur des catalogues, **ligne à ligne et non parseur Markdown générique** : une forme restreinte
  qui refuse ce qu'elle ne reconnaît pas sert mieux l'intégrité qu'un parseur qui accepte tout.
- Catalogues **minimaux** du projet garde : la description est le document fondateur ; le glossaire
  est son §2 ; trois à cinq invariants avec leur source et leur mesure. Pas le tri complet.
- Chemins portés par les entrées ; comparaison des deux arbres.
- Intégrité (§4.1) sur ce qui existe, et le **premier harnais-cœur** qui la porte.
- `garde declarer` : les identifiants touchés, et le plancher que les chemins imposent.
- `garde juger`, verdict exact et court, référence affichée.

**Échafaudages retirés** : déclaration remplie à la main ; absence de contrôle d'intégrité.

**Preuve de sortie** : la déclaration d'une intégration est produite par l'outil et non tapée ; un
projet fabriqué par motif de refus échoue sur son motif et sur lui seul ; rien n'est écrit chez le
projet jugé.

## Tranche C — Les vérifications qui se demandent

C'est la tranche qui soulage le plus, et c'est pour ça qu'elle vient avant la couverture.

- Registre : entrée → harnais, vérification manuelle, renvoi, ou dette.
- `garde demander` : les vérifications des engagements déclarés **et de ceux qui les couvrent par
  renvoi**. C'est le renvoi qui fait la valeur : c'est exactement ce qu'un humain oublie.
- Rouge tant qu'une vérification manque son analyse ou sa validation.
- Format et verbe d'enregistrement d'une validation, tenus par la garde.
- Vérification manuelle de conformité demandée par rang dès qu'une intégration touche un catalogue.

**Échafaudages retirés** : vérifications demandées à la main ; format et verbe de validation tenus
par le workflow.

**Preuve de sortie** : une intégration reçoit ses vérifications sans que personne ne les ait
listées ; une validation est enregistrée puis annulée par un changement d'état.

## Tranche D — L'écriture des catalogues

Sa valeur croît avec la taille des catalogues : elle vient ici parce qu'à partir de la tranche C, ils
grossissent vite.

- Verbes d'ajout et d'amendement par catalogue, par rang.
- Identifiants attribués par translittération déterministe, figés à la création.
- Refus qui réaffiche la prose refusée avec sa raison ; la garde ne réécrit jamais la prose.
- Rejeu d'une édition externe sur les seules entrées touchées.
- Normalisation Unicode, version des données épinglée et attestée par la livraison.

**Échafaudage retiré** : catalogues maintenus à la main.

**Preuve de sortie** : prose acceptée consignée à l'identique ; prose refusée montrée et non
consignée ; une édition faite hors de l'outil est rejouée.

## Tranche E — La couverture et l'exécution

- Les deux formes de harnais. La forme *commande* d'abord, qui ne demande aucune analyse lexicale ;
  la forme *test nommé* ensuite, avec l'épluchage.
- `garde jouer` : lancement, répertoire de travail, collecte des quatre issues, rapport attaché à
  l'empreinte de l'état joué.
- Mode strict en argument du passage ; criticité par couple harnais × outil ; le strict l'emporte.
- Empreinte du corps d'un harnais, calculée des deux côtés, jamais consignée.
- Témoins : rouge désigné et localisé, redû quand l'empreinte diverge ; vert sur la référence ou
  dette qui l'explique.

**Preuve de sortie** : un harnais qui rougit, un qui passe, un qui ne peut pas tourner et le dit, un
qui est interrompu — chacun rendant l'issue attendue dans les deux modes. Un corps modifié redemande
son témoin.

Le **code réservé** se fixe ici, en observant ce que rendent les lanceurs visés. Bandes à éviter :
les petits codes des lanceurs, 64–78, 123–127, 128 et au-delà.

## Tranche F — Conformité, rapprochement, promotions

- Comparaison lexicale : texte normalisé, n‑grammes de mots et de caractères, recouvrement, distance
  d'édition. Seuil exigeant, rapprochement automatique et consigné, raison explicable.
- Seuil lâche, proposition avec score, ouvert au sémantique — **local d'abord**, cosinus calculé sur
  les catalogues du projet, sans modèle embarqué. Scores arrondis avant comparaison.
- Signaux de promotion, rendus au même passage que la conformité.
- Rapprochements écartés notés pour ne pas revenir.

**Échafaudage retiré** : absence de contrôle de conformité.

**Preuve de sortie** : deux entrées jumelles rapprochées automatiquement avec leur raison ; deux
entrées synonymes sans mot commun proposées avec leur score ; aucune décision prise sur un score
sémantique.

## Tranche G — Le premier client externe

Jusqu'ici la garde n'a gardé qu'elle-même. C'est ici que §9 se joue : un second projet, qui n'a pas
été écrit en même temps qu'elle, et qui la révèle générique ou non.

- Tri des catalogues du client, extraction des invariants avec leur source et leur mesure.
- Registre du client, harnais existants raccordés, dettes nommées et non masquées.
- Réécriture des 29 fichiers d'amorçage en tests de la garde, sur des projets fabriqués.
- Application complète de l'annexe C chez le client.

**Preuve de sortie** : les sept conditions de §9.

---

## Ce qui se vérifie à chaque tranche

Quatre propriétés qui ne sont d'aucune tranche et de toutes. À défaut, elles se perdent en chemin.

1. **Non-intrusion** — un projet conforme au formalisme est jugeable sans qu'on lui ajoute ni ne lui
   écrive quoi que ce soit.
2. **Séparation des emplois** — `juger` n'ouvre, n'exécute et ne charge rien du projet jugé. Le jour
   où quelqu'un ajoute une construction au contrôle privilégié pour gagner une étape, le contrôle
   devient un vecteur d'exécution.
3. **Déterminisme** — rien qui sorte ne dépend d'un ordre de parcours non spécifié, d'une horloge, ou
   d'une version de données non épinglée.
4. **La garde ne décide pas** — tout ce qui relève du sens sort en proposition ou en vérification
   manuelle, jamais en verdict.

## Risques nommés

- **L'épluchage lexical** de la tranche E est la partie la plus fragile. Atténuation : la forme
  *commande* la contourne entièrement, et un projet qui veut de la robustesse peut l'employer.
- **Le lien par le titre** entre un invariant et son harnais se rompt à un renommage. La garde le
  verra comme absent — verdict exact, cause obscure. Atténuation : proposer le titre le plus proche
  dans le même fichier, ce qui est le rapprochement de la tranche F appliqué ailleurs.
- **Le rythme des versions rouvre §8.** Sept tranches, sept versions épinglées, et le schéma change à
  chacune : les catalogues du projet garde cessent d'y être conformes. La question mise de côté
  — initialiser et migrer sont la même opération — se pose donc dès la tranche C, et non à la fin.
  C'est plutôt une bonne nouvelle : on l'apprend quand les catalogues ont dix entrées, pas deux
  cents. À trancher avant la tranche C, pas après.
- **L'arbitrage flottant** près d'un seuil sémantique. Atténuation : arrondi ou quantification avant
  comparaison, et le sémantique cantonné à la proposition.

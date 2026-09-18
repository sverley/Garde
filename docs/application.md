# L'application au projet de la garde

Documentation simple. Elle décrit comment les concepts du document fondateur sont mis en œuvre
**dans ce dépôt-ci**. Elle n'a autorité sur rien : toute contradiction avec le document fondateur se
tranche en faveur du document.

L'annexe C décrit l'application au premier projet client, qui est en JS/TS. Celle-ci en diverge là
où le contexte diffère — un projet Rust, et une garde qui n'existe pas encore.

---

## L'état d'amorçage

§6 : *avant la première version publiée, la garde est seulement testée, pas gardée.* Il n'y a
aujourd'hui ni binaire, ni harnais-cœur, ni catalogues. Ce qui est en place ne peut donc porter que
sur ce qui ne dépend pas d'eux.

**Ce qui est contrôlé :** la forme du processus. Qu'une intégration déclare les engagements qu'elle
touche, que chaque vérification demandée porte une analyse, qu'une validation existe et qu'elle porte
sur l'état courant, qu'un passer-outre ouvre une alerte.

**Ce qui ne l'est pas :** le fond. Rien ne sait aujourd'hui *quelles* vérifications devraient être
demandées, ni si une décision est conforme à son rang, ni si un invariant a son harnais. C'est ce que
la garde apportera, et ce que les échafaudages ci-dessous devront lui rendre.

Construire cette couche à la main est voulu : c'est l'épreuve de réalité que §7 invoquait en écartant
la branche vide. Ce qu'on sentira ici est ce que la garde devra automatiser.

## Les trois contrôles

### `garde.yml` — le contrôle d'une intégration

Joué **depuis la référence**, par `pull_request_target` : le fichier du workflow, le script
d'empreinte et la liste d'exclusions sont pris dans la base, jamais dans la tête de l'intégration.
C'est ce qui empêche une intégration de désarmer sa propre garde (§5).

Il **ne lit que des noms de fichiers et du texte**. Il n'exécute, ne construit et n'installe rien
venu de l'intégration. C'est la condition qui rend sûr de le lancer avec un jeton en écriture, et
elle ne souffre aucune exception : y ajouter un `cargo build` transformerait le contrôle en vecteur
d'exécution privilégiée.

Ses propres chemins figurent parmi ceux de l'empreinte : le modifier annule les validations en
cours.

### `alerte.yml` — passer outre se voit

Une écriture directe dans la référence, ou une fusion dont la tête n'était pas verte, ouvre une issue
étiquetée « alerte ». Seul un humain la ferme, en écrivant la raison. Il ne distingue pas la main du
porteur de celle d'un agent quand tous écrivent avec le même compte — limite assumée.

### `construction.yml` — le passage complet

Celui-ci exécute bien le code de l'intégration, et c'est pourquoi il tourne sur `pull_request` avec
un jeton en lecture seule. **Les deux ne doivent jamais être confondus** : l'un juge et n'exécute
rien, l'autre joue et n'a aucun privilège.

`GARDE_STRICT=1` y est posé : dans le passage complet, un outil manquant fait échouer au lieu de
sauter. Le harnais n'a rien à lire — le mode est un argument du passage, et c'est la garde qui
interprétera le code réservé (§4.2).

## La validation

Divergence assumée avec l'annexe C, qui emploie des cases à cocher.

Le contrôle calcule l'**empreinte de l'état validable** — l'arbre restreint, documentation et
harnais exclus (§4.4) — et l'affiche. Le porteur valide en commentant exactement :

```
Validé: <empreinte>
```

Ce qu'on y gagne sur la case à cocher : la validation **dit ce qu'elle valide**. Une validation
posée avant un push ne porte pas sur l'état d'après, sans qu'aucune règle d'annulation n'ait à
l'énoncer — l'empreinte change, et le contrôle repasse au rouge de lui-même. Les conditions
d'annulation de l'annexe C, qui étaient une liste de cas, deviennent une comparaison, qu'aucun
chemin oublié ne contourne.

L'enregistrement est posté par le compte de la plateforme : support inimitable (§4.4).

### Ce que l'empreinte couvre

Déclaré dans `.garde/empreinte-exclusions`, et gardé comme le reste.

Sont **exclus** la documentation simple et les harnais : les modifier n'annule pas une validation.
Sont **inclus** les catalogues — dont `docs/document-fondateur.md`, qui est la description —, le
produit, les workflows et le fichier d'exclusions lui-même. Exclure un catalogue reviendrait à
pouvoir amender la description sans annuler la validation qui portait dessus.

## Les moments d'exécution

| Moment | Ce qui s'y joue |
|---|---|
| Pré-commit | scripts, empreinte, forme et tests du cœur ; rapide par construction |
| Intégration | `garde.yml` depuis la référence, à chaque poussée et à chaque édition |
| Passage complet | `construction.yml` sur les trois cibles, en mode strict |
| Référence | l'alerte, sur poussée directe et sur fusion |

Les crochets vivent dans le dépôt ; `.githooks/install.sh` les branche. Ils ne prouvent rien : un
crochet se contourne, et il tourne sur le poste de qui pourrait vouloir le contourner. **Ce qui tient
la continuité, c'est le rejeu au niveau supérieur** (§4.4) — celui qui intègre exécute avec son
artefact et n'a à croire aucun verdict venu d'en dessous.

## Ce qui reste à la main du porteur

Deux choses que le dépôt ne peut pas se donner à lui-même, et sans lesquelles les contrôles sont des
indications et non des barrières :

1. **La protection de la référence** — exiger que `garde` et `construction` soient verts avant
   fusion, et interdire la poussée directe sur `main`. Sans elle, la couleur du contrôle est le seul
   signal, comme le note l'annexe C.
2. **Le dépôt protégé de l'artefact**, quand il y aura un artefact. C'est lui qui fera l'ancrage hors
   du poste (§5), et c'est ce que l'empreinte seule ne peut pas donner.

## Les dettes

Tout ce qui suit est un échafaudage provisoire : une implémentation parallèle qui dérivera si on
l'oublie. Chacune doit devenir un besoin ouvert et être rendue à la garde quand elle existera.

| Échafaudage | Ce que la garde devra reprendre | Tranche |
|---|---|---|
| `empreinte.sh` | §4.4 donne l'empreinte de l'état validé à la garde. Le script est une préfiguration en shell. | A |
| Déclaration remplie à la main | Le plancher se tire des chemins des entrées comparés à ce qui est modifié. Sans catalogues, il n'y a pas de chemins. | B |
| Aucun contrôle d'intégrité | §4.1 n'est pas joué : rien ne vérifie l'unicité d'un identifiant, un renvoi mort, un invariant orphelin. | B |
| Vérifications demandées à la main | La garde les demande à partir des engagements déclarés et de ceux qui les couvrent par renvoi. C'est le renvoi qu'un humain oublie. | C |
| Format et verbe de validation | §4.4 : *la garde possède le format et le verbe*. Ici, c'est le workflow qui les tient. | C |
| Catalogues maintenus à la main | Identifiants attribués, refus avec sa raison, rejeu d'une édition externe (§3). | D |
| Aucune couverture | §4.2 n'est pas joué : rien ne vérifie qu'un invariant a son harnais, qu'il tourne, qu'il a ses témoins. | E |
| Aucun contrôle de conformité | §4.3 n'est pas joué : rien ne vérifie qu'une décision se conforme à son rang, rien ne signale les promotions. | F |
| Liste d'exclusions écrite | Elle appartiendra au schéma, avec un emplacement conventionnel et une déclaration pour les seuls écarts (§3). | D |

Chaque échafaudage est une implémentation parallèle : elle dérivera si on l'oublie. L'ordre dans
lequel ils tombent est celui de `docs/plan-de-construction.md`, et il est choisi par le soulagement
qu'il apporte, non par la commodité du constructeur.

**Ce tableau est la mesure du chantier.** Quand il est vide, la garde garde ce qu'elle promet.

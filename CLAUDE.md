# Instructions de travail

Ce fichier dit **où vit l'autorité**, pas ce qu'elle dit.

Il ne porte aucun engagement. S'il en énonçait un, il deviendrait une source de plus, il
dériverait d'avec les documents qu'il résume, et l'on ne saurait plus lequel fait foi. Il route,
il ne légifère pas.

## À lire avant tout travail, à chaque session

Ces documents se lisent **en entier et à chaque session**, avant toute analyse, tout code, toute
réponse à une question sur le projet. Une session qui commence à travailler sans les avoir lus
travaille contre eux sans le savoir — c'est ainsi qu'on se met à garder des exigences que personne
n'a prises.

```
docs/document-fondateur.md
docs/catalogues/contraintes.md
docs/catalogues/regles.md
docs/chemins.md
```

Cette liste est mesurée : `tests/documents.sh` vérifie que chaque document nommé existe, et
qu'aucun catalogue n'en est absent. Un catalogue nouveau qu'on oublierait d'y ajouter rougit.

Ce que la mesure ne peut pas faire, c'est vérifier qu'on a lu. Ça ne se code pas, et ça ne se
contourne qu'au prix de tout le reste : un travail fondé sur ce qu'on croit que les documents
disent finit par les contredire, et c'est ce que la garde est faite pour rendre visible.

`docs/application.md` et `docs/plan-de-construction.md` se lisent dès qu'on touche au processus ou
à l'ordre de construction. Ils n'ont pas autorité (voir ci-dessous), mais ils disent où en est le
chantier.

## Ce qui fait autorité, par rang

| Rang | Où | Ce que c'est |
|---|---|---|
| 1 | `docs/document-fondateur.md` | la description : l'objet, le vocabulaire, ce que la garde garantit. **Les principes y vivent.** |
| 2 | `docs/catalogues/` | invariants, usages, contraintes, cibles, règles, décisions |
| 3 | le porteur, dans le fil de l'intégration | ce qu'il tranche, qui devient une entrée de catalogue |

Le reste — `docs/application.md`, `docs/plan-de-construction.md`, `README.md` — est de la
documentation simple. Elle n'a autorité sur rien. Toute contradiction se tranche en faveur du rang
supérieur.

## Ces documents sont toujours respectés

C'est le point autour duquel tout le projet est bâti.

**On ne travaille jamais contre eux, et jamais à côté d'eux.** Une tâche qui demanderait de s'en
écarter n'est pas une tâche : c'est un amendement à proposer. On amende le document, on obtient la
validation, puis on travaille — dans cet ordre, jamais l'inverse. Un contournement qui « marche »
est un document qui a cessé d'être vrai sans que personne le sache.

**Ils sont la cible de ce qui est gardé.** Ce qui les modifie passe par une validation ; ce qui les
contredit doit rougir. Un travail qui les laisse intacts mais faux est un travail raté.

## Une exigence nomme sa source

Voir `docs/catalogues/regles.md`, entrée `une exigence nomme sa source`, et sa mesure dans
`tests/temoins.sh`.

Ce qui suit en découle et mérite d'être répété ici, parce que c'est l'erreur qui a coûté le plus
cher :

**Un corps d'issue n'est pas une source.** Il a pu être rédigé par une session d'agent. Une phrase
qui s'y trouve est une proposition tant que le porteur ne l'a pas reprise ou qu'elle ne renvoie pas
au document. Avant de coder un harnais, nommer ce qui engage ce qu'il mesure ; si rien ne
l'engage, **demander** au lieu de coder.

Une session n'engage pas le projet, y compris quand elle a écrit le besoin, et y compris quand
c'est cette session-ci.

## Mesurer, pas affirmer

Une phrase qui affirme qu'une propriété tient ne la garde pas. Si elle est programmable, elle se
code — un harnais programmable doit être codé. Sinon elle devient une vérification manuelle
décrite, et seulement là où le codage est impossible.

Une garde qu'on ajoute, qu'on amende ou qu'on retire demande la même analyse et la même validation
dans les trois cas.

## Le travail

Le processus de travail — analyse, audit du besoin, codage, audit du codage — est donné par le
porteur à chaque session. Il n'est pas recopié ici : le recopier créerait une seconde version qui
vieillirait.

Trois choses ne se font jamais sans autorisation explicite du porteur :

- cocher ou poser une validation ;
- fusionner dans une branche parente ou dans `main` ;
- toucher aux branches d'une autre issue.

## Vérifier avant d'affirmer

Ne pas rapporter un état lu dans sa copie de travail. Relire la branche après avoir poussé : une
poussée qui n'envoie rien ne se distingue pas d'une poussée qui réussit.

---

Ce fichier est dans le plancher : le modifier appelle une validation, comme tout ce qui engage.

# Règles

Comment le projet se conduit (§3). Une règle se conforme aux principes de la description et aux
autres règles ; si elle les heurte, elle propose les amendements qui rétablissent la cohérence.

Les identifiants sont attribués à la main tant que la garde ne les attribue pas elle-même —
échafaudage « Catalogues maintenus à la main » de `docs/application.md`.

---

## `une exigence nomme sa source`

Toute exigence qu'un harnais mesure nomme ce qui l'engage. Une source est la description, un
catalogue, ou le porteur. **Un corps d'issue n'en est pas une**, ni une description d'intégration,
ni une documentation simple, ni ce qu'une session a écrit dans un fil.

Ce qui en découle :

- avant d'écrire un harnais, on nomme la source de ce qu'il mesure ; si on ne la trouve pas,
  l'exigence n'existe pas encore et se demande au porteur au lieu de se coder ;
- une exigence sans source ne devient pas une vérification manuelle. Une vérification manuelle
  mesure un engagement qui existe ; elle ne le crée pas ;
- une session d'agent n'engage pas le projet, y compris quand elle a rédigé le besoin. Ce qu'elle
  écrit est une proposition tant que le porteur ne l'a pas reprise.

La règle vient d'un fait : le corps de #4 demandait une version minimale de Rust déclarée. Personne
ne l'avait demandée. Elle est devenue une vérification de harnais, puis une vérification manuelle,
puis une exigence de justification, puis une proposition de tâche de construction — quatre étages
au-dessus d'une phrase sans autorité, et rien ne l'a vu. Ce qui distingue une exigence d'une idée,
c'est d'où elle vient, et cela se déclare.

**Mesure** — `tests/temoins.sh`, section « sources » : chaque harnais déclare `//! SOURCE …`, et
la source doit renvoyer au document, au plan, à un catalogue ou au porteur. Absente ou sans
autorité, le harnais rend 2 : c'est le harnais qui est en cause, pas le produit.

Close #

## Déclaration

<!--
Les engagements touchés, et ceux dont le lien pourrait être masqué (§4.4).
Le plancher — ce que les chemins modifiés imposent — est aujourd'hui tiré par le
contrôle de `docs/chemins.md` ; la garde le tirera des chemins portés par les
entrées quand les catalogues existeront.
-->

| Entrée | Pourquoi elle est touchée |
|---|---|
|  |  |

## Vérifications manuelles

<!--
Deux formes possibles, et une seule est exigée.

1. Une section `### VM-<nom>` par vérification demandée, suivie de son analyse.
   Une vérification sans analyse laisse l'intégration rouge.

   L'analyse dit : ce que les modifications changent, ce qu'elles ne touchent
   pas, ce qui reste à constater. Honnête — les écarts pris et ce qui n'a pas pu
   être vérifié autant que ce qui marche — et concise.

2. Aucune vérification : l'écrire ici, et dire pourquoi le produit n'est pas
   atteint (§4.4, consignes proportionnées). Une intégration qui ne touche que
   des harnais, de l'outillage ou de la documentation n'a pas à inventer une
   vérification pour avoir quelque chose à valider. Le contrôle constate que la
   justification est là ; il ne la juge pas.

Un commentaire HTML ne vaut ni analyse ni justification : ce bloc-ci ne compte
pas, il faut écrire en dessous.
-->

## Validation

<!--
Une validation n'est due que là où l'intégration touche un engagement : un
catalogue, un harnais, ou la garde en place. Le contrôle dit quel chemin
l'impose, et ne la réclame pas autrement (§4.4, §6 premier membre).

Quand elle est due, seul le porteur de cette intégration valide, et elle vaut
pour l'état validé. Le contrôle affiche l'empreinte ; le porteur commente alors,
dans un commentaire à part et exactement :

    Validé: <empreinte>

Une validation posée avant un push ne porte pas sur l'état d'après : l'empreinte
change, et le contrôle repasse au rouge de lui-même.
-->

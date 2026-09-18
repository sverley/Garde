# Témoins des harnais de A1

Un harnais qui ne sait pas rougir ne garde rien, et un harnais rouge en permanence
cesse d'être lu (§4.2). Chaque harnais porte donc sa mutation désignée et son état
attendu sur la référence.

Ce fichier est un harnais : il vit hors de l'empreinte, et l'amender n'annule aucune
validation.

| Harnais | Mutation désignée | Témoin vert |
|---|---|---|
| `manifeste` | retirer la ligne `rust-version` de `Cargo.toml` | dû au codage |
| `contrat_appel` | dans `analyser`, ignorer la valeur de `--projet` et rendre toujours le projet de la garde | dû au codage |
| `canaux` | dans `main`, écrire le message d'erreur sur la sortie standard | dû au codage |
| `separation_emplois` | poser `std::process::Command` dans un fichier de `src/juger/` | dû au codage |
| `dependances` | ajouter une dépendance à `Cargo.toml` sans lui écrire son entrée | dû au codage |
| `empreinte_couvre_le_produit` | poser un fichier sous `src/autoportes/tests/` | **acquis** |
| `surface_d_appel` | dans `porte`, chercher `--aide` avant d'analyser au lieu d'analyser d'abord | **acquis** |

## Ce qui a été constaté à l'audit

`empreinte_couvre_le_produit` est le seul harnais vert sur l'amorce : il garde une
propriété que rien ne menace encore. Sa mutation a été jouée, et elle rougit —
`src/autoportes/tests/couverture.rs` est écarté par `*/tests/*` et par `*/*/tests/*`.
Le piège est donc réel, et non supposé.

La mutation de `separation_emplois` a été jouée elle aussi : elle rougit, et la même
ligne écrite en commentaire ne rougit pas. L'effacement des commentaires fait ce
qu'il promet.

Les quatre autres sont rouges sur l'amorce, faute de produit. Leur témoin vert est
dû à la fin du codage, et leur mutation désignée sera jouée à ce moment-là : une
mutation ne prouve rien tant que le harnais n'est pas vert sans elle.

## `surface_d_appel`, ajouté après le premier audit du codage

Ce harnais est né d'un trou : les six premiers laissaient `garde --inconnue --aide`
sortir vert, sans nommer la faute. Ajouter une garde demande la même analyse que
d'en retirer une (§4.2), et les deux témoins ont donc été établis avant de le poser.

**Il sait rougir** : joué sur le code qu'il vient juger, il rougit sur les trois
défauts et les nomme — quatre options acceptées que l'aide n'annonce pas, `--aide`
que l'analyse ne connaît pas, un argument surnuméraire appelé verbe inconnu. Seul
`l_aide_seule_reste_une_reponse` y est vert, et c'est juste : demander l'aide
fonctionne.

**Il sait verdir** : une correction d'épreuve, écrite pour l'occasion puis jetée, le
rend vert en entier, et les six autres restent verts avec elle. Le harnais est donc
satisfiable, et il ne contredit aucun de ses voisins. Cette correction n'est pas
livrée : elle prouve seulement qu'une sortie existe. Sur cette base verte, la
mutation désignée a été jouée et rougit.

## Amendement pour la convention GNU

Le porteur a tranché : options longues, valeur au mot suivant sans la forme `=`,
`--` pour fermer les options. Un test a dû être **amendé**, et non seulement
ajouté — §4.2 demande pour cela la même analyse que pour en retirer un.

`une_option_privee_de_sa_valeur_ne_se_laisse_pas_masquer` exigeait que
`--projet --aide` soit un mauvais appel nommant `--projet`. En convention GNU,
c'est un appel légitime : le projet s'appelle `--aide`. Le test est remplacé par
`une_valeur_est_prise_au_mot_suivant_quel_qu_il_soit`, qui mesure la nouvelle
règle *et* garde l'ancienne inquiétude : avalée comme valeur, `--aide` n'est plus
une demande, donc rien ne passe au vert. Le trou d'origine reste tenu par
`une_faute_l_emporte_sur_une_demande_d_aide`, qui n'est pas touché.

Sept vérifications nouvelles ou amendées, rouges sur le code qu'elles jugent,
vertes sous une correction d'épreuve écrite puis jetée — les 41 vérifications le
sont alors, sans qu'aucun harnais en contredise un autre.

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

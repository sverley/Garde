<!--
Deux familles de témoins ont pris ce même nom de fichier sur deux branches : ceux
du plancher (#20) et ceux du squelette Rust (#4). Ce ne sont pas deux versions
d'un même document, ce sont deux harnais. Ils sont conservés tels quels ci-dessous,
l'un après l'autre. Où vivent les témoins quand les harnais se multiplient est une
question ouverte, et elle n'est pas tranchée par cette fusion.
-->

# Témoins des harnais de #20

Un harnais qui ne sait pas rougir ne garde rien, et un harnais rouge en permanence
cesse d'être lu (§4.2). Chaque mesure porte donc sa mutation désignée et son état
attendu sur la référence.

Ce fichier est un harnais : il vit hors de l'empreinte, et l'amender n'annule aucune
validation.

## Les mutations désignées

Deux familles, parce qu'il y a deux choses à garder. La **règle** — ce que rend
`verdict.sh` — est gardée par les projets inventés, qui portent leur propre liste de
chemins : une mutation de la règle les fait rougir. La **liste du dépôt** —
`docs/chemins.md` — est gardée par la section « liste de chemins du dépôt » : une
mutation de la liste la fait rougir, et ne touche pas aux inventés. C'est voulu, et
c'est la raison d'être des deux sections.

| Mesure | Mutation désignée | Ce qui rougit | Témoin vert |
|---|---|---|---|
| plancher par exclusion | dans `verdict.sh`, inverser le sens de l'exclusion | `catalogue-touche`, `chemin-inconnu`, `doc-simple`, `produit-seul`… | dû au codage |
| la liste est lue | dans `verdict.sh`, tout exclure sans lire la liste | les quatre cas à exigences | dû au codage |
| l'oubli fait jouer plus | dans `docs/chemins.md`, remplacer `src/*` par `*` | la liste du dépôt, sur tout ce qui est dû | dû au codage |
| le chemin est nommé | dans `verdict.sh`, rendre `plancher` sans son argument | tous les cas à exigences | dû au codage |
| un harnais appelle une validation (§4.2) | ajouter `tests/*` à `docs/chemins.md` | `tests/plancher.sh`, dans la liste du dépôt | dû au codage |
| la garde en place aussi (§6) | ajouter `.github/*` à `docs/chemins.md` | `garde.yml` et le gabarit, dans la liste du dépôt | dû au codage |
| la documentation simple, non (§4.5) | retirer `docs/application.md` de `docs/chemins.md` | `docs/application.md`, dans la liste du dépôt | dû au codage |
| la justification est exigée | dans `verdict.sh`, accepter une section vide | `sans-vm-muet` | dû au codage |
| une analyse manquante reste rouge | dans `verdict.sh`, renommer l'exigence rendue | `vm-sans-analyse` | dû au codage |
| les chemins viennent du juge | dans `garde.yml`, appeler le verdict avec `arbre/docs/chemins.md` | `chemins_pris_dans_la_base` | dû au codage |
| le contrôle demande son verdict | dans `garde.yml`, retirer l'appel | `le_controle_appelle_le_verdict` | dû au codage |
| aucune forge, aucun git | ajouter un `git rev-parse` à `verdict.sh` | `aucune_forge` | dû au codage |
| l'appel mal formé se distingue | dans `verdict.sh`, rendre 1 au lieu de 2 | `appel-mal-forme` | dû au codage |

Les témoins de la plomberie ont en outre leurs **workflows inventés**, qui ne
dépendent d'aucun codage : `conforme.yml` ne fait rougir aucune mesure,
`chemins-de-l-arbre.yml` et `sans-appel.yml` font rougir la leur et elle seule. Le
harnais sort **2** si l'un d'eux ment : une mesure qui ne sait pas rougir ne garde
rien, une mesure qui rougit sur le conforme rougira partout.

## Ce qui a été constaté à l'audit

**Le harnais sait verdir.** Une épreuve a été écrite pour l'occasion puis jetée : un
`verdict.sh` minimal, une `docs/chemins.md` reprise d'un cas inventé, et une ligne
d'appel dans `garde.yml`. Sur cette base, les trois sections passent au vert et le
harnais sort 0. Cette épreuve **n'est pas livrée** : elle prouve seulement qu'une
sortie existe, et que les exigences attendues sont atteignables — un harnais qu'on
n'a jamais vu vert peut n'être satisfaisable par rien.

**Le harnais sait rougir.** Les treize mutations ci-dessus ont été jouées **sur cette
base verte**, une à une, chacune rendue après coup. Les treize rougissent, et chacune
nomme ce qu'on attendait d'elle. Une mutation ne prouve rien tant que le harnais
n'est pas vert sans elle : c'est pourquoi elles ont été jouées là et pas sur l'état
livré.

**L'état livré est rouge, et c'est l'état attendu.** Trois rouges : `verdict.sh`
n'existe pas, `docs/chemins.md` non plus, et `garde.yml` ne demande son verdict à
personne. Ce sont les trois choses que le codage doit poser. Aucune dette n'est
masquée derrière un vert.

**Ce qui n'est pas mesuré ici, et pourquoi.** L'absence de git est vérifiée
lexicalement sur `verdict.sh`, non en rejouant les cas dans un répertoire sans dépôt.
Le contrat rend la seconde mesure superflue : la règle reçoit trois fichiers en
arguments et ne connaît pas d'arbre. Si un jour elle en reçoit un, la mesure devra
changer de forme.

**La plomberie de la forge n'est pas jouée.** `pull_request_target`, les appels
d'API, l'écriture du rapport : ni hors ligne, ni déterministes. Les deux mesures
lexicales disent que le contrôle demande son verdict et d'où il prend ses chemins ;
le reste est demandé en vérification manuelle.

## Amendement de `chemins_pris_dans_la_base`, à l'audit du codage

La mesure reprochait deux choses au contrôle produit par le codage, et les deux étaient **fausses** :
une occurrence de `verdict.sh` dans un commentaire d'en-tête, et un appel correct dont l'argument
`base/docs/chemins.md` vivait sur la ligne suivante, après une continuation `\`. Elle lisait ligne à
ligne, donc elle jugeait des fragments.

Amender une garde demande la même analyse qu'en ajouter une (§4.2). Ce qui a été établi avant de
livrer l'amendement :

- **La mesure sait toujours rougir.** Les deux mutations désignées ont été rejouées sur le contrôle
  du codage, qui est vert sans elles : appeler le verdict avec `arbre/docs/chemins.md` rougit, retirer
  l'appel rougit. Le reproche nomme désormais le texte en cause au lieu d'un numéro de ligne, qui ne
  voulait plus rien dire après recollement.
- **Les trois témoins inventés tiennent** : `conforme.yml` ne fait rougir aucune mesure,
  `chemins-de-l-arbre.yml` et `sans-appel.yml` font rougir la leur et elle seule.
- **Ce que l'amendement retire est exactement ce qui était faux.** Un commentaire ne s'exécute pas ;
  un appel coupé par un `\` reste un seul appel. Aucune violation réelle ne cesse d'être vue : une
  ligne active qui prend ses chemins dans l'arbre jugé rougit toujours, sur une ligne comme sur deux.

Un faux rouge n'est pas anodin : il force celui qui code à se contorsionner pour satisfaire une
mesure qui a tort, et c'est ainsi qu'on finit par réécrire le harnais qui gêne.

## Amendement du branchement, à l'audit du second codage

Le crochet jouait le harnais avec `> /dev/null`. Celui qui code ne voyait donc que « 14 rouge(s) »
sur l'erreur standard, et devait **ouvrir le harnais** pour savoir lesquels — précisément ce qu'on
lui demande de ne pas faire. §5 dit l'inverse : *le verdict lui vient des moments que le projet a
choisis, **avec le message d'échec**, et il n'ouvre pas le code du harnais.* Le branchement
contredisait le document.

Le crochet capture désormais la sortie et ne la montre que si le harnais rougit : le vert reste
court, le rouge arrive entier. Constaté avant de livrer : sur un arbre vert le crochet ne dit que
`· plancher` et rend 0 ; sur le même arbre avec `src/**` élargi en `**`, il rend 1 et affiche les
huit rouges nommés.

Le défaut venait de l'audit, pas du codage. Il a été relevé par celui qui codait.

---

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

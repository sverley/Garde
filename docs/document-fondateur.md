# La garde — document fondateur

Outil de contrôle macroscopique d'un projet logiciel : il protège les **engagements** du projet
contre les glissements et contre les intégrations qui leur seraient contraires. Il garantit que ce
que le projet se promet est mesurable, mesuré, et qu'aucune intégration ne passe sans que ces mesures
soient faites et lisibles.

Ce document décrit **l'outil et ce sur quoi il s'appuie** : les concepts. La façon de les mettre en
œuvre dans un environnement donné — un dépôt git, une forge, des crochets, une CI — est une
*application* de ces concepts, et ne les définit pas. Une application de référence est décrite en
annexe C, pour ne rien perdre de ce qui a été construit ; elle n'a pas autorité sur le corps.

Il est écrit comme un ensemble cohérent, sans notion de temps : on l'amende, on ne l'empile pas. La
provenance des idées est reportée en annexe.

---

## 1. Objet

**Une garde des engagements d'un projet, quels que soient ceux qui y contribuent.**

Les engagements sont ce que le projet tient pour vrai et se promet de garder : ce qu'il poursuit, ce
qu'il s'interdit, ce qui doit rester mesurablement vrai, ce que son environnement lui impose, la
façon dont il se conduit, les choix qu'il a faits. Certains de ces engagements sont
organisationnels ; beaucoup ne le sont pas.

La garde ne présume rien des contributeurs : humains ou non, un ou plusieurs, de confiance ou non.
Elle vaut pour un projet entièrement développé par des humains comme pour un projet où des agents
écrivent. Elle ne présume pas non plus d'un porteur unique : elle présume qu'à chaque intégration,
**un** humain répond.

Elle ne connaît rien du produit qu'elle garde. Tout ce qui le concerne lui est donné en données. Elle
est générique par conception, et l'abstraction ne se fait qu'à mesure qu'un second projet la
réclame : on code générique quand cela ne coûte rien, on n'abstrait pas d'avance.

**Elle n'est pas intrusive.** Un projet qui suit le formalisme est jugeable tel quel, sans rien
ajouter à son dépôt et sans qu'elle y écrive quoi que ce soit. Ce qu'elle exige, elle le porte
elle-même.

Elle **signale et refuse, elle ne décide pas**. Elle ne décide en particulier jamais de sa propre
évolution. Ce qui découle de cette limite court dans tout le document : elle ne garantit pas qu'on
l'exécute, ni lequel de ses artefacts on exécute ; elle rend visible ce qui s'en écarte. Ce sont le
processus, les droits et le jugement humain qui empêchent.

---

## 2. Vocabulaire

**Engagement** — ce que le projet tient pour vrai et se promet de garder. Les catalogues (§3) sont la
forme écrite des engagements.

**Besoin** — terme générique : ce qui motive un travail. Un besoin est *organisationnel* (il touche
aux engagements ou à la conduite du projet), *fonctionnel* (le produit), *d'outil*, ou *de
documentation*. Le type du besoin commande qui écrit son harnais, où il vit, quand il est joué.

**Harnais** — terme générique : l'ensemble de vérifications qui établit que ce qui a été produit
répond au besoin. Un harnais a une **forme** (§4.2), un **corps** localisable, et rend une des quatre
issues d'exécution.

**Garde** — l'outil décrit ici. Deux emplois à ne jamais confondre : garantir que **ce qu'elle garde
n'est pas altéré**, et garantir que **le contenu de ce qu'elle garde est respecté**. Le second, elle
le fait ; le premier, elle le constate et le signale.

**Juger** — le premier des deux emplois de l'outil : rendre un verdict sur un projet, sans rien
exécuter ni charger de ce projet. C'est cet emploi qui peut tourner sur du code non revu.

**Jouer** — le second : exécuter les harnais déclarés et rapporter leurs issues, dans un contexte où
le code est de confiance. Ce qu'il produit est une donnée d'entrée de *juger*, attachée à
l'empreinte de l'état joué.

**Tests de la garde** — dans le projet de la garde, la garde est le produit : ses harnais sont des
tests, comme pour tout produit. Ils gardent les engagements du projet garde. C'est ce que le premier
projet appelait « amorçages » ; le concept ne change pas, il change de projet.

**Harnais-cœur** — les harnais que la garde apporte avec elle pour garantir qu'un projet cible est
conforme à ses attentes : que les catalogues sont intègres, que la couverture tient, que les
contrôles se déclenchent. *Anciennement nommés « harnais autoportés » ; le nom a changé parce qu'il
décrivait leur mode de livraison plutôt que leur objet, et parce qu'il ne distinguait pas ce que la
garde apporte de ce que le projet ajoute.* Ils sont **embarqués dans l'artefact** avec leurs entrées,
intégrées au moment de la livraison. Ils ne sont donc ni copiés dans le cible, ni éditables par lui :
inamovibles par construction. Seuls la garde et ses harnais-cœur sont distribués ; les tests de la
garde ne sortent jamais de son projet.

**Harnais-garde** — l'extension des harnais-cœur, portée par le projet cible. Ils mesurent les
conditions d'application de la garde dans ce projet-ci — que l'artefact exécuté est celui qui était
prévu, que les crochets existent et appellent la garde, que la continuité tient — là où le cœur ne
peut rien exiger d'universel. Ils sont éditables par le projet, donc au registre, avec leurs témoins,
et leur amendement est un point de validation comme celui de toute garde. **Un harnais-garde ne doit
pas produire un résultat qui contredit un harnais-cœur.**

**Tests** — les harnais des besoins fonctionnels et d'outil. Ce qui découle d'une décision et se
mesure relève des tests, pas de la garde.

**Documentation simple** — la documentation non organisationnelle. Le projet peut lui donner des
harnais s'il le souhaite ; il n'y est pas tenu. La garde vérifie pour sa part qu'elle ne contredit ni
la garde ni ce qu'elle garde (§4.5).

**Catalogue** — un ensemble cohérent d'entrées identifiées, sans dimension temporelle, tenu par la
garde.

**Mesure** — ce qu'on observe et ce à quoi on conclut. Un harnais est une mesure programmée ; une
vérification manuelle est une mesure faite par un humain selon une procédure écrite. *Mesurable* ne
veut pas dire *automatisable*.

**Vérification manuelle** — une mesure humaine : ce qu'on fait, sur quoi, ce qu'on doit constater.
Demandée à une intégration, analysée, puis validée par le porteur de cette intégration. Elle ne vaut
que là où le codage est impossible : **un harnais programmable doit être codé**.

**Témoin rouge** — la preuve qu'un harnais sait échouer : une mutation désignée qui le fait rougir.
C'est une **obligation au point d'écriture**, pas un état permanent : la preuve est faite quand le
harnais est écrit ou amendé, et attestée ensuite. Elle est redue dès que le corps du harnais change.

**Témoin vert** — la preuve qu'un harnais sait passer : il est vert sur la référence du projet. Un
harnais rouge sur la référence sans dette ouverte qui l'explique n'a pas de témoin vert, et c'est un
défaut du harnais, pas un état.

**Intégration** — tout ce qui prétend faire entrer un changement dans la référence du projet. Le
terme est volontairement neutre : dans une forge c'est une demande de fusion, ailleurs autre chose.

**Porteur** — l'humain responsable d'un développement. Un projet peut en avoir plusieurs ; **chaque
intégration en a un seul**, et c'est lui qui valide. La garde n'a pas besoin de savoir qui : le
porteur est pour elle un rôle anonyme. Le porteur d'un niveau d'intégration répond de l'artefact avec
lequel il exécute la garde à ce niveau (§5).

---

## 3. Ce sur quoi la garde s'appuie : les catalogues

Huit ensembles, chacun disant à quoi il se subordonne. Tous sont tenus par la garde, y compris les
décisions : « gardé » veut dire *intègre*, pas *figé*. L'autorité requise pour éditer est un autre
axe, en dégradé.

| Catalogue | Contenu | Qui l'édite |
|---|---|---|
| **Description** | les paroles du porteur du projet, mot pour mot, jamais reformulées. **Les principes y vivent** : ce qu'on poursuit et ce qui borne. Soit ils se traduisent en invariants et en harnais, soit ce sont des intentions sans traduction concrète, et ils restent dans la description. | le porteur du projet seul |
| **Glossaire** | les termes employés, appliqués | le porteur du projet seul |
| **Invariants** | la déclinaison **mesurable** d'un principe, d'une règle, d'un usage ou d'une contrainte ; fixés par un humain | pas au détour d'un développement |
| **Usages** | des mises en application qui éclairent un choix ; durables, amendables, extensibles ; un usage peut être mis de côté | pas au détour d'un développement |
| **Contraintes** | ce que l'environnement impose ; portée : toutes les cibles par défaut, restreinte si besoin | pas au détour d'un développement |
| **Cibles** | les environnements servis ; borne le champ d'un contributeur ; une cible peut être mise de côté | pas au détour d'un développement |
| **Règles** | comment le projet se conduit | une intégration, sous conformité et validation |
| **Décisions** | comment le produit est fait | une intégration, sous conformité |

**La description est un document ou un ensemble de documents.** Un dossier n'en fait pas plusieurs
catalogues : c'est un seul catalogue réparti. Ce qui en découle est traité en §4.1.

À ces huit s'ajoute la **priorité**, qui dit ce qu'on traite maintenant. Elle porte deux sortes
d'entrées, qui répondent à la même question sans toucher aux engagements :

- une par couple **usage × cible**, qui dit si on le traite ;
- une par couple **harnais × outil**, qui dit si l'absence de l'outil est critique (§4.2).

Chaque entrée peut porter des **chemins** : modifier un fichier qui y répond impose de déclarer
l'entrée à l'intégration (§4.4). C'est tout ce que la garde a besoin de savoir du projet ; quand et
comment le projet joue ses vérifications ne la regarde pas (§5).

### Relations entre catalogues

- **Subordination.** Une décision se conforme aux invariants, aux contraintes et aux règles ; elle n'a
  aucun pouvoir sur eux. Une règle se conforme aux principes et aux autres règles ; si elle les heurte,
  elle **propose les amendements** qui rétablissent la cohérence — obligation, pas impasse.
- **La source d'un invariant n'est jamais une décision.** Un invariant engage tout le projet ; une
  décision, non. Le test est celui de la survie : si la décision était amendée et que l'invariant
  tombait, il n'engageait qu'un choix de mise en œuvre. **Une décision qui paraît fonder un invariant
  contient un principe ou une règle qu'on n'a pas extrait.**
- **Toute propriété mesurable n'est pas un invariant.** La conséquence mesurable d'une décision se
  garde par un test.
- **Chaque invariant nomme sa source et sa mesure.** Une règle, un usage ou une contrainte se cite par
  son identifiant. **Un principe se cite par un extrait, mot pour mot, de la description** : la garde
  vérifie que l'extrait s'y trouve tel quel, dans l'un quelconque de ses documents. Ni catalogue ni
  identifiant pour les principes. Un invariant sans source est orphelin ; un invariant mal renseigné
  sort rouge ; un principe sans invariant est une intention, ce qui n'est pas une faute mais se voit.
- **Une contrainte porte une ligne `Cibles :`** qui nomme les cibles qu'elle concerne ; absente, elle
  vaut pour toutes. La priorité nomme l'usage et la cible par leurs identifiants ; pour le couple
  harnais × outil, elle nomme le harnais par son entrée au registre et l'outil par le nom que la
  déclaration de ce harnais lui donne — **l'outil n'a pas de catalogue.**
- **Les usages ne se retirent jamais d'un catalogue.** Ce qu'on traite se dit par la priorité, ou par
  la mise de côté. Ne pas traiter un usage est légitime ; le rendre impossible ne l'est pas.
- **Un principe sur les usages** vaut par lui-même : aucun usage n'est le préalable ni le mode
  dégradé d'un autre ; un usage non encore outillé compte autant que les autres.
- **Une cible** ne se retire pas en passant, et les contraintes comme la priorité la nomment : son
  intégrité référentielle est gardée. Sa livraison ne l'est pas.

### Forme des catalogues

- **Sans temps.** Pas de date dans le texte, pas de renvoi « relevé par… », pas d'historique daté, pas
  de chaîne d'amendements. Une entrée nouvelle qui crée une incohérence fait amender les autres, y
  compris anciennes. L'historique appartient au système de versions.
- **Application rétroactive.** Une règle ou une décision vaut pour tout l'existant. Une décision n'a
  pas d'intention datée.
- **Identifiants attribués, jamais choisis.** C'est la garde qui les donne. L'identifiant est une
  **translittération déterministe de l'intitulé** : quelques mots normalisés, figés à la création et
  jamais recalculés si l'intitulé est amendé. Deux branches ne produisent donc le même identifiant
  que pour le même intitulé — et c'est alors exactement le doublon que §4.3 rapproche. La collision
  redevient un signal au lieu d'un accident, et aucun compteur, donc aucun état partagé, n'est
  requis. Un identifiant est **lisible — quelques mots — mais court**, unique, stable.
- **La garde édite les catalogues.** On lui donne de la prose ; soit elle passe et est consignée
  telle quelle, soit elle est **refusée, la prose refusée affichée avec la raison du refus**. La garde
  ne réécrit pas la prose ; elle ne pose que ce qui lui appartient — l'identifiant, la place de
  l'entrée dans son catalogue.
- **Une édition faite hors de la garde n'est pas interdite, elle est rejouée.** La garde compare
  l'état courant à l'état antérieur qu'on lui donne (§5), isole les entrées ajoutées, modifiées ou
  supprimées, et se rejoue sur elles comme si on les lui avait données. Ce qui passe est consigné ;
  ce qui ne passe pas est refusé et montré.
- **Lisible par un humain.** Un catalogue reste un document qu'on lit sur un petit écran.
- **Emplacement conventionnel.** Le schéma dit où la garde attend les catalogues. Un projet qui suit
  la convention n'a rien à déclarer ; un projet qui s'en écarte déclare son écart, et paie pour lui.
  Le chemin le moins cher est celui de la convention.

---

## 4. Ce que la garde garantit

### 4.1 Intégrité des catalogues

- Identifiant unique ; renvois vivants ; aucun invariant orphelin ; aucune contrainte visant une cible
  disparue ; aucune cible sans contrainte qui la couvre.
- Une entrée qui s'ouvre sur un identifiant sans en avoir la forme est **refusée en le nommant**,
  jamais lue comme une variante : casse, ponctuation voisine, mauvais catalogue, autre mise en forme.
- Un invariant qui nomme une décision pour source est **refusé** — le seul refus de promotion, parce
  qu'il est exact.
- Une entrée modifiée hors de l'outil est **rejouée** (§3) ; ce qui échoue au rejeu est refusé et
  montré.
- Deux entrées d'un même catalogue qui disent la même chose sous deux intitulés sont rapprochées
  (§4.3).
- **La description répartie sur plusieurs documents** reste un ensemble : un extrait cité par un
  invariant est cherché dans tous, et sa présence dans l'un suffit. Le même extrait présent dans
  plusieurs documents n'est pas une faute mais est signalé, au titre du doublon.
- **La priorité** ne porte qu'une entrée par couple. Pour usage × cible, les deux membres sont des
  identifiants de catalogue. Pour harnais × outil, le premier est une entrée du registre et le second
  un nom que la déclaration de ce harnais doit donner ; sinon le couple est refusé.

### 4.2 Couverture

- **Chaque invariant a un harnais, ou une vérification manuelle décrite.** Un identifiant sans
  entrée, une entrée sans mesure, un renvoi en boucle : échec.
- **Un harnais a une forme.** La forme est du code de la garde ; l'instance est de la donnée du
  projet. Deux formes au départ :
  - **commande** — un exécutable ou un script, avec son répertoire de travail. Le corps certifié est
    le fichier invoqué. Couvre tout ce qui n'est pas un cadre de test : traitement de texte, calcul,
    émulation d'un serveur ou d'un navigateur.
  - **test nommé** — un titre dans un fichier, joué par le lanceur déclaré du projet. C'est le seul
    cas qui demande l'analyse lexicale, pour savoir qu'un test est présent mais désactivé sans rien
    exécuter.

  Une forme nouvelle s'ajoute par du code dans la garde, jamais par de la configuration du cible.
- **Quatre issues d'exécution**, aucune configurée, toutes observées :
  - **passé** — code de retour nul ;
  - **n'a pas pu tourner** — un **code réservé**, fixé par la garde et non choisi par le projet. Le
    signal est ainsi volontaire : un harnais qui plante de façon imprévue rend un code quelconque,
    donc rouge. L'inattendu ne peut pas se déguiser en excuse ;
  - **rouge** — tout autre code non nul ;
  - **interrompu** — terminaison par signal, sans code de retour. Rouge, avec le signal nommé.
- **Un harnais doit exister et tourner.** Un harnais nommé est cherché parmi les vérifications
  réellement actives du fichier cité — pas dans un commentaire, pas dans une chaîne quelconque.
  Désactivé, seulement prévu, dans un groupe désactivé, ou groupe sans vérification active : il compte
  comme **absent**. Le message dit s'il est absent ou présent sans tourner.
- **Le mode strict est un argument du passage, pas une variable que chaque harnais consulte.** Un
  harnais qui ne peut pas conclure rend toujours le code réservé ; c'est la garde qui l'interprète :
  - passage ordinaire : passe si le code est nul **ou** réservé ;
  - passage strict : passe si le code est nul.

  Le harnais n'a donc rien à lire, et la condition cesse d'être déclarative : on n'a plus besoin de
  croire le projet sur parole, on observe ce que le harnais rend selon le mode.
- **La criticité d'un outil** se déclare par la priorité, couple harnais × outil : elle dit si
  l'absence de cet outil rougit dans un passage ordinaire. **Le mode strict l'emporte toujours** —
  dans le passage complet, toute absence rougit, quelle que soit la criticité déclarée. C'est ce qui
  permet de laisser la criticité au rang des décisions : elle allège un contexte dégradé, elle ne
  désarme rien là où le vert compte.
- **Un harnais seulement prévu ne garde rien.** Une ligne « à bâtir » est une dette, portée par un
  besoin ouvert, jamais une couverture.
- **Chaque harnais prouve qu'il sait rougir**, par un témoin rouge — une mutation désignée — **au
  moment où il est écrit ou amendé**, et qu'il sait verdir : il est vert sur la référence, ou une
  dette ouverte dit pourquoi il ne l'est pas. Un harnais qui n'a pas bougé depuis son intégration n'a
  rien à rejouer.
- **L'empreinte du corps dit quand le témoin est redû.** La garde compare l'empreinte du **corps** du
  harnais — pas du fichier, sinon renommer un test voisin redemanderait un témoin pour rien — entre
  l'état courant et l'état antérieur qu'on lui donne. Identiques : le témoin reste valide.
  Divergentes : il est redû, et la validation qui portait sur cet état tombe (§4.4). Aucun catalogue
  d'empreintes n'est écrit : elles sont calculées à la volée des deux côtés, donc rien à corrompre et
  rien à maintenir.
- **L'empreinte certifie le texte, pas le comportement.** Un harnais qui invoque un outil externe, un
  navigateur ou une bibliothèque du projet a un comportement qui dépend de ce que la garde ne tient
  pas. On certifie le corps ; le reste relève du projet. **Le témoin rouge reste donc le seul lien
  avec le comportement réel** : l'empreinte dit quand le redemander, jamais ce qu'il vaut.
- **Un harnais global n'est exigible que sur une tâche atomique.** Une tâche à sous-tâches est
  vérifiée par les leurs ; son harnais global s'écrit quand elles sont **fermées** — et si une
  question ressurgit, on rouvre ce qui bloque l'intégration globale. Un rouge qui dure cesse d'être un
  signal.
- **Amender ou retirer une garde** (harnais ou vérification manuelle) demande la même analyse et la
  même validation qu'en ajouter une.

### 4.3 Conformité et promotion

- Une décision est vérifiée conforme au rang supérieur ; une règle nouvelle est vérifiée conforme aux
  principes, et si elle heurte l'ensemble, l'intégration porte les amendements proposés.
- **La garde signale les promotions possibles** au même passage que le contrôle de conformité : une
  décision qui dit « toujours », « jamais », « ne doit pas » ; une décision qui en amende plusieurs ;
  un incident consigné sans règle ni harnais attaché ; une décision citée comme source d'un invariant.
- **Rapprochement** : chaque entrée ajoutée ou éditée est comparée aux entrées des rangs supérieurs
  (reclassement) et de son propre rang (doublon). **Deux seuils, deux régimes :**
  - Le **seuil exigeant**, au-dessus duquel le rapprochement est fait automatiquement et consigné,
    n'emploie que des mesures **lexicales, déterministes et explicables** — texte normalisé,
    n‑grammes de mots et de caractères, recouvrement, distance d'édition sur les intitulés. Un refus
    doit montrer sa raison (§3) : « ces deux intitulés partagent sept trigrammes » s'explique, un
    score opaque non.
  - Le **seuil lâche**, au-dessus duquel le rapprochement est seulement proposé avec son score, peut
    employer des mesures **sémantiques**. Un humain lit de toute façon, et c'est là que le lexical
    échoue le plus souvent : deux entrées qui disent la même chose sans partager un mot.

  Un rapprochement écarté se note pour ne pas revenir.
- **Le sémantique propose, le lexical décide.** Ce n'est pas l'apprentissage qui est écarté — il est
  figé dans la version de la bibliothèque, et l'inférence est une fonction pure. Ce qui est écarté,
  c'est le sémantique dans le chemin qui décide, pour deux raisons : il ne s'explique pas dans un
  refus, et il n'est reproductible qu'à un soin près (l'arithmétique flottante diffère d'une machine
  à l'autre, et §4.3 a des seuils). Un score employé près d'un seuil est arrondi ou quantifié avant
  comparaison.
- **Les mesures locales avant les modèles généraux.** Sur ce corpus, un cosinus calculé sur les
  catalogues du projet lui-même n'embarque aucun modèle, ne gonfle pas l'artefact, et connaît le
  vocabulaire du **glossaire** — qu'un modèle entraîné sur du texte général ignore. Les plongements
  figés restent une option au-dessus, quand le local montrera ses limites.
- Un jugement de conformité ne se programme pas : il est demandé comme vérification manuelle, par
  rang, dès qu'une intégration touche un catalogue.

### 4.4 Contrôle d'une intégration

Les concepts, indépendants de la forge ou du système de versions qui les met en œuvre.

- **Déclaration.** L'intégration déclare les engagements qu'elle touche et ceux dont le lien pourrait
  être masqué. Un plancher, tiré de ce qu'elle modifie comparé aux chemins des entrées, impose les
  évidents ; l'analyse ajoute le reste. Dans le doute, on demande.
- **Vérifications manuelles demandées.** Chaque vérification des engagements déclarés — et de ceux
  qui les couvrent par renvoi — est **analysée** puis **validée**. Tant qu'il en manque une,
  l'intégration est rouge. L'analyse peut être préparée par n'importe qui ; **seul le porteur de
  l'intégration valide**. Une validation faite sur délégation cite la délégation.
- **Une validation vaut pour l'état validé, et l'état validé est une empreinte.** Elle porte sur
  l'arbre restreint à ce qui compte — documentation, harnais et analyse exclus. Si l'empreinte
  change, la validation est annulée et la raison est dite ; ce qui ne la modifie pas ne l'annule pas.
  Les conditions d'annulation cessent ainsi d'être une liste de cas et deviennent une comparaison
  d'empreintes, qu'aucun chemin oublié ne contourne.
- **La garde possède le format et le verbe d'enregistrement d'une validation** ; l'application
  fournit le **support inimitable** (§5, annexe C). Aucun enregistrement n'est recevable s'il ne vient
  pas du verbe ; une validation sans enregistrement ne vaut rien.
- **Vert veut dire validé.** L'intégration ne passe au vert que lorsque chaque vérification demandée
  est analysée et validée, jamais sur leur seule présence.
- **Consignes proportionnées.** Une intégration qui ne touche que des harnais, de l'outillage ou de
  la documentation ne déclenche ni manipulation du produit ni construction lourde ; son analyse dit
  pourquoi le produit n'est pas atteint. La garde ne classe pas elle-même : c'est un jugement, écrit
  dans l'analyse, contrôlé à la validation.
- **Le rapport de qui a produit.** Il rend compte de ce qui appelle une validation humaine : pour
  chaque vérification demandée, ce que ses modifications changent, ce qu'elles ne touchent pas, ce qui
  reste à constater ; **honnête** — les écarts pris et ce qu'il n'a pas pu vérifier autant que ce qui
  marche — et **concis**. Ce rapport nourrit l'analyse, ne s'y substitue pas, ne vaut jamais
  validation.
- **L'intégration ferme son besoin.** Une discussion née en route se range avant l'intégration :
  bloquante, ce qu'elle décide entre dans l'intégration ; non bloquante, elle devient un besoin à
  part. Rien ne dort dans un fil qui va se fermer.
- **Passer outre se voit.** Une intégration malgré un rouge, ou une écriture directe dans la
  référence, est signalée et seul un humain lève le signal.
- **Rétroactivité et rouges.** Une règle nouvelle peut allumer des rouges sur l'existant ; ces rouges
  deviennent des besoins et n'empêchent pas de poser la règle. Ce qui doit être vert pour intégrer,
  c'est le harnais de l'intégration elle-même — jamais le rouge qui dirait que la règle n'a pas été
  livrée.
- **La continuité tient par le rejeu, pas par une attestation.** À chaque niveau d'intégration,
  celui qui intègre exécute la garde avec **son** artefact ; il n'a donc à croire aucun verdict venu
  d'en dessous. Un contournement local ne survit pas au passage suivant. Un projet qui fusionne sans
  rejouer perd la propriété, et c'est un trou de certification dont il répond (§5).

### 4.5 Documentation

Vérifier que la documentation simple ne contredit ni la garde ni ce qu'elle garde est **un
harnais-cœur**. Il mesure ce qui se mesure : un renvoi vers un identifiant qui n'existe plus ou qui a
été remplacé ; un document qui se présente comme normatif sans être un catalogue. Ce qui relève du
sens — un document qui décrit autrement ce qu'une règle en vigueur prescrit — n'est pas programmable
et se demande en vérification manuelle.

Un projet peut ajouter ses propres harnais sur sa documentation. Ce sont alors des harnais comme les
autres : au registre, avec leurs témoins, et leur amendement est un point de validation.

### 4.6 Ce que la garde ne sélectionne pas

La garde ne choisit ni quelles vérifications un projet joue, ni quand, ni dans quel budget : ce sont
des choix du projet (§5), et elle n'a pas besoin de connaître les familles de ses vérifications.
*Jouer* prend en argument les entrées à jouer ; c'est le projet qui décide lesquelles.

Elle n'a besoin que de trois choses sur un harnais : qu'il existe, qu'il tourne (§4.2), et que
l'outil dont il dépend, s'il en dépend, soit requis dans le passage complet.

Un harnais qui rougit en permanence, ou qui n'a pas de témoin, cesse d'être lu : ce sont des défauts
à corriger, pas des états à tolérer.

---

## 5. Modèle d'exécution

### Deux emplois, deux niveaux de confiance

Le danger n'est pas d'exécuter — c'est qu'un seul chemin fasse les deux emplois, car le jugement
hériterait alors de la confiance la plus basse.

- **`juger`** n'exécute rien et ne charge rien du projet jugé. Il analyse les vérifications
  lexicalement. C'est lui que tourne le contrôle depuis la référence, sur du code non revu. Il peut
  **lire** un rapport d'exécution qu'on lui donne ; il ne le produit pas.
- **`jouer`** exécute les harnais déclarés, dans un contexte où le code est de confiance : le poste
  du porteur, la CI du projet. Il produit un rapport attaché à l'empreinte de l'état joué. Un rapport
  dont l'empreinte ne correspond pas à l'état jugé ne vaut rien.

Le critère n'est pas d'exécuter ou non, c'est **d'où vient le corps de ce qui s'exécute**. Un harnais
livré dans l'artefact épinglé est de confiance partout, y compris en contexte privilégié : une
intégration ne peut pas le modifier, donc pas le désarmer. Un harnais venu du projet jugé ne l'est
que là où ce code est déjà de confiance.

### Ce qui vaut preuve

**Ce qui vaut preuve est ce que l'intégration n'a pas pu produire.** Le reste est du signalement.
Cette règle unique couvre l'artefact épinglé, l'enregistrement d'une validation et la continuité.

Il en découle que les trois seules formes d'ancrage sont : un **dépôt protégé** dont les droits
d'écriture font l'attestation ; une **exécution depuis la référence**, dont le fichier de contrôle
vient de la base et non de l'intégration ; un **compte de plateforme** qu'aucun contributeur ne peut
imiter. Toutes trois ont en commun d'être hors du poste du contributeur.

Un marqueur produit sur un poste n'atteste rien : celui qui veut contourner détient le binaire, le
dépôt et tout secret qu'ils contiendraient. Il peut donc **détecter l'erreur et documenter
l'intention**, jamais empêcher l'acte délibéré. C'est la limite que §1 assume, et elle vaut pour tout
ce que la garde garde, pas seulement pour son propre artefact.

### Version et artefact

- **La désignation de la version est une responsabilité du porteur**, dont la garde n'impose pas la
  forme : un dépôt protégé d'où la CI tire l'artefact, une empreinte consignée dans un fichier gardé,
  ou autre chose. C'est cette version qui s'exécute, jamais le code que l'intégration propose. Monter
  de version est une ligne visible, et c'est un point de validation.
- **Usage recommandé, non obligatoire** : consigner l'empreinte de l'artefact dans un fichier gardé
  et lui attacher un **harnais-garde** qui la compare à celle de l'exécutable en cours. Un écart
  rougit — version périmée, cache obsolète, montée oubliée. Une modification de l'empreinte
  elle-même est visible entre les deux arbres, déclarée, validée. On ne peut donc pas changer
  d'artefact sans que ça se voie, tant que la garde s'exécute — et c'est le rejeu au niveau supérieur
  (§4.4) qui garantit qu'elle s'exécute.
- **Le socle dérivé**, où une organisation recalibrerait la garde pour plusieurs projets, est noté et
  non construit : c'est un besoin à plusieurs projets, et le premier chantier n'en a qu'un. S'il
  vient, le socle est un dépôt gardé comme un dépôt, dont sort un artefact que les projets épinglent
  — et vu du cible, rien ne change.

### Ce qu'elle lit et ce qu'elle écrit

- **Deux arbres.** La garde prend l'état courant et l'état antérieur en arguments ; elle ne connaît
  pas le système de versions. La détection d'un dépôt est une **commodité d'appel** qui remplit ces
  arguments quand on ne les donne pas ; deux répertoires explicites restent toujours possibles, donc
  un projet sans versionnement reste jugeable — il perd seulement la détection de modification.
  La comparaison des deux arbres sert deux fois : le rejeu d'une édition externe (§3) et le plancher
  de déclaration (§4.4).
- **Quel état antérieur** est un choix du projet — la base de l'intégration, la dernière livraison,
  le dernier état validé —, pas de la garde, qui prend ce qu'on lui donne. **Le juge dit ce qu'il a
  supposé** : quelle référence, détectée ou donnée. Une référence tacite produirait sinon un vert
  dont on ne peut pas remonter l'origine.
- **Lecture seule sur ce qu'elle juge**, pour `juger`. Elle n'y écrit rien qui n'ait été demandé.
- **Déterministe et hors ligne.** Deux exécutions sur le même texte donnent le même verdict. Aucun
  réseau requis. Rien de ce qui sort ne dépend d'un ordre de parcours non spécifié ni d'une horloge.
  La **version des données Unicode** employée pour la normalisation est épinglée et attestée par la
  livraison : une version différente normaliserait le même texte différemment.
- **Elle écrit les catalogues, par des verbes.** Chaque catalogue a sa conduite : ajouter une décision
  vérifie sa conformité, attribue son identifiant, signale ou fait une promotion ; ajouter une règle
  applique la garde à la règle elle-même ; et ainsi de suite par rang. Pas de service web ni d'API :
  un outil en ligne de commande, appelé par le projet.
- **Elle rend un verdict, pas un avis.** Ce qui bloque est exact et court ; tout le reste est signalé
  et laissé au jugement.
- **Quand elle s'exécute n'est pas défini ici.** Chaque projet choisit les moments — avant un commit,
  avant une intégration, en continu, à la demande — et ce qu'il y joue (§4.6). Il répond des trous de
  certification que son processus crée ; ce qui est exigé, c'est que l'usage prévu ou recommandé
  préserve la continuité. Ce qui produit un changement ne lance pas le harnais lui-même : le verdict
  lui vient des moments que le projet a choisis, avec le message d'échec, et il n'ouvre pas le code du
  harnais.

### Artefact et distribution

- **Rust**, trois cibles : Windows, macOS, Linux, compilées nativement.
- **Signature ad-hoc seulement**, appliquée par l'éditeur de liens sur macOS. Pas de Developer ID,
  pas d'Authenticode : les deux barrages des plateformes — quarantaine et marque du Web — sont posés
  par le téléchargeur, et une installation en ligne de commande ne les déclenche pas.
- **Canaux en ligne de commande** : gestionnaires de paquets, paquet à dépendances natives
  optionnelles, script d'installation. La page de livraisons reste, documentée comme le chemin qui,
  lui, déclenchera les barrages.
- **Les entrées des harnais-cœur sont embarquées dans l'exécutable** au moment de la livraison, dans
  la même forme que les catalogues d'un projet — un seul parseur.
- **Règle de dépendances** : chaque dépendance du noyau est une décision justifiée. Un outil dont le
  métier est la certification n'a pas de raison de s'exempter de ce qu'il impose.

### Ce qui ne peut pas être désarmé

Le contrôle qui ne doit pas pouvoir être désarmé — validation exigée sur toute modification du
contenu gardé hors décisions — n'est qu'un cas de la version épinglée : c'est la garde de la
référence, appliquée à l'intégration, qui le rend. Une intégration ne peut pas désarmer ce qui ne
vient pas d'elle.

---

## 6. La garde évolue, elle ne se réécrit pas

Deux membres, pas d'exception :

- **Ajouter, en respectant la garde en place** : aucun de ses harnais ne rougit ; rien de plus n'est
  demandé. Les rouges nouveaux sont ceux de l'application rétroactive, sur l'existant.
- **Modifier un comportement en place** : des tests de la garde, ou des harnais-cœur, rougissent.
  C'est le critère, et il suffit : la garde s'appuie sur la comparaison des deux arbres pour savoir ce
  qui a changé, et ce savoir est programmé dans la garde elle-même.

Deux invariants de la garde, qui valent dans tous les cas et donc à tout appel, y compris à celui qui
la modifie : **tout changement de son comportement est validé manuellement**, et **expliqué puis
justifié dans l'intégration**. La validation porte sur le couple — ce qui rougit, et ce que
l'intégration a changé dans le harnais lui-même — sinon réécrire le harnais qui gêne redevient un
moyen de passer au vert.

**Un harnais-garde ne doit pas produire un résultat qui contredit un harnais-cœur.** C'est le seul
endroit où la contradiction est possible, puisque le cœur vient d'un artefact unique.

**L'auto-garde, en trois temps.** Avant la première version publiée, la garde est seulement testée,
pas gardée. À la première version publiée, son propre projet l'épingle et se garde avec elle ; les
rouges rétroactifs deviennent des besoins. Ensuite, monter de version est une ligne visible et un
point de validation, y compris pour elle-même. La version N épinglée juge des catalogues écrits
contre N+1 : c'est le décalage de n'importe quel cible, et la garde est ainsi son propre premier
utilisateur du mécanisme de montée de version. Elle ne peut donc pas exiger du schéma ce que la
version précédente ignore — c'est un de ses invariants, mesurable, gardé par un test de la garde.

La garde **propose** son évolution : les signaux de §4.3 sont son service, rendu au même passage que
la conformité. Elle ne la **décide** pas : une garde qui déciderait de sa propre évolution serait
juge et législateur.

---

## 7. Ce qui a été essayé et écarté

Gardé pour ne pas y revenir.

| Écarté | Pourquoi |
|---|---|
| Un journal pour les règles (entrées datées) | On lisait une même règle en deux endroits ; deux contributeurs ont pris le même numéro le même jour ; une règle écrivait l'avenir d'une règle. Les catalogues sont des ensembles sans temps. |
| « Règles primaires » comme catégorie | Mélange trois sources et un instrument ; donne à la garde l'autorité de ce qu'elle sert. |
| Un catalogue de principes à part de la description | Les principes sont dans la description : soit ils se traduisent en invariants et harnais, soit ce sont des intentions sans traduction. |
| Une garde « organisationnelle » et un porteur unique | La garde protège des engagements, dont certains seulement sont organisationnels ; un projet peut avoir plusieurs porteurs, une intégration un seul. |
| La garde jugée par la version que porte l'intégration | Une intégration qui affaiblit la garde passe au vert. Remplacé par la version épinglée. |
| Une exception pour « l'intégration dont le besoin est la garde » | Inutile : le membre « modifier un comportement » couvre le cas, avec validation. |
| Une forme canonique | Redondante : la comparaison des deux arbres donne déjà ce qui a changé, et le rejeu sur ces entrées dit si cela passe. |
| Un service web ou une API d'édition des catalogues | Machinerie disproportionnée ; l'outil en ligne de commande rend le même service. |
| **Le sémantique dans le chemin qui décide** | *Amendé.* Le motif d'origine — « perdrait le déterminisme et le hors-ligne » — était faux : l'apprentissage est figé dans la version de la bibliothèque, l'inférence est une fonction pure. Ce qui reste vrai est plus étroit : un score sémantique ne s'explique pas dans un refus, et n'est reproductible qu'à un soin près. Il est donc admis au seuil lâche, en proposition, jamais au seuil qui décide (§4.3). |
| Distribuer la garde en paquet, sans plus | Ne règle pas le désarmement : c'est l'origine de l'exécution qui compte, pas celle de l'outil. |
| Une branche vide dans le même dépôt pour recoder la garde | Prive de l'épreuve de réalité, finit en intégration géante invérifiable. |
| Faire arbitrer par la garde si un harnais est « programmable » | C'est un jugement ; il revient à qui audite le besoin. |
| Une classification des vérifications connue de la garde | La garde n'en a pas besoin : elle vérifie qu'un harnais existe et tourne ; ce que le projet joue et quand est son affaire. |
| Un témoin vert comme artefact à part | Le témoin vert, c'est le harnais vert sur la référence. Rien à construire. |
| Fixer dans la garde quand elle s'exécute | Chaque projet choisit ses moments, et répond de ses trous de certification. |
| Sous-tâches « vertes » comme condition du harnais global | « Fermées », et l'on rouvre si une question ressurgit. |
| Abstraire les usages pour un second projet | Pas voulu tant qu'un second projet ne le réclame pas. |
| Trois niveaux de harnais | Faux : il n'y a qu'un besoin et deux côtés ; l'intervention humaine n'est pas un niveau de plus. |
| **« Amorçage » comme catégorie** | *Précisé.* Le mot recouvrait deux choses. Les tests de la garde gardent les engagements du projet garde. Les harnais-cœur amorcent l'intégrité d'un projet cible. Ce ne sont pas les mêmes, et les deux ont désormais leur nom. |
| Un catalogue de principes, avec identifiants | Un principe se cite par un extrait mot pour mot de la description, que la garde vérifie. |
| **Des catalogues du cœur exportés dans le projet cible** | Deux jeux de même forme, un mécanisme d'export, et des entrées copiées qui se périment. Les harnais-cœur portent leurs entrées, embarquées à la livraison. |
| **Copier les entrées du cœur dans les catalogues du cible à l'initialisation** | Rend la garde intrusive : un projet conforme au formalisme ne serait plus jugeable tel quel. Les entrées du cœur sont adressables par identifiant à préfixe réservé, résolues contre l'artefact ; un identifiant disparu devient un renvoi mort, que §4.1 refuse déjà bruyamment. |
| **Un binaire régénéré par le projet cible avec ses propres réglages** | Ce qui juge viendrait du projet jugé : une intégration qui affaiblit une exigence régénère et passe au vert. L'empreinte n'y change rien — elle atteste que tout le monde exécute le même artefact, pas que cet artefact exige encore quelque chose. C'est la cohérence, pas l'intégrité. |
| **Un grain secret embarqué pour rendre un marqueur d'exécution infalsifiable** | Le secret est sur la machine de qui veut le contourner, et le binaire qui calcule le marqueur est celui dont on veut attester l'exécution. Aucun secret embarqué ne corrige ça. |
| **Un marqueur local dispensant de rejouer au niveau supérieur** | Mettrait le niveau supérieur à la merci d'un fichier écrit par le niveau inférieur, et ferait perdre la seule propriété qui ne demande de croire personne (§4.4). L'économie se fait par harnais, sur un corps et un état inchangés, jamais sur le verdict. |
| **Un code de retour négatif pour distinguer « n'a pas pu tourner »** | Non portable : POSIX tronque à 8 bits non signés, et sur Windows le négatif est déjà produit par les plantages. Remplacé par un code réservé et deux régimes d'interprétation (§4.2), strictement équivalent et vrai partout. |
| **Un catalogue de codes de retour** | Le harnais dit lui-même son problème par sa sortie, que le verdict affiche. Une auto-documentation des commandes pourra s'ajouter si l'utilité s'en voit ; elle n'enrichirait qu'un message déjà montré. |

---

## 8. Questions ouvertes

Une seule : **la montée de version du schéma**. Si le schéma change entre deux livraisons, les
catalogues d'un cible n'y sont plus conformes. C'est la garde qui édite les catalogues, donc c'est
elle qui porte la réécriture — et **initialiser et migrer sont la même opération**, l'une sur un
catalogue vide, l'autre sur un catalogue déjà écrit. Mise de côté volontairement : elle n'entre pas
dans §9, qui ne demande qu'une version épinglée. Le seul soin d'ici là est de consigner la version du
schéma, pour que la garde sache plus tard d'où elle part.

Les autres questions posées et tranchées sont reportées en annexe D.

---

## 9. Sortie du premier chantier

Le premier projet client est le projet d'origine. La garde est utilisable quand, ensemble :

1. les huit catalogues du client existent, chacun disant à quoi il se subordonne, et le tri est fait —
   ce qui ne se mesure pas reste dans la description, le reste devient invariant avec sa source et sa
   mesure ;
2. l'outil lit et écrit tous les catalogues, attribue les identifiants, refuse le cas exact, signale
   ou fait les rapprochements, rejoue une édition externe ;
3. les harnais-cœur s'exercent sur le client sans que rien n'ait été copié chez lui ;
4. chaque invariant a son harnais qui existe et tourne, avec son témoin rouge attesté à l'écriture et
   son témoin vert, ou sa mesure manuelle décrite ; aucune dette n'est masquée ;
5. `juger` et `jouer` sont séparés, le contrôle depuis la référence est en place, et le client
   désigne une version par le moyen qu'il choisit ;
6. la chaîne se démontre de bout en bout : un harnais qui sait rougir et verdir, un harnais qui ne
   peut pas tourner et le dit, une intégration qui demande ses vérifications, une validation lisible,
   un signal si l'on passe outre ;
7. les tests de la garde gardent les engagements du projet garde ; les projets fabriqués qui leur
   servent de matière ne sortent jamais de son dépôt.

---

## Annexe A — Provenance

Tout ce qui précède vient d'un premier projet, Tirelire, et de la conversation entre son porteur et
des sessions d'agent entre le 11 et le 18 septembre 2026. Les décisions D61 à D70 de ce projet, les
issues #58 à #134, `docs/description-projet.md`, `docs/glossaire.md` et `docs/objectif-0.md` en sont
les sources. Les mots du porteur qui fondent le vocabulaire et les catalogues y sont conservés mot
pour mot.

Ce que le premier projet apporte comme matière : 25 entrées de registre (20 avec harnais, 3 par
vérification manuelle, 2 par renvoi) ; 67 décisions dont plusieurs contiennent des invariants ou des
principes à extraire (D01, D19, D29, D22, D30, D08 ; D20, D40, D12, D34 ; D17, D14, D15) ; 29
fichiers d'amorçage, 6 900 lignes, qui sont la matière des tests de la garde — à réécrire ; une liste
de chemins fonctionnels réduite à deux entrées.

## Annexe B — Ce que le premier projet a mesuré

- Le crochet de pré-commit tenait 17 s, puis 20,5 s, puis, réorganisé, 5 s ; le budget est passé de
  20 à 30 s avant qu'on ne trie plutôt que d'attendre.
- Les vérifications de la garde : 20 rapides en 38 s, 5 lentes en 104 s.
- Une semaine d'outillage : 23 intégrations, 157 commits, 142 dans l'outil et ses documents, 1 dans
  le produit ; 1 300 minutes de CI sur 2 000 mensuelles ; la couverture du produit inchangée sur
  trois jours.
- L'outil et ses harnais : 11 000 lignes ; le produit qu'il gardait : 6 800 de cœur, 5 100
  d'interface.

Ces nombres ne sont pas des règles. Ils disent où le premier projet a commencé à peser. Ils donnent
aussi l'ordre de grandeur du rejeu complet — 45 s — qui est le prix à payer pour que la continuité
tienne sans attestation (§4.4).

## Annexe C — Une application de référence : git et GitHub

Ce que le premier projet a mis en œuvre pour les concepts de §4.4, §4.6 et §5. C'est une façon de
faire, pas la seule ; elle est conservée pour ne pas la redécouvrir.

**Deux arbres** : l'état courant est le répertoire de travail ; la référence par défaut est la base
de la branche courante, sur l'hypothèse qu'une fusion a été précédée d'une garde verte. L'hypothèse
est tenable parce que §4.4 signale une écriture directe dans la référence ou une intégration passée
malgré un rouge. Le juge affiche toujours la référence qu'il a retenue, détectée ou donnée.

**Intégration** : une pull request. **Déclaration** : une section de sa description, préparée par
une commande de l'outil, listant les identifiants touchés, le plancher imposé par les chemins
modifiés, et pour chaque vérification manuelle une analyse et une case « Validée ».

**Enregistrement d'une validation** : quand une édition de la description coche une case, un
workflow poste un commentaire — auteur, date, tête de la PR, branche cible — écrit par le compte de
la plateforme, qu'aucun contributeur ne peut imiter. **Annulation** : l'empreinte de l'état validé
change. Sur une offre sans protection de branche, la couleur du contrôle est le seul signal ; une
règle écrite interdit de cocher sans autorisation explicite du porteur, et l'autorisation se cite en
commentaire.

**Passer outre** : un workflow sur `push` vers `main` et sur la fermeture d'une PR ouvre une issue
étiquetée « alerte » quand la tête fusionnée n'était pas vérifiée ou quand le push est direct ; seul
un humain la ferme. Il ne distingue pas la main du porteur de celle d'un agent quand tous écrivent
avec le même compte.

**Contrôle jugé depuis la référence** : un workflow déclenché par `pull_request_target`, dont le
fichier est pris dans la branche de base, qui récupère la base dans un répertoire et y exécute
`juger` en lui désignant l'arbre de la PR ; il ne lit que des noms de fichiers et du texte, n'exécute
rien de la PR, et ses propres chemins figurent parmi ceux qu'il surveille.

**Artefact** : tiré d'un dépôt protégé où l'intégration n'écrit pas, ce qui fait l'ancrage hors du
poste. Un harnais-garde compare l'empreinte de l'exécutable en cours à celle consignée dans un
fichier gardé.

**Sélection et budgets** (choix du projet, hors garde) : les vérifications sont triées par
finalité ; la nature d'un besoin se lit aux fichiers modifiés comparés à une liste de chemins
fonctionnels (tout chemin absent est organisationnel, un oubli fait jouer plus, jamais moins) ; un
besoin fonctionnel joue les familles des paquets touchés en 30 s, un besoin organisationnel la garde
et toutes ses vérifications en 45 s ; un dépassement se signale, le suivant s'allège.

**Moments d'exécution** : pré-commit rapide (5 s après réorganisation) ; pré-push et pré-fusion
jugeant l'état commis, avec sélection par nature du besoin ; CI complète une fois par PR prête, en
mode strict où un outil manquant fait échouer ; rien sur un brouillon ; sur `main`, seuls la
construction et le déploiement. Les crochets vivent dans le dépôt, et un harnais-garde vérifie qu'ils
existent et appellent la garde — sans pouvoir attester qu'ils ont tourné, ce que seul le rejeu au
niveau supérieur établit.

**Registre** : un registre en Markdown relie chaque entrée à ses harnais, à ses vérifications
manuelles, à ses renvois et à ses dettes. La couverture lit les titres des groupes et des cas par
analyse lexicale, commentaires effacés, chaînes et expressions régulières masquées ; les marqueurs de
désactivation comptent comme absents. Témoins rouges : une mutation désignée par harnais, rejouée
quand l'empreinte du corps change.

**Alerte de quota** : 570 runs et 1 300 minutes en une semaine ; la réponse a été de réserver le
passage complet aux PR prêtes, d'annuler les runs remplacés, et de ne rien jouer sur les brouillons.

## Annexe D — Questions posées et tranchées

Reportées ici pour qu'on ne les repose pas.

- **Identifiants ou intitulés ?** Identifiants lisibles, translittération déterministe de l'intitulé, figés à la création.
- **Collision d'identifiants entre branches ?** Impossible sauf pour un même intitulé — et c'est alors le doublon que §4.3 rapproche.
- **Forme canonique ?** Non : la prose passe ou est refusée avec sa raison ; deux arbres disent ce qui a changé.
- **Détecter un comportement modifié ?** Un test de la garde ou un harnais-cœur qui rougit.
- **La documentation simple ?** Un harnais-cœur vérifie ce qui se mesure ; le sens revient à une vérification manuelle ; le projet peut ajouter ses propres harnais.
- **Portée d'une contrainte, priorité ?** Une ligne `Cibles :` ; absente, toutes. La priorité nomme usage × cible, et harnais × outil pour la criticité.
- **Seuils du rapprochement ?** Deux : exigeant et lexical pour un rapprochement automatique, lâche et ouvert au sémantique pour une proposition.
- **Un invariant sans seuil ?** Les invariants sont fixés par un humain ; mal renseigné, il sort rouge.
- **Tests dépendant d'un outil ?** Code réservé rendu par le harnais, interprété selon le mode ; criticité déclarée par la priorité ; le strict l'emporte.
- **Plusieurs porteurs ?** Un seul par intégration ; la garde ne sait pas qui.
- **Édition directe dans la référence ?** Rejeu sur les entrées modifiées.
- **Où vivent les principes ?** Dans la description, document ou dossier ; un invariant les cite par extrait mot pour mot.
- **Témoin vert ?** Le harnais vert sur la référence, ou une dette qui dit pourquoi.
- **Familles de vérifications ?** La garde n'en a pas besoin.
- **Contrôle depuis la référence ?** C'est la version épinglée appliquée à l'intégration.
- **D'autres engagements que les catalogues ?** Non.
- **La garde exécute-t-elle les harnais ?** Oui, par `jouer` ; jamais par `juger`.
- **Un harnais dans plusieurs langages ?** Oui : la forme « commande » couvre tout ce qui rend un code de retour.
- **Qui écrit l'enregistrement d'une validation ?** La garde possède le format et le verbe ; l'application fournit le support inimitable.
- **Le témoin rouge est-il permanent ?** Non : obligation à l'écriture, redue quand l'empreinte du corps change.
- **Sur quoi porte l'empreinte d'un harnais ?** Le corps, pas le fichier. Elle certifie le texte, pas le comportement.
- **Les harnais-cœur ont-ils un catalogue dans le cible ?** Non : ils portent leurs entrées, embarquées à la livraison, adressables par préfixe réservé.
- **Un projet peut-il mettre de côté une exigence du cœur ?** Non, puisqu'il ne peut pas l'éditer. Il peut en revanche exiger davantage, par ses propres invariants et harnais-garde.
- **Peut-on garantir l'exécution par un marqueur ?** Non sur un poste : ce qui vaut preuve est ce que l'intégration n'a pas pu produire. La continuité tient par le rejeu à chaque niveau.
- **Langage et distribution ?** Rust, trois cibles, signature ad-hoc, canaux en ligne de commande.

---

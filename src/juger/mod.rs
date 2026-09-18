//! **Juger** — lire un projet et en rendre un verdict.
//!
//! Rien de ce qui vit ici n'ouvre, n'exécute ni ne charge quoi que ce soit du
//! projet jugé (§5) : les vérifications s'analysent lexicalement, aucune
//! dépendance du projet n'est chargée. C'est ce qui rend sûr d'exécuter la
//! garde depuis un contexte privilégié.
//!
//! L'emploi est vide en A1, et c'est voulu : #4 demande que la séparation se
//! voie dans l'arborescence avant que quoi que ce soit l'habite, pour que le
//! jour où une exécution s'y glisse, elle se voie aussi. Le harnais
//! `separation_emplois` mesure cette absence d'exécution ; il la mesurera de
//! la même façon quand le module sera plein.
//!
//! §1 interdit d'abstraire d'avance : rien n'est posé ici en prévision.

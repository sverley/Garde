//! **Harnais autoportés** — ce que la garde emporte avec elle chez un projet cible.
//!
//! Ils garantissent qu'elle y fonctionne : qu'elle est appelée, que la
//! couverture se joue, que les contrôles se déclenchent (§2). Ils sont **livrés**,
//! et c'est ce qui les distingue des tests de la garde : ceux-ci vivent dans
//! `tests/`, hors de l'empreinte, et ne sortent jamais du projet ; ceux-là sont
//! du produit, dans l'empreinte, et les modifier annule une validation.
//!
//! Le harnais `empreinte_couvre_le_produit` garde précisément cela : un fichier
//! posé sous `src/autoportes/tests/` tomberait hors de l'empreinte, et il rougit.
//!
//! L'emploi est vide en A1.

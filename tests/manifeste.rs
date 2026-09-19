//! Le manifeste déclare ce que #4 demande : la caisse, son édition, sa version
//! minimale de Rust.
//!
//! Il mesure la *déclaration*, jamais la valeur : quelle édition et quelle
//! version sont tenables est un jugement, et il va en vérification manuelle.
//!
//! Lecture ligne à ligne, et non parseur générique : une forme restreinte qui
//! refuse ce qu'elle ne reconnaît pas sert mieux l'intégrité.
//!
//! Témoin rouge désigné : retirer la ligne `rust-version` de `Cargo.toml`.
//!
//! Source de l'exigence — ce qui l'engage, et qui n'est ni un corps d'issue,
//! ni une documentation simple, ni la parole d'une session.
//! SOURCE porteur — la scission bibliothèque + binaire, tranchée en Q2 de l'audit de #4
//!
//! Témoin rouge — la mutation qui doit le faire rougir, écrite ici pour que
//! `tests/temoins.sh` puisse l'appliquer. Une prose qui l'affirmerait ne se
//! relit pas ; ceci s'exécute.
//! TÉMOIN fichier Cargo.toml
//! TÉMOIN ancien edition = "2021"
//! TÉMOIN nouveau # edition non declaree

mod commun;

fn champ(nom: &str) -> Option<String> {
    for ligne in commun::lignes_utiles(&commun::lire("Cargo.toml")) {
        if let Some((gauche, droite)) = ligne.split_once('=') {
            if gauche.trim() == nom {
                return Some(droite.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}

#[test]
fn la_caisse_se_nomme_garde() {
    assert_eq!(
        champ("name").as_deref(),
        Some("garde"),
        "la caisse doit se nommer `garde`"
    );
}

#[test]
fn l_edition_est_declaree() {
    let edition = champ("edition").expect("`Cargo.toml` ne déclare pas d'`edition`");
    assert!(
        edition.chars().all(|c| c.is_ascii_digit()) && edition.len() == 4,
        "édition mal déclarée : {edition}"
    );
}

#[test]
fn la_bibliotheque_et_le_binaire_sont_declares() {
    let sections = commun::lignes_utiles(&commun::lire("Cargo.toml"));
    assert!(
        sections.iter().any(|l| l == "[lib]"),
        "pas de bibliothèque : le cœur ne serait atteignable que par la ligne de commande"
    );
    assert!(
        sections.iter().any(|l| l == "[[bin]]"),
        "pas de binaire déclaré"
    );
}

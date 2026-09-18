//! Chaque caisse du noyau est une décision justifiée.
//!
//! #4 pose la règle et §5 la fonde — déterministe, hors ligne, sans dépendance.
//! Sans harnais, elle resterait une phrase : ce fichier la rend mesurable dans
//! les deux sens, aucune dépendance sans entrée et aucune entrée orpheline.
//!
//! Il ne juge pas la *qualité* d'une justification : c'est une lecture, et elle
//! va en vérification manuelle.
//!
//! Témoin rouge désigné : ajouter une dépendance à `Cargo.toml` sans lui écrire
//! son entrée.

mod commun;

const DECISIONS: &str = "docs/catalogues/decisions.md";

/// Les caisses déclarées sous `[dependencies]`, lecture ligne à ligne.
fn dependances_declarees() -> Vec<String> {
    let mut caisses = Vec::new();
    let mut dedans = false;
    for ligne in commun::lignes_utiles(&commun::lire("Cargo.toml")) {
        if ligne.starts_with('[') {
            dedans = ligne == "[dependencies]"
                || ligne == "[dev-dependencies]"
                || ligne == "[build-dependencies]";
            continue;
        }
        if dedans {
            if let Some((nom, _)) = ligne.split_once('=') {
                caisses.push(nom.trim().to_string());
            }
        }
    }
    caisses.sort();
    caisses.dedup();
    caisses
}

/// Les caisses qui ont une entrée : un intitulé `### caisse : <nom>`.
fn caisses_decidees() -> Vec<String> {
    commun::lire(DECISIONS)
        .lines()
        .filter_map(|l| l.strip_prefix("### caisse :").map(|n| n.trim().to_string()))
        .collect()
}

#[test]
fn le_document_des_decisions_existe() {
    assert!(
        commun::existe(DECISIONS),
        "{DECISIONS} manque : #4 demande que l'arbitrage soit écrit, \
         et il préfigure le catalogue des décisions de la tranche B"
    );
}

#[test]
fn l_ecart_a_clap_est_ecrit() {
    let decisions = commun::lire(DECISIONS);
    assert!(
        decisions.contains("clap"),
        "l'arbitrage nommé par #4 n'est pas consigné : trancher ici, et écrire pourquoi"
    );
}

#[test]
fn aucune_dependance_sans_entree() {
    let decidees = caisses_decidees();
    let orphelines: Vec<String> = dependances_declarees()
        .into_iter()
        .filter(|caisse| !decidees.contains(caisse))
        .collect();
    assert!(
        orphelines.is_empty(),
        "caisses déclarées sans décision qui les justifie : {}",
        orphelines.join(", ")
    );
}

#[test]
fn aucune_entree_sans_dependance() {
    let declarees = dependances_declarees();
    let mortes: Vec<String> = caisses_decidees()
        .into_iter()
        .filter(|caisse| !declarees.contains(caisse))
        .collect();
    assert!(
        mortes.is_empty(),
        "décisions qui nomment une caisse absente du manifeste — un renvoi mort (§4.1) : {}",
        mortes.join(", ")
    );
}

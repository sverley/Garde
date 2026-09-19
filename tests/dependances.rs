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
//!
//! Source de l'exigence — ce qui l'engage, et qui n'est ni un corps d'issue,
//! ni une documentation simple, ni la parole d'une session.
//! SOURCE §5 — règle de dépendances : chaque dépendance du noyau est une décision justifiée
//!
//! Témoin rouge — la mutation qui doit le faire rougir, écrite ici pour que
//! `tests/temoins.sh` puisse l'appliquer. Une prose qui l'affirmerait ne se
//! relit pas ; ceci s'exécute.
//! TÉMOIN fichier Cargo.toml
//! TÉMOIN ancien [lib]
//! TÉMOIN nouveau [dependencies]
//! TÉMOIN nouveau serde = "1"
//! TÉMOIN nouveau
//! TÉMOIN nouveau [lib]

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

/// Les caisses nommées par un intitulé de troisième niveau, sous la rubrique
/// donnée.
///
/// L'intitulé est lu tel qu'un humain l'écrit — `### clap` — et non sous un
/// marqueur inventé pour la commodité du harnais. Le marqueur d'avant,
/// `### caisse : …`, n'était employé par aucune entrée : le harnais lisait donc
/// zéro entrée et passait parce qu'il n'y avait zéro dépendance. Un vert qui ne
/// mesure rien.
fn caisses_sous(rubrique: &str) -> Vec<String> {
    let mut dedans = false;
    let mut trouvees = Vec::new();
    for ligne in commun::lire(DECISIONS).lines() {
        let l = ligne.trim_end();
        if let Some(titre) = l.strip_prefix("## ") {
            dedans = titre.trim() == rubrique;
            continue;
        }
        if l.starts_with("# ") {
            dedans = false;
            continue;
        }
        if dedans {
            if let Some(nom) = l.strip_prefix("### ") {
                trouvees.push(nom.trim().to_string());
            }
        }
    }
    trouvees
}

fn caisses_retenues() -> Vec<String> {
    caisses_sous("Caisses retenues")
}

fn caisses_ecartees() -> Vec<String> {
    caisses_sous("Caisses écartées")
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
fn l_ecart_a_clap_est_lu_comme_une_entree() {
    // Ancre le lecteur sur une entrée connue. Sans elle, une lecture qui ne
    // trouve rien passerait tant que le manifeste est vide — c'est exactement
    // ce qui s'est produit, et rien ne l'a dit.
    assert!(
        caisses_ecartees().iter().any(|c| c == "clap"),
        "l'écart à `clap` n'est pas lu comme une entrée de « Caisses écartées » : \
         intitulés trouvés = {:?}",
        caisses_ecartees()
    );
}

#[test]
fn aucune_dependance_sans_entree() {
    let decidees = caisses_retenues();
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
    let mortes: Vec<String> = caisses_retenues()
        .into_iter()
        .filter(|caisse| !declarees.contains(caisse))
        .collect();
    assert!(
        mortes.is_empty(),
        "décisions qui nomment une caisse absente du manifeste — un renvoi mort (§4.1) : {}",
        mortes.join(", ")
    );
}

#[test]
fn aucune_caisse_ecartee_n_est_declaree() {
    // Une caisse écartée qui reviendrait par le manifeste serait une décision
    // contredite en silence. La bijection ne le voyait pas.
    let declarees = dependances_declarees();
    let revenues: Vec<String> = caisses_ecartees()
        .into_iter()
        .filter(|c| declarees.contains(c))
        .collect();
    assert!(
        revenues.is_empty(),
        "caisses écartées par une décision et pourtant déclarées : {}",
        revenues.join(", ")
    );
}

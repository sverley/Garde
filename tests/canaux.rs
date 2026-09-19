//! Les canaux et les codes de sortie, mesurés sur le binaire lui-même.
//!
//! §5 : elle rend un verdict, pas un avis. Le verdict va sur la sortie standard,
//! le message sur la sortie d'erreur — un appelant qui lit l'un ne doit jamais
//! recevoir l'autre.
//!
//! Le code « rouge » n'est pas mesuré ici : aucun verbe ne rend encore de
//! verdict, et un harnais qui mesurerait un comportement inexistant ne garderait
//! rien (§4.2). Il le sera avec le premier verbe qui juge.
//!
//! Témoin rouge désigné : dans `main`, écrire le message d'erreur sur la sortie
//! standard au lieu de la sortie d'erreur.
//!
//! Témoin rouge — la mutation qui doit le faire rougir, écrite ici pour que
//! `tests/temoins.sh` puisse l'appliquer. Une prose qui l'affirmerait ne se
//! relit pas ; ceci s'exécute.
//! TÉMOIN fichier src/main.rs
//! TÉMOIN ancien         eprintln!("{erreur}");
//! TÉMOIN nouveau         println!("{erreur}");

use std::process::Command;

fn lancer(arguments: &[&str]) -> (i32, String, String) {
    let issue = Command::new(env!("CARGO_BIN_EXE_garde"))
        .args(arguments)
        .output()
        .expect("le binaire doit se lancer");
    (
        issue.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&issue.stdout).to_string(),
        String::from_utf8_lossy(&issue.stderr).to_string(),
    )
}

#[test]
fn un_appel_sans_verbe_est_un_mauvais_appel() {
    let (code, standard, erreur) = lancer(&[]);
    assert_eq!(code, 2, "aucun verbe : l'outil n'a rien pu juger");
    assert!(
        standard.is_empty(),
        "rien sur la sortie standard quand il n'y a pas de verdict"
    );
    assert!(!erreur.is_empty(), "le message va sur la sortie d'erreur");
}

#[test]
fn une_option_inconnue_est_refusee_en_la_nommant() {
    let (code, standard, erreur) = lancer(&["--inconnue"]);
    assert_eq!(code, 2);
    assert!(standard.is_empty());
    assert!(
        erreur.contains("--inconnue"),
        "le message doit nommer l'option refusée, pas la deviner : {erreur}"
    );
}

#[test]
fn un_verbe_inconnu_est_refuse_en_le_nommant() {
    let (code, standard, erreur) = lancer(&["invente"]);
    assert_eq!(code, 2);
    assert!(standard.is_empty());
    assert!(erreur.contains("invente"), "message : {erreur}");
}

#[test]
fn l_aide_est_une_reponse_et_non_une_erreur() {
    let (code, standard, erreur) = lancer(&["--aide"]);
    assert_eq!(code, 0, "demander l'aide n'est pas un appel fautif");
    assert!(!standard.is_empty(), "l'aide va sur la sortie standard");
    assert!(erreur.is_empty(), "rien sur la sortie d'erreur : {erreur}");
}

#[test]
fn la_version_est_celle_du_manifeste() {
    let (code, standard, _) = lancer(&["--version"]);
    assert_eq!(code, 0);
    assert!(
        standard.contains(env!("CARGO_PKG_VERSION")),
        "la version annoncée doit être celle du manifeste : {standard}"
    );
}

#[test]
fn les_codes_de_sortie_sont_ceux_du_coeur() {
    use garde::Sortie;
    assert_eq!(Sortie::Vert.code(), 0);
    assert_eq!(Sortie::Rouge.code(), 1);
    assert_eq!(Sortie::MauvaisAppel.code(), 2);
}

//! Le contrat d'appel, mesuré sur le cœur.
//!
//! De l'extérieur, le repli de `--projet` ne s'observerait qu'à travers un verbe
//! — et aucun n'existe encore. Inventer un verbe pour rendre la règle visible
//! reviendrait à abstraire d'avance. Le cœur, lui, se mesure directement.
//!
//! Témoin rouge désigné : dans `analyser`, ignorer la valeur de `--projet` et
//! rendre toujours le projet de la garde.

use garde::appel::{analyser, Anterieur, Erreur};
use std::path::PathBuf;

fn arguments(bruts: &[&str]) -> Vec<String> {
    bruts.iter().map(|a| a.to_string()).collect()
}

#[test]
fn sans_argument_la_garde_retombe_sur_son_propre_projet() {
    let appel = analyser(&[]).expect("un appel vide est légitime");
    assert_eq!(
        appel.projet,
        std::env::current_dir().expect("répertoire courant"),
        "§5 : elle retombe sur le sien quand rien n'est donné"
    );
    assert_eq!(appel.anterieur, Anterieur::RacineDeBranche);
    assert_eq!(appel.verbe, None);
}

#[test]
fn le_projet_donne_est_retenu() {
    let appel = analyser(&arguments(&["--projet", "/un/ailleurs"])).expect("appel légitime");
    assert_eq!(appel.projet, PathBuf::from("/un/ailleurs"));
}

#[test]
fn l_anterieur_donne_en_chemin_est_un_chemin() {
    let appel = analyser(&arguments(&["--anterieur", "../reference"])).expect("appel légitime");
    assert_eq!(
        appel.anterieur,
        Anterieur::Chemin(PathBuf::from("../reference"))
    );
}

#[test]
fn l_anterieur_donne_en_empreinte_est_un_commit() {
    let appel = analyser(&arguments(&["--anterieur", "9fe1825"])).expect("appel légitime");
    assert_eq!(appel.anterieur, Anterieur::Commit("9fe1825".to_string()));
}

#[test]
fn l_empreinte_se_reconnait_sans_toucher_au_disque() {
    use garde::appel::est_empreinte_de_commit;
    assert!(est_empreinte_de_commit("9fe1825"));
    assert!(est_empreinte_de_commit(&"a".repeat(40)));
    assert!(!est_empreinte_de_commit("9fe182"), "moins de sept signes");
    assert!(
        !est_empreinte_de_commit(&"a".repeat(41)),
        "plus de quarante"
    );
    assert!(!est_empreinte_de_commit("docs"), "hors de l'hexadécimal");
    assert!(
        !est_empreinte_de_commit("./9fe1825"),
        "un chemin reste un chemin"
    );
}

#[test]
fn l_anterieur_n_est_jamais_resolu() {
    let appel = analyser(&arguments(&["--anterieur", "9fe1825"])).expect("appel légitime");
    match appel.anterieur {
        Anterieur::Commit(empreinte) => assert_eq!(
            empreinte, "9fe1825",
            "l'empreinte est portée telle quelle : la résolution appartient au verbe qui la consomme"
        ),
        autre => panic!("résolution prématurée : {autre:?}"),
    }
}

#[test]
fn une_valeur_manquante_est_refusee() {
    assert_eq!(
        analyser(&arguments(&["--projet"])),
        Err(Erreur::ValeurManquante("--projet".to_string()))
    );
}

#[test]
fn une_option_inconnue_est_refusee_en_la_nommant() {
    assert_eq!(
        analyser(&arguments(&["--inconnue"])),
        Err(Erreur::OptionInconnue("--inconnue".to_string())),
        "§4.1 : refusée en la nommant, jamais lue comme une variante"
    );
}

#[test]
fn un_verbe_est_retenu_sans_etre_execute() {
    let appel = analyser(&arguments(&["empreinte", "--projet", "/ailleurs"]))
        .expect("un verbe s'analyse même si rien ne le sert encore");
    assert_eq!(appel.verbe.as_deref(), Some("empreinte"));
}

//! L'empreinte doit couvrir tout ce qui est livré.
//!
//! Une validation porte sur l'empreinte de l'état validable. §4.4 en écarte les
//! harnais qui mesurent l'intégration — jamais le produit. Or les harnais
//! autoportés *sont* du produit : ils partent chez le client, et §6 fait de leur
//! rougissement le critère du changement de comportement. Un fichier livré qui
//! tomberait hors de l'empreinte serait modifiable sans annuler la moindre
//! validation, et rien ne le dirait.
//!
//! Le piège est réel : la liste écarte `*/tests/*`, et `src/autoportes/tests/`
//! y tomberait.
//!
//! Le motif est appliqué avec la sémantique du `case` du shell, qui est ce qui
//! décide aujourd'hui — `*` y franchit les `/`. Le harnais mesure le
//! comportement réel, pas l'intention écrite en commentaire.
//!
//! Témoin rouge désigné : poser un fichier sous `src/autoportes/tests/`.

mod commun;

const EXCLUSIONS: &str = ".garde/empreinte-exclusions";

fn motifs() -> Vec<String> {
    commun::lignes_utiles(&commun::lire(EXCLUSIONS))
}

#[test]
fn aucun_fichier_livre_n_est_ecarte() {
    let motifs = motifs();
    let mut ecartes = Vec::new();
    let mut livres = commun::fichiers("src");
    livres.push("Cargo.toml".to_string());
    for fichier in livres {
        for motif in &motifs {
            if commun::motif_couvre(motif, &fichier) {
                ecartes.push(format!("{fichier} écarté par `{motif}`"));
            }
        }
    }
    assert!(
        ecartes.is_empty(),
        "du produit hors de l'empreinte : il serait modifiable sans annuler une validation\n{}",
        ecartes.join("\n")
    );
}

#[test]
fn les_catalogues_restent_dans_l_empreinte() {
    let motifs = motifs();
    for catalogue in ["docs/document-fondateur.md", "docs/catalogues/decisions.md"] {
        for motif in &motifs {
            assert!(
                !commun::motif_couvre(motif, catalogue),
                "{catalogue} écarté par `{motif}` : on pourrait amender un catalogue \
                 sans annuler la validation qui portait dessus"
            );
        }
    }
}

#[test]
fn les_harnais_restent_hors_de_l_empreinte() {
    let motifs = motifs();
    for harnais in commun::fichiers("tests") {
        assert!(
            motifs.iter().any(|m| commun::motif_couvre(m, &harnais)),
            "{harnais} entre dans l'empreinte : le corriger annulerait une validation, \
             ce que §4.4 exclut"
        );
    }
}

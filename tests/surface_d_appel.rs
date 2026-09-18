//! La surface d'appel : ce que l'outil accepte, et ce qu'il fait d'une faute.
//!
//! Ce harnais est né d'un trou de l'audit. Les canaux étaient mesurés, la
//! reconnaissance des options aussi, mais rien ne mesurait leur *rencontre* :
//! `garde --inconnue --aide` sortait vert, sans jamais nommer la faute. Pour un
//! outil dont le métier est de rendre un verdict, un vert rendu sur un appel
//! qu'il n'a pas compris est le pire des défauts — §4.4 pose que vert veut dire
//! validé.
//!
//! Il mesure trois choses que les cinq autres laissaient passer :
//! une faute l'emporte sur une demande d'aide ; la surface acceptée est celle
//! que l'aide annonce, dans les deux sens ; un argument surnuméraire est refusé
//! sous son propre nom.
//!
//! Témoin rouge désigné : dans `porte`, chercher `--aide` dans les arguments
//! avant de les analyser, au lieu de les analyser d'abord.

mod commun;

use garde::appel::{analyser, Erreur};
use garde::Sortie;

/// Les options que l'aide annonce : le premier mot en tiret de chaque ligne.
fn options_annoncees() -> Vec<String> {
    garde::aide()
        .lines()
        .filter_map(|ligne| {
            let mot = ligne.trim().split_whitespace().next()?;
            mot.starts_with('-').then(|| mot.to_string())
        })
        .collect()
}

/// Ce qui ressemble à une option dans un littéral de chaîne.
fn est_une_option(mot: &str) -> bool {
    let corps = mot.trim_start_matches('-');
    mot.starts_with('-')
        && corps.len() + 1 <= mot.len()
        && !corps.is_empty()
        && corps
            .chars()
            .all(|s| s.is_ascii_alphanumeric() || s == '-' || s == '_')
}

/// Les options écrites en dur dans le produit, lues lexicalement.
///
/// Une liste fixe de variantes à refuser ne prouverait rien : elle ne verrait
/// que ce qu'on a pensé à y mettre. Lire le produit voit ce qui y est.
fn options_du_produit() -> Vec<String> {
    let mut trouvees = Vec::new();
    for fichier in commun::fichiers("src") {
        let source = commun::sans_commentaires(&commun::lire(&fichier));
        let signes: Vec<char> = source.chars().collect();
        let mut i = 0;
        while i < signes.len() {
            if signes[i] != '"' {
                i += 1;
                continue;
            }
            let mut contenu = String::new();
            let mut j = i + 1;
            while j < signes.len() && signes[j] != '"' {
                if signes[j] == '\\' {
                    j += 1;
                } else {
                    contenu.push(signes[j]);
                }
                j += 1;
            }
            if est_une_option(&contenu) {
                trouvees.push(contenu);
            }
            i = j + 1;
        }
    }
    trouvees.sort();
    trouvees.dedup();
    trouvees
}

#[test]
fn aucune_option_ne_repond_sans_etre_annoncee() {
    let annoncees = options_annoncees();
    let silencieuses: Vec<String> = options_du_produit()
        .into_iter()
        .filter(|option| !annoncees.contains(option))
        .collect();
    assert!(
        silencieuses.is_empty(),
        "options acceptées que l'aide n'annonce pas : {} — §4.5 : la documentation \
         ne doit pas décrire une surface plus petite que la vraie",
        silencieuses.join(", ")
    );
}

#[test]
fn toute_option_annoncee_est_reconnue_par_l_analyse() {
    for option in options_annoncees() {
        match analyser(&[option.clone()]) {
            Err(Erreur::OptionInconnue(inconnue)) => panic!(
                "{inconnue} est annoncée par l'aide mais l'analyse ne la connaît pas : \
                 l'aide et la version doivent être analysées comme le reste, non \
                 détectées avant"
            ),
            _ => continue,
        }
    }
}

#[test]
fn une_faute_l_emporte_sur_une_demande_d_aide() {
    for arguments in [
        vec!["--inconnue", "--aide"],
        vec!["--aide", "--inconnue"],
        vec!["--inconnue", "--version"],
    ] {
        let bruts: Vec<String> = arguments.iter().map(|a| a.to_string()).collect();
        let (sortie, standard, erreur) = garde::porte(&bruts);
        assert_eq!(
            sortie,
            Sortie::MauvaisAppel,
            "{arguments:?} : une faute reste une faute, même à côté d'une demande d'aide"
        );
        assert!(
            standard.is_empty(),
            "{arguments:?} : rien sur la sortie standard"
        );
        assert!(
            erreur.contains("--inconnue"),
            "{arguments:?} : la faute doit être nommée (§4.1) — reçu : {erreur}"
        );
    }
}

#[test]
fn une_option_privee_de_sa_valeur_ne_se_laisse_pas_masquer() {
    let arguments: Vec<String> = ["--projet", "--aide"]
        .iter()
        .map(|a| a.to_string())
        .collect();
    let (sortie, standard, erreur) = garde::porte(&arguments);
    assert_eq!(
        sortie,
        Sortie::MauvaisAppel,
        "une option ne prend pas une autre option pour valeur"
    );
    assert!(standard.is_empty());
    assert!(erreur.contains("--projet"), "reçu : {erreur}");
}

#[test]
fn l_aide_seule_reste_une_reponse() {
    let arguments = vec!["--aide".to_string()];
    let (sortie, standard, erreur) = garde::porte(&arguments);
    assert_eq!(sortie, Sortie::Vert, "demander l'aide n'est pas une faute");
    assert!(!standard.is_empty());
    assert!(erreur.is_empty());
}

#[test]
fn un_argument_surnumeraire_est_refuse_sous_son_propre_nom() {
    let arguments: Vec<String> = ["empreinte", "trop"]
        .iter()
        .map(|a| a.to_string())
        .collect();
    let faute = analyser(&arguments).expect_err("un second mot n'a pas de place");
    let message = faute.to_string();
    assert!(
        message.contains("trop"),
        "la faute doit nommer le mot : {message}"
    );
    assert!(
        !message.contains("verbe inconnu"),
        "`trop` n'est pas un verbe inconnu, c'est un argument en trop — §5 : exact et court"
    );
}

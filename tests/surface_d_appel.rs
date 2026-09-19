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
//!
//! Témoin rouge — la mutation qui doit le faire rougir, écrite ici pour que
//! `tests/temoins.sh` puisse l'appliquer. Une prose qui l'affirmerait ne se
//! relit pas ; ceci s'exécute.
//! TÉMOIN fichier src/lib.rs
//! TÉMOIN ancien     match appel::analyser(arguments) {
//! TÉMOIN nouveau     if arguments.iter().any(|a| a == "--aide") {
//! TÉMOIN nouveau         return (Sortie::Vert, aide(), String::new());
//! TÉMOIN nouveau     }
//! TÉMOIN nouveau     match appel::analyser(arguments) {

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
fn une_valeur_est_prise_au_mot_suivant_quel_qu_il_soit() {
    // Convention GNU, et ligne directrice 7 de POSIX : une valeur d'option peut
    // commencer par un tiret. `ls --format --help` prend `--help` pour valeur et
    // le dit. Refuser une valeur en tiret rendrait innommable un chemin qui en
    // porte un, sans que `--` puisse l'aider : `--` protège les opérandes, pas
    // les valeurs d'option.
    //
    // Le trou que ce fichier garde ne se rouvre pas pour autant : `--aide` avalé
    // par `--projet` n'est plus une demande d'aide, donc rien ne passe au vert —
    // l'appel reste sans verbe, et `porte` le refuse.
    let arguments: Vec<String> = ["--projet", "--aide"]
        .iter()
        .map(|a| a.to_string())
        .collect();
    let appel = analyser(&arguments).expect("une valeur en tiret reste une valeur");
    assert_eq!(appel.projet, std::path::PathBuf::from("--aide"));
    assert!(
        appel.demande.is_none(),
        "avalée comme valeur, `--aide` n'est plus une demande"
    );

    let (sortie, standard, _) = garde::porte(&arguments);
    assert_eq!(
        sortie,
        Sortie::MauvaisAppel,
        "sans verbe, l'appel reste refusé : aucune faute ne passe au vert"
    );
    assert!(
        standard.is_empty(),
        "et rien ne sort sur la sortie standard"
    );
}

#[test]
fn une_option_sans_valeur_en_fin_de_ligne_est_refusee() {
    // La seule valeur manquante qui reste : il n'y a plus de mot après.
    let faute = analyser(&["--projet".to_string()]).expect_err("il n'y a rien à prendre");
    assert_eq!(faute, Erreur::ValeurManquante("--projet".to_string()));
}

#[test]
fn la_forme_avec_egal_est_refusee_en_nommant_l_option() {
    // Écartée : les valeurs se donnent au mot suivant. Une forme voisine d'une
    // option connue doit être refusée en la nommant, jamais rangée parmi les
    // inconnues (§4.1) — sans quoi l'appelant relit l'aide sans y voir sa faute.
    let faute = analyser(&["--projet=/ailleurs".to_string()])
        .expect_err("la forme avec égal n'est pas servie");
    let message = faute.to_string();
    assert!(
        message.contains("--projet"),
        "l'option doit être nommée : {message}"
    );
    assert!(
        !message.contains("option inconnue"),
        "`--projet=…` n'est pas une option inconnue, c'est une forme non servie : {message}"
    );
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

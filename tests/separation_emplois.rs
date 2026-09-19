//! La séparation des emplois, rendue mesurable.
//!
//! §5 : `juger` n'ouvre, n'exécute et ne charge rien du projet jugé. C'est ce qui
//! rend sûr de lancer la garde depuis un contexte privilégié. Le plan en fait une
//! propriété de toutes les tranches — donc quelque chose qui doit se mesurer, et
//! non se promettre.
//!
//! `autoportes` est un frère de `juger` et de `jouer`, non un enfant : ce qui le
//! distingue n'est pas ce qu'il fait, c'est qu'il part chez un client.
//!
//! Témoin rouge désigné : poser `std::process::Command` dans un fichier de
//! `src/juger/`.
//!
//! Source de l'exigence — ce qui l'engage, et qui n'est ni un corps d'issue,
//! ni une documentation simple, ni la parole d'une session.
//! SOURCE §5 — `juger` n'ouvre, n'exécute et ne charge rien du projet jugé ; plan, propriété 2
//!
//! Témoin rouge — la mutation qui doit le faire rougir, écrite ici pour que
//! `tests/temoins.sh` puisse l'appliquer. Une prose qui l'affirmerait ne se
//! relit pas ; ceci s'exécute.
//! TÉMOIN fichier src/juger/mutation-temoin.rs
//! TÉMOIN nouveau pub fn lire() {
//! TÉMOIN nouveau     let _ = std::process::Command::new("git");
//! TÉMOIN nouveau }

mod commun;

/// Ce qui fait exécuter quelque chose. Lire n'en est pas : `juger` doit lire.
const PRIMITIVES_D_EXECUTION: &[&str] = &[
    "std::process",
    "Command::new",
    "process::Command",
    "libloading",
    "dlopen",
    "execvp",
    "Stdio",
];

#[test]
fn les_trois_emplois_existent_dans_l_arborescence() {
    for emploi in ["src/juger", "src/jouer", "src/autoportes"] {
        assert!(
            commun::existe(emploi),
            "{emploi} manque : la séparation doit se voir dans l'arborescence, \
             même là où rien n'est encore écrit"
        );
    }
}

#[test]
fn juger_n_execute_rien() {
    let mut fautes = Vec::new();
    for fichier in commun::fichiers("src/juger") {
        let corps = commun::sans_commentaires(&commun::lire(&fichier));
        for primitive in PRIMITIVES_D_EXECUTION {
            if corps.contains(primitive) {
                fautes.push(format!("{fichier} : {primitive}"));
            }
        }
    }
    assert!(
        fautes.is_empty(),
        "exécution du côté `juger` — c'est le vecteur que §5 ferme :\n{}",
        fautes.join("\n")
    );
}

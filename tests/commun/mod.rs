//! Outils partagés par les harnais. Aucun test ici : ce fichier ne mesure rien.

use std::path::PathBuf;

/// La racine du dépôt. Posée par cargo à la compilation : le verdict ne dépend
/// donc pas du répertoire d'où le harnais est lancé.
pub fn racine() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn lire(relatif: &str) -> String {
    let chemin = racine().join(relatif);
    std::fs::read_to_string(&chemin)
        .unwrap_or_else(|e| panic!("{} illisible : {e}", chemin.display()))
}

pub fn existe(relatif: &str) -> bool {
    racine().join(relatif).exists()
}

/// Tous les fichiers sous `depart`, en chemins relatifs à la racine, triés.
/// L'ordre est imposé : un parcours de répertoire n'en garantit aucun, et §5
/// interdit qu'un verdict en dépende.
pub fn fichiers(depart: &str) -> Vec<String> {
    let mut trouves = Vec::new();
    descendre(&racine().join(depart), &racine(), &mut trouves);
    trouves.sort();
    trouves
}

fn descendre(dossier: &std::path::Path, racine: &std::path::Path, trouves: &mut Vec<String>) {
    let Ok(entrees) = std::fs::read_dir(dossier) else {
        return;
    };
    for entree in entrees.flatten() {
        let chemin = entree.path();
        if chemin.is_dir() {
            descendre(&chemin, racine, trouves);
        } else if let Ok(relatif) = chemin.strip_prefix(racine) {
            trouves.push(relatif.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Les lignes utiles d'un fichier de configuration : commentaires retirés,
/// lignes vides écartées.
pub fn lignes_utiles(contenu: &str) -> Vec<String> {
    contenu
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Un motif de la liste d'exclusions, avec la sémantique du `case` du shell qui
/// l'applique aujourd'hui : `*` franchit les `/`. C'est le comportement réel qui
/// décide de ce qui entre dans l'empreinte, pas l'intention écrite en commentaire.
pub fn motif_couvre(motif: &str, chemin: &str) -> bool {
    let m: Vec<char> = motif.chars().collect();
    let c: Vec<char> = chemin.chars().collect();
    correspond(&m, &c)
}

fn correspond(motif: &[char], texte: &[char]) -> bool {
    match motif.first() {
        None => texte.is_empty(),
        Some('*') => (0..=texte.len()).any(|coupe| correspond(&motif[1..], &texte[coupe..])),
        Some('?') => !texte.is_empty() && correspond(&motif[1..], &texte[1..]),
        Some(signe) => {
            !texte.is_empty() && texte[0] == *signe && correspond(&motif[1..], &texte[1..])
        }
    }
}

/// Le corps d'un fichier Rust, commentaires de ligne retirés. Un harnais qui
/// lit du texte ne doit pas rougir sur une phrase écrite en commentaire (§4.2).
pub fn sans_commentaires(source: &str) -> String {
    source
        .lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

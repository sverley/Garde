//! L'analyse de la ligne de commande, et rien de plus.
//!
//! Aucune résolution ici : `--anterieur` est reconnu, jamais traduit en arbre.
//! Ce que coûte cette traduction est porté par une issue à part.

use std::path::PathBuf;

/// L'état de référence, tel qu'il est donné — jamais résolu.
#[derive(Debug, PartialEq, Eq)]
pub enum Anterieur {
    /// Un chemin vers un dossier source.
    Chemin(PathBuf),
    /// Une empreinte de commit.
    Commit(String),
    /// Rien n'est donné : la racine de la branche courante.
    RacineDeBranche,
}

/// Un appel analysé.
#[derive(Debug, PartialEq, Eq)]
pub struct Appel {
    pub verbe: Option<String>,
    pub projet: PathBuf,
    pub anterieur: Anterieur,
}

/// Ce qui rend un appel fautif.
#[derive(Debug, PartialEq, Eq)]
pub enum Erreur {
    OptionInconnue(String),
    ValeurManquante(String),
    VerbeInconnu(String),
}

impl std::fmt::Display for Erreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Exact et court (§5), et la faute est nommée : c'est ce qui permet de
        // la corriger sans relire l'aide.
        match self {
            Erreur::OptionInconnue(option) => write!(f, "option inconnue : {option}"),
            Erreur::ValeurManquante(option) => write!(f, "{option} attend une valeur"),
            Erreur::VerbeInconnu(verbe) => write!(f, "verbe inconnu : {verbe}"),
        }
    }
}

/// Reconnaît une empreinte de commit : hexadécimal, de 7 à 40 signes.
///
/// Le départage ne touche pas au système de fichiers : une analyse qui lirait
/// le disque cesserait d'être déterministe.
pub fn est_empreinte_de_commit(valeur: &str) -> bool {
    (7..=40).contains(&valeur.len()) && valeur.chars().all(|signe| signe.is_ascii_hexdigit())
}

/// Le projet sur lequel la garde retombe quand rien n'est donné : le sien (§5).
///
/// Demander au système où le processus se tient n'est pas lire le projet jugé :
/// rien du projet n'est ouvert ici. Le repli est rendu résolu plutôt que sous
/// la forme `.`, pour qu'un verbe qui l'emploiera plus tard ne dépende pas de
/// l'endroit d'où on le rappelle.
fn projet_par_defaut() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Analyse les arguments, celui du programme exclu.
pub fn analyser(arguments: &[String]) -> Result<Appel, Erreur> {
    let mut verbe: Option<String> = None;
    let mut projet: Option<PathBuf> = None;
    let mut anterieur = Anterieur::RacineDeBranche;

    let mut reste = arguments.iter();
    while let Some(argument) = reste.next() {
        match argument.as_str() {
            "--projet" => {
                let valeur = reste
                    .next()
                    .ok_or_else(|| Erreur::ValeurManquante("--projet".to_string()))?;
                projet = Some(PathBuf::from(valeur));
            }
            "--anterieur" => {
                let valeur = reste
                    .next()
                    .ok_or_else(|| Erreur::ValeurManquante("--anterieur".to_string()))?;
                // Reconnu, pas résolu : ce qui départage est la forme du mot,
                // jamais ce que le disque en dirait.
                anterieur = if est_empreinte_de_commit(valeur) {
                    Anterieur::Commit(valeur.clone())
                } else {
                    Anterieur::Chemin(PathBuf::from(valeur))
                };
            }
            option if option.starts_with('-') => {
                return Err(Erreur::OptionInconnue(option.to_string()));
            }
            mot if verbe.is_none() => verbe = Some(mot.to_string()),
            // Le verbe est déjà pris : le second mot n'en est pas un.
            mot => return Err(Erreur::VerbeInconnu(mot.to_string())),
        }
    }

    Ok(Appel {
        verbe,
        projet: projet.unwrap_or_else(projet_par_defaut),
        anterieur,
    })
}

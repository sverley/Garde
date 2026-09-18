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

/// Ce que l'appel demande de lui-même, plutôt qu'un travail sur un projet.
///
/// Portée par l'appel et non détectée avant lui : une demande d'aide reste
/// soumise à l'analyse, et une faute l'emporte sur elle.
#[derive(Debug, PartialEq, Eq)]
pub enum Demande {
    Aide,
    Version,
}

/// Un appel analysé.
#[derive(Debug, PartialEq, Eq)]
pub struct Appel {
    pub verbe: Option<String>,
    pub projet: PathBuf,
    pub anterieur: Anterieur,
    pub demande: Option<Demande>,
}

/// Ce qui rend un appel fautif.
#[derive(Debug, PartialEq, Eq)]
pub enum Erreur {
    OptionInconnue(String),
    ValeurManquante(String),
    VerbeInconnu(String),
    /// Un mot de trop : le verbe était déjà pris. Ce n'est pas un verbe inconnu.
    ArgumentEnTrop(String),
}

impl std::fmt::Display for Erreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Exact et court (§5), et la faute est nommée : c'est ce qui permet de
        // la corriger sans relire l'aide.
        match self {
            Erreur::OptionInconnue(option) => write!(f, "option inconnue : {option}"),
            Erreur::ValeurManquante(option) => write!(f, "{option} attend une valeur"),
            Erreur::VerbeInconnu(verbe) => write!(f, "verbe inconnu : {verbe}"),
            Erreur::ArgumentEnTrop(mot) => write!(f, "argument en trop : {mot}"),
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

/// Une valeur ne peut pas être une autre option : `--projet --anterieur` laisse
/// `--projet` sans valeur, il ne lui donne pas `--anterieur` pour valeur.
fn valeur<'a>(
    reste: &mut impl Iterator<Item = &'a String>,
    option: &str,
) -> Result<&'a String, Erreur> {
    match reste.next() {
        Some(valeur) if !valeur.starts_with('-') => Ok(valeur),
        _ => Err(Erreur::ValeurManquante(option.to_string())),
    }
}

/// Analyse les arguments, celui du programme exclu.
///
/// Tout passe par ici, l'aide et la version comprises : rien n'est reconnu en
/// amont de l'analyse, sans quoi une faute posée à côté d'une demande d'aide
/// passerait au vert.
pub fn analyser(arguments: &[String]) -> Result<Appel, Erreur> {
    let mut verbe: Option<String> = None;
    let mut projet: Option<PathBuf> = None;
    let mut anterieur = Anterieur::RacineDeBranche;
    let mut demande: Option<Demande> = None;

    let mut reste = arguments.iter();
    while let Some(argument) = reste.next() {
        match argument.as_str() {
            "--aide" => demande = demande.or(Some(Demande::Aide)),
            "--version" => demande = demande.or(Some(Demande::Version)),
            "--projet" => projet = Some(PathBuf::from(valeur(&mut reste, "--projet")?)),
            "--anterieur" => {
                let donne = valeur(&mut reste, "--anterieur")?;
                // Reconnu, pas résolu : ce qui départage est la forme du mot,
                // jamais ce que le disque en dirait.
                anterieur = if est_empreinte_de_commit(donne) {
                    Anterieur::Commit(donne.clone())
                } else {
                    Anterieur::Chemin(PathBuf::from(donne))
                };
            }
            option if option.starts_with('-') => {
                return Err(Erreur::OptionInconnue(option.to_string()));
            }
            mot if verbe.is_none() => verbe = Some(mot.to_string()),
            // Le verbe est déjà pris : ce mot n'en est pas un second, il est en trop.
            mot => return Err(Erreur::ArgumentEnTrop(mot.to_string())),
        }
    }

    Ok(Appel {
        verbe,
        projet: projet.unwrap_or_else(projet_par_defaut),
        anterieur,
        demande,
    })
}

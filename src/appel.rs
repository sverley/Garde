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

/// Les options servies : leur nom, ce qu'elles prennent, ce qu'elles font.
///
/// **Une seule liste**, lue par l'analyse et par l'aide. Le harnais mesure les
/// deux sens — aucune option ne répond sans être annoncée, aucune annoncée n'est
/// ignorée de l'analyse — et deux listes finiraient par diverger.
pub const OPTIONS: &[(&str, &str, &str)] = &[
    (
        "--projet",
        "<chemin>",
        "le projet à juger ; absent, le projet courant",
    ),
    (
        "--anterieur",
        "<référence>",
        "l'état de référence : un chemin, une empreinte de commit, ou rien",
    ),
    ("--aide", "", "affiche cette aide"),
    ("--version", "", "affiche la version"),
];

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
    /// `--nom=valeur` : une forme voisine d'une option servie, refusée en
    /// nommant l'option (§4.1) et non rangée parmi les inconnues.
    FormeNonServie(String),
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
            Erreur::FormeNonServie(option) => write!(
                f,
                "la forme {option}=valeur n'est pas servie : donner la valeur au mot suivant"
            ),
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

/// La valeur d'une option est le **mot suivant, quel qu'il soit** — tiret compris.
///
/// C'est la ligne directrice 7 de POSIX, que GNU ne lève pas : `ls --format --help`
/// prend `--help` pour valeur. Seule la fin de ligne prive une option de sa valeur.
fn valeur<'a>(
    reste: &mut impl Iterator<Item = &'a String>,
    option: &str,
) -> Result<&'a String, Erreur> {
    reste
        .next()
        .ok_or_else(|| Erreur::ValeurManquante(option.to_string()))
}

/// Reconnaît `--nom=valeur` dont `--nom` est une option servie, et rend ce nom.
///
/// La forme est écartée, mais elle n'est pas inconnue : la refuser sans nommer
/// l'option la rangerait parmi les fautes de frappe (§4.1).
fn forme_avec_egal(mot: &str) -> Option<String> {
    let (nom, _) = mot.split_once('=')?;
    OPTIONS
        .iter()
        .any(|(servie, _, _)| *servie == nom)
        .then(|| nom.to_string())
}

/// Analyse les arguments, celui du programme exclu.
///
/// Convention GNU : options longues, valeur au mot suivant, `--` ferme les
/// options. Tout passe par ici, l'aide et la version comprises : rien n'est
/// reconnu en amont, sans quoi une faute posée à côté d'une demande d'aide
/// passerait au vert.
pub fn analyser(arguments: &[String]) -> Result<Appel, Erreur> {
    let mut verbe: Option<String> = None;
    let mut projet: Option<PathBuf> = None;
    let mut anterieur = Anterieur::RacineDeBranche;
    let mut demande: Option<Demande> = None;
    let mut options_fermees = false;

    let mut reste = arguments.iter();
    while let Some(argument) = reste.next() {
        let mot = argument.as_str();

        if !options_fermees {
            // `--` ferme les options. Il protège les opérandes, pas les valeurs
            // d'option : consommé comme valeur au tour d'avant, il ne ferme rien.
            // Il n'est pas lui-même un opérande — un second `--`, lui, en est un.
            if mot == "--" {
                options_fermees = true;
                continue;
            }
            match mot {
                "--aide" => {
                    demande = demande.or(Some(Demande::Aide));
                    continue;
                }
                "--version" => {
                    demande = demande.or(Some(Demande::Version));
                    continue;
                }
                "--projet" => {
                    projet = Some(PathBuf::from(valeur(&mut reste, "--projet")?));
                    continue;
                }
                "--anterieur" => {
                    let donne = valeur(&mut reste, "--anterieur")?;
                    // Reconnu, pas résolu : ce qui départage est la forme du mot,
                    // jamais ce que le disque en dirait.
                    anterieur = if est_empreinte_de_commit(donne) {
                        Anterieur::Commit(donne.clone())
                    } else {
                        Anterieur::Chemin(PathBuf::from(donne))
                    };
                    continue;
                }
                _ => {}
            }
            if mot.starts_with('-') {
                return Err(match forme_avec_egal(mot) {
                    Some(nom) => Erreur::FormeNonServie(nom),
                    None => Erreur::OptionInconnue(mot.to_string()),
                });
            }
        }

        // Un opérande : le verbe, puis ce qui serait en trop.
        match verbe {
            None => verbe = Some(mot.to_string()),
            Some(_) => return Err(Erreur::ArgumentEnTrop(mot.to_string())),
        }
    }

    Ok(Appel {
        verbe,
        projet: projet.unwrap_or_else(projet_par_defaut),
        anterieur,
        demande,
    })
}

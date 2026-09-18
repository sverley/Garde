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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("codage")
    }
}

/// Reconnaît une empreinte de commit : hexadécimal, de 7 à 40 signes.
///
/// Le départage ne touche pas au système de fichiers : une analyse qui lirait
/// le disque cesserait d'être déterministe.
pub fn est_empreinte_de_commit(_valeur: &str) -> bool {
    todo!("codage")
}

/// Analyse les arguments, celui du programme exclu.
pub fn analyser(_arguments: &[String]) -> Result<Appel, Erreur> {
    todo!("codage")
}

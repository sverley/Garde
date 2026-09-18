//! La garde — le cœur.
//!
//! Le binaire n'est qu'une porte : tout ce qui se mesure vit ici, pour être
//! atteignable par un harnais sans passer par la ligne de commande.
//!
//! Amorce posée par l'audit : les signatures sont le contrat que le harnais
//! mesure ; les corps appartiennent au codage.

pub mod appel;
pub mod autoportes;
pub mod jouer;
pub mod juger;

/// Ce que rend un appel. §5 : un verdict, pas un avis.
#[derive(Debug, PartialEq, Eq)]
pub enum Sortie {
    /// Rien à signaler.
    Vert,
    /// Un engagement n'est pas tenu.
    Rouge,
    /// L'appel lui-même est fautif : l'outil n'a rien pu juger.
    MauvaisAppel,
}

impl Sortie {
    /// Le code rendu au système.
    ///
    /// Le code réservé « n'a pas pu tourner » appartient à la tranche E : il se
    /// fixe en observant ce que rendent les lanceurs, et rien ici ne l'emploie.
    pub fn code(&self) -> i32 {
        match self {
            Sortie::Vert => 0,
            Sortie::Rouge => 1,
            Sortie::MauvaisAppel => 2,
        }
    }
}

/// Exécute un appel déjà analysé et rend ce qui va sur chaque canal.
///
/// Le verdict part sur la sortie standard, le message sur la sortie d'erreur.
/// Séparer ici plutôt que dans le binaire rend la règle mesurable.
pub fn executer(appel: &appel::Appel) -> (Sortie, String, String) {
    match &appel.verbe {
        // Aucun verbe n'existe en A1 : `garde empreinte` est A2 (#5). Tout verbe
        // est donc inconnu, et c'est un mauvais appel — pas un rouge. Un rouge
        // dirait qu'un engagement n'est pas tenu ; ici l'outil n'a rien jugé.
        Some(verbe) => (
            Sortie::MauvaisAppel,
            String::new(),
            appel::Erreur::VerbeInconnu(verbe.clone()).to_string(),
        ),
        // Sans verbe, l'aide est le message de la faute, non une réponse : elle
        // part sur la sortie d'erreur et le code est non nul.
        None => (
            Sortie::MauvaisAppel,
            String::new(),
            format!("aucun verbe\n\n{}", aide()),
        ),
    }
}

/// L'aide. Exacte et courte : ce que l'outil prend, rien de plus.
pub fn aide() -> String {
    format!(
        "\
{nom} {version}

Usage : {nom} <verbe> [options]

Options :
  --projet <chemin>       le projet à juger ; absent, le projet courant
  --anterieur <référence> l'état de référence : un chemin, une empreinte de
                          commit, ou rien — la racine de la branche courante
  --aide                  affiche cette aide
  --version               affiche la version

Aucun verbe n'est encore servi.",
        nom = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
    )
}

/// La porte : de la ligne de commande brute aux deux canaux et au code.
///
/// Elle vit dans le cœur, non dans le binaire, pour être atteignable sans
/// lancer un processus. Le binaire ne fait que poser les canaux et sortir.
pub fn porte(arguments: &[String]) -> (Sortie, String, String) {
    // Demandées, l'aide et la version sont des réponses : sortie standard,
    // code vert. C'est ce qui les sépare de la même aide affichée sur une faute.
    if arguments
        .iter()
        .any(|a| a == "--aide" || a == "-a" || a == "--help" || a == "-h")
    {
        return (Sortie::Vert, aide(), String::new());
    }
    if arguments.iter().any(|a| a == "--version" || a == "-V") {
        return (
            Sortie::Vert,
            env!("CARGO_PKG_VERSION").to_string(),
            String::new(),
        );
    }

    match appel::analyser(arguments) {
        Ok(appel) => executer(&appel),
        Err(faute) => (Sortie::MauvaisAppel, String::new(), faute.to_string()),
    }
}

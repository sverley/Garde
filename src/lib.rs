//! La garde — le cœur.
//!
//! Le binaire n'est qu'une porte : tout ce qui se mesure vit ici, pour être
//! atteignable par un harnais sans passer par la ligne de commande.
//!
//! Amorce posée par l'audit : les signatures sont le contrat que le harnais
//! mesure ; les corps appartiennent au codage.

pub mod appel;

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
pub fn executer(_appel: &appel::Appel) -> (Sortie, String, String) {
    todo!("codage")
}

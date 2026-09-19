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
    // Une demande passe d'abord : elle a traversé l'analyse comme le reste, donc
    // aucune faute ne l'accompagne. Demandée, l'aide est une réponse — sortie
    // standard, code vert — et c'est ce qui la sépare de l'aide affichée sur une faute.
    match appel.demande {
        Some(appel::Demande::Aide) => return (Sortie::Vert, aide(), String::new()),
        Some(appel::Demande::Version) => {
            return (
                Sortie::Vert,
                env!("CARGO_PKG_VERSION").to_string(),
                String::new(),
            )
        }
        None => {}
    }

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

/// L'aide. Exacte et courte, et elle annonce **exactement** la surface servie :
/// elle est construite depuis `appel::OPTIONS`, la même liste que l'analyse lit.
/// Une option acceptée que l'aide tait décrirait l'outil plus petit qu'il n'est
/// (§4.5) ; une option annoncée que l'analyse ignore, plus grand.
pub fn aide() -> String {
    let large = appel::OPTIONS
        .iter()
        .map(|(nom, prend, _)| nom.len() + 1 + prend.len())
        .max()
        .unwrap_or(0);
    let options: Vec<String> = appel::OPTIONS
        .iter()
        .map(|(nom, prend, quoi)| {
            let gauche = if prend.is_empty() {
                nom.to_string()
            } else {
                format!("{nom} {prend}")
            };
            format!("  {gauche:<large$}  {quoi}")
        })
        .collect();

    format!(
        "\
{nom} {version}

Usage : {nom} [options] [--] <verbe>

Options :
{options}

`--` ferme les options : ce qui suit est un opérande.
Aucun verbe n'est encore servi.",
        nom = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        options = options.join("\n"),
    )
}

/// La porte : de la ligne de commande brute aux deux canaux et au code.
///
/// Elle analyse d'abord, sans exception : rien n'est reconnu en amont. Une
/// faute l'emporte donc sur une demande d'aide, quel que soit leur ordre.
pub fn porte(arguments: &[String]) -> (Sortie, String, String) {
    match appel::analyser(arguments) {
        Ok(appel) => executer(&appel),
        Err(faute) => (Sortie::MauvaisAppel, String::new(), faute.to_string()),
    }
}

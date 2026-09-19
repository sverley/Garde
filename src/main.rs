//! La porte. Elle analyse, appelle le cœur, pose les canaux et le code.
//!
//! Mince par construction : tout ce qui se mesure vit dans le cœur. Ce qui
//! reste ici est ce qui ne s'observe que de l'extérieur — les deux canaux et
//! le code de sortie.

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (sortie, standard, erreur) = garde::porte(&arguments);

    // Le verdict sur la sortie standard, le message sur la sortie d'erreur,
    // jamais l'inverse (§5).
    if !standard.is_empty() {
        println!("{standard}");
    }
    if !erreur.is_empty() {
        eprintln!("{erreur}");
    }

    std::process::exit(sortie.code());
}

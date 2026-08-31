pub fn slug(titre: &str) -> String {
    
    let new_string: String = titre
        .chars()
        .map(|lettre| lettre.to_ascii_lowercase()
        .to_string())
        .collect();

    new_string
}

fn main() {
    let input_text: &str = "Hello World".into();
    dbg!(&slug(&input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_lettres_passent_de_majuscule_a_minuscule() {
        let input_text: &str = "Hello World".into();
        let output_text: &str = "hello world".into();
        assert_eq!(slug(&input_text), output_text); 
    }
}
/*

Elle transforme un titre d'article en identifiant d'URL selon ces règles :

toutes les lettres passent en minuscules
les lettres et les chiffres sont conservés tels quels
tout autre caractère (espace, ponctuation, apostrophe...) devient un tiret -
jamais deux tirets consécutifs dans le résultat
pas de tiret au tout début ni à la toute fin

Résultat :

Hello World                 -> hello-world
  Rust, c'est top !         -> rust-c-est-top
Chapitre 3: les slices      -> chapitre-3-les-slices
---bonjour---               -> bonjour
!!!                         ->
*/

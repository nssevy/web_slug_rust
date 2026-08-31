pub fn slug(titre: &str) -> String {

    let mut caracteres: Vec<char> = titre
        .chars()
        .map(|c| {
            if c.is_ascii_punctuation() || c.is_ascii_whitespace() {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    
    caracteres.dedup_by(|a, b| *a == '-' && *b == '-');
    
    let new_string: String = caracteres.into_iter().collect();
    
    new_string.trim_matches('-').to_string()
}


fn main() {
    let input_text: &str = "Hello World 942 --- !! et oui".into();
    dbg!(&slug(&input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supprime_les_espaces_en_debut_de_phrase() {
        let input_text: &str = "  Rust, c'est top !".into();
        let output_text: &str = "rust-c-est-top".into();
        assert_eq!(slug(&input_text), output_text)
    }

    #[test]
    fn pas_de_tiret_au_debut_et_a_la_fin() {
        let input_text: &str = "--H,ello 88 !!World-".into();
        let output_text: &str = "h-ello-88-world".into();
        assert_eq!(slug(&input_text), output_text)
    }


    #[test]
    fn jamais_deux_tirets_consecutif() {
        let input_text: &str = "Hello World 942 --- !! et oui".into();
        let output_text: &str = "hello-world-942-et-oui".into();
        assert_eq!(slug(&input_text), output_text)
    }

    #[test]
    fn les_lettres_et_les_chiffres_sont_inchange() {
        let input_text: &str = "Hello 88 World".into();
        let output_text: &str = "hello-88-world".into();
        assert_eq!(slug(&input_text), output_text)
    }

    #[test]
    fn les_lettres_passent_de_majuscule_a_minuscule() {
        let input_text: &str = "Hello World".into();
        let output_text: &str = "hello-world".into();
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

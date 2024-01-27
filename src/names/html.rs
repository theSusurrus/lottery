use crate::names;

use std::fs::OpenOptions;
use std::io::Read;

pub struct HTML {
    source_path: String,
    source: String,
}

impl HTML {
    fn load_html(source_path: &str) -> Result<String, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(source_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    pub fn new(source_path: &str) -> Result<HTML, std::io::Error> {
        let content =  Self::load_html(source_path)?;
        Ok(
            HTML {
                source_path: source_path.to_string(),
                source: content,
            }
        )
    }
}

impl names::Provider for HTML {
    fn get_names(&self) -> Result<Vec<String>, String> {
        Ok(vec!["Jakub".to_string(), "Fiszu".to_string()])
    }
}

mod tests {
    use std::vec;

    use crate::names::Provider;

    use super::*;
    const TEST_FILEPATH: &str = "src/test_names.htm";

    #[test]
    fn file_parsing() {
        let provider_creation = HTML::new(TEST_FILEPATH);
        assert!(provider_creation.is_ok());

        let provider = provider_creation.unwrap();
        let names_parsing = provider.get_names();
        assert!(names_parsing.is_ok());

        let names = names_parsing.unwrap();
        assert_eq!(names,
            vec![
            "Andrzej Jóźwiak".to_string(),
            "Andrzej K".to_string(),
            "Artur Ceran".to_string(),
            "Artur Papierski".to_string(),
            "Bartosz Rutowicz".to_string(),
            "Bryan Gerding".to_string(),
            "Błażej Drużdż".to_string(),
            "Dominik Wróbel".to_string(),
            "Filip Józefacki".to_string(),
            "Filip Szymczak".to_string(),
            "Grzegorz Majchrzak".to_string(),
            "Grzegorz Turkowski".to_string(),
            "Iwo Kacprzak".to_string(),
            "Iza Stasiak".to_string(),
            "Jakub Dudarewicz".to_string(),
            "Jakub Walerysek".to_string(),
            "Kacper Kupisz".to_string(),
            "Karolina Grochala".to_string(),
            "Krzysztof Sitek".to_string(),
            "Lech Sokołowski".to_string(),
            "Maciej Stępiński".to_string(),
            "Marek Domanski".to_string(),
            "Mateusz Seferynowicz".to_string(),
            "Piotr Rzepecki".to_string(),
            "Rafal Grzelak".to_string(),
            "Wojciech Jurek".to_string(),
            ]
        );
    }
}
use crate::names;

use std::io::{Read, Error as IoError, ErrorKind as IoErrorKind};
use html_parser::{Dom, Node, Node::{Text, Element as Element}, Element as ElementStruct};

pub struct HtmlProvider {
    source_path: String,
}

impl HtmlProvider {
    fn load_html(source_path: &str) -> Result<String, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(source_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    pub fn new(source_path: &str) -> HtmlProvider {
        HtmlProvider {
            source_path: source_path.to_string(),
        }
    }
}

struct DomTraverser {
    top_found: bool,
    bottom_found: bool,
    names: Vec<String>,
}

impl DomTraverser {
    fn new() -> DomTraverser {
        DomTraverser {
            top_found: false,
            bottom_found: false,
            names: vec![],
        }
    }

    fn traverse_node(&mut self, node: Node) {
        const TOP_DIVIDER: &str = " ------------------------------------------------------------------------------- ";
        const BOTTOM_DIVIER: &str = " EventLink - Copyright";

        /* For every element, traverse its children nodes */
        let mut element_traverser = | element: ElementStruct | {
            for node in element.children {
                self.traverse_node(node);
                if self.bottom_found {
                    return;
                }
            }
        };

        match node {
            Text(string) => {
                /* Only Text Elements are interesting for us */
                if string.contains(TOP_DIVIDER) {
                    self.top_found = true;
                    return;
                }

                if string.contains(BOTTOM_DIVIER) {
                    self.bottom_found = true;
                    return;
                }

                if self.top_found && !self.bottom_found {
                    /* filter garbage whitespace */
                    let filtered_name =
                        string.replace("&nbsp;", "")
                              .replace("\r\n", "");
                    let trimmed_name = filtered_name.trim();
                    self.names.push(trimmed_name.to_string());
                }
            },
            /* For every node, traverse its elements */
            Element(element) => element_traverser(element),
            _ => (),
        }
    }

    fn traverse_dom(&mut self, dom: Dom) {
        for node in dom.children {
            self.traverse_node(node);
            if self.bottom_found {
                return;
            }
        }
    }
}

impl names::Provider for HtmlProvider {
    fn get_names(&self) -> Result<Vec<String>, std::io::Error> {
        let source = Self::load_html(&self.source_path)?;

        let dom = match Dom::parse(&source) {
            Ok(dom) => dom,
            Err(error) => return Err(
                IoError::new(IoErrorKind::InvalidData, error)
            ),
        };

        let mut traverser = DomTraverser::new();
        traverser.traverse_dom(dom);

        let names = traverser.names;
        if names.len() > 1 {
            // println!("HTML provider names = {:?}", names);
            Ok(names)
        } else {
            Err(IoError::new(IoErrorKind::InvalidData, "No names found in HTML"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::names::Provider;
    use super::*;

    #[test]
    fn file_parsing() {
        let provider = HtmlProvider::new("src/test_names.htm");

        let names_parsing = provider.get_names();
        assert!(names_parsing.is_ok());

        let names = names_parsing.unwrap();
        assert_eq!(names,
            vec![
                "Baltazar Brzęczyszczykiewicz".to_string(),
                "Łukasz W".to_string(),
                "Alojzy Łajza".to_string(),
                "Witold Pierdoła".to_string(),
                "Barnaba Boberek".to_string(),
                "Frida Dundersztyc".to_string(),
                "Forest Leśnik".to_string(),
                "Świnka Balbinka".to_string(),
                "Mojżesz Niemojżesz".to_string(),
                "Agata Paździoch".to_string(),
                "Mirosław Mechanicki".to_string(),
                "Jarosław Bebela".to_string(),
                "Amadeusz Bach".to_string(),
                "Ludwig van Bęc".to_string(),
                "Jacek Zięba".to_string(),
                "Grzegorz Warszawa".to_string(),
                "Jerzy Jeżyk".to_string(),
                "Dymitr Duriański".to_string(),
                "Beata  Bebela".to_string(),
                "Mirosława Studnicka".to_string(),
                "Iwan Kałasznikow".to_string(),
                "Siergiej Mąciwoda".to_string(),
                "Józef Zbagien".to_string(),
                "Michał Wiśniewski".to_string(),
                "Kamil Stolarski".to_string(),
                "Jonasz Kapusta".to_string(),
            ]
        );
    }
}
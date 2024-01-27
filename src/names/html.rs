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
        const BOTTOM_DIVIER: &str = " EventLink - Copyright © 2023 - Wizards of the Coast LLC";
    
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
            println!("HTML provider names = {:?}", names);
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
use pdf_extract;

pub mod html;

pub trait Provider {
    fn get_names(&self) -> Result<Vec<String>, std::io::Error>;
}

#[derive(Debug, Clone)]
pub struct GenericProvider {
}

impl GenericProvider {
    pub fn new() -> GenericProvider {
        GenericProvider {
        }
    }
}

impl Provider for GenericProvider {
    fn get_names(&self) -> Result<Vec<String>, std::io::Error> {
        Ok(vec![])
    }
}

pub struct PDF {
    source_path: String,
}

impl PDF {
    pub fn new(source_path: &str) -> PDF {
        PDF {
            source_path: source_path.to_string()
        }
    }
}

impl Provider for PDF {
    fn get_names(&self) -> Result<Vec<String>, std::io::Error> {
        let out = pdf_extract::extract_text(self.source_path.as_str());
    
        match out {
            Ok(out) => {
                let names_split = out.split('\n').filter(|&x| !x.is_empty()).to_owned();
    
                let mut names : Vec<String> = vec![];
                for name in names_split.into_iter() {
                    names.push(name.to_string());
                }
                Ok(names)
            },
            Err(error) =>
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
        }
    }
}

use pdf_extract;

pub trait Provider {
    fn get_names(&self) -> Result<Vec<String>, String>;
}

#[derive(Debug, Clone)]
pub struct TestProvider {
}

impl TestProvider {
    pub fn new() -> TestProvider {
        TestProvider {
        }
    }
}

impl Provider for TestProvider {
    fn get_names(&self) -> Result<Vec<String>, String> {
        Ok(vec!["Jakub".to_string(), "Fiszu".to_string()])
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
    fn get_names(&self) -> Result<Vec<String>, String> {
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
            Err(error) => Err(error.to_string()),
        }
    }
}

pub struct HTML {
    source_path: String,
}

impl HTML {
    pub fn new(source_path: &str) -> HTML {
        HTML {
            source_path: source_path.to_string()
        }
    }
}

impl Provider for HTML {
    fn get_names(&self) -> Result<Vec<String>, String> {
        Ok(vec!["Jakub".to_string(), "Fiszu".to_string()])
    }
}

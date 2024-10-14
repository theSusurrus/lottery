use crate::names;

pub struct DemoProvider {
}

impl DemoProvider {
    #[allow(dead_code)]
    pub fn new() -> DemoProvider {
        DemoProvider {}
    }
}

impl names::Provider for DemoProvider {
    fn get_names(&self) -> Result<Vec<String>, std::io::Error> {
        Ok(
            vec![
                "Test A".to_string(),
                "Test B".to_string(),
                "Test C".to_string(),
                "Test D".to_string(),
                "Test E".to_string(),
                "Test F".to_string(),
                "Test G".to_string(),
                ]
        )
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    fn get_names(provider: &impl names::Provider) -> Vec<String> {
        match provider.get_names() {
            Ok(names) => names,
            Err(_) => vec![],
        }
    }

    #[test]
    fn provider_demo() {
        let provider = DemoProvider::new();

        let names = get_names(&provider);

        assert_eq!(names,
            vec![
                "Test A".to_string(),
                "Test B".to_string(),
                "Test C".to_string(),
                "Test D".to_string(),
                "Test E".to_string(),
                "Test F".to_string(),
                "Test G".to_string(),
            ]
        );
    }
}

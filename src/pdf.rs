use pdf_extract;

pub fn get_names(path : &str) -> Result<Vec<String>, String> {
    let out = pdf_extract::extract_text(path);

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

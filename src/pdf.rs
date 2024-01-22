use pdf_extract;

pub fn get_names(path : &str) -> Vec<String> {
    let out = pdf_extract::extract_text(path).unwrap();

    println!("{out:?}");

    let names_split = out.split('\n').skip_while(|&x| !x.is_empty()).to_owned();

    let mut names : Vec<String> = vec![];
    for name in names_split.into_iter() {
        names.push(name.to_string());
    } 

    println!("{names:?}");

    names
}
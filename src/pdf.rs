use lopdf::Document;

pub fn get_names(path : &str) -> Vec<&str> {

    match Document::load(path) {
        Ok(document) => {
            let pages = document.get_pages();
            let mut texts = Vec::new();

            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;
                let text = document.extract_text(&[page_number]);
                texts.push(text.unwrap_or_default());
            }

            println!("Texts {:?}", texts);
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    let names = vec!["Kuba", "Fiszu"];

    names
}
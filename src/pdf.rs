use lopdf::Document;

pub fn get_names(path : &str) -> Vec<&str> {
    let doc = Document::load(path).unwrap();
    
    println!("{:?}", doc);

    let names = vec!["Kuba", "Fiszu"];

    names
}
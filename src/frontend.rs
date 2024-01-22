use std::fs;

fn make_name_list(names: Vec<String>) -> String {
    let mut name_list : String = "let listaGraczy = [".to_string();
        
    for name in names {
        name_list += "\"";
        name_list += name.as_str();
        name_list += "\", ";
    }
    let end_cap = "];";
    name_list += end_cap;

    name_list
}

pub fn get_frontend(names: Vec<String>) -> String {
    let html_raw = fs::read_to_string("src/frontend.html").unwrap();

    let name_list = make_name_list(names);

    let html_contents = html_raw.replace("//LISTA_GRACZY", name_list.as_str());

    // print!("{}", html_contents.to_string());

    html_contents
}
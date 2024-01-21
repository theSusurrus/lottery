use std::fs;

pub fn get_frontend() -> Result<String, std::io::Error> {
    let html_contents = fs::read_to_string("src/frontend.html");

    html_contents
}
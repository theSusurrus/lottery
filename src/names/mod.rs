pub mod html;
pub mod paste;

pub trait Provider: Send + Sync{
    fn get_names(&self) -> Result<Vec<String>, std::io::Error>;
}

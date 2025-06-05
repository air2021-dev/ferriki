use std::fs;
use std::path::PathBuf;

pub fn load_markdown(slug: &str) -> Option<String> {
    let mut path = PathBuf::from("docs");
    path.push(format!("{}.md", slug));
    
    fs::read_to_string(path).ok()
}
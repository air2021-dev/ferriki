use std::fs;
use std::path::PathBuf;

// 마크다운 문서를 읽어옴
pub fn load_markdown(slug: &str) -> Result<String, String> {
    let mut path = PathBuf::from("docs");
    path.push(format!("{}.md", slug));

    fs::read_to_string(&path).map_err(|_| format!("문서 '{}'를 찾을 수 없습니다.", slug))
}
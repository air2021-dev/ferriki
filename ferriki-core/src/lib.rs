use std::fs;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
pub struct MarkdownEntry {
    pub slug: String,
    pub title: String,
}
// 마크다운 문서를 읽어옴
pub fn load_markdown(slug: &str) -> Result<String, String> {
    let mut path = PathBuf::from("docs");
    path.push(format!("{}.md", slug));

    fs::read_to_string(&path).map_err(|_| format!("문서 '{}'를 찾을 수 없습니다.", slug))
}

pub fn list_markdown_slugs() -> Vec<MarkdownEntry> {
    let dir = fs::read_dir("docs").unwrap_or_else(|_| panic!("docs 디렉토리를 열 수 없습니다."));

    dir.filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.extension()? != "md" {
            return None;
        }

        let slug = path.file_stem()?.to_string_lossy().to_string();
        let contents = fs::read_to_string(&path).ok()?;

        let title = contents
            .lines()
            .find_map(|line| {
                if line.trim_start().starts_with('#') {
                    Some(line.trim_start().trim_start_matches('#').trim().to_string())
                }else{
                    None
                }
            })
            .unwrap_or_else(|| slug.clone());

        Some(MarkdownEntry{ slug, title})
    })
    .collect()
}
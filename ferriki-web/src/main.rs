use axum::{extract::{Path, State}, response::Html, routing::get, Router};
use ferriki_core::load_markdown;
use pulldown_cmark::{Parser, html::push_html};
use tokio::net::TcpListener;
use tera::{Tera, Context};
use std::{env, sync::Arc};
async fn wiki_handler(
    Path(slug): Path<String>,
    State(tera): State<Arc<Tera>>
) -> Html<String> {
    match load_markdown(&slug){
        Ok(md) => {
            let parser = Parser::new(&md);
            let mut html_output = String::new();
            push_html(&mut html_output, parser);

            let mut ctx = Context::new();
            ctx.insert("title", &slug);
            ctx.insert("content", &html_output);

            let rendered = tera.render("wiki.html", &ctx).unwrap();
            Html(rendered)
        },
        Err(_) => {
            let rendered = tera.render("404.html", &Context::new()).unwrap();
            Html(rendered)
        }
    }
}

fn resolve_template_path() -> String {
    let mut path = env::current_dir().expect("현재 작업 디렉토리 확인 실패");
    
    // "templates" 폴더가 나올 때까지 상위 폴더로 이동
    for _ in 0..3 {
        let try_path = path.join("templates");
        if try_path.exists(){
            return try_path.join("**/*").to_string_lossy().to_string();
        }
        path = path.parent().unwrap().to_path_buf();
    }
    
    "ferriki-web/templates/**/*".to_string()
}

#[tokio::main]
async fn main() {
    let tera_path = resolve_template_path();
    let tera = Arc::new(Tera::new(&tera_path).expect("Tera 템플릿 로딩 실패"));

    let app = Router::new()
        .route("/", get(|| async { "Hello, Ferriki!"}))
        .route("/wiki/{slug}", get(wiki_handler))
        .with_state(tera.clone());

    let addr = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(addr, app).await.unwrap();
}

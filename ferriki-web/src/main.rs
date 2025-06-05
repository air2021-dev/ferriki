use axum::{extract::Path, response::Html, routing::get, Router};
use ferriki_core::load_markdown;
use pulldown_cmark::{Parser, html::push_html};
use tokio::net::TcpListener;
async fn wiki_handler(Path(slug): Path<String>) -> Html<String> {
    match load_markdown(&slug){
        Ok(md) => {
            let parser = Parser::new(&md);
            let mut html_output = String::new();
            push_html(&mut html_output, parser);

            Html(html_output)
        },
        Err(msg) => {
            Html(format!("<h1>404 Not Found</h1><p>{}</p>", msg))
        }
    }
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async { "Hello, Ferriki!"}))
        .route("/wiki/{slug}", get(wiki_handler));

    let addr = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(addr, app).await.unwrap();
}

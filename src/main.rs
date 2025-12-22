use axum::{
    routing::{get, post},
    Router,
};
use askama::Template;

// 1. Define the Template Structure
// This links to the HTML file. Askama compiles this into Rust code!
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

// 2. Define a small HTML fragment for the dynamic part
#[derive(Template)]
#[template(source = "<p style='color: blue'>I was rendered by Rust at {{ time }}!</p>", ext = "html")]
struct ClickedTemplate {
    time: String,
}
#[derive(Template)]
#[template(source = "<div>{{text}}</div>", ext ="html")]
struct DivTemplate {
    text:String,
}

#[tokio::main]
async fn main() {
    // 3. Build the Router
    let app = Router::new()
        .route("/", get(index))
        .route("/clicked", post(clicked))
        .route("/div", get(show_div));
    println!("Listening on http://localhost:3000");

    // 4. Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the initial page load
async fn index() -> IndexTemplate {
    IndexTemplate
}

// Handler for the HTMX click
async fn clicked() -> ClickedTemplate {
    let now = std::time::SystemTime::now();
    let time_str = format!("{:?}", now); // Simplified time string for demo
    
    ClickedTemplate { time: time_str }
}

async fn show_div() -> DivTemplate{
    let message:String = "Hello API".to_string();

    DivTemplate{ text: message }
}
use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use crate::discovery::DiscoveryStore;
use std::sync::Arc;

pub async fn index(store: web::Data<Arc<DiscoveryStore>>) -> ActixResult<HttpResponse> {
    let announcements = store.discover("");
    let mut html = "<html><body><h1>theskillbay Skills</h1><ul>".to_string();
    for ann in announcements {
        let name = ann.metadata.get("name").unwrap_or(&ann.skill_id);
        html.push_str(&format!("<li>{} - {} (Score: {})</li>", name, ann.skill_id, ann.reputation.score));
    }
    html.push_str("</ul></body></html>");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn run_web_server(store: Arc<DiscoveryStore>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(store.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
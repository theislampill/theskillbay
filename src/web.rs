use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use crate::discovery::DiscoveryStore;
use crate::models::{ReviewRecord, SubmittedPatch};
use std::sync::Arc;
use serde::Deserialize;

#[derive(Deserialize)]
struct ReviewForm {
    rating: u8,
    comment: String,
}

#[derive(Deserialize)]
struct PatchForm {
    description: String,
    diff: String,
}

pub async fn index(store: web::Data<Arc<DiscoveryStore>>) -> ActixResult<HttpResponse> {
    let announcements = store.discover("");
    let mut html = "<html><body><h1>theskillbay Skills</h1><ul>".to_string();
    for ann in announcements {
        let name = ann.metadata.get("name").unwrap_or(&ann.skill_id);
        html.push_str(&format!(
            "<li>{} - {} (Score: {}) <a href='/review/{}'>Review</a> <a href='/patch/{}'>Patch</a></li>",
            name,
            ann.skill_id,
            ann.reputation.score,
            ann.skill_id,
            ann.skill_id
        ));
    }
    html.push_str("</ul></body></html>");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn review_form(path: web::Path<String>) -> ActixResult<HttpResponse> {
    let skill_id = path.into_inner();
    let html = format!(
        r#"<html><body>
        <h1>Review Skill: {}</h1>
        <form action="/review/{}" method="post">
            Rating (1-5): <input type="number" name="rating" min="1" max="5" required><br>
            Comment: <textarea name="comment" required></textarea><br>
            <input type="submit" value="Submit Review">
        </form>
        </body></html>"#,
        skill_id, skill_id
    );
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn submit_review(
    path: web::Path<String>,
    form: web::Form<ReviewForm>,
    store: web::Data<Arc<DiscoveryStore>>,
) -> ActixResult<HttpResponse> {
    let skill_id = path.into_inner();
    let review = ReviewRecord {
        skill_id: skill_id.clone(),
        rating: form.rating,
        comment: form.comment.clone(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
    };
    if let Err(e) = store.add_review(review) {
        return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)));
    }
    Ok(HttpResponse::Ok().body("Review submitted successfully!"))
}

pub async fn patch_form(path: web::Path<String>) -> ActixResult<HttpResponse> {
    let skill_id = path.into_inner();
    let html = format!(
        r#"<html><body>
        <h1>Submit Patch for Skill: {}</h1>
        <form action="/patch/{}" method="post">
            Description: <input type="text" name="description" required><br>
            Diff: <textarea name="diff" required></textarea><br>
            <input type="submit" value="Submit Patch">
        </form>
        </body></html>"#,
        skill_id, skill_id
    );
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn submit_patch(
    path: web::Path<String>,
    form: web::Form<PatchForm>,
    store: web::Data<Arc<DiscoveryStore>>,
) -> ActixResult<HttpResponse> {
    let skill_id = path.into_inner();
    let patch = SubmittedPatch {
        skill_id: skill_id.clone(),
        description: form.description.clone(),
        diff: form.diff.clone(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
    };
    if let Err(e) = store.add_submitted_patch(patch) {
        return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)));
    }
    Ok(HttpResponse::Ok().body("Patch submitted successfully!"))
}

pub async fn run_web_server(store: Arc<DiscoveryStore>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(store.clone()))
            .route("/", web::get().to(index))
            .route("/review/{skill_id}", web::get().to(review_form))
            .route("/review/{skill_id}", web::post().to(submit_review))
            .route("/patch/{skill_id}", web::get().to(patch_form))
            .route("/patch/{skill_id}", web::post().to(submit_patch))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
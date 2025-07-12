use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize)]
struct ProxyRequest {
    url: String,
}

#[derive(Serialize)]
struct ProxyResponse {
    html: String,
    status: u16,
}

async fn proxy_handler(req: web::Json<ProxyRequest>) -> Result<HttpResponse> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()
        .unwrap();

    match client.get(&req.url).send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            match response.text().await {
                Ok(html) => {
                    let proxy_response = ProxyResponse {
                        html,
                        status,
                    };
                    Ok(HttpResponse::Ok().json(proxy_response))
                }
                Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to read response body"
                })))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Failed to fetch the URL"
        })))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🦀 Proxy Server starting on http://localhost:8888");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .route("/proxy", web::post().to(proxy_handler))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}

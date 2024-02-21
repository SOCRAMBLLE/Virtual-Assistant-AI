use actix_web::{web, HttpResponse, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct OAuthCallbackParams {
    code: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::get().to(login));
    cfg.route("/auth/google", web::get().to(google_login));
    cfg.route("/auth/google/callback", web::get().to(login_callback));
}

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login Page")
}

async fn google_login() -> impl Responder {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");
    let auth_url = format!("https://accounts.google.com/o/oauth2/auth?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email%20profile&access_type=offline", client_id, redirect_uri);

    HttpResponse::TemporaryRedirect()
        .header("Location", auth_url)
        .finish()
}

async fn login_callback(query: web::Query<OAuthCallbackParams>) -> impl Responder {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");

    let client = Client::new();
    let params = [
        ("code", query.code.as_str()),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("redirect_uri", &redirect_uri),
        ("grant_type", "authorization_code"),
    ];
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await;

    match res {
        Ok(response) => {
            let token_res: serde_json::Value = response.json().await.unwrap();
            HttpResponse::Ok().json(token_res)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to exchange code for token."),
    }
}

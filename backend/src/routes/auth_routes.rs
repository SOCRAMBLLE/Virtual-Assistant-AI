use actix_web::{web, HttpResponse, Responder};
use google_authenticator::GoogleAuthenticator;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login));
    cfg.route("/auth/google", web::get().to(google_login)); // Adicione a rota para login com o Google
                                                            // cfg.route("/auth/google/callback", web::get().to(login_callback));

    // Adicione mais rotas de autenticação conforme necessário
}

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login successful")
}

async fn google_login() -> impl Responder {
    // Redirecionar o usuário para a página de login do Google
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://accounts.google.com/o/oauth2/auth?response_type=code&client_id=SEU_CLIENT_ID&redirect_uri=SEU_REDIRECT_URI&scope=openid%20email%20profile")
        .finish()
}

// // Rota para lidar com o retorno do login do Google
// async fn login_callback(query: web::Query<OAuthCallbackParams>) -> impl Responder {
//     // Obter o código de autorização do Google
//     let code = query.code.clone();

//     // Trocar o código de autorização por tokens de acesso usando as credenciais do seu aplicativo
//     let google_auth = GoogleAuthenticator::new();
//     let tokens = google_auth
//         .exchange_code(&code, "SEU_REDIRECT_URI")
//         .await
//         .unwrap();

//     // Extrair informações do usuário do token de acesso
//     let user_info = google_auth
//         .get_user_info(&tokens.access_token)
//         .await
//         .unwrap();

//     // Exemplo: Exibir informações do usuário
//     HttpResponse::Ok().body(format!("Usuário autenticado: {:?}", user_info))
// }

// Parâmetros da consulta de retorno do Google OAuth
#[derive(serde::Deserialize)]
struct OAuthCallbackParams {
    code: String,
}

/* use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use google_authenticator::GoogleAuthenticator;

// Rota para iniciar o processo de login
async fn login_start() -> impl Responder {
    // Redirecionar o usuário para a página de login do Google
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://accounts.google.com/o/oauth2/auth?response_type=code&client_id=SEU_CLIENT_ID&redirect_uri=SEU_REDIRECT_URI&scope=openid%20email%20profile")
        .finish()
}

// Rota para lidar com o retorno do login do Google
async fn login_callback(query: web::Query<OAuthCallbackParams>) -> impl Responder {
    // Obter o código de autorização do Google
    let code = query.code.clone();

    // Trocar o código de autorização por tokens de acesso usando as credenciais do seu aplicativo
    let google_auth = GoogleAuthenticator::new("SEU_CLIENT_ID", "SEU_CLIENT_SECRET");
    let tokens = google_auth.exchange_code(&code, "SEU_REDIRECT_URI").await.unwrap();

    // Extrair informações do usuário do token de acesso
    let user_info = google_auth.get_user_info(&tokens.access_token).await.unwrap();

    // Exemplo: Exibir informações do usuário
    HttpResponse::Ok().body(format!("Usuário autenticado: {:?}", user_info))
}

// Parâmetros da consulta de retorno do Google OAuth
#[derive(serde::Deserialize)]
struct OAuthCallbackParams {
    code: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login_start))
            .route("/login/callback", web::get().to(login_callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 */

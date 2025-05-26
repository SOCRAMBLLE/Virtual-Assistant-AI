// // routes/protected_routes.rs

// use actix_identity::Identity;
// use actix_web::{web, HttpResponse};

// // Manipulador de rota para rota protegida
// pub async fn protected_route(id: Identity) -> HttpResponse {
//     if let Some(user_id) = id.identity() {
//         println!("Usuário autenticado com ID: {}", user_id);
//         // O usuário está autenticado, continue com a lógica da rota
//         HttpResponse::Ok().finish()
//     } else {
//         println!("Usuário não autenticado");
//         // O usuário não está autenticado, redirecione para a página de login ou retorne Unauthorized
//         HttpResponse::Unauthorized().finish()
//     }
// }

// pub async  fn index(_req: web::HttpRequest) -> Result<HttpResponse, actix_web::Error> {


        



// }

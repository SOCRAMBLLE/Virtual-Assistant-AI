use warp::Filter;

#[tokio::main]
async fn main() {
    let chat_route = warp::path!("api" / "chat")
        .and(warp::post())
        .map(|| warp::reply::html("Esta e a rota do chat!"));

    let routes = chat_route
        .or(warp::path("api").map(|| warp::reply::html("Bem-vindo ao Assistente Virtual API!")));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

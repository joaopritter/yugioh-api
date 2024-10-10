mod routers;
mod tools;

use axum::{middleware, Extension};
use axum::{response::Response, Router};
use routers::cards_router::routes_cards;
use tokio::net::TcpListener;
use tools::{card_client::YuGiClient, debug_tools::print_debug};
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    let yu_gi_client = YuGiClient::new().await;
    let routes_all = Router::new()
        .merge(routes_cards())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(Extension(yu_gi_client));
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    print_debug("main_response_mapper", "RES_MAPPER");
    println!("");
    res
}

use crate::tools::{
    card_client::{get_cards, YuGiClient},
    debug_tools::print_debug,
    response::build_response,
};
use axum::extract::Query;
use axum::Extension;
use axum::Router;
use axum::{response::IntoResponse, routing::get};
use mongodb::bson::doc;
use serde::Deserialize;

// Montagem das rotas
pub fn routes_cards() -> Router {
    Router::new()
        .route("/data_by_id", get(handler_get_data_by_id))
        .route("/get_all_cards", get(handler_get_all_cards))
}

// Struct para argumentos da query
#[derive(Debug, Deserialize)]
struct CardParams {
    id: Option<i64>,
}

async fn handler_get_data_by_id(
    Query(params): Query<CardParams>,
    Extension(client): Extension<YuGiClient>,
) -> impl IntoResponse {
    print_debug("handler_get_data_by_id", "HANDLER");
    match get_cards(client, doc! { "id": params.id }).await {
        Some(card) => {
            let card_data = card.into_iter().nth(0).unwrap();
            return build_response(200, "success", "Card retrieved", card_data);
        }
        None => {
            return build_response(404, "not found", "Card not found", "None");
        }
    }
}

async fn handler_get_all_cards(Extension(client): Extension<YuGiClient>) -> impl IntoResponse {
    print_debug("handler_get_all_cards", "HANDLER");
    match get_cards(client, doc! {}).await {
        Some(cards) => return build_response(200, "success", "Cards retrieved succesfully", cards),
        None => {
            return build_response(404, "not found", "No cards found.", "None");
        }
    }
}

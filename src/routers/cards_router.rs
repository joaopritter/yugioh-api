use crate::tools::{
    card_client::{card_to_response, get_all_cards, get_cards, CardResponse, YuGiClient},
    debug_tools::print_debug,
    response::build_response,
};
use axum::routing::get;
use axum::Extension;
use axum::Router;
use axum::{extract::Query, Json};
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
) -> Json<serde_json::Value> {
    print_debug("handler_get_data_by_id", "HANDLER");
    match get_cards(client, doc! { "id": params.id }).await {
        Some(card) => {
            let card_data = card_to_response(card);
            return build_response(200, "success", "Card retrieved", card_data);
        }
        None => {
            return build_response(404, "not found", "Card not found", "None");
        }
    }
}

async fn handler_get_all_cards(
    Extension(client): Extension<YuGiClient>,
) -> Json<Vec<CardResponse>> {
    print_debug("handler_get_all_cards", "HANDLER");
    let cards = get_all_cards(client).await;
    axum::Json(cards)
}

use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone)]
pub struct YuGiClient {
    pub card_db: Collection<Card>,
}

impl YuGiClient {
    // Struct para iniciar conexão com o banco quando a API é inicializada
    pub async fn new() -> Self {
        dotenv().ok();
        let db_uri = env::var("DATABASE_URI").unwrap();
        let options = ClientOptions::parse(db_uri).await.unwrap();
        let client = Client::with_options(options).unwrap();
        let database = client.database("YuGiOh");
        let collection = database.collection::<Card>("Cards");

        YuGiClient {
            card_db: collection,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardResponse {
    // Struct que será convertida em JSON para response
    pub id: usize,
    pub name: String,
    pub card_type: String,
    pub frame_type: String,
    pub desc: String,
    pub attack: usize,
    pub def: usize,
    pub level: usize,
    pub race: String,
    pub attribute: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    // Struct de referência para cartas recebidas do banco,
    // necessário para error handling
    pub _id: ObjectId,
    pub id: Option<usize>,
    pub name: Option<String>,
    pub card_type: Option<String>,
    pub frame_type: Option<String>,
    pub desc: Option<String>,
    pub attack: Option<usize>,
    pub def: Option<usize>,
    pub level: Option<usize>,
    pub race: Option<String>,
    pub attribute: Option<String>,
    pub card_images: Option<Vec<CardImage>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImage {
    pub id: usize,
    pub image_url: String,
    pub image_url_small: String,
    pub image_url_cropped: String,
}

pub async fn get_cards(connection: YuGiClient, filter: Document) -> Option<Card> {
    // Função para buscar cartas no banco
    match connection.card_db.find_one(filter).await.unwrap() {
        Some(card_data) => return Some(card_data),
        None => return None,
    }
}

pub async fn get_all_cards(connection: YuGiClient) -> Vec<CardResponse> {
    let mut data = connection.card_db.find(doc! {}).await.unwrap();
    let mut cards: Vec<CardResponse> = vec![];
    while let Some(result) = data.next().await {
        match result {
            Ok(card) => {
                cards.push(card_to_response(card));
            }
            Err(_e) => {}
        }
    }
    cards
}

pub fn card_to_response(card_data: Card) -> CardResponse {
    // Desembrulha a carta recebida e trata valores nulos
    // (No Rust não existe Null, Nil, None, etc como valor)
    let card = CardResponse {
        id: card_data.id.unwrap_or(0),
        name: card_data.name.unwrap_or("Unnamed Card".to_owned()),
        card_type: card_data
            .card_type
            .unwrap_or("Unknown Card Type".to_owned()),
        frame_type: card_data
            .frame_type
            .unwrap_or("Unknown Frame Type".to_owned()),
        desc: card_data.desc.unwrap_or("No Description Found".to_owned()),
        attack: card_data.attack.unwrap_or(0),
        def: card_data.def.unwrap_or(0),
        level: card_data.level.unwrap_or(0),
        race: card_data.race.unwrap_or("Unknown Race".to_owned()),
        attribute: card_data
            .attribute
            .unwrap_or("Unknown Attribute".to_owned()),
    };
    card
}

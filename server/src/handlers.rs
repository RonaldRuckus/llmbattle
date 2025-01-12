// src/handlers.rs
use actix_web::{web, HttpResponse, Responder};
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::Mutex;
use crate::{db, models::RoomCreated, models::RoomJoined};

// Struct to deserialize the request body for "create" and "join" endpoints
#[derive(Deserialize)]
pub struct NamePayload {
    name: String,
}

// Handle the "/create" endpoint
pub async fn handle_create(data: web::Data<Mutex<Connection>>, payload: web::Json<NamePayload>) -> impl Responder {
    let conn = data.lock().unwrap();
    let name = payload.name.clone(); // Extract the name from the request body

    match db::create_game(&conn, &name) {
        Ok((game_id, owner_token)) => {
            HttpResponse::Ok().json(RoomCreated {
                game_id: game_id.to_string(),
                token: owner_token.to_string(),
            })
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to create game."),
    }
}

// Handle the "/join/{game_id}" endpoint
pub async fn handle_join(
    path: web::Path<i64>,
    data: web::Data<Mutex<Connection>>,
    payload: web::Json<NamePayload>,
) -> impl Responder {
    let game_id = path.into_inner();
    let conn = data.lock().unwrap();
    let name = payload.name.clone(); // Extract the name from the request body

    match db::join_game(&conn, game_id, &name) {
        Ok(player_token) => {
            HttpResponse::Ok().json(RoomJoined {
                token: player_token.to_string(),
            })
        }
        Err(_) => HttpResponse::NotFound().body("Game not found or invalid state."),
    }
}

// Handle the "/poll" endpoint
pub async fn handle_poll(
    web::Query(params): web::Query<std::collections::HashMap<String, String>>,
    data: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = data.lock().unwrap();

    if let Some(game_id) = params.get("game_id").and_then(|id| id.parse::<i64>().ok()) {
        let timestamp = match params.get("timestamp").and_then(|t| t.parse::<i64>().ok()) {
            Some(t) => t,
            None => return HttpResponse::BadRequest().body("Invalid or missing timestamp."),
        };

        match db::poll_game_state(&conn, game_id, timestamp) {
            Ok(true) => HttpResponse::Ok().body("State has changed."),
            Ok(false) => HttpResponse::Ok().body("No change."),
            Err(_) => HttpResponse::InternalServerError().body("Error polling game state."),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid or missing game_id.")
    }
}

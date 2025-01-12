// src/handlers.rs
use actix_web::{web, HttpResponse, Responder};
use rusqlite::Connection;
use std::sync::Mutex;
use crate::{db, models::{game::{NameRequest, PollRequest}, room::{Created, Joined}}};

// Handle the "/create" endpoint
pub async fn handle_create(data: web::Data<Mutex<Connection>>, payload: web::Json<NameRequest>) -> impl Responder {
    let conn = data.lock().unwrap();
    let name = payload.name.clone(); // Extract the name from the request body

    match db::create_game(&conn, &name) {
        Ok((game_id, owner_token)) => {
            HttpResponse::Ok().json(Created {
                game_id: game_id,
                token: owner_token,
            })
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to create game."),
    }
}

// Handle the "/join/{game_id}" endpoint
pub async fn handle_join(
    path: web::Path<i64>,
    data: web::Data<Mutex<Connection>>,
    payload: web::Json<NameRequest>,
) -> impl Responder {
    let game_id = path.into_inner();
    let conn = data.lock().unwrap();
    let name = payload.name.clone(); // Extract the name from the request body

    match db::join_game(&conn, game_id, &name) {
        Ok(player_token) => {
            HttpResponse::Ok().json(Joined {
                token: player_token,
            })
        }
        Err(_) => HttpResponse::NotFound().body("Game not found or invalid state."),
    }
}

// Handle the "/poll" endpoint
pub async fn handle_poll(
    path: web::Path<i64>,
    web::Query(params): web::Query<PollRequest>,
    data: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = data.lock().unwrap();
    let game_id = path.into_inner();

    match db::poll_game_state(&conn, game_id, params.timestamp) {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to poll game state."),
    }
}

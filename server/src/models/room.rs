use serde::Serialize;

#[derive(Serialize)]
pub struct Created {
    pub game_id: i64,
    pub token: i64,
}

#[derive(Serialize)]
pub struct Joined {
    pub token: i64,
}
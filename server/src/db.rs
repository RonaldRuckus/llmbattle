use rusqlite::{Connection, Result};
use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Invalid game state")]
    InvalidGameState,
}

/// Initializes the database schema by reading the provided schema file.
pub fn initialize_database(conn: &Connection, schema_file_path: &str) {
    let schema = fs::read_to_string(schema_file_path)
        .expect(&format!("Failed to read schema file: {}", schema_file_path));

    conn.execute_batch(&schema)
        .expect("Failed to initialize database schema.");
}

/// Creates a new game and returns its unique ID and an owner token.
pub fn create_game(conn: &Connection, name: &str) -> Result<(i64, i64)> {
    conn.execute(
        "INSERT INTO Game (phase) VALUES (?1)",
        &["setup"],
    )?;
    let game_id = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO Player (game_id, name) VALUES (?1, ?2)",
        // Avoid converting `game_id` to `String`
        &[&game_id as &dyn rusqlite::ToSql, &name as &dyn rusqlite::ToSql],
    )?;
    let owner_token = conn.last_insert_rowid();

    Ok((game_id, owner_token))
}

/// Allows a player to join a game if there's exactly one player already in it. \
/// Returns a unique player token for the joining player.
pub fn join_game(conn: &Connection, game_id: i64, name: &str) -> Result<i64> {
    // Fetch the number of Player in the game
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM Player WHERE game_id = ?1")?;
    let player_count: i64 = stmt.query_row(&[&game_id], |row| row.get(0))?;

    // Ensure there is exactly one player currently in the game
    if player_count != 1 {
        return Err(rusqlite::Error::InvalidQuery);
    }

    // Insert the new player into the Player table
    conn.execute(
        "INSERT INTO Player (game_id, name) VALUES (?1, ?2)",
        &[&game_id as &dyn rusqlite::ToSql, &name as &dyn rusqlite::ToSql],
    )?;

    // Retrieve the ID of the newly added player
    let player_token = conn.last_insert_rowid();

    Ok(player_token)
}

/// Polls the game state to check if it has changed since a given timestamp.
pub fn poll_game_state(conn: &Connection, game_id: i64, timestamp: i64) -> Result<bool> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM Game WHERE id = ?1 AND timestamp > ?2)",
        &[&game_id, &timestamp],
        |row| row.get(0),
    )?;
    Ok(exists)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::env;
    use dotenv::dotenv;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
        initialize_database(&conn, "database/schema.sql");
        conn
    }

    #[test]
    fn test_initialize_database() {
        let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
        
        dotenv().ok();
        let schema_file_path = env::var("SCHEMA_FILE_PATH").expect("SCHEMA_FILE_PATH must be set in .env");

        initialize_database(&conn, &schema_file_path);

        // Verify that the Game and Player tables exist
        let game_table_exists: bool = conn
            .query_row(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='Game';",
                [],
                |row| row.get::<usize, String>(0),
            )
            .is_ok();
        let player_table_exists: bool = conn
            .query_row(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='Player';",
                [],
                |row| row.get::<usize, String>(0),
            )
            .is_ok();

        assert!(game_table_exists);
        assert!(player_table_exists);
    }

    #[test]
    fn test_create_game() {
        let conn = setup_test_db();

        let name = "test";
        let result = create_game(&conn, name).unwrap();
        // assert!(result.is_ok());

        let (game_id, _) = result;//.unwrap();

        // Verify the game was created
        let game_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM Game WHERE id = ?1;",
                [&game_id],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()
            > 0;

        // Verify the player was added
        let player_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM Player WHERE game_id = ?1 AND name = ?2;",
                [&game_id as &dyn rusqlite::ToSql, &name as &dyn rusqlite::ToSql],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()
            > 0;

        assert!(game_exists);
        assert!(player_exists);
    }

    #[test]
    fn test_join_game() {
        let conn = setup_test_db();

        let (game_id, _) = create_game(&conn, "test").unwrap();

        // Add a new player
        let result = join_game(&conn, game_id, "test2");
        assert!(result.is_ok());

        let new_player_token = result.unwrap();

        println!("new_player_token: {}", new_player_token);

        // Verify the new player was added
        let new_player_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM Player WHERE game_id = ?1 AND name = ?2;",
                [&game_id as &dyn rusqlite::ToSql, &"test2" as &dyn rusqlite::ToSql],
                |row| row.get::<_, i32>(0),
            )
            .unwrap()
            > 0;

        assert!(new_player_exists);
    }

    #[test]
    fn test_join_game_invalid_player_count() {
        let conn = setup_test_db();

        let (game_id, _) = create_game(&conn, "test").unwrap();

        // Add two Player (violating the "exactly one player" rule)
        join_game(&conn, game_id, "test").unwrap();
        let result = join_game(&conn, game_id, "test");

        assert!(result.is_err());
    }

    #[test]
    fn test_poll_game_state() {
        let conn = setup_test_db();

        let (game_id, _owner_token) = create_game(&conn, "test").unwrap();

        // Check initial state (should not have been updated)
        let result = poll_game_state(&conn, game_id, 0);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}

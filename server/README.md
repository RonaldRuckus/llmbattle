# Currently available endpoints

- POST /create
- POST /{game_id}/join/
- GET /{game_id}/poll/
- GET /{game_id}/creatures
- POST /{game_id}/creatures/create

## POST/create
```json
{
    "name": "player name"
}
```

### Response
```json
{
    "game_id": 0.0,
    "token": 0.0
}
```

## POST /{game_id}/join/
```json
{
    "name": "player name"
}
```

### Response
```json
{
    "token": 0.0
}
```

## GET /{game_id}/poll?timestamp={timestamp}

### Response (HTTP status code)
- 200 if new content
- 204 otherwise

## GET /{game_id}/creatures

### Response
```rust
pub struct State {
    pub health: u32,
    pub abilities: Vec<Ability>,
    pub modifiers: Vec<(i8, Attribute)>
}
```

## POST /{game_id}/creatures/create
```rust
pub struct CreateRequest {
    pub game_id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub image: Option<String>, // A prompt of the image
    pub elements: Vec<Element>,
    pub abilities: Vec<AbilityRequest>,
    pub attributes: HashMap<Attribute, u8>
}
```

### Response
200 OK
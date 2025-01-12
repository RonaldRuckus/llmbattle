# Currently available endpoints

- /create
- /join/{game_id}
- /poll

## /create
### POST
```json
{
    "name": String
}
```

### Response
```json
{
    "game_id": i64,
    "token": i64
}
```

## /{game_id}/join/
### POST
```json
{
    "name": String
}
```

### Response
```json
{
    "token": i64
}
```

## /{game_id}/poll?timestamp={timestamp}
### GET

### Response (HTTP status code)
- 200 if new content
- 204 otherwise
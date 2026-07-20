use crate::steam::{
    mapper::map_owned_game,
    models::{GetOwnedGamesResponse, SteamApp},
};

#[test]
fn maps_owned_game() {
    let game = SteamApp {
        appid: 570,
        name: "Dota 2".into(),
        playtime_forever: Some(120),
    };

    let mapped = map_owned_game(game);

    assert_eq!(mapped.title, "Dota 2");
}

#[test]
fn deserialize_owned_games_response() {
    let json = r#"
    {
        "response": {
            "game_count": 1,
            "games": [
                {
                    "appid": 570,
                    "name": "Dota 2",
                    "playtime_forever": 1234
                }
            ]
        }
    }
    "#;

    let response: GetOwnedGamesResponse = serde_json::from_str(json).unwrap();

    let games = response.response.games.unwrap();

    assert_eq!(games.len(), 1);
    assert_eq!(games[0].appid, 570);
    assert_eq!(games[0].name, "Dota 2");
}

#[test]
fn deserialize_empty_library() {
    let json = r#"
    {
        "response": {
            "game_count": 0,
            "games": []
        }
    }
    "#;

    let response: GetOwnedGamesResponse = serde_json::from_str(json).unwrap();

    assert!(response.response.games.unwrap().is_empty());
}

#[test]
fn deserialize_without_games_field() {
    let json = r#"
    {
        "response": {
            "game_count": 0
        }
    }
    "#;

    let response: GetOwnedGamesResponse = serde_json::from_str(json).unwrap();

    assert!(response.response.games.is_none());
}

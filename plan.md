# Vision 

You open this link `http://media.home`


            Browser
          (React SPA)

               │
         REST/WebSocket

               │
      Rust Server (Axum)

 ┌──────────┬──────────┬──────────┐
 │          │          │          │
Metadata   Library    Users     Jobs
Service    Service    Service   Service

               │

          SQLite/Postgres


```rust

pub struct Media {
    pub id: Uuid,
    pub title: String,
    pub media_type: MediaType,
    pub metadata: MediaMetadata,
    pub details: MediaDetails,
}

pub enum MediaDetails {
    Movie(MovieDetails),
    TvShow(TvShowDetails),
    Game(GameDetails),
    Book(BookDetails),
}

pub struct GameDetails {
    pub media_id: Uuid,

    pub developers: Vec<String>,
    pub publishers: Vec<String>,

    pub platforms: Vec<Platform>,

    pub achievements_supported: bool,
    pub multiplayer: bool,
}

//db
media
-----
id
type
title


game_details
-------------
media_id
multiplayer
achievement_support


movie_details
--------------
media_id
runtime


tv_show_details
---------------
media_id
episode_count

```


itonda/
├── apps/
│   ├── server/                 # Axum API
│   ├── web/                    # React frontend
│   └── agent/                  # Tauri local agent
    └── cli/
│ 
│
├── crates/
│   ├── domain/                 # Core business logic
│   ├── database/               # SQLx/Prisma/etc
│   ├── metadata/               # Metadata providers
│   ├── scanner/                # Import/discovery system
│   ├── launcher/               # Launch providers
│   └── common/                # Shared types
│
├── migrations/
├── docker-compose.yml
├── Cargo.toml
└── README.md



1. db connection/migrations/schema
5. update project structure (move db into crates/ instead of apps/server/)
2. openapi for generating types
3. react/tailwind/tanstack router
4. create db schemas


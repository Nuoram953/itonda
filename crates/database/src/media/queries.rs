use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use uuid::Uuid;

use super::models::{
    DbMediaFilterOptions, DbMediaSortField, DbSortOrder, MediaRow, PaginatedMediaRows,
};
use crate::{
    error::DatabaseError,
    media::{
        MediaAssetInsert, MediaAssetRow, MediaAssetSearchInsert, MediaAssetSearchRow,
        MediaGameDetailsRow, MediaGameDetailsUpsert, MediaInsert, MediaInstallationRow,
        MediaInstallationUpsert, MediaLaunchRow, MediaLaunchSessionInsert, MediaLaunchSessionRow,
        MediaLaunchUpsert, MediaStatusHistoryRow, MediaStorefrontRow, MediaStorefrontUpsert,
    },
    models::{UpsertAction, UpsertResult},
};

pub async fn find_assets_by_media_ids(
    pool: &SqlitePool,
    ids: &[String],
) -> Result<Vec<MediaAssetRow>, DatabaseError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, media_id, asset_id, path FROM media_assets WHERE media_id IN (",
    );

    let mut separated = qb.separated(", ");

    for id in ids {
        separated.push_bind(id);
    }

    separated.push_unseparated(")");

    qb.build_query_as::<MediaAssetRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)
}

pub async fn find_all(
    pool: &SqlitePool,
    media_type: Option<&str>,
) -> Result<Vec<MediaRow>, DatabaseError> {
    let mut qb = QueryBuilder::<Sqlite>::new("SELECT id, title, media_type, status_id FROM media");

    if let Some(media_type) = media_type {
        qb.push(" WHERE media_type = ");
        qb.push_bind(media_type);
    }

    qb.build_query_as::<MediaRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)
}

pub async fn find_paginated(
    pool: &SqlitePool,
    options: DbMediaFilterOptions<'_>,
) -> Result<PaginatedMediaRows, DatabaseError> {
    let page = if options.page == 0 { 1 } else { options.page };
    let limit = if options.limit == 0 {
        24
    } else {
        options.limit
    };
    let offset = (page - 1) * limit;

    let needs_storefront_join = options.storefront_id.is_some();

    let mut count_qb = QueryBuilder::<Sqlite>::new("SELECT COUNT(DISTINCT media.id) FROM media");

    if needs_storefront_join {
        count_qb.push(" INNER JOIN media_storefronts ON media_storefronts.media_id = media.id");
    }

    let mut has_where = false;
    if let Some(media_type) = options.media_type {
        count_qb.push(" WHERE media.media_type = ");
        count_qb.push_bind(media_type);
        has_where = true;
    }

    if let Some(search) = options.search
        && !search.trim().is_empty()
    {
        if has_where {
            count_qb.push(" AND ");
        } else {
            count_qb.push(" WHERE ");
            has_where = true;
        }
        count_qb.push("media.title LIKE ");
        count_qb.push_bind(format!("%{}%", search.trim()));
    }

    if let Some(status_id) = options.status_id {
        if has_where {
            count_qb.push(" AND ");
        } else {
            count_qb.push(" WHERE ");
            has_where = true;
        }
        count_qb.push("media.status_id = ");
        count_qb.push_bind(status_id);
    }

    if let Some(storefront_id) = options.storefront_id {
        if has_where {
            count_qb.push(" AND ");
        } else {
            count_qb.push(" WHERE ");
        }
        count_qb.push("media_storefronts.storefront_id = ");
        count_qb.push_bind(storefront_id);
    }

    let total: (i64,) = count_qb
        .build_query_as()
        .fetch_one(pool)
        .await
        .map_err(DatabaseError::from)?;

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT media.id, media.title, media.media_type, media.status_id FROM media",
    );

    let needs_details_join = matches!(options.sort_by, Some(DbMediaSortField::LastPlayedAt));
    if needs_details_join {
        qb.push(" LEFT JOIN media_game_details ON media_game_details.media_id = media.id");
    }

    if needs_storefront_join {
        qb.push(" INNER JOIN media_storefronts ON media_storefronts.media_id = media.id");
    }

    let mut has_where = false;
    if let Some(media_type) = options.media_type {
        qb.push(" WHERE media.media_type = ");
        qb.push_bind(media_type);
        has_where = true;
    }

    if let Some(search) = options.search
        && !search.trim().is_empty()
    {
        if has_where {
            qb.push(" AND ");
        } else {
            qb.push(" WHERE ");
            has_where = true;
        }
        qb.push("media.title LIKE ");
        qb.push_bind(format!("%{}%", search.trim()));
    }

    if let Some(status_id) = options.status_id {
        if has_where {
            qb.push(" AND ");
        } else {
            qb.push(" WHERE ");
            has_where = true;
        }
        qb.push("media.status_id = ");
        qb.push_bind(status_id);
    }

    if let Some(storefront_id) = options.storefront_id {
        if has_where {
            qb.push(" AND ");
        } else {
            qb.push(" WHERE ");
        }
        qb.push("media_storefronts.storefront_id = ");
        qb.push_bind(storefront_id);
    }

    let sort_order_str = match options.sort_order.unwrap_or(DbSortOrder::Asc) {
        DbSortOrder::Asc => "ASC",
        DbSortOrder::Desc => "DESC",
    };

    match options.sort_by {
        Some(DbMediaSortField::Title) => {
            qb.push(format!(
                " ORDER BY media.title {}, media.id ASC",
                sort_order_str
            ));
        }
        Some(DbMediaSortField::LastPlayedAt) => {
            qb.push(format!(
                " ORDER BY COALESCE(media_game_details.last_played_at, 0) {}, media.title ASC, media.id ASC",
                sort_order_str
            ));
        }
        None => {
            qb.push(" ORDER BY media.title ASC, media.id ASC");
        }
    }

    qb.push(" LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);

    let items = qb
        .build_query_as::<MediaRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok(PaginatedMediaRows {
        items,
        total: total.0 as u64,
    })
}

pub async fn find_media_by_id(
    pool: &SqlitePool,
    media_id: String,
) -> Result<MediaRow, DatabaseError> {
    sqlx::query_as!(
        MediaRow,
        r#"
    SELECT
        id,
        title,
        media_type,
        status_id
    FROM media
    where id=?
    "#,
        media_id
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_asset_by_id(
    pool: &SqlitePool,
    id: String,
) -> Result<Option<MediaAssetRow>, DatabaseError> {
    sqlx::query_as!(
        MediaAssetRow,
        r#"
        SELECT
            id,
            media_id,
            asset_id,
            path
        FROM media_assets
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_by_title(
    pool: &SqlitePool,
    title: String,
) -> Result<Option<MediaRow>, DatabaseError> {
    sqlx::query_as!(
        MediaRow,
        r#"
    SELECT
        id,
        title,
        media_type,
        status_id
    FROM media
    WHERE title=?
    "#,
        title
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_media(
    pool: &SqlitePool,
    media: MediaInsert,
) -> Result<MediaRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as!(
        MediaRow,
        r#"
        INSERT INTO media (
            id,
            title,
            media_type,
            status_id
        )
        VALUES (?, ?, ?, ?)
        RETURNING
            id,
            title,
            media_type,
            status_id
        "#,
        id,
        media.title,
        media.media_type,
        media.status_id
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_launch_by_media_id(
    pool: &SqlitePool,
    media_id: String,
) -> Result<Vec<MediaLaunchRow>, DatabaseError> {
    sqlx::query_as!(
        MediaLaunchRow,
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        from media_launches
        where media_id=?
        "#,
        media_id,
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_launch_by_id(
    pool: &SqlitePool,
    launch_id: String,
) -> Result<MediaLaunchRow, DatabaseError> {
    sqlx::query_as!(
        MediaLaunchRow,
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        from media_launches
        where id=?
        "#,
        launch_id,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_media_launch(
    pool: &SqlitePool,
    media_launch: MediaLaunchUpsert,
) -> Result<UpsertResult<MediaLaunchRow>, DatabaseError> {
    let existing = sqlx::query_as!(
        MediaLaunchRow,
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        FROM media_launches
        WHERE media_id = ?
          AND agent_id IS ?
          AND name = ?
          AND launch_type = ?
        "#,
        media_launch.media_id,
        media_launch.agent_id,
        media_launch.name,
        media_launch.launch_type,
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)?;

    let Some(existing) = existing else {
        let row = insert_media_launch(pool, media_launch).await?;

        return Ok(UpsertResult {
            value: row,
            action: UpsertAction::Created,
        });
    };

    let changed = existing.program != media_launch.program
        || existing.arguments != media_launch.arguments
        || existing.working_directory != media_launch.working_directory
        || existing.is_default != media_launch.is_default
        || existing.enabled != media_launch.enabled;

    if !changed {
        return Ok(UpsertResult {
            value: existing,
            action: UpsertAction::Unchanged,
        });
    }

    let row = sqlx::query_as!(
        MediaLaunchRow,
        r#"
        UPDATE media_launches
        SET
            program = ?,
            arguments = ?,
            working_directory = ?,
            is_default = ?,
            enabled = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        "#,
        media_launch.program,
        media_launch.arguments,
        media_launch.working_directory,
        media_launch.is_default,
        media_launch.enabled,
        existing.id,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(UpsertResult {
        value: row,
        action: UpsertAction::Updated,
    })
}

async fn insert_media_launch(
    pool: &SqlitePool,
    media_launch: MediaLaunchUpsert,
) -> Result<MediaLaunchRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as!(
        MediaLaunchRow,
        r#"
        INSERT INTO media_launches (
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default,
            enabled
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        "#,
        id,
        media_launch.media_id,
        media_launch.agent_id,
        media_launch.name,
        media_launch.launch_type,
        media_launch.program,
        media_launch.arguments,
        media_launch.working_directory,
        media_launch.is_default,
        media_launch.enabled,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn update_media_status(
    pool: &SqlitePool,
    media_id: &str,
    status_id: i64,
) -> Result<(), DatabaseError> {
    let mut tx = pool.begin().await.map_err(DatabaseError::from)?;
    let id = Uuid::new_v4().to_string();

    sqlx::query!(
        r#"
        UPDATE media
        SET status_id = ?
        WHERE id = ?
        "#,
        status_id,
        media_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(DatabaseError::from)?;

    sqlx::query!(
        r#"
        INSERT INTO media_status_history (
            id,
            media_id,
            status_id
        )
        VALUES (?, ?, ?)
        "#,
        id,
        media_id,
        status_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(DatabaseError::from)?;

    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(())
}

pub async fn insert_media_asset(
    pool: &SqlitePool,
    asset: MediaAssetInsert,
) -> Result<MediaAssetRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as!(
        MediaAssetRow,
        r#"
        INSERT INTO media_assets (
            id,
            media_id,
            asset_id,
            path
        )
        VALUES (?, ?, ?, ?)
        RETURNING
            id,
            media_id,
            asset_id,
            path
        "#,
        id,
        asset.media_id,
        asset.asset_id,
        asset.path,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_status_history(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaStatusHistoryRow>, DatabaseError> {
    sqlx::query_as!(
        MediaStatusHistoryRow,
        r#"
        SELECT
            id,
            media_id,
            status_id,
            created_at
        FROM media_status_history
        WHERE media_id = ?
        ORDER BY created_at
        "#,
        media_id
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_media_game_details(
    pool: &SqlitePool,
    details: MediaGameDetailsUpsert,
) -> Result<UpsertResult<MediaGameDetailsRow>, DatabaseError> {
    let existing = sqlx::query_as!(
        MediaGameDetailsRow,
        r#"
        SELECT
            media_id,
            playtime_minutes,
            last_played_at
        FROM media_game_details
        WHERE media_id = ?
        "#,
        details.media_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)?;

    let Some(existing) = existing else {
        let row = sqlx::query_as!(
            MediaGameDetailsRow,
            r#"
            INSERT INTO media_game_details (
                media_id,
                playtime_minutes,
                last_played_at
            )
            VALUES (?, ?, ?)
            RETURNING
                media_id,
                playtime_minutes,
                last_played_at
            "#,
            details.media_id,
            details.playtime_minutes,
            details.last_played_at,
        )
        .fetch_one(pool)
        .await
        .map_err(DatabaseError::from)?;

        return Ok(UpsertResult {
            value: row,
            action: UpsertAction::Created,
        });
    };

    if existing.playtime_minutes == details.playtime_minutes
        && existing.last_played_at == details.last_played_at
    {
        return Ok(UpsertResult {
            value: existing,
            action: UpsertAction::Unchanged,
        });
    }

    let row = sqlx::query_as!(
        MediaGameDetailsRow,
        r#"
        UPDATE media_game_details
        SET
            playtime_minutes = ?,
            last_played_at = ?
        WHERE media_id = ?
        RETURNING
            media_id,
            playtime_minutes,
            last_played_at
        "#,
        details.playtime_minutes,
        details.last_played_at,
        details.media_id,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(UpsertResult {
        value: row,
        action: UpsertAction::Updated,
    })
}

pub async fn find_game_details(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Option<MediaGameDetailsRow>, DatabaseError> {
    let row = sqlx::query_as!(
        MediaGameDetailsRow,
        r#"
        SELECT
            media_id,
            playtime_minutes,
            last_played_at
        FROM media_game_details
        WHERE media_id = ?
        "#,
        media_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(row)
}

pub async fn find_media_launches_by_media_ids(
    pool: &SqlitePool,
    ids: &[String],
) -> Result<Vec<MediaLaunchRow>, DatabaseError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default,
            enabled
        FROM media_launches
        WHERE media_id IN (
        "#,
    );

    let mut separated = qb.separated(", ");

    for id in ids {
        separated.push_bind(id);
    }

    separated.push_unseparated(")");

    qb.build_query_as::<MediaLaunchRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)
}

pub async fn find_media_launches_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaLaunchRow>, DatabaseError> {
    let row = sqlx::query_as!(
        MediaLaunchRow,
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        FROM media_launches
        WHERE media_id = ?
        "#,
        media_id
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(row)
}

pub async fn find_asset_searches_by_media_ids(
    pool: &SqlitePool,
    ids: &[String],
) -> Result<Vec<MediaAssetSearchRow>, DatabaseError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT media_id, asset_id, searched_at FROM media_asset_searches WHERE media_id IN (",
    );

    let mut separated = qb.separated(", ");

    for id in ids {
        separated.push_bind(id);
    }

    separated.push_unseparated(")");

    qb.build_query_as::<MediaAssetSearchRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)
}

pub async fn find_asset_searches_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaAssetSearchRow>, DatabaseError> {
    sqlx::query_as!(
        MediaAssetSearchRow,
        r#"
        SELECT
            media_id,
            asset_id,
            searched_at
        FROM media_asset_searches
        WHERE media_id = ?
        "#,
        media_id
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_media_asset_search(
    pool: &SqlitePool,
    search: MediaAssetSearchInsert,
) -> Result<MediaAssetSearchRow, DatabaseError> {
    sqlx::query_as!(
        MediaAssetSearchRow,
        r#"
        INSERT INTO media_asset_searches (
            media_id,
            asset_id
        )
        VALUES (?, ?)
        ON CONFLICT(media_id, asset_id) DO UPDATE SET searched_at = CURRENT_TIMESTAMP
        RETURNING
            media_id,
            asset_id,
            searched_at
        "#,
        search.media_id,
        search.asset_id
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_media_launch_session(
    pool: &SqlitePool,
    session: MediaLaunchSessionInsert,
) -> Result<MediaLaunchSessionRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as!(
        MediaLaunchSessionRow,
        r#"
        INSERT INTO media_launch_sessions (
            id,
            launch_id,
            started_at,
            completed_at,
            duration_seconds
        )
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            id,
            launch_id,
            started_at,
            completed_at,
            duration_seconds
        "#,
        id,
        session.launch_id,
        session.started_at,
        session.completed_at,
        session.duration_seconds,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_launch_sessions_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaLaunchSessionRow>, DatabaseError> {
    sqlx::query_as::<Sqlite, MediaLaunchSessionRow>(
        r#"
        SELECT
            s.id,
            s.launch_id,
            s.started_at,
            s.completed_at,
            s.duration_seconds
        FROM media_launch_sessions s
        INNER JOIN media_launches l ON l.id = s.launch_id
        WHERE l.media_id = ?
        "#,
    )
    .bind(media_id)
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_media_storefront(
    pool: &SqlitePool,
    details: MediaStorefrontUpsert,
) -> Result<UpsertResult<MediaStorefrontRow>, DatabaseError> {
    let existing = sqlx::query_as::<Sqlite, MediaStorefrontRow>(
        r#"
        SELECT
            media_id,
            storefront_id,
            external_id,
            playtime_minutes,
            last_played_at
        FROM media_storefronts 
        WHERE media_id = ? AND storefront_id = ?
        "#,
    )
    .bind(&details.media_id)
    .bind(&details.storefront_id)
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)?;

    let Some(existing) = existing else {
        let row = sqlx::query_as::<Sqlite, MediaStorefrontRow>(
            r#"
            INSERT INTO media_storefronts (
                media_id,
                storefront_id,
                external_id,
                playtime_minutes,
                last_played_at
            )
            VALUES (?, ?, ?, ?, ?)
            RETURNING
                media_id,
                storefront_id,
                external_id,
                playtime_minutes,
                last_played_at
            "#,
        )
        .bind(&details.media_id)
        .bind(&details.storefront_id)
        .bind(&details.external_id)
        .bind(details.playtime_minutes)
        .bind(details.last_played_at)
        .fetch_one(pool)
        .await
        .map_err(DatabaseError::from)?;

        return Ok(UpsertResult {
            value: row,
            action: UpsertAction::Created,
        });
    };

    if existing.playtime_minutes == details.playtime_minutes
        && existing.last_played_at == details.last_played_at
    {
        return Ok(UpsertResult {
            value: existing,
            action: UpsertAction::Unchanged,
        });
    }

    let row = sqlx::query_as::<Sqlite, MediaStorefrontRow>(
        r#"
        UPDATE media_storefronts 
        SET
            playtime_minutes = ?,
            last_played_at = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE media_id = ? AND storefront_id = ?
        RETURNING
            media_id,
            storefront_id,
            external_id,
            playtime_minutes,
            last_played_at
        "#,
    )
    .bind(details.playtime_minutes)
    .bind(details.last_played_at)
    .bind(&details.media_id)
    .bind(&details.storefront_id)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(UpsertResult {
        value: row,
        action: UpsertAction::Updated,
    })
}

pub async fn find_storefronts_by_media_ids(
    pool: &SqlitePool,
    ids: &[String],
) -> Result<Vec<MediaStorefrontRow>, DatabaseError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT media_id, storefront_id, external_id, playtime_minutes, last_played_at FROM media_storefronts WHERE media_id IN (",
    );

    let mut separated = qb.separated(", ");

    for id in ids {
        separated.push_bind(id);
    }

    separated.push_unseparated(")");

    qb.build_query_as::<MediaStorefrontRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)
}

pub async fn find_storefronts_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaStorefrontRow>, DatabaseError> {
    sqlx::query_as::<Sqlite, MediaStorefrontRow>(
        r#"
        SELECT
            media_id,
            storefront_id,
            external_id,
            playtime_minutes,
            last_played_at
        FROM media_storefronts
        WHERE media_id = ?
        "#,
    )
    .bind(media_id)
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_by_storefront(
    pool: &SqlitePool,
    storefront_id: &str,
    external_id: &str,
) -> Result<Option<MediaRow>, DatabaseError> {
    sqlx::query_as::<Sqlite, MediaRow>(
        r#"
        SELECT
            m.id,
            m.title,
            m.media_type,
            m.status_id
        FROM media m
        INNER JOIN media_storefronts ms ON ms.media_id = m.id
        WHERE ms.storefront_id = ? AND ms.external_id = ?
        "#,
    )
    .bind(storefront_id)
    .bind(external_id)
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_media_installation(
    pool: &SqlitePool,
    installation: MediaInstallationUpsert,
) -> Result<UpsertResult<MediaInstallationRow>, DatabaseError> {
    let existing = sqlx::query_as::<Sqlite, MediaInstallationRow>(
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            storefront_id,
            external_id,
            path
        FROM media_installations
        WHERE media_id = ?
          AND agent_id = ?
          AND storefront_id IS ?
        "#,
    )
    .bind(&installation.media_id)
    .bind(&installation.agent_id)
    .bind(&installation.storefront_id)
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)?;

    let Some(existing) = existing else {
        let id = Uuid::new_v4().to_string();
        let row = sqlx::query_as::<Sqlite, MediaInstallationRow>(
            r#"
            INSERT INTO media_installations (
                id,
                media_id,
                agent_id,
                storefront_id,
                external_id,
                path
            )
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING
                id,
                media_id,
                agent_id,
                storefront_id,
                external_id,
                path
            "#,
        )
        .bind(id)
        .bind(&installation.media_id)
        .bind(&installation.agent_id)
        .bind(&installation.storefront_id)
        .bind(&installation.external_id)
        .bind(&installation.path)
        .fetch_one(pool)
        .await
        .map_err(DatabaseError::from)?;

        return Ok(UpsertResult {
            value: row,
            action: UpsertAction::Created,
        });
    };

    if existing.external_id == installation.external_id && existing.path == installation.path {
        return Ok(UpsertResult {
            value: existing,
            action: UpsertAction::Unchanged,
        });
    }

    let row = sqlx::query_as::<Sqlite, MediaInstallationRow>(
        r#"
        UPDATE media_installations
        SET
            external_id = ?,
            path = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING
            id,
            media_id,
            agent_id,
            storefront_id,
            external_id,
            path
        "#,
    )
    .bind(&installation.external_id)
    .bind(&installation.path)
    .bind(&existing.id)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(UpsertResult {
        value: row,
        action: UpsertAction::Updated,
    })
}

pub async fn delete_media_installation(
    pool: &SqlitePool,
    agent_id: &str,
    media_id: &str,
    storefront_id: Option<&str>,
) -> Result<(), DatabaseError> {
    sqlx::query(
        r#"
        DELETE FROM media_installations
        WHERE agent_id = ?
          AND media_id = ?
          AND storefront_id IS ?
        "#,
    )
    .bind(agent_id)
    .bind(media_id)
    .bind(storefront_id)
    .execute(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(())
}

pub async fn find_installations_by_media_ids(
    pool: &SqlitePool,
    ids: &[String],
) -> Result<Vec<MediaInstallationRow>, DatabaseError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, media_id, agent_id, storefront_id, external_id, path FROM media_installations WHERE media_id IN (",
    );

    let mut separated = qb.separated(", ");

    for id in ids {
        separated.push_bind(id);
    }

    separated.push_unseparated(")");

    qb.build_query_as::<MediaInstallationRow>()
        .fetch_all(pool)
        .await
        .map_err(DatabaseError::from)
}

pub async fn find_installations_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaInstallationRow>, DatabaseError> {
    sqlx::query_as::<Sqlite, MediaInstallationRow>(
        r#"
        SELECT
            id,
            media_id,
            agent_id,
            storefront_id,
            external_id,
            path
        FROM media_installations
        WHERE media_id = ?
        "#,
    )
    .bind(media_id)
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

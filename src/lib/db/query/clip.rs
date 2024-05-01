use super::super::models::clip;
use super::super::DatabasePool;
use crate::domain::db::fields::id::DbError;

pub type Result<T> = std::result::Result<T, DbError>;

pub async fn get_clip<I: Into<clip::GetClip>>(
    identifier: I,
    pool: &DatabasePool,
) -> Result<clip::Clip> {
    let clip_model: clip::GetClip = identifier.into();
    let short_code = clip_model.short_code.as_str();

    Ok(sqlx::query_as!(
        clip::Clip,
        "SELECT * FROM clip WHERE short_code = $1",
        short_code
    )
    .fetch_one(pool)
    .await?)
}

pub async fn insert_clip<D: Into<clip::InsertClip>>(
    data: D,
    pool: &DatabasePool,
) -> Result<clip::Clip> {
    let clip_model: clip::InsertClip = data.into();

    sqlx::query!(
        r#"INSERT INTO clip(
            short_code,
            title,
            content,
            password,
            expires_at,
            no_of_hits) VALUES($1, $2, $3, $4, $5, $6)"#,
        clip_model.short_code,
        clip_model.title,
        clip_model.content,
        clip_model.password,
        clip_model.expires_at,
        0
    )
    .execute(pool)
    .await?;

    get_clip(clip_model.short_code, pool).await
}

pub async fn update_clip<D: Into<clip::UpdateClip>>(
    data: D,
    pool: &DatabasePool,
) -> Result<clip::Clip> {
    let clip_model: clip::UpdateClip = data.into();

    let mut update_fields: Vec<String> = Vec::new();

    if let Some(_) = &clip_model.title {
        update_fields.push(String::from("title = $2"));
    };

    if let Some(_) = &clip_model.content {
        update_fields.push(String::from("content = $3"));
    };

    if let Some(_) = &clip_model.password {
        update_fields.push(String::from("password = $4"));
    };

    if let Some(_) = &clip_model.expires_at {
        update_fields.push(String::from("expires_at = $5"));
    };

    let mut update_sql = String::from("");

    for field in update_fields {
        update_sql = if update_sql.len() == 0 {
            format!("{}", field)
        } else {
            format!("{}, {}", &update_sql, field)
        };
    }

    if update_sql.len() > 0 {
        let sql = format!(
            r#"
            UPDATE clip SET 
                {}
            WHERE short_code = $1"#,
            update_sql
        );
        println!("{:?}", sql);
        sqlx::query(&sql)
            .bind(&clip_model.short_code)
            .bind(&clip_model.title)
            .bind(&clip_model.content)
            .bind(&clip_model.password)
            .bind(&clip_model.expires_at)
            .execute(pool)
            .await?;
    }

    get_clip(clip_model.short_code, pool).await
}

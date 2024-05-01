use super::ServiceError;
use crate::db::{query, DatabasePool};
use crate::domain::clip as domain_clip;
use crate::dto::clip as clip_dto;

type Result<T> = std::result::Result<T, ServiceError>;

pub async fn get_clip<DTO: Into<clip_dto::GetClip>>(
    dto: DTO,
    pool: &DatabasePool,
) -> Result<domain_clip::Clip> {
    let clip_dto: clip_dto::GetClip = dto.into();
    let clip: domain_clip::Clip = query::clip::get_clip(clip_dto.short_code, pool)
        .await?
        .try_into()?;

    if clip.password.has_password() {
        if let Some(password) = clip_dto.password.into_inner() {
            if clip.password.are_passwords_equal(&password) {
                return Ok(clip);
            } else {
                return Err(ServiceError::PermissionDenied(String::from(
                    "Password is incorrect.",
                )));
            }
        } else {
            return Err(ServiceError::PermissionDenied(String::from(
                "Password is required.",
            )));
        }
    }
    Ok(clip)
}

pub async fn create_clip(
    dto: clip_dto::CreateClip,
    pool: &DatabasePool,
) -> Result<domain_clip::Clip> {
    let clip: domain_clip::Clip = query::clip::insert_clip(dto, pool).await?.try_into()?;
    Ok(clip)
}

pub async fn update_clip(
    dto: clip_dto::UpdateClip,
    pool: &DatabasePool,
) -> Result<domain_clip::Clip> {
    let clip: domain_clip::Clip = query::clip::update_clip(dto, pool).await?.try_into()?;
    Ok(clip)
}

pub async fn delete_clip<DTO: Into<clip_dto::DeleteClip>>(
    dto: DTO,
    pool: &DatabasePool,
) -> Result<bool> {
    let dto = dto.into();
    query::clip::delete_clip(dto, pool).await?;
    Ok(true)
}

use actix_web::{delete, get, patch, post, web};
use serde::{Deserialize, Serialize};

use crate::domain::clip::fields::short_code::ShortCode;
use crate::domain::clip::{Clip, ClipError};
use crate::domain::AppData;
use crate::dto::clip::{
    CreateClip as CreateClipDto, DeleteClip as DeleteClipDto, UpdateClip as UpdateClipDto,
};
use crate::{db::query, service::ServiceError};

#[derive(Serialize, Deserialize)]
struct GetClipParams {
    short_code: String,
}

#[get("/clips")]
pub async fn get_clips(app_data: web::Data<AppData>) -> Result<web::Json<Vec<Clip>>, ServiceError> {
    if let Ok(clip_models) = query::clip::get_clips(&app_data.db_pool).await {
        let clips: Vec<Clip> = clip_models
            .iter()
            .map(|clip_model| clip_model.to_owned().try_into().unwrap())
            .collect();
        return Ok(web::Json(clips));
    }
    Err(ServiceError::InternalServer)
}

#[get("/clips/{short_code}")]
pub async fn get_clip(
    params: web::Path<GetClipParams>,
    app_data: web::Data<AppData>,
) -> Result<web::Json<Clip>, ServiceError> {
    if let Ok(clip_model) =
        query::clip::get_clip(String::from(&params.short_code), &app_data.db_pool).await
    {
        if let Ok(clip) = clip_model.try_into() {
            return Ok(web::Json(clip));
        }
        Err(ServiceError::NotFound(String::from("Clip not found.")))
    } else {
        Err(ServiceError::NotFound(String::from("Clip not found.")))
    }
}

#[post("/clips")]
pub async fn create_clip(
    body: web::Json<CreateClipDto>,
    app_data: web::Data<AppData>,
) -> Result<web::Json<Clip>, ServiceError> {
    let clip_model = query::clip::insert_clip(
        CreateClipDto {
            short_code: body.short_code.to_owned(),
            title: body.title.to_owned(),
            content: body.content.to_owned(),
            password: body.password.to_owned(),
            expires_at: body.expires_at.to_owned(),
        },
        &app_data.db_pool,
    )
    .await?;

    let clip_model_result: Result<Clip, ClipError> = clip_model.try_into();
    return match clip_model_result {
        Ok(clip) => Ok(web::Json(clip)),
        Err(err) => Err(ServiceError::Clip(err)),
    };
}

#[patch("/clips/{short_code}")]
pub async fn update_clip(
    params: web::Path<GetClipParams>,
    body: web::Json<UpdateClipDto>,
    app_data: web::Data<AppData>,
) -> Result<web::Json<Clip>, ServiceError> {
    let short_code: ShortCode = params.short_code.to_owned().into();
    let clip_model = query::clip::update_clip(
        short_code,
        UpdateClipDto {
            content: body.content.to_owned(),
            title: body.title.to_owned(),
            password: body.password.to_owned(),
            expires_at: body.expires_at.to_owned(),
        },
        &app_data.db_pool,
    )
    .await?;

    let result: Result<Clip, ClipError> = clip_model.try_into();
    return match result {
        Ok(clip) => Ok(web::Json(clip)),
        Err(err) => Err(ServiceError::Clip(err)),
    };
}

#[delete("/clips/{short_code}")]
pub async fn delete_clip(
    params: web::Path<DeleteClipDto>,
    app_data: web::Data<AppData>,
) -> Result<String, ServiceError> {
    let short_code: ShortCode = params.short_code.to_owned().into();
    if let Err(err) = query::clip::delete_clip(short_code, &app_data.db_pool).await {
        return Err(ServiceError::Database(err));
    }
    return Ok(String::from("true"));
}

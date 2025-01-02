use actix_web::{web, HttpResponse, Result, HttpRequest};
use mongodb::Database;
use chrono::Utc;

use crate::models::{Url, CreateUrlRequest, CreateUrlResponse};
use crate::utils::{generate_unique_code, validate_url};

pub async fn shorten_url(
    req: HttpRequest,
    db: web::Data<Database>,
    url_req: web::Json<CreateUrlRequest>,
) -> Result<HttpResponse> {
    // URL'yi doğrula
    if !validate_url(&url_req.url) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Geçersiz URL formatı"
        })));
    }

    // Benzersiz kısa kod oluştur
    let short_code = generate_unique_code(&db).await;

    // Yeni URL kaydı oluştur
    let url = Url {
        id: None,
        long_url: url_req.url.clone(),
        short_code: short_code.clone(),
        clicks: 0,
        created_at: Utc::now(),
    };

    // Veritabanına kaydet
    let collection = db.collection::<Url>("urls");
    collection.insert_one(url.clone(), None).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    // Base URL'yi al
    let connection_info = req.connection_info();
    let base_url = format!("{}://{}", connection_info.scheme(), connection_info.host());

    // Başarılı yanıt döndür
    let response = CreateUrlResponse {
        short_url: format!("{}/{}", base_url, short_code),
        original_url: url_req.url.clone(),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn redirect_url(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let mut short_code = path.into_inner();
    let is_stats = short_code.ends_with('+');
    
    if is_stats {
        short_code.pop(); // '+' işaretini kaldır
        return Ok(HttpResponse::Ok().content_type("text/html").body(include_str!("../static/stats.html")));
    }

    let collection = db.collection::<Url>("urls");

    // URL'yi bul
    if let Some(mut url) = collection
        .find_one(mongodb::bson::doc! { "short_code": &short_code }, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    {
        // Tıklanma sayısını artır
        url.clicks += 1;
        collection
            .update_one(
                mongodb::bson::doc! { "short_code": &short_code },
                mongodb::bson::doc! { "$set": { "clicks": url.clicks } },
                None,
            )
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // Yönlendir
        Ok(HttpResponse::Found()
            .append_header(("Location", url.long_url))
            .finish())
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "URL bulunamadı"
        })))
    }
}

pub async fn get_url_stats(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let short_code = path.into_inner();
    let collection = db.collection::<Url>("urls");

    if let Some(url) = collection
        .find_one(mongodb::bson::doc! { "short_code": &short_code }, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    {
        Ok(HttpResponse::Ok().json(url))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "URL bulunamadı"
        })))
    }
} 
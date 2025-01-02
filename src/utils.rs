use rand::{thread_rng, Rng};
use mongodb::Database;
use crate::models::Url;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CODE_LENGTH: usize = 6;

pub async fn generate_unique_code(db: &Database) -> String {
    loop {
        let code = generate_random_code();
        
        // Veritabanında bu kodun var olup olmadığını kontrol et
        let collection = db.collection::<Url>("urls");
        let exists = collection
            .find_one(mongodb::bson::doc! { "short_code": &code }, None)
            .await
            .unwrap()
            .is_some();

        if !exists {
            return code;
        }
    }
}

fn generate_random_code() -> String {
    let mut rng = thread_rng();
    (0..CODE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn validate_url(url: &str) -> bool {
    match url::Url::parse(url) {
        Ok(parsed) => parsed.scheme() == "http" || parsed.scheme() == "https",
        Err(_) => false,
    }
} 
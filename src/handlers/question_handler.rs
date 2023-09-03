use actix_web::{ web, HttpResponse };
use crate::utils::cache::{ AppCache, get_from_cache, store_in_cache };

pub async fn get_cached_question(
    cache: web::Data<AppCache>,
    key: web::Path<String>
) -> HttpResponse {
    if let Some(value) = get_from_cache(&cache, &key) {
        return HttpResponse::Ok().body(value);
    }

    // Retrieve the question from the database or other source
    let question = "Sample question data".to_owned();

    // Store the question in the cache
    store_in_cache(&cache, &key, question.clone()).await;

    HttpResponse::Ok().body(question)
}

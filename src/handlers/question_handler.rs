use std::io::Read;
use std::fs::File;
use serde_json::Value;

pub fn read_quiz_data() -> Result<Value, Box<dyn std::error::Error>> {
    // Read the JSON file
    let mut file = File::open("src/assets/quiz_data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON data into a serde_json::Value
    let json_data: Value = serde_json::from_str(&contents)?;

    Ok(json_data)
}

// pub async fn get_cached_question(
//     cache: web::Data<AppCache>,
//     key: web::Path<String>
// ) -> HttpResponse {
//     if let Some(value) = get_from_cache(&cache, &key) {
//         return HttpResponse::Ok().body(value);
//     }

//     // Retrieve the question
//     let question = "Sample question data".to_owned();

//     // Store the question in the cache
//     store_in_cache(&cache, &key, question.clone()).await;

//     HttpResponse::Ok().body(question)
// }

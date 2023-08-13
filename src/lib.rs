// todo: Implement Helper function to create a rate limiter

mod test;

pub mod lib {
    use std::fs::File;
    use std::io::Read;
    use serde_json::Value;
    use serde::{ Deserialize, Serialize };
    use actix_web::{ get, HttpResponse, Result };

    #[derive(Debug, Serialize, Deserialize)]
    pub struct QueryParams {
        pub filter_fields: Option<String>,
        pub limit: Option<usize>,
    }

    pub fn read_quiz_data() -> Result<Value, Box<dyn std::error::Error>> {
        // Read the JSON file
        let mut file = File::open("src/quiz_data.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the JSON data into a serde_json::Value
        let json_data: Value = serde_json::from_str(&contents)?;

        Ok(json_data)
    }

    #[get("/api/questions")]
    pub async fn get_all_questions() -> Result<HttpResponse> {
        // Read the data from the JSON file
        let json_data = match read_quiz_data() {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error reading JSON file: {}", err);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        };

        // Return all questions
        Ok(HttpResponse::Ok().append_header(("Content-Type", "application/json")).json(json_data))
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct QuizQuestion {
        pub id: u32,
        pub question: String,
        pub question_type: String,
        pub answers: Vec<String>,
        pub correct_answer: String,
    }
}

pub use lib::get_all_questions;
pub use lib::read_quiz_data;
pub use lib::QueryParams;

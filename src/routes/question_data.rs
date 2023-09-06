use serde_json::Value;
use crate::models::QuizQuestion;

pub struct QuestionData {
    pub questions: Vec<QuizQuestion>,
}

impl QuestionData {
    pub fn new(json_data: &Value) -> Result<Self, actix_web::Error> {
        let questions = Self::extract_questions(json_data)?;
        Ok(Self { questions })
    }

    pub fn extract_questions(json_data: &Value) -> Result<Vec<QuizQuestion>, actix_web::Error> {
        let mut questions = Vec::new();
        if let Some(categories) = json_data.get("categories").and_then(|c| c.as_object()) {
            for category in categories.values() {
                if let Some(subcategories) = category.as_object() {
                    for subcategory in subcategories.values() {
                        if let Some(questions_array) = subcategory.as_array() {
                            for question in questions_array {
                                if let Ok(quiz_question) = serde_json::from_value::<QuizQuestion>(question.clone()) {
                                    questions.push(quiz_question);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(questions)
    }
}

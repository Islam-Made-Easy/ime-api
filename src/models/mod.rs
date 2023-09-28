use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct QueryParams {
    pub page: Option<usize>, // Page number
    pub limit: Option<usize>,
    pub month: Option<String>,
    pub filter_fields: Option<String>,
    pub items_per_page: Option<usize>, // Items per page
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuizQuestion {
    pub id: u32,
    pub month: String,
    pub question: String,
    pub question_type: String,
    pub answers: Vec<String>,
    pub correct_answer: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct PathParams {
    pub category: String,
    pub subcategory: String,
}

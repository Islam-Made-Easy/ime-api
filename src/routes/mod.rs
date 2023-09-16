mod question_data;

use rand::Rng;
use log::{ info, error };
use actix_web::{ get, HttpResponse, web, Responder };
use crate::{
    models::{ QueryParams, QuizQuestion, PathParams },
    handlers::question_handler::read_quiz_data,
};
use crate::routes::question_data::QuestionData;

#[get("/api/questions")]
pub(crate) async fn get_all_questions(qp: web::Query<QueryParams>) -> impl Responder {
    // Read the data from the JSON file
    let json_data = match read_quiz_data() {
        Ok(data) => data,
        Err(err) => {
            error!("Error reading JSON file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Return all questions
    let limit = qp.limit.unwrap_or(usize::MAX); // Default to no limit
    let page = qp.page.unwrap_or(1);
    let items_per_page = qp.items_per_page.unwrap_or(100); // Default to 100 items per page

    let data = match QuestionData::new(&json_data) {
        Ok(data) => data,
        Err(err) => {
            error!("Error initializing QuestionData: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    info!("Number of Questions: {}", data.questions.len());

    // Perform pagination
    let start_index = (page - 1) * items_per_page;
    let end_index = start_index + items_per_page;

    let paginated_questions = if start_index < data.questions.len() {
        data.questions[start_index..std::cmp::min(end_index, data.questions.len())].to_vec()
    } else {
        Vec::new()
    };

    // Return the limited questions
    if paginated_questions.len() > limit {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&paginated_questions[..limit])
    } else {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&paginated_questions)
    }
}

#[get("/api/questions/{category}/{subcategory}")]
pub(crate) async fn get_questions_by_category(
    path_params: web::Path<PathParams>,
    qp: web::Query<QueryParams>
) -> impl Responder {
    // Read the data from the JSON file
    let json_data = match read_quiz_data() {
        Ok(data) => data,
        Err(err) => {
            error!("Error reading JSON file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Return questions for the specified category and subcategory
    let limit = qp.limit.unwrap_or(usize::MAX); // Default to no limit
    let page = qp.page.unwrap_or(1);
    let items_per_page = qp.items_per_page.unwrap_or(100); // Default to 100 items per page

    let mut questions = Vec::new();

    if let Some(categories) = json_data.get("categories").and_then(|c| c.as_object()) {
        if
            let Some(subcategories) = categories
                .get(&path_params.category)
                .and_then(|s| s.get(&path_params.subcategory))
        {
            if let Some(questions_array) = subcategories.as_array() {
                for question in questions_array {
                    if
                        let Ok(quiz_question) = serde_json::from_value::<QuizQuestion>(
                            question.clone()
                        )
                    {
                        questions.push(quiz_question);
                    }
                }
            }
        }
    }

    info!("Number of Questions: {}", questions.len());

    // Perform pagination
    let start_index = (page - 1) * items_per_page;
    let end_index = start_index + items_per_page;

    let paginated_questions = if start_index < questions.len() {
        questions[start_index..std::cmp::min(end_index, questions.len())].to_vec()
    } else {
        Vec::new()
    };

    // Return the limited questions
    if paginated_questions.len() > limit {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&paginated_questions[..limit])
    } else {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&paginated_questions)
    }
}

#[get("/api/questions/type")]
pub(crate) async fn get_questions_by_type(
    qp: web::Query<QueryParams>,
    web::Query(query_params): web::Query<QueryParams>
) -> impl Responder {
    // Read the data from the JSON file
    let json_data = match read_quiz_data() {
        Ok(data) => data,
        Err(err) => {
            error!("Error reading JSON file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Get the desired question type from query parameters
    let desired_question_type = query_params.filter_fields.unwrap_or_default();

    // Return all questions of the desired type
    let limit = qp.limit.unwrap_or(usize::MAX); // Default to no limit

    let data: QuestionData = match QuestionData::new(&json_data) {
        Ok(data) => data,
        Err(err) => {
            error!("Error initializing QuestionData: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    info!("Number of Questions of Type '{}': {}", desired_question_type, data.questions.len());

    // Return the limited questions
    if data.questions.len() > limit {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&data.questions[..limit])
    } else {
        HttpResponse::Ok().append_header(("Content-Type", "application/json")).json(&data.questions)
    }
}

#[get("/api/questions/filter")]
pub(crate) async fn get_filtered_questions(
    qp: web::Query<QueryParams>,
    web::Query(query_params): web::Query<QueryParams>
) -> impl Responder {
    // Read the data from the JSON file
    let json_data = match read_quiz_data() {
        Ok(data) => data,
        Err(err) => {
            error!("Error reading JSON file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Get query parameters
    let desired_question_type = query_params.filter_fields.unwrap_or_default();
    let limit = qp.limit.unwrap_or(usize::MAX); // Default to no limit

    let mut filtered_questions = Vec::new();

    if let Some(categories) = json_data.get("categories").and_then(|c| c.as_object()) {
        for category in categories.values() {
            if let Some(subcategories) = category.as_object() {
                for subcategory in subcategories.values() {
                    if let Some(questions_array) = subcategory.as_array() {
                        for question in questions_array {
                            if
                                let Ok(quiz_question) = serde_json::from_value::<QuizQuestion>(
                                    question.clone()
                                )
                            {
                                // Filter based on query parameters
                                if quiz_question.question_type == desired_question_type {
                                    filtered_questions.push(quiz_question);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    info!("Number of Filtered Questions: {}", filtered_questions.len());

    // Return the limited filtered questions
    if filtered_questions.len() > limit {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&filtered_questions[..limit])
    } else {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&filtered_questions)
    }
}

#[get("/api/questions/random")]
pub(crate) async fn get_random_questions(qp: web::Query<QueryParams>) -> impl Responder {
    // Read the data from the JSON file
    let json_data = match read_quiz_data() {
        Ok(data) => data,
        Err(err) => {
            error!("Error reading JSON file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Get query parameters
    let limit = qp.limit.unwrap_or(usize::MAX); // Default to no limit

    let mut data: QuestionData = match QuestionData::new(&json_data) {
        Ok(data) => data,
        Err(err) => {
            error!("Error initializing QuestionData: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut rng = rand::thread_rng();
    let mut random_questions = Vec::new();

    // Randomly select questions
    while !data.questions.is_empty() && random_questions.len() < limit {
        let random_index = rng.gen_range(0..data.questions.len());
        random_questions.push(data.questions.remove(random_index));
    }

    info!("Number of Random Questions: {}", random_questions.len());

    // Return the random questions
    HttpResponse::Ok().append_header(("Content-Type", "application/json")).json(&random_questions)
}

#[get("/api/questions/month")]
pub(crate) async fn get_questions_by_month(qp: web::Query<QueryParams>) -> impl Responder {
    // Read the data from the JSON file
    let json_data = match read_quiz_data() {
        Ok(data) => data,
        Err(err) => {
            error!("Error reading JSON file: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Get query parameters
    let limit = qp.limit.unwrap_or(usize::MAX); // Default to no limit
    let page = qp.page.unwrap_or(1);
    let items_per_page = qp.items_per_page.unwrap_or(100); // Default to 100 items per page
    let month_filter = qp.month.clone(); // Get the month filter from query parameters

    let mut questions = Vec::new();

    if let Some(categories) = json_data.get("categories").and_then(|c| c.as_object()) {
        for category in categories.values() {
            if let Some(subcategories) = category.as_object() {
                for subcategory in subcategories.values() {
                    if let Some(questions_array) = subcategory.as_array() {
                        for question in questions_array {
                            if
                                let Ok(quiz_question) = serde_json::from_value::<QuizQuestion>(
                                    question.clone()
                                )
                            {
                                // Check if the event month matches the filter
                                if let Some(ref month) = month_filter {
                                    if quiz_question.month != *month {
                                        continue; // Skip questions with different months
                                    }
                                }
                                questions.push(quiz_question);
                            }
                        }
                    }
                }
            }
        }
    }

    info!("Number of Questions: {}", questions.len());

    // Perform pagination
    let start_index = (page - 1) * items_per_page;
    let end_index = start_index + items_per_page;

    let paginated_questions = if start_index < questions.len() {
        questions[start_index..std::cmp::min(end_index, questions.len())].to_vec()
    } else {
        Vec::new()
    };

    // Return the limited questions
    if paginated_questions.len() > limit {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&paginated_questions[..limit])
    } else {
        HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .json(&paginated_questions)
    }
}

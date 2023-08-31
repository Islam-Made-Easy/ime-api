#[actix_rt::test]
async fn test_get_all_questions() {
    use actix_rt::System;
    use actix_web::{ test, App };

    use crate::routes::get_all_questions;
    System::new().block_on(async {
        let mut app = test::init_service(App::new().service(get_all_questions)).await;

        let req = test::TestRequest::get().uri("/api/questions").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        let expected_response_content =
            r#"
[
    {
        "id": 1,
        "question": "Who was the first President of the United States?",
        "question_type": "multiple-choice",
        "answers": [
            "George Washington",
            "Abraham Lincoln",
            "Thomas Jefferson",
            "John Adams"
        ],
        "correct_answer": "George Washington"
    },
    {
        "id": 2,
        "question": "What is the capital of France?",
        "question_type": "multiple-choice",
        "answers": [
            "Berlin",
            "London",
            "Madrid",
            "Paris"
        ],
        "correct_answer": "Paris"
    }
]
"#;

        // Checks the content of the response body
        assert!(body_str.contains(expected_response_content));
    });
}

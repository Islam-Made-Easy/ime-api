#[actix_rt::test]

async fn test_filtered_questions() {
    use actix_rt::System;
    use actix_web::{test, App};

    use crate::routes::get_filtered_questions;
    System::new().block_on(async {
        let mut app = test::init_service(App::new().service(get_filtered_questions)).await;

        let req = test::TestRequest::get()
            .uri("/api/questions?type=multiple-choice")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        let expected_response_content = r#"
        [
            {
                "id": 1,
                "question": "What is the greatest worship?",
                "question_type": "multiple-choice",
                "answers": [
                    "The prayer",
                    "Zakat",
                    "Unification. -- Tawheed"
                ],
                "correct_answer": "Unification. -- Tawheed"
            },
            {
                "id": 2,
                "question": "Fasting is one of the pillars",
                "question_type": "multiple-choice",
                "answers": [
                    "Islam",
                    "Faith",
                    "Charity",
                    "Taqwah"
                ],
                "correct_answer": "Islam"
            }
        ]
    "#;

        // Checks the content of the response body
        assert!(body_str.contains(expected_response_content));
    });
}

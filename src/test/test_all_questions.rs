#[actix_rt::test]
async fn test_all_questions() {
    use actix_web::{test, App};

    use crate::routes::get_all_questions;

    let mut app = test::init_service(App::new().service(get_all_questions)).await;

    let req = test::TestRequest::get().uri("/api/questions").to_request();
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
                "Islam",
                "Faith",
                "Charity",
                "Taqwah"
            ],
            "correct_answer": "Islam",
            "month": "null"
        },
        {
            "id": 31,
            "question": "What is the surah in which the basmalah was mentioned twice?",
            "question_type": "multiple-choice",
            "answers": [
                "Surah Al-Mulk",
                "Surah Maryam",
                "Surat Al-Naml",
                "Surat Al-Shuara"
            ],
            "correct_answer": "Surat Al-Naml",
            "month": "null"
        },
        {
            "id": 32,
            "question": "In which surah is the story of Taloot and Goliath mentioned?",
            "question_type": "multiple-choice",
            "answers": [
                "the norms",
                "the table",
                "the cow",
                "poets"
            ],
            "correct_answer": "Surat Al-Naml",
            "month": "null"
        },
        
    ]
"#;

    assert_eq!(body_str.trim(), expected_response_content.trim());
}

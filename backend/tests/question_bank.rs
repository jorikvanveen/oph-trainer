use std::collections::{BTreeMap, HashSet};

use oph_trainer_backend::{
    dto::{AnswerSubmissionDto, ExerciseMaterialDto, FeedbackStatusDto, QuizSelectionRequestDto},
    practice::PracticeService,
    question_bank::{AnswerCriteria, QuestionBank},
};
use rand::{SeedableRng, rngs::StdRng};

#[test]
fn questions_support_ten_distinct_questions_and_smaller_topic_quizzes() {
    let practice = PracticeService::new(QuestionBank::questions());
    let catalog = practice.catalog();
    assert_eq!(
        catalog
            .topics
            .iter()
            .map(|topic| topic.topic_id.as_str())
            .collect::<HashSet<_>>(),
        HashSet::from([
            "sources",
            "formats",
            "mp3",
            "transcodes",
            "torrenting",
            "spectral",
            "cd",
            "dupes",
            "rules"
        ])
    );

    let mut rng = StdRng::seed_from_u64(5);
    let quiz = practice
        .select_quiz(
            QuizSelectionRequestDto { topic_ids: None },
            &BTreeMap::new(),
            &mut rng,
        )
        .unwrap();
    assert_eq!(quiz.questions.len(), 10);
    assert_eq!(
        quiz.questions
            .iter()
            .map(|question| &question.question_id)
            .collect::<HashSet<_>>()
            .len(),
        10
    );

    for topic in catalog.topics {
        let quiz = practice
            .select_quiz(
                QuizSelectionRequestDto {
                    topic_ids: Some(vec![topic.topic_id.clone()]),
                },
                &BTreeMap::new(),
                &mut rng,
            )
            .unwrap();
        let pool_size = catalog
            .questions
            .iter()
            .filter(|q| q.topic_id == topic.topic_id)
            .count();
        assert!(pool_size > 0, "{}: empty topic", topic.topic_id);
        assert_eq!(quiz.questions.len(), 10.min(pool_size));
        assert_eq!(
            quiz.questions
                .iter()
                .map(|q| &q.question_id)
                .collect::<HashSet<_>>()
                .len(),
            quiz.questions.len()
        );
        assert!(
            quiz.questions
                .iter()
                .all(|question| question.topic_id == topic.topic_id)
        );
    }
}

#[test]
fn authored_answers_drive_grading_and_official_feedback() {
    let bank = QuestionBank::questions();
    let practice = PracticeService::new(bank.clone());
    for question in bank.questions {
        let answers = match &question.grading {
            AnswerCriteria::ShortAnswer {
                accepted_answers,
                aliases,
            } => accepted_answers
                .iter()
                .chain(aliases)
                .map(|answer| {
                    (
                        format!("  {}\n", answer.to_uppercase()),
                        FeedbackStatusDto::Correct,
                    )
                })
                .chain([("An unauthored answer".into(), FeedbackStatusDto::Incorrect)])
                .collect::<Vec<_>>(),
            AnswerCriteria::Explanation { .. } => vec![(
                "An explanation to assess against the reference".into(),
                FeedbackStatusDto::NeedsSelfAssessment,
            )],
        };
        for (answer, expected) in answers {
            let feedback = practice
                .grade_answer(AnswerSubmissionDto {
                    question_id: question.question_id.clone(),
                    answer,
                })
                .unwrap();
            assert_eq!(feedback.status, expected, "{}", question.question_id);
            assert_eq!(feedback.reference_answer, question.reference_answer);
            assert_eq!(feedback.source_url, question.source_url);
        }
    }

    let feedback = practice
        .grade_answer(AnswerSubmissionDto {
            question_id: "mp3-vbr".into(),
            answer: "variable bitrates".into(),
        })
        .unwrap();
    assert_eq!(feedback.status, FeedbackStatusDto::Incorrect);
}

#[test]
fn questions_have_unique_ids_and_known_topics() {
    let bank = QuestionBank::questions();
    let mut topic_ids = HashSet::new();
    for topic in &bank.topics {
        assert!(!topic.topic_id.trim().is_empty(), "Blank topic ID");
        assert!(
            !topic.name.trim().is_empty(),
            "{}: blank topic name",
            topic.topic_id
        );
        assert!(
            topic_ids.insert(topic.topic_id.as_str()),
            "Duplicate topic ID: {}",
            topic.topic_id
        );
    }

    let mut question_ids = HashSet::new();
    for question in &bank.questions {
        assert!(!question.question_id.trim().is_empty(), "Blank question ID");
        assert!(
            question_ids.insert(question.question_id.as_str()),
            "Duplicate question ID: {}",
            question.question_id
        );
        assert!(
            topic_ids.contains(question.topic_id.as_str()),
            "{}: unknown topic {}",
            question.question_id,
            question.topic_id
        );
    }
}

#[test]
fn questions_have_prompts_references_and_official_sources() {
    for question in QuestionBank::questions().questions {
        for (field, value) in [
            ("prompt", &question.prompt),
            ("reference_answer", &question.reference_answer),
        ] {
            assert!(
                !value.trim().is_empty(),
                "{}: blank {field}",
                question.question_id
            );
        }
        let source = question
            .source_url
            .parse::<axum::http::Uri>()
            .unwrap_or_else(|error| {
                panic!("{}: invalid source URL: {error}", question.question_id)
            });
        assert_eq!(
            source.scheme_str(),
            Some("https"),
            "{}: source must use HTTPS",
            question.question_id
        );
        assert_eq!(
            source.authority().map(|authority| authority.as_str()),
            Some("interview.orpheus.network"),
            "{}: source must be the official syllabus",
            question.question_id
        );
        assert_ne!(
            source.path(),
            "/",
            "{}: link to a relevant syllabus page",
            question.question_id
        );
    }
}

#[test]
fn questions_have_authored_grading_criteria() {
    for question in QuestionBank::questions().questions {
        match &question.grading {
            AnswerCriteria::ShortAnswer {
                accepted_answers,
                aliases,
            } => {
                assert!(
                    !accepted_answers.is_empty(),
                    "{}: accepted_answers must not be empty",
                    question.question_id
                );
                for (field, answers) in
                    [("accepted_answers", accepted_answers), ("aliases", aliases)]
                {
                    assert!(
                        answers.iter().all(|answer| !answer.trim().is_empty()),
                        "{}: blank entry in {field}",
                        question.question_id
                    );
                }
            }
            AnswerCriteria::Explanation {
                required_points,
                optional_points,
                contradictions,
            } => {
                assert!(
                    !required_points.is_empty(),
                    "{}: required_points must not be empty",
                    question.question_id
                );
                for (field, points) in [
                    ("required_points", required_points),
                    ("optional_points", optional_points),
                    ("contradictions", contradictions),
                ] {
                    assert!(
                        points.iter().all(|point| !point.trim().is_empty()),
                        "{}: blank entry in {field}",
                        question.question_id
                    );
                }
            }
        }
    }
}

#[test]
fn exercise_material_is_readable_and_images_are_bundled() {
    let mut image_count = 0;
    let static_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../frontend/static");
    for question in QuestionBank::questions().questions {
        for material in question.material {
            match material {
                ExerciseMaterialDto::Image { src, description } => {
                    image_count += 1;
                    assert!(
                        !description.trim().is_empty(),
                        "{}: missing image description",
                        question.question_id
                    );
                    assert!(
                        src.starts_with("/exercises/") && !src.contains(".."),
                        "{}: image must be a local exercise asset",
                        question.question_id
                    );
                    let bytes = std::fs::read(static_dir.join(src.trim_start_matches('/')))
                        .unwrap_or_else(|error| {
                            panic!("{}: missing asset {src}: {error}", question.question_id)
                        });
                    assert!(!bytes.is_empty(), "{}: empty image", question.question_id);
                }
                ExerciseMaterialDto::Excerpt { description, text } => {
                    assert!(
                        !description.trim().is_empty(),
                        "{}: missing excerpt description",
                        question.question_id
                    );
                    assert!(
                        !text.trim().is_empty() && text.contains('\n'),
                        "{}: excerpt must preserve its lines",
                        question.question_id
                    );
                }
            }
        }
    }
    assert!(image_count > 0, "Spectral exercises are required");
}

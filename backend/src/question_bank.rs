use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::dto::{QuestionDto, TopicDto};

mod questions;
mod syllabus;

#[derive(Debug, Clone, Default)]
pub struct QuestionBank {
    pub topics: Vec<TopicDto>,
    pub questions: Vec<QuestionDto>,
}

impl QuestionBank {
    pub fn questions() -> Self {
        let mut bank = questions::bank();
        let syllabus = syllabus::bank();
        bank.topics.extend(syllabus.topics);
        bank.questions.extend(syllabus.questions);
        bank
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
pub enum AnswerCriteria {
    ShortAnswer {
        accepted_answers: Vec<String>,
        aliases: Vec<String>,
    },

    Explanation {
        required_points: Vec<String>,
        optional_points: Vec<String>,
        contradictions: Vec<String>,
    },
}
